use failure::Error;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};
use xml::ParserConfig;

#[derive(Clone, Debug)]
pub struct DatFile {
    pub header: String,
    pub version: String,
    pub names: HashSet<String>,
}

pub fn from_path<P: AsRef<Path>>(path: P) -> Result<DatFile, Error> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    from_reader(file)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    None,
    Header,
    Game,
}

pub fn from_reader<R: Read>(reader: R) -> Result<DatFile, Error> {
    let config = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(true);
    let parser = EventReader::new_with_config(reader, config);
    let mut buffer = None;
    let mut result = DatFile {
        header: String::new(),
        version: String::new(),
        names: HashSet::new(),
    };
    let mut state = State::None;
    for e in parser {
        match e? {
            XmlEvent::StartElement { name, .. } => match name.borrow().local_name {
                "header" => state = State::Header,
                "game" => state = State::Game,
                "version" => {
                    if state == State::Header {
                        buffer = Some(String::new());
                    } else {
                        buffer = None;
                    }
                }
                "description" => {
                    if state == State::None {
                        buffer = None;
                    } else {
                        buffer = Some(String::new());
                    }
                }
                _ => (),
            },
            XmlEvent::EndElement { name, .. } => match name.borrow().local_name {
                "header" | "game" => state = State::None,
                "version" => {
                    if let Some(buffer) = buffer.take() {
                        result.version = buffer;
                    }
                }
                "description" => {
                    if let Some(buffer) = buffer.take() {
                        if buffer.len() > 0 {
                            match state {
                                State::Header => result.header = buffer,
                                State::Game => {
                                    result.names.insert(buffer);
                                }
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            },
            XmlEvent::Characters(ref text) if buffer.is_some() => {
                if let Some(buffer) = buffer.as_mut() {
                    buffer.push_str(text);
                }
            }
            _ => (),
        }
    }
    Ok(result)
}
