use std::any::Any;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::{
    runner::{RunnersStatus, Selection, Status},
    AdventOfCode,
};

use super::{Widget, WidgetKind};

#[derive(Default)]
pub struct ChallengeOutput {
    selected_tab: usize,
    selected_day: Option<Selection>,
    status: Option<Status>,
}

impl<B: Backend> Widget<B> for ChallengeOutput {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, aoc: &AdventOfCode, selected: bool)
    where
        B: Backend,
    {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(3)])
            .split(area);

        let tabs = Tabs::new(vec![
            Spans::from(Span::raw("Result")),
            Spans::from(Span::raw("Output")),
            Spans::from(Span::raw("Error output")),
        ])
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
                .title(<Self as Widget<B>>::title(self, aoc, selected)),
        )
        .highlight_style(Style::default().fg(Color::Cyan))
        .select(self.selected_tab);

        let success_style = Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD);

        let contents = self
            .status
            .as_ref()
            .map(|status| match self.selected_tab {
                0 => match &status.result {
                    None => vec![Spans::from(Span::raw(""))],
                    Some(Ok(result)) => {
                        if let Some(part) = self.selected_day.as_ref().unwrap().part {
                            let result = result.iter().find(|(p, _)| *p == part).unwrap().1.clone();

                            vec![Spans::from(vec![
                                Span::styled("Result: ", success_style),
                                Span::raw(result),
                            ])]
                        } else {
                            let part_1 = &result[0];
                            let part_2 = &result[1];

                            debug_assert_eq!(part_1.0, 1);
                            debug_assert_eq!(part_2.0, 2);

                            vec![
                                Spans::from(vec![
                                    Span::styled("Part 1: ", success_style),
                                    Span::raw(part_1.1.clone()),
                                ]),
                                Spans::from(vec![
                                    Span::styled("Part 2: ", success_style),
                                    Span::raw(part_2.1.clone()),
                                ]),
                            ]
                        }
                    }
                    Some(Err(error)) => vec![Spans::from(vec![
                        Span::styled(
                            "Error: ",
                            Style::default()
                                .fg(Color::Red)
                                .add_modifier(Modifier::BOLD)
                                .add_modifier(Modifier::RAPID_BLINK),
                        ),
                        Span::raw(error.clone()),
                    ])],
                },
                1 => status
                    .stdout
                    .iter()
                    .map(|s| Spans::from(Span::raw(s.clone())))
                    .collect(),
                2 => status
                    .stderr
                    .iter()
                    .map(|s| Spans::from(Span::raw(s.clone())))
                    .collect(),
                _ => unreachable!(),
            })
            .unwrap_or_default();

        let paragraph = Paragraph::new(contents)
            .block(Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT));

        f.render_widget(tabs, areas[0]);
        f.render_widget(paragraph, areas[1]);
    }

    fn kind(&self) -> WidgetKind {
        WidgetKind::ChallengeOutput
    }

    fn as_any(&self) -> &dyn Any {
        self as _
    }

    fn name(&self, _aoc: &AdventOfCode) -> String {
        String::from(" Challenge results & outputs ")
    }

    fn update(
        &mut self,
        selected_day: Option<Selection>,
        runner_status: &RunnersStatus,
        _: &AdventOfCode,
    ) {
        if let Some(day) = selected_day {
            if let Some(status) = runner_status.get(&day) {
                self.status = Some(status.clone());
            }
        }
        self.selected_day = selected_day;
    }

    fn handle_input(&mut self, input: KeyEvent) -> Result<bool> {
        match input.code {
            KeyCode::Tab => self.selected_tab = (self.selected_tab + 1) % 3,
            KeyCode::BackTab => {
                if self.selected_tab == 0 {
                    self.selected_tab = 2;
                } else {
                    self.selected_tab -= 1;
                }
            }
            _ => {}
        }
        Ok(false)
    }
}
