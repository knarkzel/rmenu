use orbtk::prelude::*;
use orbtk::shell::prelude::{Key, KeyEvent};

use std::process::exit;
use walkdir::WalkDir;

const FONT_SIZE: f32 = 28.0;
const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = FONT_SIZE + 6.0;
const SEARCH_ID: &str = "search_id";
const ITEMS_ID: &str = "items_id";

enum Message {
    Key(KeyEvent),
}

#[derive(Default, AsAny)]
struct MenuState {
    search: String,
    items: Vec<String>,
    message: Option<Message>,
    search_entity: Entity,
    items_entity: Entity,
}

impl MenuState {
    fn send_message(&mut self, message: Message) {
        self.message = Some(message);
    }
    fn render(&mut self, ctx: &mut Context) {
        ctx.get_widget(self.search_entity).set::<String>("text", format!("{}|", self.search));
    }
    fn get_matches(&self) -> Vec<String> {
        self.items
            .clone()
            .into_iter()
            .filter(|entry| entry.contains(&self.search))
            .collect::<Vec<_>>()
    }
}

impl State for MenuState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.items = WalkDir::new("/usr/bin")
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<_>>();
        self.search_entity = ctx.entity_of_child(SEARCH_ID).unwrap();
        self.items_entity = ctx.entity_of_child(ITEMS_ID).unwrap();
        self.render(ctx);
    }
    fn update(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        if let Some(message) = &self.message {
            match message {
                Message::Key(key_event) => {
                    let key = key_event.key;
                    match key {
                        Key::Backspace => {
                            self.search.pop();
                        }
                        Key::Escape => exit(0),
                        _ => self.search.push_str(&key.to_string()),
                    }
                }
            };
            self.message = None;
            self.render(ctx);
        }
    }
}

type List = Vec<String>;
widget!(MenuView<MenuState>: ActivateHandler, KeyDownHandler { text: String, list: List });

impl Template for MenuView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Menu")
            .child(
                Stack::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(FONT_SIZE)
                    .child(
                        TextBlock::new()
                            .id(SEARCH_ID)
                            .font_size(FONT_SIZE)
                            .build(ctx),
                    )
                    .child(
                        Stack::new()
                            .id(ITEMS_ID)
                            .orientation(Orientation::Horizontal)
                            .spacing(20)
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
