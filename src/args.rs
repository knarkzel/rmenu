const HELP: &str = "\
usage: rmenu [-bfi] [-p prompt]
";

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Args {
    pub receiving_stdin: bool,
    pub bottom_screen: bool,
    pub case_insensitive: bool,
    pub prompt: Option<String>,
}

impl Args {
    pub fn new() -> Result<Self, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();

        if pargs.contains(["-h", "--help"]) {
            print!("{}", HELP);
            std::process::exit(0);
        }

        let args = Args {
            receiving_stdin: atty::isnt(atty::Stream::Stdin),
            bottom_screen: pargs.contains("-b"),
            case_insensitive: pargs.contains("-i"),
            prompt: pargs.opt_value_from_str("-p")?,
        };
        Ok(args)
    }
}
