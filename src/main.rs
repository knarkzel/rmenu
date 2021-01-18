use orbtk::prelude::*;
use orbtk::shell::prelude::{Key, KeyEvent};

use std::process::exit;
use walkdir::WalkDir;

mod args;
use args::*;

const FONT_SIZE: f32 = 28.0;
const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = FONT_SIZE + 6.0;

type List = Vec<String>;

#[derive(Default, AsAny)]
struct ItemsState {
    items: List,
}

// impl ItemsState {
//     fn get_matches(&self, search: &String) -> List {
//         self.items
//             .clone()
//             .into_iter()
//             .filter(|entry| entry.contains(search))
//             .collect::<Vec<_>>()
//     }
// }

impl State for ItemsState {
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
        ctx.widget().set::<List>("items", self.items[..5].into());
    }
}

widget!(ItemsView<ItemsState> { items: List });

impl Template for ItemsView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let items = ctx.get_widget(id).get::<List>("items").clone();
        let stack = Stack::new()
                .orientation(Orientation::Horizontal)
                .spacing(FONT_SIZE)
                .build(ctx);
        for item in items {
            let text = TextBlock::new().text(item.to_string()).font_size(FONT_SIZE).build(ctx);
            ctx.append_child(stack, text);
        }
        self.id("items_list").child(stack)
    }
}

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
    fn render(&mut self, ctx: &mut Context) {
        ctx.widget()
            .set::<String>("text", format!("{}|", self.search));
    }
}

impl State for MenuState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.render(ctx);
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
            self.render(ctx);
            self.message = None;
        }
    }
}

widget!(MenuView<MenuState>: KeyDownHandler { text: String });

impl Template for MenuView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Menu")
            .child(
                Stack::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(FONT_SIZE)
                    .child(TextBlock::new().text(id).font_size(FONT_SIZE).build(ctx))
                    .build(ctx),
            )
            .child(ItemsView::new().build(ctx))
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
