use ansi_term::Colour::{Green, Red, Yellow};
use ansi_term::Style;
use koans::KoanCollection;
use std::process::Command;

fn main() {
    let mut koans = KoanCollection::new("jira-wip/src");
    let message = if !seek_the_path(&koans) || walk_the_path(&mut koans) {
        "Eternity lies ahead of us, and behind. Your path is not yet finished. 🍂"
    } else {
        "What is the sound of one hand clapping (for you)? 🌟"
    };

    println!("\t{}\n", Style::default().italic().paint(message));
}

fn seek_the_path(koans: &KoanCollection) -> bool {
    print!(" \n\n");
    for koan in koans.opened() {
        let koan_outcome = run_tests(&koans.manifest_path, Some(&koan.name));
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

fn run_tests(manifest_path: &str, filter: Option<&str>) -> TestOutcome {
    let mut args = vec!["test", "--manifest-path", manifest_path, "-q"];

    if let Some(test_filter) = filter {
        args.push(test_filter);
    }

    let output = Command::new("cargo")
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
