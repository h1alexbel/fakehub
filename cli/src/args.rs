use clap::Parser;

// @todo #13:15min Handle port argument
//  We should process the port argument and
//  pass it to the server on `start` command.
//  Start command should be added also with clap
#[derive(Parser, Debug)]
pub(crate) struct Args {
    /// The port to run
    #[arg(short, long, default_value_t = 3000)]
    pub(crate) port: usize,
}
