use orbtk::{
    prelude::*,
    shell::prelude::{Key, KeyEvent},
};
use std::io::{self, Read};
use std::{process::exit, process::Command};

mod args;
use args::*;

mod programs;
use programs::*;

const FONT_SIZE: f32 = 28.0;
const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = FONT_SIZE + 6.0;
//const SCREEN_WIDTH: f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;
const WRAP: usize = 10;

enum Message {
    Key(KeyEvent),
}

#[derive(Default, AsAny)]
struct MenuState {
    args: Args,
    search: String,
    message: Option<Message>,
    candidates: Vec<String>,
    current_len: usize,
    cursor: isize,
    search_entity: Entity,
    stack_entity: Entity,
}

impl MenuState {
    fn send_message(&mut self, message: Message) {
        self.message = Some(message);
    }
    fn get_filtered_matches(&self, search: &str) -> Vec<&String> {
        // TODO: Make this better
        if self.args.case_insensitive {
            self.candidates
                .iter()
                .filter(|entry| entry.to_lowercase().contains(&search.to_lowercase()))
                .collect::<Vec<_>>()
        } else {
            self.candidates
                .iter()
                .filter(|entry| entry.contains(search))
                .collect::<Vec<_>>()
        }
    }
    fn render(&mut self, ctx: &mut Context) {
        // update search bar
        ctx.get_widget(self.search_entity)
            .set::<String>("text", self.search.clone());

        // update candidates
        ctx.clear_children_of(self.stack_entity);
        let filtered_candidates = self.get_filtered_matches(&self.search);
        for (i, candidate) in filtered_candidates.iter().take(WRAP).enumerate() {
            let textblock = if self.cursor as usize == i {
                TextBlock::new()
                    .text(candidate.to_string())
                    .foreground("lightblue")
                    .font_size(FONT_SIZE)
            } else {
                TextBlock::new()
                    .text(candidate.to_string())
                    .opacity(0.5)
                    .font_size(FONT_SIZE)
            };
            ctx.append_child_to(textblock, self.stack_entity);
        }
        let len = filtered_candidates.len();
        self.current_len = if len > WRAP { WRAP } else { len };
    }
}

impl State for MenuState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.search_entity = ctx.entity_of_child("text").unwrap();
        self.stack_entity = ctx.entity_of_child("stack").unwrap();

        if let Some(prompt) = &self.args.prompt {
            let prompt_entity = ctx.entity_of_child("prompt").unwrap();
            ctx.append_child_to(
                TextBlock::new()
                    .text(format!("   {}", prompt))
                    .margin((10, 0, 10, 0))
                    .font_size(FONT_SIZE),
                prompt_entity,
            )
        }

        self.candidates = if self.args.receiving_stdin {
            // get stdin
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer.lines().map(|s| s.to_string()).collect::<Vec<_>>()
        } else {
            // default behaviour is get programs
            Programs::new().binaries
        };

        ctx.switch_theme(theme_fluent_dark());
        self.render(ctx);
    }
    fn update(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        if let Some(message) = &self.message {
            match message {
                Message::Key(key_event) => {
                    let key = key_event.key;
                    match key {
                        Key::Escape => exit(0),
                        Key::Right => {
                            if self.current_len > 0 {
                                self.cursor = (self.cursor + 1) % self.current_len as isize;
                            }
                        }
                        Key::Left => {
                            if self.current_len > 0 {
                                self.cursor -= 1;
                                if self.cursor < 0 {
                                    self.cursor = self.current_len as isize - 1;
                                }
                            }
                        }
                        Key::Enter => {
                            let matches = self.get_filtered_matches(&self.search);
                            let candidate = matches.get(self.cursor as usize);

                            if let Some(candidate) = candidate {
                                if self.args.receiving_stdin {
                                    // print it
                                    println!("{}", candidate);
                                } else {
                                    // execute it
                                    Command::new(candidate).spawn().unwrap();
                                }
                            }
                            exit(0);
                        }
                        Key::Backspace => {
                            self.search.pop();
                        }
                        _ => {
                            self.search.push_str(&key.to_string());
                            self.cursor = 0;
                        }
                    }
                }
            };
            self.message = None;
            self.render(ctx);
        }
    }
}

widget!(MenuView<MenuState>: KeyDownHandler);

impl Template for MenuView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .orientation(Orientation::Horizontal)
                .spacing(10)
                .child(Container::new().id("prompt").background("black").build(ctx))
                .child(TextBlock::new().id("text").font_size(FONT_SIZE).build(ctx))
                .spacing(FONT_SIZE)
                .child(
                    Stack::new()
                        .id("stack")
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
            // args stuff
            let args = Args::new().unwrap_or(Args::default());
            let position_x: f64 = 0.0;
            let position_y: f64 = if args.bottom_screen {
                (SCREEN_HEIGHT - HEIGHT) as f64
            } else {
                0.0
            };

            let mut menuview = MenuView::new();
            menuview.state_mut().args = args;

            // window and ctx
            Window::new()
                .title("rmenu")
                .position((position_x, position_y))
                .size(WIDTH, HEIGHT)
                .child(menuview.build(ctx))
                .build(ctx)
        })
        .run();
}
