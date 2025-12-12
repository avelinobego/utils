#[macro_export]
macro_rules! node {
    ($s:expr) => {
        Node {
            name: $s.to_string(),
            attributes: Vec::default(),
            ..Default::default()
        }
    };

    ($s:expr; @val $val: expr) => {
        Node {
            name: $s.to_string(),
            attributes: Vec::default(),
            value: Some($val.into()),
            ..Default::default()
        }
    };

    ($s:expr; @attr $($name:ident = $at:expr),*) => {
        Node {
            name: $s.to_string(),
            attributes: vec![$((stringify!($name).into(),$at.into())),*],
            ..Default::default()
        }
    };

    ($s:expr; @attr $($name:ident = $at:expr),*; @val $val: expr) => {
        Node {
                name: $s.to_string(),
                attributes: vec![$((stringify!($name).into(),$at.into())),*],
                value: Some($val.into()),
                ..Default::default()
        }
    };

    ($s:expr; @attr $at:expr) => {
        Node {
                name: $s.to_string(),
                attributes: $at.into_iter()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect(),
                ..Default::default()
        }
    };

    ($s:expr; @attr $at:expr; @val $val: expr) => {
        Node {
                name: $s.to_string(),
                attributes: $at.into_iter()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect(),
                value: Some($val.into()),
                ..Default::default()
        }
    };

    ($s:expr; @attr $at:expr; @val $val: expr) => {
        Node {
                name: $s.to_string(),
                attributes: $at,
                value: Some($val.into()),
                ..Default::default()
        }
    };

    ($s:expr; @child [$($val: expr ),*]) => {
        Node {
                name: $s.to_string(),
                attributes: Vec::default(),
                value: None,
                children: vec![$($val),*]
        }
    };

    ($s:expr; @child $val: expr) => {
        Node {
                name: $s.to_string(),
                attributes: Vec::default(),
                value: None,
                children: $val
        }
    };


}
