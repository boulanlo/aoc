use color_eyre::Result;
use crossterm::event::KeyEvent;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::AdventOfCode;

use super::{input::DatasetInput, list::ChallengeList, output::ChallengeOutput, UIAction};

pub enum Widget {
    ChallengeList(ChallengeList),
    DatasetInput(DatasetInput),
    ChallengeOutput(ChallengeOutput),
}

struct WidgetLayout {
    challenge_list: Rect,
    dataset_input: Rect,
    challenge_output: Rect,
}

impl Widget {
    pub fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect, aoc: &AdventOfCode, selected: bool)
    where
        B: Backend,
    {
        let layout = Self::layout(area);
        match self {
            Widget::ChallengeList(w) => w.draw(f, layout.challenge_list, aoc, selected),
            Widget::DatasetInput(w) => w.draw(f, layout.dataset_input, aoc, selected),
            Widget::ChallengeOutput(w) => w.draw(f, layout.challenge_output, aoc, selected),
        }
    }

    fn layout(area: Rect) -> WidgetLayout {
        let left_right = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        let right_top_bottom = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(left_right[1]);

        WidgetLayout {
            challenge_list: left_right[0],
            dataset_input: right_top_bottom[0],
            challenge_output: right_top_bottom[1],
        }
    }

    pub fn handle_input<B>(&mut self, event: KeyEvent) -> Option<Result<UIAction>>
    where
        B: Backend,
    {
        match self {
            Widget::ChallengeList(w) => w.keymap().handle_input(w, event.code),
            Widget::DatasetInput(w) => w.keymap().handle_input(w, event.code),
            Widget::ChallengeOutput(w) => w.keymap().handle_input(w, event.code),
        }
    }
}
