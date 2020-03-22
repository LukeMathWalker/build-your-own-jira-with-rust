/// Playing around with the CLI using `cargo run --bin jira-wip` you might have noticed
/// that is quite tricky to actually exercise all the functionality we implemented:
/// the store is created anew for every execution, nothing is persisted!
///
/// Time to put a remedy to that: we want to persist our store to disk between CLI invocations,
/// reloading it before performing the next command.
///
/// We will be relying on the `serde` crate (`Ser`ialisation/`De`serialisation):
/// it can serialise data to many different file formats as long as your struct or enums
/// implement serde's `Serialize` trait.
/// `Deserialize`, instead, is needed for the opposite journey.
///
/// You don't need to implement this manually: just add `#[derive(Serialize, Deserialize)]`
/// where needed, and the `load` and `save` functions should just work!
pub mod persistence {
    use std::fs::read_to_string;
    use std::path::Path;
    use super::store_recap::TicketStore;

    /// Fetch authentication parameters from a configuration file, if available.
    pub fn load(path: &Path) -> TicketStore {
        println!("Reading data from {:?}", path);
        // Read the data in memory, storing the value in a string
        match read_to_string(path) {
            Ok(data) => {
                // Deserialize configuration from YAML format
                serde_yaml::from_str(&data).expect("Failed to parse serialised data.")
            }
            Err(e) => match e.kind() {
                // The file is missing - this is the first time you are using IronJira!
                std::io::ErrorKind::NotFound => {
                    // Return default configuration
                    TicketStore::new()
                }
                // Something went wrong - crash the CLI with an error message.
                _ => panic!("Failed to read data."),
            },
        }
    }

    /// Save tickets on disk in the right file.
    pub fn save(ticket_store: &TicketStore, path: &Path) {
        // Serialize data to YAML format
        let content = serde_yaml::to_string(ticket_store).expect("Failed to serialize tickets");
        println!("Saving tickets to {:?}", path);
        // Save to disk
        std::fs::write(path, content).expect("Failed to write tickets to disk.")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use super::super::store_recap::{TicketStore, Status, TicketDraft, TicketDescription, TicketTitle};
        use fake::Fake;
        use tempfile::NamedTempFile;

        #[test]
        fn load_what_you_save() {
            let mut store = TicketStore::new();
            let draft = generate_ticket_draft(Status::ToDo);
            store.save(draft);

            // We use the `tempfile` crate to generate a temporary path on the fly
            // which will be cleaned up at the end of the test.
            // See https://docs.rs/tempfile/3.1.0/tempfile/ for more details.
            let temp_path = NamedTempFile::new().unwrap().into_temp_path();

            save(&store, temp_path.as_ref());
            let loaded_store = load(temp_path.as_ref());

            assert_eq!(store, loaded_store);
        }

        #[test]
        fn return_default_if_the_file_is_missing() {

        }

        fn generate_ticket_draft(status: Status) -> TicketDraft {
            let description = TicketDescription::new((0..3000).fake()).unwrap();
            let title = TicketTitle::new((1..50).fake()).unwrap();

            TicketDraft {
                title,
                description,
                status
            }
        }
    }
}