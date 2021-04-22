mod cli;
mod task;

use cli::{Action::*, CommandLineArgs};
use structopt::StructOpt;
use task::Task;

fn main() {
    let CommandLineArgs {
        action,
        journal_file,
    } = cli::CommandLineArgs::from_args();

    let journal_file = journal_file.expect("Failed to find journal file.");

    match action {
        Add { text } => task::add_task(journal_file, Task::new(text)),
        List => task::list_tasks(journal_file),
        Done { position } => task::complete_task(journal_file, position),
    }
    .expect("Failed to perform action")
}
