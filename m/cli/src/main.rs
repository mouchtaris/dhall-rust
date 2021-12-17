fn main() {
    eprintln!("TEST DRIVE");
    pretty_env_logger::init();
    let mut buf = String::new();
    let mut inp = std::fs::File::open("sample.dhall").unwrap();
    std::io::Read::read_to_string(&mut inp, &mut buf).unwrap();
    let mut lex = lex::Lex::new(&buf);
    let r = parse::dhall::ExprParser::new().parse(&mut lex).unwrap();
    eprintln!("{:?}", r);
}
