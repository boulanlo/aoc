use std::{
    any::Any,
    collections::HashMap,
    io::{self, Stdout},
    time::{Duration, Instant},
};

use color_eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    Frame, Terminal,
};

use crate::{
    runner::{Pool, RunnersStatus, Selection},
    AdventOfCode,
};

use self::list::ChallengeList;

mod input;
mod list;
mod output;

trait Widget<B> {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, aoc: &AdventOfCode, selected: bool)
    where
        B: Backend;

    fn kind(&self) -> WidgetKind;
    #[allow(unused_variables)]
    fn handle_input(&mut self, input: KeyEvent) -> Result<bool> {
        Ok(false)
    }
    #[allow(unused_variables)]
    fn update(
        &mut self,
        selected_day: Option<Selection>,
        runner_status: &RunnersStatus,
        aoc: &AdventOfCode,
    ) {
    }
    fn as_any(&self) -> &dyn Any;
    fn name(&self, aoc: &AdventOfCode) -> String;
    fn title(&self, aoc: &AdventOfCode, selected: bool) -> Span {
        Span::styled(
            self.name(aoc),
            if selected {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            },
        )
    }
    fn on_run_all(&mut self) {}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
enum WidgetKind {
    #[default]
    ChallengeList,
    DatasetInput,
    ChallengeOutput,
}

pub struct UI<B> {
    aoc: AdventOfCode,
    widgets: Vec<Box<dyn Widget<B>>>,
    selected_widget: WidgetKind,
    pool: Pool,
    runner_status: RunnersStatus,
    should_quit: bool,
}

impl<B: Backend> UI<B> {
    pub(crate) fn new(aoc: AdventOfCode) -> Self {
        Self {
            aoc,
            selected_widget: WidgetKind::default(),
            widgets: vec![
                Box::new(list::ChallengeList::default()),
                Box::new(input::DatasetInput::default()),
                Box::new(output::ChallengeOutput::default()),
            ],
            pool: Pool::new(4),
            runner_status: RunnersStatus::default(),
            should_quit: false,
        }
    }

    fn layout(&self, area: Rect) -> HashMap<WidgetKind, Rect> {
        let left_right = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        let right_top_bottom = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(70)])
            .split(left_right[1]);

        [
            (WidgetKind::ChallengeList, left_right[0]),
            (WidgetKind::DatasetInput, right_top_bottom[0]),
            (WidgetKind::ChallengeOutput, right_top_bottom[1]),
        ]
        .into_iter()
        .collect()
    }

    fn draw(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        terminal.draw(|f| {
            let layout = self.layout(f.size());

            for w in self.widgets.iter_mut() {
                let selected = w.kind() == self.selected_widget;
                w.draw(f, *layout.get(&w.kind()).unwrap(), &self.aoc, selected);
            }
        })?;

        Ok(())
    }

    fn get_day_selection(&self) -> Option<Selection> {
        self.widgets
            .iter()
            .find_map(|w| {
                if w.kind() == WidgetKind::ChallengeList {
                    Some(w.as_ref())
                } else {
                    None
                }
            })
            .unwrap()
            .as_any()
            .downcast_ref::<ChallengeList>()
            .unwrap()
            .current_selection()
    }

    fn get_selected_widget(&mut self) -> Option<&mut Box<dyn Widget<B>>> {
        self.widgets
            .iter_mut()
            .find(|w| w.kind() == self.selected_widget)
    }

    fn handle_input(&mut self) -> Result<()> {
        while event::poll(Duration::from_secs(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                    KeyCode::Char('l') => self.selected_widget = WidgetKind::ChallengeList,
                    KeyCode::Char('d') => self.selected_widget = WidgetKind::DatasetInput,
                    KeyCode::Char('o') => self.selected_widget = WidgetKind::ChallengeOutput,
                    KeyCode::Char('a') => self.run_all()?,
                    KeyCode::Right => {
                        self.selected_widget = match self.selected_widget {
                            WidgetKind::ChallengeList | WidgetKind::DatasetInput => {
                                WidgetKind::DatasetInput
                            }
                            WidgetKind::ChallengeOutput => WidgetKind::ChallengeOutput,
                        }
                    }
                    KeyCode::Left => {
                        self.selected_widget = match self.selected_widget {
                            WidgetKind::ChallengeList
                            | WidgetKind::DatasetInput
                            | WidgetKind::ChallengeOutput => WidgetKind::ChallengeList,
                        }
                    }

                    _ => {
                        if let Some(widget) = self.get_selected_widget() {
                            if widget.handle_input(key)? {
                                self.should_quit = true;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn update(&mut self) {
        let day_selection = self.get_day_selection();
        self.pool.update(&mut self.runner_status).unwrap();

        for w in self.widgets.iter_mut() {
            w.update(day_selection, &self.runner_status, &self.aoc)
        }
    }

    fn run_all(&mut self) -> Result<()> {
        if self.pool.is_finished() {
            for w in self.widgets.iter_mut() {
                w.on_run_all();
            }

            self.runner_status.clear();

            for task in self
                .aoc
                .available_challenges()
                .into_iter()
                .map(|day| self.aoc.task_for(Selection::day(day)).unwrap())
            {
                self.pool.send_task(task)?;
            }
        }

        Ok(())
    }
}

impl UI<CrosstermBackend<Stdout>> {
    pub fn run(mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut last_draw = Instant::now();
        let framerate = Duration::from_millis(16);

        while !self.should_quit {
            if last_draw.elapsed() >= framerate {
                last_draw = Instant::now();
                self.handle_input()?;
                self.update();
                self.draw(&mut terminal)?;
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }
}
