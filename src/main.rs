mod parser;
mod analyzer;

fn main() {
    print!("{}", 1);
    let lexer = parser::lexer::Lexer::new("SELECT col1 FROM table1");

    let ast = parser::Parser::new(lexer).parse();

    // let logical_plan = analyzer::Analyzer::new().analyze(&ast);

    print!("{:?}", ast);
}