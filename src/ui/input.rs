use std::any::Any;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Tabs},
    Frame,
};

use crate::{
    runner::{RunnersStatus, Selection},
    AdventOfCode, Dataset,
};

use super::{Widget, WidgetKind};

pub struct DatasetInput {
    current_day: Option<Selection>,
    current_dataset: Option<Dataset>,
    selected_data: usize,
    list_scroll: ListState,
}

impl Default for DatasetInput {
    fn default() -> Self {
        let mut list_scroll = ListState::default();
        list_scroll.select(Some(0));

        Self {
            current_day: Default::default(),
            current_dataset: Default::default(),
            selected_data: Default::default(),
            list_scroll,
        }
    }
}

impl DatasetInput {
    fn titles(&self) -> Vec<Spans> {
        if let Some(dataset) = &self.current_dataset {
            vec![
                Spans::from(Span::raw("Example data")),
                Spans::from(Span::raw("Example result (1)")),
                Spans::from(Span::raw("Example result (2)")),
                Spans::from(Span::raw("Real data")),
                Spans::from(Span::styled(
                    "Real result (1)",
                    if dataset.real_results[0].is_some() {
                        Style::default()
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                )),
                Spans::from(Span::styled(
                    "Real result (2)",
                    if dataset.real_results[1].is_some() {
                        Style::default()
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                )),
            ]
        } else {
            Vec::new()
        }
    }

    fn contents(&self) -> Vec<ListItem> {
        if let Some(dataset) = &self.current_dataset {
            match self.selected_data {
                0 => dataset
                    .example_data
                    .iter()
                    .map(|s| ListItem::new(Span::raw(s)))
                    .collect(),
                1 => vec![ListItem::new(Span::raw(&dataset.example_results[0]))],
                2 => vec![ListItem::new(Span::raw(&dataset.example_results[1]))],
                3 => dataset
                    .real_data
                    .iter()
                    .map(|s| ListItem::new(Span::raw(s)))
                    .collect(),
                4 => {
                    if let Some(s) = dataset.real_results[0].as_ref() {
                        vec![ListItem::new(Span::raw(s))]
                    } else {
                        vec![]
                    }
                }
                5 => {
                    if let Some(s) = dataset.real_results[0].as_ref() {
                        vec![ListItem::new(Span::raw(s))]
                    } else {
                        vec![]
                    }
                }
                _ => unreachable!(),
            }
        } else {
            vec![]
        }
    }

    fn current_contents_len(&self) -> Option<usize> {
        self.current_dataset
            .as_ref()
            .and_then(|x| match self.selected_data {
                0 => Some(x.example_data.len()),
                1 | 2 => Some(1),
                3 => Some(x.real_data.len()),
                4 => x.real_results[0].as_ref().map(|_| 1),
                5 => x.real_results[1].as_ref().map(|_| 1),
                _ => unreachable!(),
            })
    }
}

impl<B: Backend> Widget<B> for DatasetInput {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, aoc: &AdventOfCode, selected: bool)
    where
        B: tui::backend::Backend,
    {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(3)])
            .split(area);

        let tabs = Tabs::new(self.titles())
            .block(
                Block::default()
                    .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
                    .title(<Self as Widget<B>>::title(self, aoc, selected)),
            )
            .highlight_style(Style::default().fg(Color::Cyan))
            .select(self.selected_data);

        let list = List::new(self.contents())
            .block(Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT))
            .highlight_symbol("> ");

        let mut state = self.list_scroll.clone();

        f.render_widget(tabs, areas[0]);
        f.render_stateful_widget(list, areas[1], &mut state);
    }

    fn kind(&self) -> WidgetKind {
        WidgetKind::DatasetInput
    }

    fn as_any(&self) -> &dyn Any {
        self as _
    }

    fn handle_input(&mut self, input: KeyEvent) -> Result<bool> {
        match input.code {
            KeyCode::Up => {
                if self.current_dataset.is_some() {
                    self.list_scroll
                        .select(self.current_contents_len().map(|len| {
                            self.list_scroll
                                .selected()
                                .map(|x| if x == 0 { len - 1 } else { x - 1 })
                                .unwrap_or(0)
                        }))
                }
            }
            KeyCode::Down => {
                if self.current_dataset.is_some() {
                    self.list_scroll
                        .select(self.current_contents_len().map(|len| {
                            self.list_scroll
                                .selected()
                                .map(|x| (x + 1) % len)
                                .unwrap_or(0)
                        }))
                }
            }
            KeyCode::Tab => {
                if let Some(dataset) = self.current_dataset.as_ref() {
                    self.selected_data = match self.selected_data {
                        0 => 1,
                        1 => 2,
                        2 => 3,
                        3 => {
                            if dataset.real_results[0].is_some() {
                                4
                            } else {
                                0
                            }
                        }
                        4 => {
                            if dataset.real_results[1].is_some() {
                                5
                            } else {
                                0
                            }
                        }
                        5 => 0,
                        _ => unreachable!(),
                    }
                }
            }
            KeyCode::BackTab => {
                if let Some(dataset) = self.current_dataset.as_ref() {
                    self.selected_data = match self.selected_data {
                        0 => {
                            if dataset.real_results[1].is_some() {
                                5
                            } else if dataset.real_results[0].is_some() {
                                4
                            } else {
                                3
                            }
                        }
                        1 => 0,
                        2 => 1,
                        3 => 2,
                        4 => 3,
                        5 => 4,
                        _ => unreachable!(),
                    }
                }
            }
            _ => {}
        }
        Ok(false)
    }

    fn update(&mut self, selected_day: Option<Selection>, _: &RunnersStatus, aoc: &AdventOfCode) {
        if self.current_day != selected_day {
            self.current_day = selected_day;
            self.current_dataset = selected_day.and_then(|s| {
                aoc.challenges[s.day - 1]
                    .as_ref()
                    .map(|c| c.dataset().clone())
            });
            self.list_scroll.select(Some(0));
            self.selected_data = 0;
        }
    }

    fn name(&self, _aoc: &AdventOfCode) -> String {
        String::from(" Dataset ")
    }
}
