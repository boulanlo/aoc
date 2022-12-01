use std::any::Any;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use itertools::Itertools;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Tabs},
    Frame,
};

use crate::{runner::RunnersStatus, AdventOfCode, Dataset};

use super::{list::ListSelection, Widget, WidgetKind};

pub struct DatasetInput {
    current_day: Option<ListSelection>,
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
    fn available_tabs(&self) -> Vec<usize> {
        (0..5)
            .filter_map(|idx| match idx {
                0 => Some(idx),
                1 | 2 => self
                    .current_dataset
                    .as_ref()
                    .and_then(|dataset| dataset.example_results[idx - 1].as_ref().map(|_| idx)),
                3 => self
                    .current_dataset
                    .as_ref()
                    .and_then(|dataset| dataset.real_data.as_ref().map(|_| idx)),
                4 | 5 => self
                    .current_dataset
                    .as_ref()
                    .and_then(|dataset| dataset.real_results[idx - 4].as_ref().map(|_| idx)),
                _ => unreachable!(),
            })
            .collect()
    }

    fn titles(&self) -> Vec<Spans> {
        let available_tabs = self.available_tabs();

        let tabs: &[&str] = if self.current_dataset.is_some() {
            &[
                "Example data",
                "Example result (1)",
                "Example result (2)",
                "Real data",
                "Real result (1)",
                "Real result (2)",
            ]
        } else {
            &[]
        };

        tabs.iter()
            .enumerate()
            .map(|(idx, tab)| {
                let style = if available_tabs.contains(&idx) {
                    Style::default()
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                Spans::from(Span::styled(tab.to_owned(), style))
            })
            .collect()
    }

    fn contents(&self) -> Vec<ListItem> {
        if let Some(dataset) = &self.current_dataset {
            match self.selected_data {
                0 => dataset
                    .example_data
                    .iter()
                    .map(|s| ListItem::new(Span::raw(s)))
                    .collect(),
                1 => {
                    if let Some(s) = dataset.example_results[0].as_ref() {
                        vec![ListItem::new(Span::raw(s))]
                    } else {
                        vec![]
                    }
                }
                2 => {
                    if let Some(s) = dataset.example_results[1].as_ref() {
                        vec![ListItem::new(Span::raw(s))]
                    } else {
                        vec![]
                    }
                }
                3 => {
                    if let Some(s) = dataset.real_data.as_ref() {
                        s.iter().map(|s| ListItem::new(Span::raw(s))).collect()
                    } else {
                        vec![]
                    }
                }
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
                3 => x.real_data.as_ref().map(|v| v.len()),
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
        let (before, _, after) = self
            .available_tabs()
            .into_iter()
            .circular_tuple_windows()
            .find(|(_, now, _)| *now == self.selected_data)
            .unwrap_or((0, 0, 0));

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
                if self.current_dataset.is_some() {
                    self.selected_data = after
                }
            }
            KeyCode::BackTab => {
                if self.current_dataset.is_some() {
                    self.selected_data = before
                }
            }
            _ => {}
        }
        Ok(false)
    }

    fn update(
        &mut self,
        selected_day: Option<ListSelection>,
        _: &RunnersStatus,
        aoc: &AdventOfCode,
    ) {
        if self.current_day != selected_day {
            self.current_day = selected_day;
            self.current_dataset = selected_day.and_then(|s| {
                aoc.challenges[s.day() - 1]
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
