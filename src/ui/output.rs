use std::any::Any;

use color_eyre::Result;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::{
    runner::{RunnersStatus, Status},
    AdventOfCode,
};

use super::{bindings::Keymap, list::ListSelection, UIAction, Widget, WidgetKind};

enum ChallengeStatus {
    Day(Vec<Status>),
    Part(Status),
}

#[derive(Default)]
pub struct ChallengeOutput {
    selected_tab: usize,
    selected_day: Option<ListSelection>,
    status: Option<ChallengeStatus>,
}

impl ChallengeOutput {
    fn paragraph(&self) -> Vec<Spans> {
        fn parse_status((idx, status): (usize, &Status)) -> Spans {
            let style_pending = Style::default().fg(Color::Yellow);
            let style_success = Style::default().fg(Color::Green);
            let style_error = Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::RAPID_BLINK);

            match &status.result {
                None => Spans::from(Span::styled(format!("Part {}:", idx + 1), style_pending)),
                Some(Ok(result)) => Spans::from(vec![
                    Span::styled(format!("Part {}: ", idx + 1), style_success),
                    Span::raw(result),
                ]),

                Some(Err(e)) => Spans::from(vec![
                    Span::styled(format!("Part {}: ", idx + 1), style_error),
                    Span::raw(e),
                ]),
            }
        }

        match &self.status {
            None => Vec::new(),
            Some(status) => match status {
                ChallengeStatus::Day(statuses) => match self.selected_tab {
                    0 => statuses.iter().enumerate().map(parse_status).collect(),
                    1 => statuses
                        .iter()
                        .flat_map(|s| s.stdout.iter().map(|l| Spans::from(Span::raw(l))))
                        .collect(),
                    2 => statuses
                        .iter()
                        .flat_map(|s| s.stderr.iter().map(|l| Spans::from(Span::raw(l))))
                        .collect(),
                    _ => unreachable!(),
                },
                ChallengeStatus::Part(status) => match self.selected_tab {
                    0 => vec![parse_status((status.selection.part - 1, status))],
                    1 => status
                        .stdout
                        .iter()
                        .map(|l| Spans::from(Span::raw(l)))
                        .collect(),
                    2 => status
                        .stderr
                        .iter()
                        .map(|l| Spans::from(Span::raw(l)))
                        .collect(),
                    _ => unreachable!(),
                },
            },
        }
    }
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

        let paragraph = Paragraph::new(self.paragraph())
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as _
    }

    fn name(&self, _aoc: &AdventOfCode) -> String {
        String::from(" Challenge results & outputs ")
    }

    fn update(
        &mut self,
        selected_day: Option<ListSelection>,
        runner_status: &RunnersStatus,
        _: &AdventOfCode,
    ) {
        self.status = selected_day.and_then(|day| match day {
            ListSelection::Day(d) => Some(ChallengeStatus::Day(
                runner_status
                    .iter()
                    .cloned()
                    .filter(|s| s.selection.day == d)
                    .collect(),
            )),
            ListSelection::Part(d, p) => runner_status
                .iter()
                .cloned()
                .find(|s| s.selection.day == d && s.selection.part == p)
                .map(ChallengeStatus::Part),
        });

        self.selected_day = selected_day;
    }

    fn keymap(&self) -> Keymap<'static, dyn Any, Result<UIAction>> {
        Keymap::<dyn Any, _>::default()
            .add_binding(
                KeyCode::Tab,
                |s| {
                    let s: &mut Self = s.downcast_mut().unwrap();
                    s.selected_tab = (s.selected_tab + 1) % 3;
                    Ok(UIAction::Nothing)
                },
                "Go to the next output tab",
            )
            .add_binding(
                KeyCode::BackTab,
                |s| {
                    let s: &mut Self = s.downcast_mut().unwrap();
                    if s.selected_tab == 0 {
                        s.selected_tab = 2;
                    } else {
                        s.selected_tab -= 1;
                    }
                    Ok(UIAction::Nothing)
                },
                "Go to the previous output tab",
            )
    }
}
