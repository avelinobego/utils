use std::fmt::Display;

use chrono::NaiveDate;
use quick_xml::{Reader, events::Event};
use std::str;

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub value: Values,
    pub children: Vec<Node>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

//----------------------------------------------------------------------------------------

impl Node {
    pub fn decode(xml: String) -> Option<Node> {
        let mut reader = Reader::from_str(&xml);
        let mut temp = Vec::new();
        let mut stack = Vec::new();
        let mut result = None;
        loop {
            let event = match reader.read_event_into(&mut temp) {
                Ok(ev) => ev,
                Err(_) => Event::Eof,
            };
            
            if let Event::Eof = event {
                break;
            } else if let Event::Start(e) = event {
                let name = String::from_utf8_lossy(e.name().into_inner()).into_owned();
                let mut attributes = Vec::new();
                e.attributes()
                    .map(|a| {
                        let attr = a.unwrap();
                        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                        let value = attr.unescape_value().unwrap().into_owned();
                        (key, value)
                    })
                    .for_each(|f| {
                        attributes.push(f);
                    });

                let node = Node {
                    name,
                    attributes,
                    value: Values::None,
                    children: Vec::default(),
                };
                stack.push(node);
            } else if let Event::Text(e) = event {
                if let Some(current) = stack.last_mut() {
                    let text = String::from_utf8_lossy(e.as_ref());
                    if text.trim().is_empty() {
                        current.value = Values::None;
                    } else {
                        current.value = text.as_ref().into();
                    }
                }
            } else if let Event::End(_) = event {
                let finished = stack.pop();
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(finished.unwrap());
                } else {
                    result = finished;
                }
            }

            temp.clear();
        }

        result
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::decode(value).expect("error on build node")
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::decode(value.into()).expect("error on build node")
    }
}

//----------------------------------------------------------------------------------------
#[derive(Debug, Clone, Default)]
pub enum Values {
    #[default]
    None,
    String(String),
    Float(f64),
    Integer(i64),
    Date(NaiveDate),
}

impl From<&str> for Values {
    fn from(value: &str) -> Self {
        convert()
    }
} 

impl From<String> for Values {
    fn from(value: String) -> Self {
        convert()
    }
}

fn convert() -> Values {
   Values::default() 
}