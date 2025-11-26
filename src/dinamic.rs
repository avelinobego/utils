use std::fmt::Display;

use once_cell::sync::Lazy;
use quick_xml::{Reader, events::Event};
use regex::Regex;
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
static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d{4})-(\d{2})$").unwrap());
static RE2: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap());

#[derive(Debug, Clone, Default)]
pub enum Values {
    #[default]
    None,
    String(String),
    Float(f64),
    Integer(i64),
    YearMonth(i32, i32),
    YearMonthDay(i32, i32, i32),
}

impl From<&str> for Values {
    fn from(value: &str) -> Self {
        let temp: Values = String::from(value).into();
        temp
    }
}

impl From<String> for Values {
    fn from(value: String) -> Self {
        if value.is_empty() {
            Values::None
        } else if let Ok(v) = value.parse::<i64>() {
            Values::Integer(v)
        } else if let Ok(v) = value.parse::<f64>() {
            Values::Float(v)
        } else if let Some(cap) = RE.captures(&value) {
            let year = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let month = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            Values::YearMonth(year, month)
        } else if let Some(cap) = RE2.captures(&value) {
            let year = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let month = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let day = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
            Values::YearMonthDay(year, month, day)
        } else {
            Values::String(value)
        }
    }
}
