use orbtk::prelude::*;
use orbtk::shell::prelude::{KeyEvent, Key};
use std::process::exit;

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
        MenuView::text_set(&mut ctx.widget(), "|");
    }
    fn update(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        if let Some(message) = &self.message {
            match message {
                Message::Key(key_event) => {
                    let key = key_event.key;
                    match key {
                        Key::Backspace => { self.search.pop(); },
                        Key::Escape => exit(0),
                        _ => self.search.push_str(&key.to_string()),
                    }
                }
            };
            MenuView::text_set(&mut ctx.widget(), format!("{}|", self.search));
            self.message = None;
        }
    }
}

widget!(MenuView<MenuState>: ActivateHandler, KeyDownHandler { text: String });

impl Template for MenuView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let mut stack = Stack::new()
            .orientation(Orientation::Horizontal)
            .spacing(20);
        for i in 0..5 {
            stack = stack.child(
                TextBlock::new()
                    .text(format!("bruh {}", i))
                    .font_size(FONT_SIZE)
                    .build(ctx),
            );
        }
        self.child(
            Stack::new()
                .orientation(Orientation::Horizontal)
                .spacing(FONT_SIZE)
                .child(
                    TextBlock::new()
                        .margin((10, 0, 500, 0))
                        .text(id)
                        .font_size(FONT_SIZE)
                        .build(ctx),
                )
                .child(stack.build(ctx))
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
