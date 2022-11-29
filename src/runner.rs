use color_eyre::Result as EResult;

use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, sync_channel, Receiver, Sender, TryRecvError},
        Arc,
    },
    thread::{spawn, JoinHandle},
};

use crate::Challenge;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Selection {
    pub(crate) day: usize,
    pub(crate) part: Option<usize>,
}

impl Selection {
    pub fn day(day: usize) -> Self {
        Self { day, part: None }
    }

    pub fn part(day: usize, part: usize) -> Self {
        Self {
            day,
            part: Some(part),
        }
    }
}

enum RunnerMessage {
    Task(Task),
    Stop,
}

pub struct Task {
    pub(crate) selection: Selection,
    pub(crate) challenge: Arc<dyn Challenge + Sync + Send>,
}

pub struct Messenger {
    stdout: Sender<String>,
    stderr: Sender<String>,
}

impl Messenger {
    pub fn print(&mut self, msg: String) {
        self.stdout.send(msg).unwrap()
    }

    pub fn println(&mut self, msg: String) {
        self.print(format!("{}\n", msg))
    }

    pub fn eprint(&mut self, msg: String) {
        self.stderr.send(msg).unwrap()
    }

    pub fn eprintln(&mut self, msg: String) {
        self.eprint(format!("{}\n", msg))
    }
}

pub struct MessengerReceiver {
    stdout: Receiver<String>,
    stderr: Receiver<String>,
}

impl MessengerReceiver {
    pub fn receive_stdout(&mut self) -> EResult<Vec<String>> {
        let mut v = Vec::new();

        loop {
            match self.stdout.try_recv() {
                Ok(msg) => v.push(msg),
                Err(TryRecvError::Empty) => break Ok(v),
                Err(e) => break Err(e.into()),
            }
        }
    }

    pub fn receive_stderr(&mut self) -> EResult<Vec<String>> {
        let mut v = Vec::new();

        loop {
            match self.stderr.try_recv() {
                Ok(msg) => v.push(msg),
                Err(TryRecvError::Empty) => break Ok(v),
                Err(e) => break Err(e.into()),
            }
        }
    }
}

fn messenger() -> (Messenger, MessengerReceiver) {
    let (stdout_tx, stdout_rx) = channel();
    let (stderr_tx, stderr_rx) = channel();

    (
        Messenger {
            stdout: stdout_tx,
            stderr: stderr_tx,
        },
        MessengerReceiver {
            stdout: stdout_rx,
            stderr: stderr_rx,
        },
    )
}

#[derive(Default, Clone)]
pub struct Status {
    pub(crate) stdout: Vec<String>,
    pub(crate) stderr: Vec<String>,
    pub(crate) result: Option<Result<Vec<(usize, String)>, String>>,
}

impl Status {
    pub fn merge(&mut self, other: Self) {
        self.stdout.extend(other.stdout.into_iter());
        self.stderr.extend(other.stderr.into_iter());
        if self.result.is_none() {
            self.result = other.result;
        }
    }
}

pub struct Runner {
    thread: Option<JoinHandle<EResult<()>>>,
    task_tx: Sender<RunnerMessage>,
    current_task_rx: Receiver<Selection>,
    current_task: Option<Selection>,
    msg_rx: MessengerReceiver,
    result_rx: Receiver<(Selection, EResult<Vec<(usize, String)>>)>,
}

impl Runner {
    pub fn new() -> Self {
        let (mut messenger, msg_rx) = messenger();
        let (result_tx, result_rx) = sync_channel(0);
        let (task_tx, task_rx) = channel();
        let (current_task_tx, current_task_rx) = channel();

        let thread = spawn(move || loop {
            let task: Task = match task_rx.recv()? {
                RunnerMessage::Stop => return Ok(()),
                RunnerMessage::Task(task) => task,
            };

            current_task_tx.send(task.selection)?;

            let result = task
                .selection
                .part
                .map(|a| match a {
                    1 => task
                        .challenge
                        .part_1_verified(&mut messenger)
                        .map(|p1| vec![(1, p1)]),
                    2 => task
                        .challenge
                        .part_2_verified(&mut messenger)
                        .map(|p2| vec![(2, p2)]),
                    _ => unreachable!(),
                })
                .unwrap_or_else(|| {
                    task.challenge
                        .part_1_verified(&mut messenger)
                        .and_then(|p1| {
                            task.challenge
                                .part_2_verified(&mut messenger)
                                .map(|p2| vec![(1, p1), (2, p2)])
                        })
                });

            result_tx.send((task.selection, result))?;
        });

        Self {
            thread: Some(thread),
            task_tx,
            current_task_rx,
            current_task: None,
            msg_rx,
            result_rx,
        }
    }

    pub fn send_task(&mut self, task: Task) -> EResult<()> {
        self.task_tx.send(RunnerMessage::Task(task))?;
        Ok(())
    }

    pub fn update(&mut self) -> EResult<Option<(Selection, Status)>> {
        let (result_selection, result) = match self.result_rx.try_recv() {
            Ok((selection, res)) => {
                self.current_task = None;
                (Some(selection), Some(res.map_err(|e| e.to_string())))
            }
            Err(TryRecvError::Empty) => (None, None),
            Err(e) => return Err(e.into()),
        };

        match self.current_task_rx.try_recv() {
            Ok(s) => {
                if Some(s) != result_selection {
                    self.current_task = Some(s)
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(e) => return Err(e.into()),
        }

        if result.is_some() {
            Ok(Some((
                result_selection.unwrap(),
                Status {
                    stdout: self.msg_rx.receive_stdout()?,
                    stderr: self.msg_rx.receive_stderr()?,
                    result,
                },
            )))
        } else if let Some(selection) = self.current_task {
            Ok(Some((
                selection,
                Status {
                    stdout: self.msg_rx.receive_stdout()?,
                    stderr: self.msg_rx.receive_stderr()?,
                    result,
                },
            )))
        } else {
            Ok(None)
        }
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Runner {
    fn drop(&mut self) {
        self.task_tx.send(RunnerMessage::Stop).unwrap();
        self.thread.take().unwrap().join().unwrap().unwrap();
    }
}

pub type RunnersStatus = HashMap<Selection, Status>;

pub struct Pool {
    runners: Vec<Runner>,
    next_runner: usize,
}

impl Pool {
    pub fn new(runners: usize) -> Self {
        assert!(runners != 0);
        Self {
            runners: (0..runners).map(|_| Runner::new()).collect(),
            next_runner: 0,
        }
    }

    pub fn send_task(&mut self, task: Task) -> EResult<()> {
        self.runners
            .get_mut(self.next_runner)
            .unwrap()
            .send_task(task)?;
        self.next_runner = (self.next_runner + 1) % self.runners.len();

        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        self.runners.iter().all(|r| r.current_task.is_none())
    }

    pub fn update(&mut self, current: &mut RunnersStatus) -> EResult<()> {
        for runner in self.runners.iter_mut() {
            if let Some((selection, result)) = runner.update()? {
                if let Some(r) = current.get_mut(&selection) {
                    r.merge(result);
                } else {
                    current.insert(selection, result);
                }
            }
        }

        Ok(())
    }
}
