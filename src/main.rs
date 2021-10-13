use orbtk::{
    prelude::*,
    shell::prelude::{Key, KeyEvent, WindowRequest},
};

use std::io::{self, Read};
use std::process::Command;

mod args;
mod programs;

const FONT_SIZE: f64 = 28.;
const MENU_CANDIDATES: usize = 10;

#[derive(Default)]
struct MenuState {
    args: args::Args,
    search: String,
    candidates: Vec<String>,
    current_key: Option<KeyEvent>,
    current_len: usize,
    cursor: isize,
    search_entity: Entity,
    stack_entity: Entity,
}

impl MenuState {
    fn set_current_key(&mut self, key: KeyEvent) {
        self.current_key = Some(key);
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
            .set::<String>("text", format!("[ {} ]", self.search));

        // update candidates
        ctx.clear_children_of(self.stack_entity);
        let filtered_candidates = self.get_filtered_matches(&self.search);
        for (i, candidate) in filtered_candidates.iter().take(MENU_CANDIDATES).enumerate() {
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
        self.current_len = if len > MENU_CANDIDATES {
            MENU_CANDIDATES
        } else {
            len
        };
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
                    .foreground("black")
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
            programs::get_programs()
        };

        ctx.switch_theme(theme_fluent_dark());
        self.render(ctx);
    }
    fn update(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        if let Some(key_event) = &self.current_key {
            let key = key_event.key;
            if ctx
                .window()
                .get::<KeyboardState>("keyboard_state")
                .is_ctrl_down()
            {
                // ctrl keybinds
                match key {
                    Key::U(_) => self.search = String::new(),
                    Key::C(_) => {
                        ctx.send_window_request(WindowRequest::Close);
                    }
                    _ => (),
                }
            } else {
                match key {
                    Key::Escape => {
                        ctx.send_window_request(WindowRequest::Close);
                    }
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
                                Command::new(candidate)
                                    .spawn()
                                    .expect(&format!("Failed to execute {}", candidate));
                            }
                        } else {
                            // turn into regular command with args then run
                            let mut args = self.search.split(" ");
                            let command = args.next().unwrap();
                            let rest_args = args.collect::<Vec<_>>();
                            Command::new(command)
                                .args(rest_args)
                                .spawn()
                                .expect(&format!("Failed to execute {}", &self.search));
                        }
                        ctx.send_window_request(WindowRequest::Close);
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
            self.current_key = None;
            self.render(ctx);
        }
    }
}

widget!(MenuView<MenuState>: KeyDownHandler);

impl Template for MenuView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let spacing = 20;
        self.child(
            Stack::new()
                .orientation(Orientation::Horizontal)
                .spacing(spacing)
                .child(
                    Container::new()
                        .id("prompt")
                        .background("lightblue")
                        .build(ctx),
                )
                .child(TextBlock::new().id("text").font_size(FONT_SIZE).build(ctx))
                .child(
                    Stack::new()
                        .id("stack")
                        .orientation(Orientation::Horizontal)
                        .spacing(spacing)
                        .build(ctx),
                )
                .build(ctx),
        )
        .on_key_down(move |states, key_event| -> bool {
            states.get_mut::<MenuState>(id).set_current_key(key_event);
            false
        })
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            // get display information, assumes monitor 0
            let size = orbclient::get_display_size().unwrap();
            let (screen_width, screen_height) = (size.0 as f64, size.1 as f64);

            let height = FONT_SIZE + 6.;

            // args stuff
            let args = args::Args::new().expect("Failed to get args");
            let position_y = if args.bottom_screen {
                screen_height - height
            } else {
                0.
            };

            let mut menuview = MenuView::new();
            menuview.state_mut().args = args;

            // window and ctx
            Window::new()
                .title("rmenu")
                .size(screen_width, height)
                .position((0., position_y))
                .child(menuview.build(ctx))
                .build(ctx)
        })
        .run();
}
