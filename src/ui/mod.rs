use std::{
    any::Any,
    collections::HashMap,
    io::{self, Stdout},
    time::{Duration, Instant},
};

use color_eyre::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
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
use bindings::Keymap;
mod input;
mod list;
use list::{ChallengeList, ListSelection};
mod minibuffer;
mod output;
mod popup;
use popup::Popup;

use self::minibuffer::{TextInput, TextInputResponse};

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
    #[allow(unused_variables)]
    fn handle_text_input_response(&mut self, response: TextInputResponse) -> Result<UIAction> {
        Ok(UIAction::Nothing)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
pub enum WidgetKind {
    #[default]
    ChallengeList,
    DatasetInput,
    ChallengeOutput,
    Minibuffer,
}

#[derive(Clone, Debug)]
enum UIAction {
    Nothing,
    Input(TextInput),
    RunChallenges(Vec<ListSelection>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum State {
    #[default]
    Normal,
    ShowHelp,
}

#[derive(Debug, Clone, Copy)]
pub enum TextInputAction {
    List(list::TextInputAction),
}

pub struct UI<B> {
    aoc: AdventOfCode,
    widgets: Vec<Box<dyn Widget<B>>>,
    selected_widget: Option<WidgetKind>,
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
            selected_widget: None,
            widgets: vec![
                Box::new(list),
                Box::new(input::DatasetInput::default()),
                Box::new(output::ChallengeOutput::default()),
                Box::new(minibuffer::Minibuffer::default()),
            ],
            pool: Pool::new(4),
            runner_status: RunnersStatus::default(),
            should_quit: false,
            state: State::Normal,
        }
    }

    fn select_widget(&mut self, which: WidgetKind) {
        self.selected_widget = Some(which);
    }

    fn get_widget(&mut self, which: WidgetKind) -> &mut Box<dyn Widget<B>> {
        self.widgets.iter_mut().find(|w| w.kind() == which).unwrap()
    }

    fn keymap<'a>() -> Keymap<'a, Self, Result<UIAction>> {
        Keymap::<Self, Result<UIAction>>::default()
            .with_name("Global mode")
            .add_binding(
                'q',
                |u| {
                    if u.selected_widget.is_some() {
                        u.selected_widget = None;
                    } else {
                        u.should_quit = true;
                    }
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
                'h',
                |u| {
                    u.state = State::ShowHelp;
                    Ok(UIAction::Nothing)
                },
                "Display this help message",
            )
            .copy_bindings('h', '?')
    }

    fn keymap_text_input<'a>() -> Keymap<'a, Self, Result<UIAction>> {
        Keymap::<Self, _>::new()
            .with_name("Input buffer")
            .add_binding(
                ('h', KeyModifiers::CONTROL),
                |u| {
                    u.state = State::ShowHelp;
                    Ok(UIAction::Nothing)
                },
                "Show this help menu",
            )
            .add_binding(
                KeyCode::Esc,
                |u| {
                    if let Some(previously_selected) = u
                        .get_widget(WidgetKind::Minibuffer)
                        .as_any_mut()
                        .downcast_mut::<minibuffer::Minibuffer>()
                        .unwrap()
                        .clear()
                    {
                        u.select_widget(previously_selected);
                    }
                    Ok(UIAction::Nothing)
                },
                "Cancel text input",
            )
            .add_binding(
                KeyCode::Backspace,
                |u| {
                    let minibuffer = u
                        .get_widget(WidgetKind::Minibuffer)
                        .as_any_mut()
                        .downcast_mut::<minibuffer::Minibuffer>()
                        .unwrap();
                    minibuffer.backspace();

                    Ok(UIAction::Nothing)
                },
                "Erase a character from the text input",
            )
            .add_binding(
                KeyCode::Enter,
                |u| {
                    let minibuffer = u
                        .get_widget(WidgetKind::Minibuffer)
                        .as_any_mut()
                        .downcast_mut::<minibuffer::Minibuffer>()
                        .unwrap();

                    if let Some(response) = minibuffer.finish() {
                        u.handle_input_response(response)
                    } else {
                        Ok(UIAction::Nothing)
                    }
                },
                "Finish text input",
            )
            .add_binding(
                KeyCode::Left,
                |u| {
                    let minibuffer = u
                        .get_widget(WidgetKind::Minibuffer)
                        .as_any_mut()
                        .downcast_mut::<minibuffer::Minibuffer>()
                        .unwrap();
                    minibuffer.move_cursor_left();
                    Ok(UIAction::Nothing)
                },
                "Move cursor left",
            )
            .add_binding(
                KeyCode::Right,
                |u| {
                    let minibuffer = u
                        .get_widget(WidgetKind::Minibuffer)
                        .as_any_mut()
                        .downcast_mut::<minibuffer::Minibuffer>()
                        .unwrap();
                    minibuffer.move_cursor_right();
                    Ok(UIAction::Nothing)
                },
                "Move cursor right",
            )
    }

    fn layout(&self, area: Rect) -> HashMap<WidgetKind, Rect> {
        let main_mini = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(1)])
            .split(area);

        let left_right = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(main_mini[0]);

        let right_top_bottom = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(70)])
            .split(left_right[1]);

        [
            (WidgetKind::ChallengeList, left_right[0]),
            (WidgetKind::DatasetInput, right_top_bottom[0]),
            (WidgetKind::ChallengeOutput, right_top_bottom[1]),
            (WidgetKind::Minibuffer, main_mini[1]),
        ]
        .into_iter()
        .collect()
    }

    fn draw(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        terminal.draw(|f| {
            let layout = self.layout(f.size());

            for w in self.widgets.iter_mut() {
                let selected = self.selected_widget.map(|k| k == w.kind()).unwrap_or(false);
                w.draw(f, *layout.get(&w.kind()).unwrap(), &self.aoc, selected);
            }

            match self.state {
                State::Normal => {}
                State::ShowHelp => {
                    if let Some(WidgetKind::Minibuffer) = self.selected_widget {
                        let mut popup = Self::keymap_text_input().popup();
                        let minibuffer = self
                            .get_widget(WidgetKind::Minibuffer)
                            .as_any_mut()
                            .downcast_mut::<minibuffer::Minibuffer>()
                            .unwrap();

                        if let Some(bindings) = minibuffer.bindings() {
                            popup.add_bindings(bindings);
                        }

                        popup.draw(f, f.size())
                    } else {
                        self.get_selected_widget()
                            .map(|w| w.keymap().popup())
                            .unwrap_or_else(|| Self::keymap().popup())
                            .draw(f, f.size())
                    }
                }
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
            .find(|w| self.selected_widget.map(|k| k == w.kind()).unwrap_or(false))
    }

    fn handle_action(&mut self, action: UIAction) -> Result<()> {
        match action {
            UIAction::Nothing => {}
            UIAction::Input(input) => {
                self.select_widget(WidgetKind::Minibuffer);
                let minibuffer = self
                    .get_widget(WidgetKind::Minibuffer)
                    .as_any_mut()
                    .downcast_mut::<minibuffer::Minibuffer>()
                    .unwrap();
                minibuffer.start(input);
            }
            UIAction::RunChallenges(selections) => {
                for s in selections {
                    self.run_challenge(s)?;
                }
            }
        }
        Ok(())
    }

    fn handle_input_response(&mut self, response: TextInputResponse) -> Result<UIAction> {
        self.select_widget(response.origin);
        let widget = self.get_widget(response.origin);
        widget.handle_text_input_response(response)
    }

    fn handle_input(&mut self) -> Result<()> {
        while event::poll(Duration::from_secs(0))? {
            if let Event::Key(key) = event::read()? {
                if self.state != State::Normal {
                    self.state = State::Normal
                } else if let Some(WidgetKind::Minibuffer) = self.selected_widget {
                    let global_input_keymap = Self::keymap_text_input();

                    if let Some(res) = global_input_keymap.handle_input(self, key) {
                        self.handle_action(res?)?
                    } else {
                        let minibuffer = self
                            .get_widget(WidgetKind::Minibuffer)
                            .as_any_mut()
                            .downcast_mut::<minibuffer::Minibuffer>()
                            .unwrap();
                        if let KeyCode::Char(c) = key.code {
                            if let Some(response) = minibuffer.push(c) {
                                let action = self.handle_input_response(response)?;
                                self.handle_action(action)?;
                            }
                        }
                    }
                } else {
                    let keymap = Self::keymap();
                    if let Some(res) = keymap
                        .handle_input(self, key)
                        .or_else(|| self.get_selected_widget().and_then(|w| w.handle_input(key)))
                    {
                        self.handle_action(res?)?
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

    fn run_challenge(&mut self, selection: ListSelection) -> Result<()> {
        for selection in Vec::<Selection>::from(selection) {
            self.runner_status.retain(|s| s.selection != selection);
            if let Some(task) = self.aoc.task_for(selection) {
                self.pool.send_task(task)?
            };
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

                std::thread::sleep(framerate)
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
