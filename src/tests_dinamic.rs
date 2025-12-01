
#[test]
fn test_from_xml() {
    use crate::dinamic::Node;
    use std::fs;
    
    let conteudo = fs::read_to_string("S1000.xml").expect("erro ao abrir arquivo");
    let din: Node = conteudo.into();
    
    println!("{:#?}", din);
}

#[test]
fn test_to_xml_float(){
    use crate::dinamic::Node;
    
    let node = Node{
        name: "salario".into(),
        value: 10000.1234567890.into(),
        attributes: Vec::default(),
        children: Vec::default(),
    };

    println!("{}", node);
}

#[test]
fn test_to_xml_year_month(){
    use crate::dinamic::Node;
    
    let node = Node{
        name: "data".into(),
        value: (2025, 12).into(),
        attributes: Vec::default(),
        children: Vec::default(),
    };

    println!("{}", node);
}

#[test]
fn test_to_xml_year_month_day(){
    use crate::dinamic::Node;
    
    let node = Node{
        name: "data".into(),
        value: (2025, 12, 1).into(),
        attributes: Vec::default(),
        children: Vec::default(),
    };

    println!("{}", node);
}

#[test]
fn test_to_xml_integer(){
    use crate::dinamic::Node;
    
    let node = Node{
        name: "inteiro".into(),
        value: 12.into(),
        attributes: Vec::default(),
        children: Vec::default(),
    };

    println!("{}", node);
}

#[test]
fn test_to_xml_string(){
    use crate::dinamic::Node;
    
    let node = Node{
        name: "nome".into(),
        value: "Avelino Bego".into(),
        attributes: Vec::default(),
        children: Vec::default(),
    };

    println!("{}", node);
}

#[test]
fn test_to_xml_none(){
    use crate::dinamic::Node;
    
    let node = Node{
        name: "vazio".into(),
        value: "".into(),
        attributes: Vec::default(),
        children: Vec::default(),
    };

    println!("{}", node);
}
