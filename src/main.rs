use orbtk::prelude::*;
use orbtk::shell::prelude::{Key, KeyEvent};
use std::process::exit;

// mod args;
// use args::*;

// mod programs;
// use programs::*;

const FONT_SIZE: f32 = 28.0;
const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = FONT_SIZE + 6.0;

enum Message {
    Key(KeyEvent),
}

#[derive(Default, AsAny)]
struct MenuState {
    search: String,
    message: Option<Message>,
}

impl MenuState {
    fn send_message(&mut self, message: Message) {
        self.message = Some(message);
    }
}

impl State for MenuState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        ctx.switch_theme(theme_fluent_dark());
        ctx.widget()
            .set::<String>("text", format!("{}|", self.search));
    }
    fn update(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        if let Some(message) = &self.message {
            match message {
                Message::Key(key_event) => {
                    let key = key_event.key;
                    match key {
                        Key::Escape => exit(0),
                        Key::Backspace => {
                            self.search.pop();
                        }
                        _ => self.search.push_str(&key.to_string()),
                    }
                }
            };
            ctx.widget()
                .set::<String>("text", format!("{}|", self.search));
            self.message = None;
        }
    }
}

widget!(MenuView<MenuState>: KeyDownHandler { text: String });

impl Template for MenuView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .orientation(Orientation::Horizontal)
                .child(
                    TextBlock::new()
                        .text(id)
                        .font_size(FONT_SIZE)
                        .offset(10)
                        .build(ctx),
                )
                .build(ctx),
        )
        .on_key_down(move |states, event| -> bool {
            states
                .get_mut::<MenuState>(id)
                .send_message(Message::Key(event));
            false
        })
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("rmenu")
                .position((0.0, 0.0))
                .size(WIDTH, HEIGHT)
                .child(MenuView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
