const HELP: &str = "\
usage: rmenu [-bfi] [-l lines] [-p prompt]
";

#[derive(Debug)]
pub struct Args {
    pub receiving_stdin: bool,
    pub bottom_screen: bool,
    pub lock_stdin: bool,
    pub case_insensitive: bool,
    pub lines: Option<u32>,
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
            receiving_stdin: atty::is(atty::Stream::Stdin),
            bottom_screen: pargs.contains("-b"),
            lock_stdin: pargs.contains("-f"),
            case_insensitive: pargs.contains("-i"),
            lines: pargs.opt_value_from_str("-l")?,
            prompt: pargs.opt_value_from_str("-p")?,
        };
        Ok(args)
    }
}
