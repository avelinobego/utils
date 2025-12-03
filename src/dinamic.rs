use std::fmt::Display;

use once_cell::sync::Lazy;
use quick_xml::{
    Reader, Writer,
    events::{BytesEnd, BytesStart, BytesText, Event},
};
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
    pub fn from_xml(xml: String) -> Option<Node> {
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

    pub fn to_xml(&self) -> String {
        let mut buffer = Vec::new();
        self.do_to_xml(&mut buffer);
        String::from_utf8_lossy(buffer.as_slice()).to_string()
    }

    fn do_to_xml(&self, buff_out: &mut Vec<u8>) {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = Writer::new(&mut buffer);
        let start = BytesStart::new(&self.name);

        if let Values::None = &self.value {
            writer
                .write_event(Event::Empty(start))
                .expect("event error");
        } else {
            writer
                .write_event(Event::Start(start.clone()))
                .expect("event error");

            if let Values::String(s) = &self.value {
                writer
                    .write_event(Event::Text(BytesText::new(s)))
                    .expect("event error");
            }

            writer
                .write_event(Event::End(BytesEnd::new(&self.name)))
                .expect("event error");

            self.children.iter().for_each(|e| {
                e.do_to_xml(buff_out);
            });
        }

        buff_out.extend_from_slice(buffer.as_slice());
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_xml())
    }
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::from_xml(value).expect("error on build node")
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::from_xml(value.into()).expect("error on build node")
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
    YearMonth(u64, u64),
    YearMonthDay(u64, u64, u64),
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
            let year = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let month = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
            Values::YearMonth(year, month)
        } else if let Some(cap) = RE2.captures(&value) {
            let year = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let month = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let day = cap.get(3).unwrap().as_str().parse::<u64>().unwrap();
            Values::YearMonthDay(year, month, day)
        } else {
            Values::String(value)
        }
    }
}

impl From<f64> for Values {
    fn from(value: f64) -> Self {
        Values::Float(value)
    }
}

impl From<(u64, u64)> for Values {
    fn from(value: (u64, u64)) -> Self {
        Values::YearMonth(value.0, value.1)
    }
}

impl From<(u64, u64, u64)> for Values {
    fn from(value: (u64, u64, u64)) -> Self {
        Values::YearMonthDay(value.0, value.1, value.2)
    }
}

impl From<i64> for Values {
    fn from(value: i64) -> Self {
        Values::Integer(value)
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Values::Float(v) = self {
            write!(f, "{:.2}", v)
        } else if let Values::YearMonth(year, month) = self {
            write!(f, "{}-{}", year, month)
        } else if let Values::YearMonthDay(year, month, day) = self {
            write!(f, "{}-{}-{}", year, month, day)
        } else if let Values::Integer(i) = self {
            write!(f, "{}", i)
        } else if let Values::String(s) = self {
            write!(f, "{}", s)
        } else if let Values::None = self {
            write!(f, "")
        } else {
            write!(f, "não implementado")
        }
    }
}
