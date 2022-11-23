use std::fmt;
use std::fmt::Display;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct ModConfig {
    pub enabled: bool,
    pub include: Option<Vec<String>>,
    pub dll: Option<Vec<String>>,

    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub date: Option<String>,
    pub author: Option<String>,
}

fn check_field<T: Display>(f: &mut std::fmt::Formatter<'_>, name: &str, infield: &Option<T>) {
    if !infield.is_some() {
        return;
    }
    write!(f, "{}: \"{}\", ", name, infield.as_ref().unwrap()).unwrap();
    return;
}

impl fmt::Debug for ModConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModConfig {{ enabled: {}, ", self.enabled)?;
        check_field(f, "name", &self.name);
        check_field(f, "description", &self.description);
        check_field(f, "version", &self.version);
        check_field(f, "date", &self.date);
        check_field(f, "author", &self.author);

        if self.include.is_some() {
            write!(f, "include: [")?;
            for i in self.include.as_ref().unwrap() {
                write!(f, "{}, ", i)?;
            }
            write!(f, "], ")?;
        }
        if self.dll.is_some() {
            write!(f, "dll: [")?;
            for i in self.dll.as_ref().unwrap() {
                write!(f, "{}, ", i)?;
            }
            write!(f, "], ")?;
        }

        write!(f, "}}")
    }
}