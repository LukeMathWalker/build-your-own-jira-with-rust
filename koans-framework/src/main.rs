use ansi_term::Colour::{Green, Red, Yellow};
use ansi_term::Style;
use koans::KoanCollection;
use read_input::prelude::*;
use std::error::Error;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// A small CLI to manage test-driven workshops and tutorials in Rust.
///
/// Each exercise is called koan and comes with a set of associated tests.
/// A suite of koans is called `collection`.
///
/// Invoking `koans` runs tests for all the koans you have opened so far in a collection
/// to check if your solutions are correct.
/// If everything runs smoothly, you will asked if you want to move forward to the next koan.
#[derive(structopt::StructOpt)]
pub struct Command {
    /// Path to the koan collection you want to work on.
    /// Both absolute and relative paths are supported.
    ///
    /// E.g. `koans --path jira-wip` if `jira-wip` is a sub-directory of your current
    /// working directory and `jira-wip/Cargo.toml` leads to the Cargo file of the koans
    /// collection.
    #[structopt(long, parse(from_os_str))]
    pub path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let command = <Command as paw::ParseArgs>::parse_args()?;

    let mut koans = KoanCollection::new(&command.path)?;
    match seek_the_path(&koans) {
        TestOutcome::Success => {
            match koans.next() {
                Some(next_koan) => {
                    println!("\t{}\n", Style::default().italic().paint("Eternity lies ahead of us, and behind. Your path is not yet finished. 🍂"));

                    let open_next = input::<String>()
                        .repeat_msg(format!(
                            "Do you want to open the next koan, {}? [y/n] ",
                            next_koan
                        ))
                        .err("Please answer either yes or no.")
                        .add_test(|s| parse_bool(s).is_some())
                        .get();

                    if parse_bool(&open_next).unwrap() {
                        let next_koan = koans.open_next().expect("Failed to open the next koan");
                        println!(
                            "{} {}",
                            Yellow.normal().paint("\n\tAhead of you lies"),
                            Yellow.bold().paint(format!("{}", &next_koan)),
                        );
                    }
                }
                None => {
                    println!(
                        "{}\n\t{}\n",
                        Green.normal().paint("\n\tThere will be no more tasks."),
                        Style::default()
                            .italic()
                            .paint("What is the sound of one hand clapping (for you)? 🌟")
                    );
                }
            }
        }
        TestOutcome::Failure { details } => {
            println!(
                "\n\n\t{}\n\n{}",
                Style::default()
                    .italic()
                    .paint("Meditate on your approach and return. Mountains are merely mountains."),
                Style::default()
                    .dimmed()
                    .paint(&String::from_utf8_lossy(&details).to_string())
            );
        }
    };
    Ok(())
}

fn parse_bool(s: &str) -> Option<bool> {
    match s.to_ascii_lowercase().as_str() {
        "yes" | "y" => Some(true),
        "no" | "n" => Some(false),
        _ => None,
    }
}

fn seek_the_path(koans: &KoanCollection) -> TestOutcome {
    print!(" \n\n");
    for koan in koans.opened() {
        let koan_outcome = run_tests(&koans.configuration().manifest_path(), Some(&koan.name));
        match koan_outcome {
            TestOutcome::Success => {
                println!("{}", Green.normal().paint(format!("\t🚀 {}", &koan)));
            }
            TestOutcome::Failure { details } => {
                println!("{}", Red.normal().paint(format!("\t❌ {}", &koan)));
                return TestOutcome::Failure { details };
            }
        }
    }
    TestOutcome::Success
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

    if output.status.success() {
        TestOutcome::Success
    } else {
        TestOutcome::Failure {
            details: [output.stdout, output.stderr].concat(),
        }
    }
}

#[derive(PartialEq)]
enum TestOutcome {
    Success,
    Failure { details: Vec<u8> },
}
