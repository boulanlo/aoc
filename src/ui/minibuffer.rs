use std::any::Any;

use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::AdventOfCode;

use super::{bindings::Binding, TextInputAction, Widget, WidgetKind};

#[derive(Clone, Debug)]
pub struct TextInput {
    pub prompt: String,
    pub origin: WidgetKind,
    pub action: TextInputAction,
    pub bindings: Vec<Binding>,
}

pub struct TextInputResponse {
    pub text: String,
    pub origin: WidgetKind,
    pub action: TextInputAction,
}

#[derive(Default)]
pub struct Minibuffer {
    event: Option<TextInput>,
    buffer: String,
    cursor: usize,
}

impl Minibuffer {
    pub fn clear(&mut self) -> Option<WidgetKind> {
        self.buffer.clear();
        self.cursor = 0;
        self.event.take().map(|e| e.origin)
    }

    pub fn bindings(&self) -> Option<Vec<Binding>> {
        self.event.as_ref().map(|e| e.bindings.clone())
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.buffer.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }

    pub fn start(&mut self, text_input: TextInput) {
        self.clear();
        self.event = Some(text_input);
    }

    pub fn push(&mut self, c: char) {
        self.buffer.insert(self.cursor, c);
        self.cursor = (self.cursor + 1).min(self.buffer.len())
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.buffer.len());
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn finish(&mut self) -> TextInputResponse {
        let event = self.event.take().unwrap();

        let response = TextInputResponse {
            text: std::mem::take(&mut self.buffer),
            origin: event.origin,
            action: event.action,
        };
        self.cursor = 0;
        response
    }
}

impl<B> Widget<B> for Minibuffer
where
    B: Backend,
{
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, _: &AdventOfCode, _: bool)
    where
        B: Backend,
    {
        let contents = if let Some(event) = self.event.as_ref() {
            let prompt = Span::styled(
                &event.prompt,
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            );
            let before_cursor =
                Span::raw(self.buffer.chars().take(self.cursor).collect::<String>());
            let cursor = Span::styled(
                self.buffer
                    .chars()
                    .nth(self.cursor)
                    .unwrap_or(' ')
                    .to_string(),
                Style::default().bg(Color::White),
            );
            let after_cursor = Span::raw(
                self.buffer
                    .chars()
                    .skip(self.cursor + 1)
                    .collect::<String>(),
            );

            Spans::from(vec![prompt, before_cursor, cursor, after_cursor])
        } else {
            Spans::default()
        };

        let paragraph = Paragraph::new(contents).block(Block::default().borders(Borders::NONE));

        f.render_widget(paragraph, area)
    }

    fn kind(&self) -> WidgetKind {
        WidgetKind::Minibuffer
    }

    fn as_any(&self) -> &dyn Any {
        self as _
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as _
    }

    fn name(&self, _: &AdventOfCode) -> String {
        String::from(" Minibuffer ")
    }
}
