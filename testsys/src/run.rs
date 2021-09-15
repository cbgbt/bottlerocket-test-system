use crate::error::Result;
use crate::run_file;
use structopt::StructOpt;

/// Run testsys tests.
#[derive(Debug, StructOpt)]
pub(crate) struct Run {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Run a test from a YAML file.
    File(run_file::RunFile),
}

impl Run {
    pub(crate) async fn run(&self) -> Result<()> {
        match &self.command {
            Command::File(run_file) => run_file.run().await,
        }
    }
}
