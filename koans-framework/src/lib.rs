use anyhow::anyhow;
use regex::Regex;
use std::ffi::OsString;
use std::fmt::Formatter;
use std::fs::{read_dir, OpenOptions};
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::path::PathBuf;

pub struct KoanConfiguration {
    pub base_path: PathBuf,
}

impl KoanConfiguration {
    pub fn new<P: Into<PathBuf>>(base_path: P) -> Result<Self, anyhow::Error> {
        let c = Self {
            base_path: base_path.into(),
        };

        if !c.manifest_path().exists() {
            let error_path = if c.manifest_path().is_absolute() {
                c.manifest_path()
            } else {
                std::env::current_dir().unwrap().join(c.manifest_path())
            };
            return Err(anyhow!("{:?} does not exist.", error_path));
        }

        Ok(c)
    }

    pub fn koans_path(&self) -> PathBuf {
        self.base_path.join("src").join("koans")
    }

    pub fn enlightenment_path(&self) -> PathBuf {
        self.base_path.join("src").join("path_to_enlightenment.rs")
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.base_path.join("Cargo.toml")
    }
}

pub struct KoanCollection {
    configuration: KoanConfiguration,
    koans: Vec<Koan>,
}

impl KoanCollection {
    pub fn new<P: Into<PathBuf>>(base_path: P) -> Result<Self, anyhow::Error> {
        let configuration = KoanConfiguration::new(base_path)?;
        let mut koans: Vec<(OsString, OsString)> = read_dir(configuration.koans_path())
            .unwrap()
            .map(|f| {
                let entry = f.unwrap();
                // Each entry in path has to be a directory!
                assert!(
                    entry.file_type().unwrap().is_dir(),
                    "Each entry in {:?} has to be a directory",
                    &configuration.koans_path()
                );
                let directory_name = entry.file_name();
                read_dir(entry.path())
                    .unwrap()
                    .map(move |f| (directory_name.to_owned(), f.unwrap().file_name()))
            })
            .flatten()
            .collect();
        // Sort them in lexicographical order - koans are prefixed with `dd_`
        koans.sort();

        Ok(Self {
            configuration,
            koans: koans.into_iter().map(|f| f.into()).collect(),
        })
    }

    pub fn configuration(&self) -> &KoanConfiguration {
        &self.configuration
    }

    pub fn n_opened(&self) -> usize {
        match OpenOptions::new()
            .read(true)
            .open(&self.configuration.enlightenment_path())
        {
            Ok(f) => BufReader::new(&f)
                .lines()
                .filter(|l| !l.as_ref().unwrap().is_empty())
                .filter(|l| &l.as_ref().unwrap().trim()[..2] != "//") // Ignores comments
                .map(|l| {
                    if l.unwrap().contains("mod") {
                        // Count the number of module declarations
                        1
                    } else {
                        0
                    }
                })
                .sum(),
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        let file = OpenOptions::new()
                            .create_new(true)
                            .write(true)
                            .open(&self.configuration.enlightenment_path())
                            .expect("Failed to open a write buffer.");
                        // Initialise as an empty file
                        write!(&file, "").expect("Failed to initialise enlightenment file.");
                        0
                    }
                    _ => panic!("Cannot read path to enlightenment file."),
                }
            }
        }
    }

    pub fn opened(&self) -> impl Iterator<Item = &Koan> {
        self.koans.iter().take(self.n_opened())
    }

    pub fn next(&self) -> Option<&Koan> {
        let n_opened = self.n_opened();
        if n_opened == self.koans.len() {
            None
        } else {
            Some(&self.koans[n_opened])
        }
    }

    pub fn open_next(&mut self) -> Result<&Koan, ()> {
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .write(true)
            .open(&self.configuration.enlightenment_path())
            .unwrap();

        let koan = self.next();
        if let Some(koan) = koan {
            let koan_filename: String = koan.into();
            let include = format!(
                "#[path = \"koans/{}.rs\"]\nmod {};\n",
                koan_filename, koan.name
            );
            writeln!(file, "{}", include).unwrap();
            Ok(koan)
        } else {
            Err(())
        }
    }
}

#[derive(Clone)]
pub struct Koan {
    pub parent_name: String,
    pub parent_number: String,
    pub name: String,
    pub number: usize,
}

impl std::fmt::Display for Koan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:02}) {} - ({:02}) {}",
            self.parent_number, self.parent_name, self.number, self.name
        )
    }
}

impl From<(OsString, OsString)> for Koan {
    fn from(x: (OsString, OsString)) -> Self {
        let (parent_dir_name, filename) = x;
        let filename = filename.into_string().unwrap();
        let parent_dir_name = parent_dir_name.into_string().unwrap();

        let re = Regex::new(r"(?P<number>\d{2})_(?P<name>\w+)\.rs").unwrap();
        let (name, number) = match re.captures(&filename) {
            None => panic!("Failed to parse koan name."),
            Some(s) => {
                let name = s["name"].into();
                let number = s["number"].parse().unwrap();
                (name, number)
            }
        };

        let re = Regex::new(r"(?P<number>\d{2})_(?P<name>\w+)").unwrap();
        let (parent_name, parent_number) = match re.captures(&parent_dir_name) {
            None => panic!("Failed to parse dir name."),
            Some(s) => {
                let name = s["name"].into();
                let number = s["number"].parse().unwrap();
                (name, number)
            }
        };

        Koan {
            parent_name,
            parent_number,
            name,
            number,
        }
    }
}

impl Into<String> for &Koan {
    fn into(self) -> String {
        format!(
            "{:02}_{}/{:02}_{}",
            &self.parent_number, &self.parent_name, &self.number, &self.name
        )
    }
}
