use koans::KoanCollection;
use read_input::prelude::*;
use std::error::Error;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use yansi::Paint;

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
    // Enable ANSI colour support on Windows, is it's supported.
    // Disable it entirely otherwise.
    if cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }
    let mut koans = KoanCollection::new(&command.path)?;
    match seek_the_path(&koans) {
        TestOutcome::Success => {
            match koans.next() {
                Some(next_koan) => {
                    println!("\t{}\n", info_style().paint("Eternity lies ahead of us, and behind. Your path is not yet finished. 🍂"));

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
                            next_style().paint("\n\tAhead of you lies"),
                            next_style().bold().paint(format!("{}", &next_koan)),
                        );
                    }
                }
                None => {
                    println!(
                        "{}\n\t{}\n",
                        success_style().paint("\n\tThere will be no more tasks."),
                        info_style().paint("What is the sound of one hand clapping (for you)? 🌟")
                    );
                }
            }
        }
        TestOutcome::Failure { details } => {
            println!(
                "\n\t{}\n\n{}\n\n",
                info_style().paint(
                    "Meditate on your approach and return. Mountains are merely mountains.\n\n"
                ),
                cargo_style().paint(&String::from_utf8_lossy(&details).to_string())
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
    println!("{}", info_style().dimmed().paint("Running tests...\n"));
    for koan in koans.opened() {
        let koan_outcome = run_tests(&koans.configuration().manifest_path(), Some(&koan.name));
        match koan_outcome {
            TestOutcome::Success => {
                println!("{}", success_style().paint(format!("\t🚀 {}", &koan)));
            }
            TestOutcome::Failure { details } => {
                println!("{}", failure_style().paint(format!("\t❌ {}", &koan)));
                return TestOutcome::Failure { details };
            }
        }
    }
    TestOutcome::Success
}

fn run_tests(manifest_path: &Path, filter: Option<&str>) -> TestOutcome {
    // Tell cargo to return colored output, unless we are on Windows and the terminal
    // doesn't support it.
    let mut color_option = "always";
    if cfg!(windows) && !Paint::enable_windows_ascii() {
        color_option = "never";
    }

    let mut args: Vec<OsString> = vec![
        "test".into(),
        "--manifest-path".into(),
        manifest_path.into(),
        "-q".into(),
        "--color".into(),
        color_option.into(),
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

pub fn info_style() -> yansi::Style {
    yansi::Style::new(yansi::Color::Default)
}
pub fn cargo_style() -> yansi::Style {
    yansi::Style::new(yansi::Color::Default).dimmed()
}
pub fn next_style() -> yansi::Style {
    yansi::Style::new(yansi::Color::Yellow)
}
pub fn success_style() -> yansi::Style {
    yansi::Style::new(yansi::Color::Green)
}
pub fn failure_style() -> yansi::Style {
    yansi::Style::new(yansi::Color::Red)
}
