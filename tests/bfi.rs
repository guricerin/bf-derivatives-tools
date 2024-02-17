use bf_derivatives_tools::{interpreter, parser};
use serde_json;
use std::io;

mod test_data;

struct PseudoStdin {}

impl io::Read for PseudoStdin {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize, io::Error> {
        Ok(0)
    }
}

#[test]
fn interpret_bz_code() {
    use test_data::test_data;

    let mut parser = parser::Parser::new(test_data::BZ_CODE);
    let from_grammar = serde_json::from_str(test_data::BZ_GRAMMAR).unwrap();
    parser.translate_to_bf(&from_grammar);

    let tokens = parser.parse();
    let reader = PseudoStdin {};
    let writer = Vec::<u8>::new();
    let mut interpreter = interpreter::Interpreter::new(30000, reader, writer);
    let result = interpreter.run(&tokens);

    assert!(result.is_ok());
}
