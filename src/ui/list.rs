use std::{any::Any, cmp::Ordering, collections::HashMap};

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
    runner::{self, RunnersStatus},
    AdventOfCode,
};

use super::{bindings::Keymap, UIAction, Widget, WidgetKind};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Status {
    Finished,
    Running,
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ListSelection {
    Day(usize),
    Part(usize, usize),
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
    selected: ListState,
    selections: Vec<ListSelection>,
    available_selections: Vec<ListSelection>,
    statuses: HashMap<ListSelection, Status>,
}

impl ChallengeList {
    pub fn new(aoc: &AdventOfCode) -> Self {
        let mut selected = ListState::default();
        selected.select(Some(0));
        Self {
            selected,
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
            statuses: HashMap::default(),
        }
    }

    pub fn current_selection(&self) -> Option<ListSelection> {
        self.selected
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
                let indicator_present = "●   ";
                let indicator_absent = "    ";

                let challenge = &aoc.challenges[day - 1];

                let spans = if let Some(challenge) = challenge.as_ref() {
                    let name = challenge.name();

                    let status = if matches!(selection, ListSelection::Day(_)) {
                        self.status_of_day(day)
                    } else {
                        self.statuses.get(selection).copied()
                    };

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
                            status_indicator,
                            Span::raw(format!("{day:2} {name}")),
                        ]),
                        ListSelection::Part(_, part) => Spans::from(vec![
                            Span::raw("  "),
                            status_indicator,
                            Span::raw(format!(" Part {part}")),
                        ]),
                    }
                } else {
                    Spans::from(Span::styled(
                        format!("{indent}{indicator_absent}{day:2}"),
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
                    .title(<Self as Widget<B>>::title(self, aoc, selected)),
            )
            .highlight_symbol(" > ");

        let mut state = self.selected.clone();

        f.render_stateful_widget(list, area, &mut state);
    }

    fn kind(&self) -> WidgetKind {
        WidgetKind::ChallengeList
    }

    fn keymap(&self) -> Keymap<'static, dyn Any, Result<UIAction>> {
        Keymap::<dyn Any, _>::default()
            .add_binding(
                KeyCode::Down,
                |s| {
                    let s: &mut Self = s.downcast_mut().unwrap();
                    s.selected.select(Some(
                        s.selected
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
                    s.selected.select(Some(
                        s.selected
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
                    if let Some(selected) = s.selected.selected() {
                        let current = s.selections[selected];

                        if s.is_expanded(current.day()) {
                            s.collapse(current.day());
                            s.selected
                                .select(s.selections.iter().position(|s| s.day() == current.day()));
                        } else if let Some(up_to) = s.is_day_available(current.day()) {
                            s.expand(current.day(), up_to)
                        }
                    }
                    Ok(UIAction::Nothing)
                },
                "Expand or collapse challenge",
            )
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

    fn on_run_all(&mut self) {
        self.statuses = HashMap::new();
    }
}
