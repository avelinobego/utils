

#[test]
fn test_dinamic() {
    use crate::dinamic::Node;
    use std::fs;

    // let conteudo = fs::read_to_string("S1000.xml").expect("erro ao abrir arquivo");
    // let din: Node = conteudo.into();

        let din: Node = r#"<indentificacao id="1" xmnls="xpto">
        <nome>Avelino</nome>
        <emprego/>
        <salario ano="2025">10000.00</salario>
        <demissao>2025-11-17</demissao>
        <identificadores id="identific">
        <cpf>16944301890</cpf>
        <cpf>16944301890</cpf>
        </identificadores>
    </indentificacao>"#
            .into();


    println!("{:#?}", din);

}
