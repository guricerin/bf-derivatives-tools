use bf_derivatives_tools::parser;


mod test_data;

#[test]
fn translate_to_bf() {
    use test_data::test_data;

    let mut parser = parser::Parser::new(test_data::BZ_CODE);
    let from_grammar = serde_json::from_str(test_data::BZ_GRAMMAR).unwrap();
    parser.translate_to_bf(&from_grammar);

    assert_eq!(parser.code(), test_data::BF_CODE);
}

#[test]
fn translate_from_bf() {
    use test_data::test_data;

    let mut parser = parser::Parser::new(test_data::BF_CODE);
    let to_grammar = serde_json::from_str(test_data::BZ_GRAMMAR).unwrap();
    parser.translate_from_bf(&to_grammar);

    assert_eq!(parser.code(), test_data::BZ_CODE);
}
