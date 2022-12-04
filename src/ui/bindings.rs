use std::{collections::HashMap, fmt};

use crossterm::event::KeyCode;

use super::popup::HelpPopup;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)]
pub struct BindKey(pub KeyCode);

impl fmt::Display for BindKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            KeyCode::Backspace
            | KeyCode::Enter
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Up
            | KeyCode::Down
            | KeyCode::Home
            | KeyCode::End
            | KeyCode::PageUp
            | KeyCode::PageDown
            | KeyCode::Tab
            | KeyCode::BackTab
            | KeyCode::Delete
            | KeyCode::Insert
            | KeyCode::Null
            | KeyCode::Esc
            | KeyCode::CapsLock
            | KeyCode::ScrollLock
            | KeyCode::NumLock
            | KeyCode::PrintScreen
            | KeyCode::Pause
            | KeyCode::Menu
            | KeyCode::KeypadBegin => write!(f, "{:?}", self.0),
            KeyCode::F(n) => write!(f, "F{n}"),
            KeyCode::Char(c) => write!(f, "{c}"),
            KeyCode::Media(m) => write!(f, "{m:?}"),
            KeyCode::Modifier(m) => write!(f, "{m:?}"),
        }
    }
}

impl PartialOrd for BindKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_string().partial_cmp(&other.to_string())
    }
}

impl Ord for BindKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl From<char> for BindKey {
    fn from(c: char) -> Self {
        BindKey(KeyCode::Char(c))
    }
}

impl From<KeyCode> for BindKey {
    fn from(k: KeyCode) -> Self {
        Self(k)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    pub(crate) keys: Vec<BindKey>,
    pub(crate) help: String,
}

impl PartialOrd for Binding {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.keys
            .first()
            .map(|b| b.to_string())
            .and_then(|l| other.keys.first().map(|r| (l, r.to_string())))
            .and_then(|(l, r)| l.partial_cmp(&r))
    }
}

impl Ord for Binding {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.keys
            .first()
            .map(|b| b.to_string())
            .unwrap_or_else(String::new)
            .cmp(
                &other
                    .keys
                    .first()
                    .map(|b| b.to_string())
                    .unwrap_or_else(String::new),
            )
    }
}

impl Binding {
    pub fn new(keys: Vec<BindKey>, help: String) -> Self {
        Self { keys, help }
    }
}

pub struct Command<'a, T: ?Sized, R> {
    function: Box<dyn Fn(&mut T) -> R + 'a>,
    help: String,
}

impl<'a, T: ?Sized, R> Command<'a, T, R> {
    pub fn get_command(&self) -> &dyn Fn(&mut T) -> R {
        &self.function
    }
}

pub struct Keymap<'a, T: ?Sized, R> {
    name: String,
    bindings: HashMap<BindKey, Command<'a, T, R>>,
    aliases: HashMap<BindKey, BindKey>,
}

impl<'a, T: ?Sized, R> Default for Keymap<'a, T, R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: ?Sized, R> Keymap<'a, T, R> {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            bindings: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    pub fn with_name<N>(mut self, name: N) -> Self
    where
        N: Into<String>,
    {
        self.name = name.into();
        self
    }

    pub fn add_binding<F, B, H>(mut self, c: B, function: F, help: H) -> Self
    where
        F: Fn(&mut T) -> R + 'a,
        B: Into<BindKey>,
        H: Into<String>,
    {
        self.bindings.insert(
            c.into(),
            Command {
                function: Box::new(function),
                help: help.into(),
            },
        );
        self
    }

    pub fn bindings(&self) -> Vec<Binding> {
        let mut bindings = self
            .bindings
            .iter()
            .map(|(c, command)| Binding::new(vec![*c], command.help.clone()))
            .collect::<Vec<_>>();
        for (alias, to) in &self.aliases {
            if let Some(b) = bindings.iter_mut().find(|b| b.keys.contains(to)) {
                b.keys.push(*alias);
                b.keys.sort();
            }
        }

        bindings.sort();
        bindings
    }

    pub fn copy_bindings<B, C>(mut self, from: B, to: C) -> Self
    where
        B: Into<BindKey>,
        C: Into<BindKey>,
    {
        let from = from.into();
        let to = to.into();
        if self.bindings.get(&from).is_some() {
            self.aliases.insert(to, from);
        } else if let Some(alias) = self.aliases.get(&from) {
            self.aliases.insert(from, *alias);
        }
        self
    }

    pub fn handle_input<B>(&self, obj: &mut T, c: B) -> Option<R>
    where
        B: Into<BindKey>,
    {
        let c = c.into();
        let key = self.aliases.get(&c).unwrap_or(&c);

        self.bindings.get(key).map(|c| (c.get_command())(obj))
    }

    pub fn popup(&self) -> HelpPopup {
        HelpPopup::with_bindings(self.bindings(), self.name.clone())
    }
}
