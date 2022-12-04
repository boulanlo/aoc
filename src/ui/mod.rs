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

mod bindings;
mod input;
mod list;
use list::{ChallengeList, ListSelection};

use self::bindings::Keymap;
mod output;
mod popup;
use popup::Popup;

trait Widget<B> {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, aoc: &AdventOfCode, selected: bool)
    where
        B: Backend;

    fn kind(&self) -> WidgetKind;
    #[allow(unused_variables)]
    fn handle_input(&mut self, input: KeyEvent) -> Option<Result<UIAction>> {
        self.keymap().handle_input(self.as_any_mut(), input.code)
    }
    fn keymap(&self) -> Keymap<'static, dyn Any, Result<UIAction>> {
        todo!()
    }
    #[allow(unused_variables)]
    fn update(
        &mut self,
        selected_day: Option<ListSelection>,
        runner_status: &RunnersStatus,
        aoc: &AdventOfCode,
    ) {
    }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UIAction {
    Nothing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum State {
    #[default]
    Normal,
    ShowHelp,
    ShowWidgetHelp(WidgetKind),
}

pub struct UI<B> {
    aoc: AdventOfCode,
    widgets: Vec<Box<dyn Widget<B>>>,
    selected_widget: WidgetKind,
    pool: Pool,
    runner_status: RunnersStatus,
    should_quit: bool,
    state: State,
}

impl<B: Backend> UI<B> {
    pub(crate) fn new(aoc: AdventOfCode) -> Self {
        let list = list::ChallengeList::new(&aoc);
        Self {
            aoc,
            selected_widget: WidgetKind::default(),
            widgets: vec![
                Box::new(list),
                Box::new(input::DatasetInput::default()),
                Box::new(output::ChallengeOutput::default()),
            ],
            pool: Pool::new(4),
            runner_status: RunnersStatus::default(),
            should_quit: false,
            state: State::Normal,
        }
    }

    fn select_widget(&mut self, which: WidgetKind) {
        self.selected_widget = which;
    }

    fn keymap<'a>() -> Keymap<'a, Self, Result<UIAction>> {
        Keymap::<Self, Result<UIAction>>::default()
            .add_binding(
                'q',
                |u| {
                    u.should_quit = true;
                    Ok(UIAction::Nothing)
                },
                "Quit the program",
            )
            .copy_bindings('q', KeyCode::Esc)
            .add_binding(
                'l',
                |u| {
                    u.select_widget(WidgetKind::ChallengeList);
                    Ok(UIAction::Nothing)
                },
                "Navigate to the list of challenges",
            )
            .add_binding(
                'd',
                |u| {
                    u.select_widget(WidgetKind::DatasetInput);
                    Ok(UIAction::Nothing)
                },
                "Navigate to the dataset widget",
            )
            .add_binding(
                'o',
                |u| {
                    u.select_widget(WidgetKind::ChallengeOutput);
                    Ok(UIAction::Nothing)
                },
                "Navigate to the output widget",
            )
            .add_binding(
                'a',
                |u| {
                    u.run_all()?;
                    Ok(UIAction::Nothing)
                },
                "Run all challenges",
            )
            .add_binding(
                KeyCode::Right,
                |u| {
                    u.selected_widget = match u.selected_widget {
                        WidgetKind::ChallengeList | WidgetKind::DatasetInput => {
                            WidgetKind::DatasetInput
                        }
                        WidgetKind::ChallengeOutput => WidgetKind::ChallengeOutput,
                    };
                    Ok(UIAction::Nothing)
                },
                "Navigate to the widget to the right",
            )
            .add_binding(
                KeyCode::Left,
                |u| {
                    u.selected_widget = match u.selected_widget {
                        WidgetKind::ChallengeList
                        | WidgetKind::DatasetInput
                        | WidgetKind::ChallengeOutput => WidgetKind::ChallengeList,
                    };
                    Ok(UIAction::Nothing)
                },
                "Navigate to the widget to the left",
            )
            .add_binding(
                'h',
                |u| {
                    u.state = State::ShowHelp;
                    Ok(UIAction::Nothing)
                },
                "Display this help message",
            )
            .copy_bindings('h', '?')
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

            match self.state {
                State::Normal => {}
                State::ShowHelp => Self::keymap().popup().draw(f, f.size()),
                State::ShowWidgetHelp(_) => todo!(),
            }
        })?;

        Ok(())
    }

    fn get_day_selection(&self) -> Option<ListSelection> {
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
                if self.state != State::Normal {
                    self.state = State::Normal
                } else {
                    let keymap = Self::keymap();
                    if let Some(res) = keymap
                        .handle_input(self, key.code)
                        .or_else(|| self.get_selected_widget().and_then(|w| w.handle_input(key)))
                    {
                        match res? {
                            UIAction::Nothing => {}
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
                .flat_map(|day| {
                    Selection::day(day)
                        .into_iter()
                        .map(|s| self.aoc.task_for(s))
                })
                .flatten()
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
