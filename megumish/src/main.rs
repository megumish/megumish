use std::{borrow::Cow, iter::repeat, path::Path};

use console::{style, Key, Term};

struct Context {
    term: Term,
    command_bar: CommandBar,
}

struct CommandBar {
    input_string: String,
}

impl Context {
    fn new() -> Self {
        Self {
            term: Term::stdout(),
            command_bar: CommandBar::new(),
        }
    }

    fn update(&mut self, k: Key) {
        match k {
            Key::Enter => self.command_bar.clear(),
            Key::Backspace => self.command_bar.backspace(),
            Key::Char(c) => self.command_bar.push(c),
            _ => { /* do nothing */ }
        }
    }
}

impl CommandBar {
    fn clear(&mut self) {
        self.input_string = String::new();
    }

    fn push(&mut self, c: char) {
        self.input_string.push(c);
    }

    fn backspace(&mut self) {
        let _ = self.input_string.pop();
    }

    fn new() -> Self {
        Self {
            input_string: String::new(),
        }
    }

    fn put(&self, context: &Context) {
        let _ = context.term.write_str(&self.input_string).unwrap();
    }
}

struct Window<'a> {
    content: Cow<'a, str>,
}

impl<'a> Window<'a> {
    fn new(term_lines: usize) -> Self {
        let content = repeat(style("~\n").blue().to_string())
            .take(term_lines)
            .collect::<String>();
        Self {
            content: Cow::from(style(content).white().to_string()),
        }
    }

    fn put(&self, context: &Context) {
        let _ = context.term.write_str(&self.content).unwrap();
    }
}

struct StatusBar;

impl StatusBar {
    fn put(&self, context: &Context) {
        let _ = context.term.write_line("").unwrap();
    }

    fn new() -> Self {
        Self
    }
}

fn main() {
    let mut context = Context::new();
    render(&context);
    loop {
        let k = get_key(&context.term);
        context.update(k);
        render(&context);
    }
}

fn get_key(term: &Term) -> Key {
    term.read_key().unwrap()
}

fn render(context: &Context) {
    let _ = context.term.clear_screen().unwrap();
    let window = Window::new(context.term.size().1 as usize);
    window.put(context);
    let status_bar = StatusBar::new();
    status_bar.put(context);
    context.command_bar.put(context);
}
