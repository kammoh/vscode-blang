#![allow(unused_imports, unused_variables)]

extern crate antlr_rust;
extern crate env_logger;
extern crate log;

pub mod bsvlexer;
pub mod bsvlistener;
pub mod bsvparser;

use antlr_rust::token::Token;
use log::{debug, error, info, log_enabled, Level};

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::rule_context::{CustomRuleContext, RuleContext};
use antlr_rust::token_factory::{ArenaCommonFactory, CommonTokenFactory, OwningTokenFactory};
use antlr_rust::tree::{ErrorNode, ParseTree, ParseTreeListener, Tree};
use antlr_rust::InputStream;

use self::bsvlexer::BSVLexer;
use self::bsvparser::BSVParser;

use self::bsvlistener::BSVListener;
use self::bsvparser::{BSVParserContext, BSVParserContextType};

struct Listener {}

impl<'input> ParseTreeListener<'input, BSVParserContextType> for Listener {
    fn visit_error_node(&mut self, node: &ErrorNode<'input, BSVParserContextType>) {
        error!(
            "error in: {:?} {:?}",
            node.get_source_interval(),
            node.symbol.get_text(),
        );
        match node.get_parent() {
            Some(parent) => {
                error!("parent: {:?}", parent);
            }
            None => {}
        }
    }
}

impl<'input> BSVListener<'input> for Listener {}

#[test]
fn parser_test() {
    env_logger::init();

    info!("hello!");

    println!("test started");
    let tf = CommonTokenFactory::default();
    let content = std::fs::read_to_string("/Users/kamyar/src/grain/SIPO.bsv").unwrap();
    let lexer = BSVLexer::new_with_token_factory(InputStream::new(content.as_str().into()), &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = BSVParser::new(token_source);
    parser.add_parse_listener(Box::new(Listener {}));
    println!("\nstart parsing parser_test_csv");
    let result = parser.top();
    assert!(result.is_ok());

    let ast_tree = result.unwrap().to_string_tree(&*parser);

    std::fs::write("bsv.ast", ast_tree).unwrap();
}
