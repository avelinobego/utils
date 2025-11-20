#[test]
fn test_dinamic() {
    use crate::dinamic::{Dynamic, Values};

    let mut din = Dynamic::new("dinamico".into());
    din.put("nome".into(), Values::String("Avelino".into()));
    din.put("sobrenome".into(), Values::String("Bego".into()));
    din.put("idade".into(), Values::Integer(53));
    din.put(
        "nascimento".into(),
        Values::Date(chrono::NaiveDate::from_ymd_opt(1971, 12, 27).unwrap()),
    );

    // din.put("interno".into(), Values::Node(din.clone()));

    println!("{:#?}", din);

    // let din: Dynamic = r#"<indentificacao>
    // <nome>Fabiana</nome>
    // <sobrenome>Bego</sobrenome>
    // </indentificacao>"#.into();

    let din: Dynamic = r#"<indentificacao><cpf>16944301890</cpf><cpf>16944301890</cpf>
    </indentificacao>"#.into();


    println!("{:#?}", din);

}
