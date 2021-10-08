pub mod bsvlexer;
pub mod bsvlistener;
pub mod bsvparser;


use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::tree::{ParseTree, ParseTreeListener};
use antlr_rust::InputStream;

use self::bsvlexer::BSVLexer;
use self::bsvparser::BSVParser;

use self::bsvlistener::BSVListener;
use self::bsvparser::{BSVParserContext, BSVParserContextType};

struct Listener {}

impl<'input> ParseTreeListener<'input, BSVParserContextType> for Listener {
    fn enter_every_rule(&mut self, ctx: &dyn BSVParserContext<'input>) {
        println!(
            "rule entered {}",
            bsvparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        )
    }
}

impl<'input> BSVListener<'input> for Listener {}

#[test]
fn parser_test_csv() {
    println!("test started");
    let tf = CommonTokenFactory::default();
    let content = std::fs::read_to_string("/Users/kamyar/src/grain/SIPO.bsv").unwrap();
    let lexer = BSVLexer::new_with_token_factory(InputStream::new(content.as_str().into()), &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = BSVParser::new(token_source);
    parser.add_parse_listener(Box::new(Listener {}));
    println!("\nstart parsing parser_test_csv");
    let result = parser.packagedef();
    assert!(result.is_ok());
    println!("Result == {:?}\n", result.unwrap().to_string_tree(&*parser));
}
