use crate::store::TicketStore;
use directories::ProjectDirs;
use std::fs::read_to_string;
use std::path::PathBuf;

// `PROJECT_NAME`, `ORGANISATION_NAME` and `QUALIFIER` are used to determine
// where to store configuration files and secrets for an application
// according to the convention of the underlying operating system.
//
// `qualifier_name` is only relevant for MacOS - we leave it blank.
const PROJECT_NAME: &'static str = "IronJIRA";
const ORGANISATION_NAME: &'static str = "RustLDNUserGroup";
const QUALIFIER: &'static str = "";

const TICKET_STORE: &'static str = "ticket_store.yaml";

fn data_store_filename() -> PathBuf {
    // Get the directory where we are supposed to store data
    // according to the convention of the underlying operating system.
    //
    // The operation could fail if some OS environment variables are not set (e.g. $HOME)
    let project_dir = ProjectDirs::from(QUALIFIER, ORGANISATION_NAME, PROJECT_NAME)
        .expect("Failed to determine path of the configuration directory.");
    let data_dir = project_dir.data_dir();
    println!("Data storage directory: {:?}", data_dir);

    // Create the data directory, if missing.
    // It also takes care of creating intermediate sub-directory, if necessary.
    std::fs::create_dir_all(data_dir).expect("Failed to create data directory.");

    // Path to the file storing our tickets
    data_dir.join(TICKET_STORE)
}

/// Fetch authentication parameters from a configuration file, if available.
pub fn load() -> TicketStore {
    let filename = data_store_filename();
    // Read the data in memory, storing the value in a string
    println!("Reading data from {:?}", filename);
    match read_to_string(filename) {
        Ok(data) => {
            // Deserialize configuration from YAML format
            serde_yaml::from_str(&data).expect("Failed to parse serialised data.")
        }
        Err(e) => match e.kind() {
            // The file is missing - this is the first you are using IronJira!
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
pub fn save(ticket_store: &TicketStore) {
    let filename = data_store_filename();
    // Serialize data to YAML format
    let content = serde_yaml::to_string(ticket_store).expect("Failed to serialize tickets");
    // Save to disk
    println!("Saving tickets to {:?}", filename);
    std::fs::write(filename, content).expect("Failed to write tickets to disk.")
}
