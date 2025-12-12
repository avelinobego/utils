
#[test]
fn test_macro() {
    use crate::dinamic::Node;
    use crate::node;
    use std::collections::HashMap;

    let e = node!("eSocial"; @attr xmnls = "xpto", salario = "real"; @val 10.2);
    dbg!(e);

    let e = node!("eSocial"; @val 7.3);
    dbg!(e);

    let mut h = HashMap::new();
    h.insert("xmlns", "xpto");
    h.insert("salario", "real");
    
    let e = node!("eSocial"; @attr h);
    dbg!(e);

    let mut h = HashMap::new();
    h.insert("xmlns", "xpto");
    h.insert("salario", "real");
    
    let e = node!("eSocial"; @attr h; @val "2025-12");
    dbg!(e);

    let v = vec![("xmlns", "xpto")];
    let e = node!("eSocial"; @attr v; @val "2025-12");
    dbg!(e);


    let child = node!("child");
    let child2 = node!("child2");
    let vc = vec![child,child2];
    let root = node!("root"; @child vc);
    dbg!(root);

    let child = node!("child"; @val "Avelino Bego");
    let child2 = node!("child2"; @val "2025-12");
    let root = node!("root"; @child [child,child2]);
    dbg!(&root);

    println!("{}", root.to_xml());


}
