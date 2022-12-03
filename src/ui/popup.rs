use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use super::bindings::Binding;

pub trait Popup {
    fn contents(&self) -> Vec<Spans>;
    fn title(&self) -> String;

    fn draw<B>(&self, f: &mut Frame<B>, global_area: Rect)
    where
        B: Backend,
    {
        let centered_area = Rect::new(
            global_area.width / 4,
            global_area.height / 4,
            global_area.width / 2,
            global_area.height / 2,
        );

        let paragraph = Paragraph::new(self.contents()).block(
            Block::default()
                .title(self.title())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );

        f.render_widget(Clear, centered_area);
        f.render_widget(paragraph, centered_area);
    }
}

impl<'a> From<Binding> for Spans<'a> {
    fn from(b: Binding) -> Self {
        let key = b
            .keys
            .iter()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let key_span = Span::styled(
            format!(" [{}]", key),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

        Spans::from(vec![key_span, Span::raw(format!(" {}", b.help))])
    }
}

#[derive(Default)]
pub struct HelpPopup {
    bindings: Vec<Binding>,
}

impl HelpPopup {
    pub fn with_bindings(bindings: Vec<Binding>) -> Self {
        Self { bindings }
    }
}

impl Popup for HelpPopup {
    fn contents(&self) -> Vec<Spans> {
        self.bindings.iter().map(|b| b.clone().into()).collect()
    }

    fn title(&self) -> String {
        String::from(" Help ")
    }
}
