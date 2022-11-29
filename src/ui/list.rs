use std::{any::Any, collections::HashMap};

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
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

use super::{Widget, WidgetKind};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Status {
    Finished,
    Running,
    Error,
}

pub struct ChallengeList {
    selected: ListState,
    selecting_part: Option<usize>,
    statuses: HashMap<usize, Vec<(usize, Status)>>,
}

impl Default for ChallengeList {
    fn default() -> Self {
        let mut selected = ListState::default();
        selected.select(Some(0));
        Self {
            selected,
            selecting_part: None,
            statuses: HashMap::default(),
        }
    }
}

impl ChallengeList {
    fn list_len(&self) -> usize {
        if self.selecting_part.is_some() {
            27
        } else {
            25
        }
    }

    pub fn current_selection(&self) -> Option<Selection> {
        self.selected.selected().map(|x| {
            if let Some(part_selection) = self.selecting_part {
                if x <= part_selection {
                    Selection::day(x + 1)
                } else if x <= part_selection + 2 {
                    Selection::part(part_selection + 1, x - part_selection)
                } else {
                    Selection::day(x - 1)
                }
            } else {
                Selection::day(x + 1)
            }
        })
    }
}

impl<B: Backend> Widget<B> for ChallengeList {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, aoc: &AdventOfCode, selected: bool)
    where
        B: Backend,
    {
        let challenges = aoc
            .challenges
            .iter()
            .enumerate()
            .flat_map(|(idx, challenge)| {
                let day = idx + 1;

                let style = if challenge.is_some() {
                    Style::default()
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                let status_indicator = self
                    .statuses
                    .get(&day)
                    .map(|s| {
                        let s = *s.iter().map(|(_, s)| s).max().unwrap();

                        let style = match s {
                            Status::Running => Style::default().fg(Color::Yellow),
                            Status::Finished => Style::default().fg(Color::Green),
                            Status::Error => Style::default()
                                .fg(Color::Red)
                                .add_modifier(Modifier::RAPID_BLINK),
                        };

                        Span::styled("● ", style)
                    })
                    .unwrap_or_else(|| Span::raw("  "));

                let day_item = ListItem::new(if let Some(challenge) = challenge.as_ref() {
                    Spans::from(vec![
                        status_indicator,
                        Span::styled(format!("{day:2} "), style),
                        Span::styled(challenge.name(), style),
                    ])
                } else {
                    Spans::from(Span::styled(format!("  {day:2} "), style))
                });

                if let Some(selected_idx) = self.selecting_part {
                    if selected_idx == idx {
                        let status_indicators = self
                            .statuses
                            .get(&day)
                            .map(|s| {
                                s.iter()
                                    .map(|(_, s)| {
                                        let style = match s {
                                            Status::Running => Style::default().fg(Color::Yellow),
                                            Status::Finished => Style::default().fg(Color::Green),
                                            Status::Error => Style::default()
                                                .fg(Color::Red)
                                                .add_modifier(Modifier::RAPID_BLINK),
                                        };

                                        Span::styled("   ● ", style)
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_else(|| vec![Span::raw("  "), Span::raw("  ")]);

                        vec![
                            day_item,
                            ListItem::new(Spans::from(vec![
                                status_indicators[0].clone(),
                                Span::styled(" Part 1", style),
                            ])),
                            ListItem::new(Spans::from(vec![
                                status_indicators[1].clone(),
                                Span::styled(" Part 2", style),
                            ])),
                        ]
                    } else {
                        vec![day_item]
                    }
                } else {
                    vec![day_item]
                }
            })
            .collect::<Vec<_>>();

        let list = List::new(challenges)
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

    fn handle_input(&mut self, input: KeyEvent) -> Result<bool> {
        match input.code {
            KeyCode::Down => self.selected.select(Some(
                self.selected
                    .selected()
                    .map(|x| (x + 1) % self.list_len())
                    .unwrap_or(0),
            )),
            KeyCode::Up => self.selected.select(Some(
                self.selected
                    .selected()
                    .map(|x| if x == 0 { self.list_len() - 1 } else { x - 1 })
                    .unwrap_or(self.list_len() - 1),
            )),
            KeyCode::Char(' ') => {
                let new_selecting_part = if let Some(current) = self.selecting_part {
                    let new = self.selected.selected().map(|x| {
                        if x <= current {
                            x
                        } else {
                            x.saturating_sub((x - current).min(2))
                        }
                    });

                    if let Some(new) = new {
                        if new > current {
                            self.selected
                                .select(self.selected.selected().map(|x| x - 2));
                        }
                    }

                    new
                } else {
                    self.selected.selected()
                };

                if self.selecting_part == new_selecting_part && self.selecting_part.is_some() {
                    self.selecting_part = None;
                } else {
                    self.selecting_part = new_selecting_part;
                }
            }
            KeyCode::Right => {
                self.selecting_part = if let Some(current) = self.selecting_part {
                    let new = self.selected.selected().map(|x| {
                        if x <= current {
                            x
                        } else {
                            x.saturating_sub((x - current).min(2))
                        }
                    });

                    if let Some(new) = new {
                        if new > current {
                            self.selected
                                .select(self.selected.selected().map(|x| x - 2));
                        }
                    }

                    new
                } else {
                    self.selected.selected()
                };
            }
            KeyCode::Left => {
                if let Some(old_selected) = self.selecting_part.take() {
                    if let Some(x) = self.selected.selected() {
                        if x >= self.list_len() {
                            self.selected.select(Some(self.list_len() - 1))
                        }
                        self.selected.select(self.selected.selected().map(|x| {
                            if x <= old_selected {
                                x
                            } else {
                                x.saturating_sub((x - old_selected).min(2))
                            }
                        }));
                    }
                }
            }
            _ => {}
        }

        Ok(false)
    }

    fn as_any(&self) -> &dyn Any {
        self as _
    }

    fn name(&self, aoc: &AdventOfCode) -> String {
        format!(" {} ", aoc.name)
    }

    fn update(&mut self, _: Option<Selection>, runner_status: &RunnersStatus, _: &AdventOfCode) {
        fn kind_from_status(status: &runner::Status) -> Status {
            match status.result {
                None => Status::Running,
                Some(Ok(_)) => Status::Finished,
                Some(Err(_)) => Status::Error,
            }
        }

        for (selection, status) in runner_status {
            self.statuses
                .entry(selection.day)
                .and_modify(|s| {
                    if let Some(part) = selection.part {
                        if let Some((_, s)) = s.iter_mut().find(|(p, _)| *p == part) {
                            *s = kind_from_status(status)
                        } else {
                            s.push((part, kind_from_status(status)))
                        }
                    } else {
                        s.iter_mut()
                            .for_each(|(_, s)| *s = kind_from_status(status))
                    }
                })
                .or_insert_with(|| {
                    if let Some(part) = selection.part {
                        vec![(part, kind_from_status(status))]
                    } else {
                        let s = kind_from_status(status);
                        vec![(1, s), (2, s)]
                    }
                });
        }
    }

    fn on_run_all(&mut self) {
        self.statuses = HashMap::new();
    }
}
