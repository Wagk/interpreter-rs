mod lexer {}

mod parser {}

fn read_string<R: std::io::Read>(mut reader: R) -> String {
    let mut s = String::new();
    let _ = reader.read_to_string(&mut s);
    s
}

fn main() {
    let s = read_string(std::io::stdin());
}
