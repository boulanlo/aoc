use std::{
    any::Any,
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use color_eyre::Result;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::{
    runner::{self, RunnersStatus, Selection},
    AdventOfCode,
};

use super::{
    bindings::Keymap,
    minibuffer::{TextInput, TextInputResponse},
    UIAction, Widget, WidgetKind,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Status {
    Finished,
    Running,
    Error,
}

#[derive(Debug, Clone, Copy)]
pub enum TextInputAction {
    RunSelection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ListSelection {
    Day(usize),
    Part(usize, usize),
}

impl From<Selection> for ListSelection {
    fn from(s: Selection) -> Self {
        ListSelection::Part(s.day, s.part)
    }
}

impl From<ListSelection> for Vec<Selection> {
    fn from(s: ListSelection) -> Self {
        match s {
            ListSelection::Day(d) => Selection::day(d),
            ListSelection::Part(d, p) => vec![Selection::part(d, p)],
        }
    }
}

impl ListSelection {
    pub fn day(&self) -> usize {
        match self {
            ListSelection::Day(day) => *day,
            ListSelection::Part(day, _) => *day,
        }
    }
}

impl PartialOrd for ListSelection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ListSelection::Day(d1), ListSelection::Day(d2))
            | (ListSelection::Day(d1), ListSelection::Part(d2, _))
            | (ListSelection::Part(d1, _), ListSelection::Day(d2)) => d1.partial_cmp(d2),
            (ListSelection::Part(d1, p1), ListSelection::Part(d2, p2)) => {
                d1.partial_cmp(d2).and_then(|d| {
                    if let Ordering::Equal = d {
                        p1.partial_cmp(p2)
                    } else {
                        Some(d)
                    }
                })
            }
        }
    }
}

impl Ord for ListSelection {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListSelection::Day(d1), ListSelection::Day(d2))
            | (ListSelection::Day(d1), ListSelection::Part(d2, _))
            | (ListSelection::Part(d1, _), ListSelection::Day(d2)) => d1.cmp(d2),
            (ListSelection::Part(d1, p1), ListSelection::Part(d2, p2)) => {
                let c = d1.cmp(d2);
                if let Ordering::Equal = c {
                    p1.cmp(p2)
                } else {
                    c
                }
            }
        }
    }
}

pub struct ChallengeList {
    list_item_selected: ListState,
    selections: Vec<ListSelection>,
    available_selections: Vec<ListSelection>,
    selected_challenges: HashSet<ListSelection>,
    statuses: HashMap<ListSelection, Status>,
}

impl ChallengeList {
    pub fn new(aoc: &AdventOfCode) -> Self {
        let mut selected = ListState::default();
        selected.select(Some(0));
        Self {
            list_item_selected: selected,
            selections: (1..=25).map(ListSelection::Day).collect(),
            available_selections: aoc
                .challenges
                .iter()
                .enumerate()
                .flat_map(|(i, c)| {
                    if let Some(c) = c.as_ref() {
                        let dataset = c.dataset();

                        if dataset.example_results[1].is_some() {
                            vec![ListSelection::Part(i + 1, 1), ListSelection::Part(i + 1, 2)]
                        } else {
                            vec![ListSelection::Part(i + 1, 1)]
                        }
                    } else {
                        vec![]
                    }
                })
                .collect(),
            selected_challenges: HashSet::default(),
            statuses: HashMap::default(),
        }
    }

    pub fn current_selection(&self) -> Option<ListSelection> {
        self.list_item_selected
            .selected()
            .and_then(|x| self.selections.get(x).copied())
    }

    fn is_day_available(&self, day: usize) -> Option<usize> {
        self.available_selections
            .iter()
            .filter(|s| matches!(s, ListSelection::Part(d, _) if *d == day))
            .map(|s| match s {
                ListSelection::Day(_) => unreachable!(),
                ListSelection::Part(_, part) => *part,
            })
            .max()
    }

    fn is_expanded(&self, day: usize) -> bool {
        self.selections
            .iter()
            .any(|s| matches!(s, ListSelection::Part(d, _) if *d == day))
    }

    fn expand(&mut self, day: usize, up_to: usize) {
        for i in 1..=up_to {
            self.selections.push(ListSelection::Part(day, i));
        }

        self.selections.sort();
    }

    fn collapse(&mut self, day: usize) {
        self.selections
            .retain(|s| s.day() != day || !matches!(s, ListSelection::Part(d, _) if *d == day))
    }

    fn status_of_day(&self, day: usize) -> Option<Status> {
        self.statuses
            .iter()
            .filter(|(k, _)| k.day() == day)
            .max()
            .map(|(_, v)| *v)
    }

    fn list(&self, aoc: &AdventOfCode) -> Vec<ListItem> {
        self.selections
            .iter()
            .map(|selection| {
                let day = selection.day();
                let indent = " ";
                let indicator_present = "● ";
                let indicator_absent = "  ";

                let selected_present = " S ";
                let selected_absent = "   ";

                let challenge = &aoc.challenges[day - 1];

                let spans = if let Some(challenge) = challenge.as_ref() {
                    let name = challenge.name();

                    let status = if matches!(selection, ListSelection::Day(_)) {
                        self.status_of_day(day)
                    } else {
                        self.statuses.get(selection).copied()
                    };

                    let selected_indicator = Span::styled(
                        if self.selected_challenges.contains(selection)
                            || (matches!(selection, ListSelection::Day(_))
                                && self
                                    .selected_challenges
                                    .contains(&ListSelection::Part(selection.day(), 1))
                                && self
                                    .available_selections
                                    .contains(&ListSelection::Part(selection.day(), 2))
                                && self
                                    .selected_challenges
                                    .contains(&ListSelection::Part(selection.day(), 2)))
                        {
                            selected_present
                        } else {
                            selected_absent
                        },
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    );

                    let status_indicator = status
                        .map(|s| {
                            let style = match s {
                                Status::Running => Style::default().fg(Color::Yellow),
                                Status::Finished => Style::default().fg(Color::Green),
                                Status::Error => Style::default()
                                    .fg(Color::Red)
                                    .add_modifier(Modifier::RAPID_BLINK),
                            };

                            Span::styled(format!("{indent}{indicator_present}"), style)
                        })
                        .unwrap_or_else(|| Span::raw(format!("{indent}{indicator_absent}")));

                    match selection {
                        ListSelection::Day(_) => Spans::from(vec![
                            selected_indicator,
                            status_indicator,
                            Span::raw(format!("{day:2} {name}")),
                        ]),
                        ListSelection::Part(_, part) => Spans::from(vec![
                            Span::raw("  "),
                            selected_indicator,
                            status_indicator,
                            Span::raw(format!(" Part {part}")),
                        ]),
                    }
                } else {
                    Spans::from(Span::styled(
                        format!("{indent}{selected_absent}{indicator_absent}{day:2}"),
                        Style::default().fg(Color::DarkGray),
                    ))
                };

                ListItem::new(spans)
            })
            .collect()
    }
}

impl<B: Backend> Widget<B> for ChallengeList {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, aoc: &AdventOfCode, selected: bool)
    where
        B: Backend,
    {
        let list = List::new(self.list(aoc))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(if selected {
                        Style::default().fg(Color::Cyan)
                    } else {
                        Style::default()
                    })
                    .title(<Self as Widget<B>>::title(self, aoc, selected)),
            )
            .highlight_symbol(" > ");

        let mut state = self.list_item_selected.clone();

        f.render_stateful_widget(list, area, &mut state);
    }

    fn kind(&self) -> WidgetKind {
        WidgetKind::ChallengeList
    }

    fn keymap(&self) -> Keymap<'static, dyn Any, Result<UIAction>> {
        Keymap::<dyn Any, _>::default()
            .with_name("Challenge list")
            .add_binding(
                KeyCode::Down,
                |s| {
                    let s: &mut Self = s.downcast_mut().unwrap();
                    s.list_item_selected.select(Some(
                        s.list_item_selected
                            .selected()
                            .map(|x| (x + 1) % s.selections.len())
                            .unwrap_or(0),
                    ));
                    Ok(UIAction::Nothing)
                },
                "Scroll down",
            )
            .add_binding(
                KeyCode::Up,
                |s| {
                    let s: &mut Self = s.downcast_mut().unwrap();
                    s.list_item_selected.select(Some(
                        s.list_item_selected
                            .selected()
                            .map(|x| {
                                if x == 0 {
                                    s.selections.len() - 1
                                } else {
                                    x - 1
                                }
                            })
                            .unwrap_or(s.selections.len() - 1),
                    ));
                    Ok(UIAction::Nothing)
                },
                "Scroll up",
            )
            .add_binding(
                ' ',
                |s| {
                    let s: &mut Self = s.downcast_mut().unwrap();
                    if let Some(selected) = s.list_item_selected.selected() {
                        let current = s.selections[selected];

                        if s.is_expanded(current.day()) {
                            s.collapse(current.day());
                            s.list_item_selected
                                .select(s.selections.iter().position(|s| s.day() == current.day()));
                        } else if let Some(up_to) = s.is_day_available(current.day()) {
                            s.expand(current.day(), up_to)
                        }
                    }
                    Ok(UIAction::Nothing)
                },
                "Expand or collapse challenge",
            )
            .add_binding(
                's',
                |s| {
                    let s: &mut Self = s.downcast_mut().unwrap();

                    let selected = s.current_selection().map(|selection| match selection {
                        ListSelection::Day(d) => {
                            if s.available_selections.contains(&ListSelection::Part(d, 2)) {
                                vec![ListSelection::Part(d, 1), ListSelection::Part(d, 2)]
                            } else {
                                vec![ListSelection::Part(d, 1)]
                            }
                        }
                        ListSelection::Part(_, _) => vec![selection],
                    });

                    if let Some(selected) = selected {
                        for selected in selected {
                            if s.selected_challenges.contains(&selected) {
                                s.selected_challenges.remove(&selected);
                            } else {
                                s.selected_challenges.insert(selected);
                            }
                        }
                    }
                    Ok(UIAction::Nothing)
                },
                "Toggle selecting the currently highlighted challenge",
            )
            .add_binding(
                'u',
                |s| {
                    let s: &mut Self = s.downcast_mut().unwrap();
                    s.selected_challenges.clear();
                    Ok(UIAction::Nothing)
                },
                "De-select all challenges",
            )
            .add_binding(
                'a',
                |s| {
                    let s: &mut Self = s.downcast_mut().unwrap();
                    s.selected_challenges = s.available_selections.iter().copied().collect();
                    Ok(UIAction::Nothing)
                },
                "Select all challenges",
            )
            .add_binding(
                'r',
                |_| {
                    Ok(UIAction::Input(TextInput {
                        prompt: String::from("Run the selected challenge(s): "),
                        origin: WidgetKind::ChallengeList,
                        action: super::TextInputAction::List(TextInputAction::RunSelection),
                        bindings: vec![
                            ('r', String::from("Run and output results")).into(),
                            ('v', String::from("Run and output results, verify against example")).into(),
                            ('a', String::from("Run and output results, verify against example and real results (if present)")).into()
                        ],
                    }))
                },
                "Run selected challenges",
            )
    }

    fn handle_text_input_response(&mut self, response: TextInputResponse) -> Result<UIAction> {
        match response.action {
            super::TextInputAction::List(action) => match action {
                TextInputAction::RunSelection => {
                    let selection = self.selected_challenges.drain().collect();
                    self.statuses.clear();
                    Ok(UIAction::RunChallenges(selection))
                }
            },
        }
    }

    fn as_any(&self) -> &dyn Any {
        self as _
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as _
    }

    fn name(&self, aoc: &AdventOfCode) -> String {
        format!(" {} ", aoc.name)
    }

    fn update(
        &mut self,
        _: Option<ListSelection>,
        runner_status: &RunnersStatus,
        _: &AdventOfCode,
    ) {
        fn kind_from_status(status: &runner::Status) -> Status {
            match status.result {
                None => Status::Running,
                Some(Ok(_)) => Status::Finished,
                Some(Err(_)) => Status::Error,
            }
        }

        for status in runner_status {
            self.statuses
                .entry(ListSelection::Part(
                    status.selection.day,
                    status.selection.part,
                ))
                .and_modify(|s| *s = kind_from_status(status))
                .or_insert_with(|| kind_from_status(status));
        }
    }
}
