use eww_config::{config::*, expr::*, lexer, parser};

fn main() {
    let parser = parser::ExprParser::new();
    let mut files = codespan_reporting::files::SimpleFiles::new();

    let input = "(12 :bar 22 (foo) (baz)";

    let file_id = files.add("foo.eww", input);
    let lexer = lexer::Lexer::new(input);

    let ast = parser.parse(file_id, lexer);
    match ast {
        Ok(ast) => {
            let element: Result<Element<Expr, Expr>, _> = Element::from_expr(ast);
            let err = element.unwrap_err();

            let diag = err.pretty_diagnostic(&files);
            use codespan_reporting::term;
            let mut writer = term::termcolor::StandardStream::stderr(term::termcolor::ColorChoice::Always);
            term::emit(&mut writer, &term::Config::default(), &files, &diag).unwrap();
        }
        Err(err) => eprintln!("{:?}", err),
    }
}
