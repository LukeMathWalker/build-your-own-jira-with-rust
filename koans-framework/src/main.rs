use ansi_term::Colour::{Green, Red, Yellow};
use ansi_term::Style;
use koans::KoanCollection;
use std::ffi::OsString;
use std::path::Path;
use std::error::Error;

#[derive(structopt::StructOpt)]
pub struct Command {
    /// Name of the koan collection you want to work on. [e.g. `jira-wip`]
    #[structopt(long)]
    pub name: String,
    #[structopt(subcommand)]
    pub action_type: ActionType,
}

#[derive(structopt::StructOpt)]
pub enum ActionType {
    /// Run all koans-framework you have opened so far in a collection to check if your solutions are correct.
    Check,
}

fn main() -> Result<(), Box<dyn Error>> {
    let command = <Command as paw::ParseArgs>::parse_args()?;
    dbg!(std::env::current_dir().unwrap());

    let mut koans = KoanCollection::new("jira-wip");
    let message = if !seek_the_path(&koans) || walk_the_path(&mut koans) {
        "Eternity lies ahead of us, and behind. Your path is not yet finished. 🍂"
    } else {
        "What is the sound of one hand clapping (for you)? 🌟"
    };

    println!("\t{}\n", Style::default().italic().paint(message));
    Ok(())
}

fn seek_the_path(koans: &KoanCollection) -> bool {
    print!(" \n\n");
    for koan in koans.opened() {
        let koan_outcome = run_tests(&koans.configuration().manifest_path(), Some(&koan.name));
        match koan_outcome {
            TestOutcome::Success => {
                println!(
                    "\t🚀 {} - {}️",
                    Green.normal().paint(&koan.parent_name),
                    Green.normal().paint(&koan.name)
                );
            }
            TestOutcome::Failure { details } => {
                println!(
                    "\t❌ {}\n\n\t{}\n\n{}",
                    Red.normal().paint(&koan.name),
                    Style::default().italic().paint(
                        "Meditate on your approach and return. Mountains are merely mountains."
                    ),
                    Style::default().dimmed().paint(details)
                );
                return false;
            }
        }
    }
    true
}

fn walk_the_path(koans: &mut KoanCollection) -> bool {
    if let Ok(new_koan) = koans.open_next() {
        println!(
            "{} {} - {}.",
            Yellow.normal().paint("\n\tAhead of you lies"),
            Yellow.bold().paint(&new_koan.parent_name),
            Yellow.bold().paint(&new_koan.name)
        );
        true
    } else {
        println!(
            "{}",
            Green.normal().paint("\n\tThere will be no more tasks.")
        );
        false
    }
}

fn run_tests(manifest_path: &Path, filter: Option<&str>) -> TestOutcome {
    let mut args: Vec<OsString> = vec![
        "test".into(),
        "--manifest-path".into(),
        manifest_path.into(),
        "-q".into(),
    ];

    if let Some(test_filter) = filter {
        args.push(test_filter.into());
    }

    let output = std::process::Command::new("cargo")
        .args(args)
        .output()
        .expect("Failed to run tests");

    let status = output.status;

    if status.success() {
        TestOutcome::Success
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        TestOutcome::Failure {
            details: [stdout, stderr].concat(),
        }
    }
}

enum TestOutcome {
    Success,
    Failure { details: String },
}
