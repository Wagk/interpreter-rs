mod lexer {
    pub type Tokens = Vec<String>;

    pub fn lex(_s: String) -> Tokens {
        todo!();
    }
}

mod parser {

    #[allow(dead_code)]
    pub enum Form {
        List(Vec<Form>),
        Boolean(bool),
        String(String),
        Integer(i64),
        Double(f64),
    }

    pub type SyntaxTree = Form;

    pub fn parse(_tokens: super::lexer::Tokens) -> SyntaxTree {
        todo!();
    }
}

fn read_string<R: std::io::Read>(mut reader: R) -> String {
    let mut s = String::new();
    let _ = reader.read_to_string(&mut s);
    s
}

fn main() {
    let s = read_string(std::io::stdin());
    let tokens = lexer::lex(s);
    let _ast = parser::parse(tokens);
}
