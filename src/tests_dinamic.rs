#[test]
fn test_to_xml_float() {
    use crate::dinamic::Node;

    let node = Node {
        name: "salario".into(),
        value: Some(10000.1234567890.into()),
        attributes: Vec::default(),
        children: Vec::default(),
    };

    assert_eq!(node.to_xml(), "<salario>10000.123456789</salario>");
}

#[test]
fn test_to_xml_year_month() {
    use crate::dinamic::Node;

    let node = Node {
        name: "data".into(),
        value: Some((2025, 1).into()),
        attributes: Vec::default(),
        children: Vec::default(),
    };

    assert_eq!(node.to_xml(), "<data>2025-01</data>");
}

#[test]
fn test_to_xml_year_month_day() {
    use crate::dinamic::Node;

    let node = Node {
        name: "data".into(),
        value: Some((2025, 1, 1).into()),
        attributes: Vec::default(),
        children: Vec::default(),
    };

    assert_eq!(node.to_xml(), "<data>2025-01-01</data>");
}

#[test]
fn test_to_xml_integer() {
    use crate::dinamic::Node;

    let node = Node {
        name: "inteiro".into(),
        value: Some(12.into()),
        attributes: Vec::default(),
        children: Vec::default(),
    };

    assert_eq!(node.to_xml(), "<inteiro>12</inteiro>");
}

#[test]
fn test_to_xml_string() {
    use crate::dinamic::Node;

    let node = Node {
        name: "nome".into(),
        value: Some("Avelino Bego".into()),
        attributes: Vec::default(),
        children: Vec::default(),
    };

    assert_eq!(node.to_xml(), "<nome>Avelino Bego</nome>");
}

#[test]
fn test_to_xml_none() {
    use crate::dinamic::Node;

    let node = Node {
        name: "vazio".into(),
        value: None,
        attributes: Vec::default(),
        children: Vec::default(),
    };

    assert_eq!(node.to_xml(), "<vazio/>");
}

#[test]
fn test_to_xml_float_attr() {
    use crate::dinamic::Node;

    let node = Node {
        name: "salario".into(),
        value: Some(10000.1234567890.into()),
        attributes: vec![
            ("tipo".into(), "mensal".into()),
            ("mes".into(), "janeiro".into()),
        ],
        children: Vec::default(),
    };

    assert_eq!(
        node.to_xml(),
        r#"<salario tipo="mensal" mes="janeiro">10000.123456789</salario>"#
    );
}

#[test]
fn test_to_xml_empty() {
    use crate::dinamic::Node;

    let node = Node {
        name: "sem_valor".into(),
        value: Some("".into()),
        attributes: vec![],
        children: Vec::default(),
    };

    assert_eq!(node.to_xml(), r#"<sem_valor></sem_valor>"#);
}

#[test]
fn test_to_xml_float_attr_children() {
    use crate::dinamic::Node;

    let nome = Node {
        name: "nome".into(),
        value: Some("Avelino Bego".into()),
        ..Default::default()
    };

    let profissao = Node {
        name: "profissao".into(),
        value: Some("Desenvolvedor".into()),
        ..Default::default()
    };

    let node = Node {
        name: "salario".into(),
        value: None,
        attributes: vec![
            ("tipo".into(), "mensal".into()),
            ("mes".into(), "janeiro".into()),
            ("valor".into(), "10000.00".into()),
        ],
        children: vec![nome, profissao],
    };

    assert_eq!(
        node.to_xml(),
        r#"<salario tipo="mensal" mes="janeiro" valor="10000.00"><nome>Avelino Bego</nome><profissao>Desenvolvedor</profissao></salario>"#
    );
}

#[test]
fn test_from_xml() {
    use crate::dinamic::Node;
    use crate::dinamic::Values;

    let node: Node = r#"<root><interno><nome>Avelino</nome></interno></root>"#.into();

    assert_eq!(
        node.children[0].children[0].value,
        Some(Values::String("Avelino".into()))
    )
}
