use std::fmt::Display;

use chrono::NaiveDate;
use indexmap::IndexMap;
use quick_xml::{
    Reader,
    events::{BytesStart, Event},
};
use std::str;

#[derive(Debug, Clone)]
pub struct Dynamic {
    name: String,
    fields: IndexMap<String, Values>,
}

impl Dynamic {
    pub fn new(name: String) -> Self {
        Dynamic {
            name,
            fields: IndexMap::new(),
        }
    }

    pub fn put(&mut self, name: String, value: Values) -> &Self {
        self.fields.insert(name, value);
        self
    }

    pub fn decode(xml: String) -> Dynamic {
        let mut reader = Reader::from_str(&xml);
        let mut level = 0i64;
        let mut root = None;
        let mut childs = Vec::new();
        loop {
            let event = match reader.read_event() {
                Ok(ev) => ev,
                Err(_) => Event::Eof,
            };

            if let Event::Eof = event {
                break;
            } else if let Event::Start(e) = event {
                if level == 0 {
                    root = Some(e);
                } else if level == 1 {
                    // Pegar as tag
                    childs.push(e);
                    // Pegar os valores
                } else if level == 2 {
                    todo!("Deve-se fazer uma recursão aqui")
                }
                level += 1;
            } else if let Event::End(_) = event {
                level -= 1;
            }
        }

        if let Some(e) = root {

            childs.iter().for_each(|el|{
                println!("{:#?}", el);
            });

            let bytes: Vec<u8> = e.local_name().as_ref().to_vec();
            Dynamic {
                name: String::from_utf8(bytes).expect("UTF-8 inválido"),
                fields: IndexMap::new(),
            }
        } else {
            Dynamic {
                name: String::new(),
                fields: IndexMap::new(),
            }
        }

    }
}

impl Display for Dynamic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<String> for Dynamic {
    fn from(value: String) -> Self {
        Dynamic::decode(value)
    }
}

impl From<&str> for Dynamic {
    fn from(value: &str) -> Self {
        Dynamic::decode(value.into())
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Values {
    Null,
    String(String),
    Float(f64),
    Integer(i64),
    Date(NaiveDate),
    Node(Dynamic),
}
