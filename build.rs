use man::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let page = Manual::new("rmenu")
        .about("dmenu in rust")
        .author(
            Author::new("Knarkzel")
                .email("knarkzel@knarkzel.xyz")
        )
        .description(
            "rmenu is a dynamic menu implemented for Redox, which reads a list of \
            newline-separated items from stdin. When the user selects an item and presses Return, \
            their choice is printed to stdout and rmenu terminates. Entering text will narrow the \
            items to those matching the tokens in the input. Default behaviour is to list \
            programs in the user's $PATH and run the result in $SHELL, unless  user pipes in \
            input.",
        )
        .flag(
            Flag::new()
                .short("-b")
                .help("rmenu appears at the bottom of the screen."),
        )
        .flag(
            Flag::new()
                .short("-i")
                .help("rmenu matches menu items case insensitively."),
        )
        .option(
            Opt::new("lines")
                .short("-l")
                .help("rmenu lists items vertically, with the given number of lines."),
        )
        .option(
            Opt::new("prompt")
                .short("-p")
                .help("defines the prompt to be displayed to the left of the input field."),
        )
        .example(
            Example::new()
                .command("rmenu")
                .output("Grants user a selection of programs in $PATH. When result is selected with \
                    Enter, program will run."),
        )
        .example(
            Example::new()
                .command(r#"echo "option 1\\noption2\\n option3" | rmenu -l 3"#)
                .output("Opens a menu from piped input seperated by newlines, displaying options \
                    vertically."),
        )
        .render();
    let mut man = File::create("rmenu.man").unwrap();
    man.write_all(page.as_bytes()).unwrap();
}
