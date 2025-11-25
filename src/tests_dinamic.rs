#[test]
fn test_dinamic() {
    use crate::dinamic::Node;

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
