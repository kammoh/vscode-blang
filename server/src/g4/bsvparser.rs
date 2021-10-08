// Generated from BSV.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::bsvlistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const T__15:isize=16; 
		pub const T__16:isize=17; 
		pub const T__17:isize=18; 
		pub const T__18:isize=19; 
		pub const T__19:isize=20; 
		pub const T__20:isize=21; 
		pub const T__21:isize=22; 
		pub const T__22:isize=23; 
		pub const T__23:isize=24; 
		pub const T__24:isize=25; 
		pub const T__25:isize=26; 
		pub const T__26:isize=27; 
		pub const T__27:isize=28; 
		pub const T__28:isize=29; 
		pub const T__29:isize=30; 
		pub const T__30:isize=31; 
		pub const T__31:isize=32; 
		pub const T__32:isize=33; 
		pub const T__33:isize=34; 
		pub const T__34:isize=35; 
		pub const T__35:isize=36; 
		pub const T__36:isize=37; 
		pub const T__37:isize=38; 
		pub const T__38:isize=39; 
		pub const T__39:isize=40; 
		pub const T__40:isize=41; 
		pub const T__41:isize=42; 
		pub const T__42:isize=43; 
		pub const T__43:isize=44; 
		pub const T__44:isize=45; 
		pub const T__45:isize=46; 
		pub const T__46:isize=47; 
		pub const T__47:isize=48; 
		pub const T__48:isize=49; 
		pub const T__49:isize=50; 
		pub const T__50:isize=51; 
		pub const T__51:isize=52; 
		pub const T__52:isize=53; 
		pub const T__53:isize=54; 
		pub const T__54:isize=55; 
		pub const T__55:isize=56; 
		pub const T__56:isize=57; 
		pub const T__57:isize=58; 
		pub const T__58:isize=59; 
		pub const T__59:isize=60; 
		pub const T__60:isize=61; 
		pub const T__61:isize=62; 
		pub const T__62:isize=63; 
		pub const T__63:isize=64; 
		pub const T__64:isize=65; 
		pub const T__65:isize=66; 
		pub const T__66:isize=67; 
		pub const T__67:isize=68; 
		pub const T__68:isize=69; 
		pub const T__69:isize=70; 
		pub const T__70:isize=71; 
		pub const T__71:isize=72; 
		pub const T__72:isize=73; 
		pub const T__73:isize=74; 
		pub const T__74:isize=75; 
		pub const T__75:isize=76; 
		pub const T__76:isize=77; 
		pub const T__77:isize=78; 
		pub const T__78:isize=79; 
		pub const T__79:isize=80; 
		pub const T__80:isize=81; 
		pub const T__81:isize=82; 
		pub const T__82:isize=83; 
		pub const T__83:isize=84; 
		pub const T__84:isize=85; 
		pub const T__85:isize=86; 
		pub const T__86:isize=87; 
		pub const T__87:isize=88; 
		pub const T__88:isize=89; 
		pub const T__89:isize=90; 
		pub const T__90:isize=91; 
		pub const T__91:isize=92; 
		pub const T__92:isize=93; 
		pub const T__93:isize=94; 
		pub const T__94:isize=95; 
		pub const T__95:isize=96; 
		pub const T__96:isize=97; 
		pub const T__97:isize=98; 
		pub const T__98:isize=99; 
		pub const T__99:isize=100; 
		pub const T__100:isize=101; 
		pub const T__101:isize=102; 
		pub const T__102:isize=103; 
		pub const PPTOK:isize=104; 
		pub const UpperCaseIdentifier:isize=105; 
		pub const LowerCaseIdentifier:isize=106; 
		pub const DollarIdentifier:isize=107; 
		pub const EscapedOperator:isize=108; 
		pub const IntLiteral:isize=109; 
		pub const IntPattern:isize=110; 
		pub const RealLiteral:isize=111; 
		pub const StringLiteral:isize=112; 
		pub const WS:isize=113; 
		pub const ONE_LINE_COMMENT:isize=114; 
		pub const INLINE_COMMENT:isize=115;
	pub const RULE_packagedef:usize = 0; 
	pub const RULE_packagedecl:usize = 1; 
	pub const RULE_endpackage:usize = 2; 
	pub const RULE_lowerCaseIdentifier:usize = 3; 
	pub const RULE_upperCaseIdentifier:usize = 4; 
	pub const RULE_anyidentifier:usize = 5; 
	pub const RULE_exportdecl:usize = 6; 
	pub const RULE_exportitem:usize = 7; 
	pub const RULE_importdecl:usize = 8; 
	pub const RULE_packagestmt:usize = 9; 
	pub const RULE_packageide:usize = 10; 
	pub const RULE_interfacedecl:usize = 11; 
	pub const RULE_interfacememberdecl:usize = 12; 
	pub const RULE_methodproto:usize = 13; 
	pub const RULE_methodprotoformals:usize = 14; 
	pub const RULE_methodprotoformal:usize = 15; 
	pub const RULE_subinterfacedecl:usize = 16; 
	pub const RULE_typedeftype:usize = 17; 
	pub const RULE_typeformals:usize = 18; 
	pub const RULE_typeformal:usize = 19; 
	pub const RULE_typedefsynonym:usize = 20; 
	pub const RULE_typedefenum:usize = 21; 
	pub const RULE_typedefenumelement:usize = 22; 
	pub const RULE_typedefstruct:usize = 23; 
	pub const RULE_typedeftaggedunion:usize = 24; 
	pub const RULE_structmember:usize = 25; 
	pub const RULE_unionmember:usize = 26; 
	pub const RULE_substruct:usize = 27; 
	pub const RULE_subunion:usize = 28; 
	pub const RULE_derives:usize = 29; 
	pub const RULE_moduleinst:usize = 30; 
	pub const RULE_tuplebind:usize = 31; 
	pub const RULE_varinit:usize = 32; 
	pub const RULE_varbinding:usize = 33; 
	pub const RULE_actionbinding:usize = 34; 
	pub const RULE_patternbinding:usize = 35; 
	pub const RULE_typeclassdecl:usize = 36; 
	pub const RULE_typeclasside:usize = 37; 
	pub const RULE_typedepends:usize = 38; 
	pub const RULE_typedepend:usize = 39; 
	pub const RULE_typelist:usize = 40; 
	pub const RULE_overloadeddecl:usize = 41; 
	pub const RULE_tctype:usize = 42; 
	pub const RULE_typeclassinstance:usize = 43; 
	pub const RULE_overloadeddef:usize = 44; 
	pub const RULE_moduledef:usize = 45; 
	pub const RULE_moduleproto:usize = 46; 
	pub const RULE_moduleprotoformals:usize = 47; 
	pub const RULE_moduleprotoformal:usize = 48; 
	pub const RULE_modulestmt:usize = 49; 
	pub const RULE_methoddef:usize = 50; 
	pub const RULE_methodformals:usize = 51; 
	pub const RULE_methodformal:usize = 52; 
	pub const RULE_methodcond:usize = 53; 
	pub const RULE_subinterfacedef:usize = 54; 
	pub const RULE_ruledef:usize = 55; 
	pub const RULE_rulecond:usize = 56; 
	pub const RULE_functiondef:usize = 57; 
	pub const RULE_functionproto:usize = 58; 
	pub const RULE_externcimport:usize = 59; 
	pub const RULE_externcfuncargs:usize = 60; 
	pub const RULE_externcfuncarg:usize = 61; 
	pub const RULE_varassign:usize = 62; 
	pub const RULE_lvalue:usize = 63; 
	pub const RULE_bsvtype:usize = 64; 
	pub const RULE_typeide:usize = 65; 
	pub const RULE_typenat:usize = 66; 
	pub const RULE_expression:usize = 67; 
	pub const RULE_caseexprpatitem:usize = 68; 
	pub const RULE_caseexpritem:usize = 69; 
	pub const RULE_caseexprdefaultitem:usize = 70; 
	pub const RULE_patterncond:usize = 71; 
	pub const RULE_binopexpr:usize = 72; 
	pub const RULE_unopexpr:usize = 73; 
	pub const RULE_exprprimary:usize = 74; 
	pub const RULE_memberbinds:usize = 75; 
	pub const RULE_memberbind:usize = 76; 
	pub const RULE_interfacestmt:usize = 77; 
	pub const RULE_beginendblock:usize = 78; 
	pub const RULE_actionblock:usize = 79; 
	pub const RULE_actionvalueblock:usize = 80; 
	pub const RULE_regwrite:usize = 81; 
	pub const RULE_stmt:usize = 82; 
	pub const RULE_ifstmt:usize = 83; 
	pub const RULE_casestmt:usize = 84; 
	pub const RULE_casestmtitem:usize = 85; 
	pub const RULE_casestmtpatitem:usize = 86; 
	pub const RULE_casestmtdefaultitem:usize = 87; 
	pub const RULE_whilestmt:usize = 88; 
	pub const RULE_forstmt:usize = 89; 
	pub const RULE_forinit:usize = 90; 
	pub const RULE_simplevardeclassign:usize = 91; 
	pub const RULE_fortest:usize = 92; 
	pub const RULE_forincr:usize = 93; 
	pub const RULE_varincr:usize = 94; 
	pub const RULE_returnstmt:usize = 95; 
	pub const RULE_pattern:usize = 96; 
	pub const RULE_constantpattern:usize = 97; 
	pub const RULE_taggedunionpattern:usize = 98; 
	pub const RULE_tuplepattern:usize = 99; 
	pub const RULE_attributeinstance:usize = 100; 
	pub const RULE_attrspec:usize = 101; 
	pub const RULE_provisos:usize = 102; 
	pub const RULE_proviso:usize = 103;
	pub const ruleNames: [&'static str; 104] =  [
		"packagedef", "packagedecl", "endpackage", "lowerCaseIdentifier", "upperCaseIdentifier", 
		"anyidentifier", "exportdecl", "exportitem", "importdecl", "packagestmt", 
		"packageide", "interfacedecl", "interfacememberdecl", "methodproto", "methodprotoformals", 
		"methodprotoformal", "subinterfacedecl", "typedeftype", "typeformals", 
		"typeformal", "typedefsynonym", "typedefenum", "typedefenumelement", "typedefstruct", 
		"typedeftaggedunion", "structmember", "unionmember", "substruct", "subunion", 
		"derives", "moduleinst", "tuplebind", "varinit", "varbinding", "actionbinding", 
		"patternbinding", "typeclassdecl", "typeclasside", "typedepends", "typedepend", 
		"typelist", "overloadeddecl", "tctype", "typeclassinstance", "overloadeddef", 
		"moduledef", "moduleproto", "moduleprotoformals", "moduleprotoformal", 
		"modulestmt", "methoddef", "methodformals", "methodformal", "methodcond", 
		"subinterfacedef", "ruledef", "rulecond", "functiondef", "functionproto", 
		"externcimport", "externcfuncargs", "externcfuncarg", "varassign", "lvalue", 
		"bsvtype", "typeide", "typenat", "expression", "caseexprpatitem", "caseexpritem", 
		"caseexprdefaultitem", "patterncond", "binopexpr", "unopexpr", "exprprimary", 
		"memberbinds", "memberbind", "interfacestmt", "beginendblock", "actionblock", 
		"actionvalueblock", "regwrite", "stmt", "ifstmt", "casestmt", "casestmtitem", 
		"casestmtpatitem", "casestmtdefaultitem", "whilestmt", "forstmt", "forinit", 
		"simplevardeclassign", "fortest", "forincr", "varincr", "returnstmt", 
		"pattern", "constantpattern", "taggedunionpattern", "tuplepattern", "attributeinstance", 
		"attrspec", "provisos", "proviso"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;104] = [
		None, Some("'package'"), Some("';'"), Some("'endpackage'"), Some("':'"), 
		Some("'export'"), Some("','"), Some("'::'"), Some("'*'"), Some("'('"), 
		Some("'..'"), Some("')'"), Some("'import'"), Some("'interface'"), Some("'endinterface'"), 
		Some("'method'"), Some("'#'"), Some("'numeric'"), Some("'type'"), Some("'typedef'"), 
		Some("'enum'"), Some("'{'"), Some("'}'"), Some("'['"), Some("']'"), Some("'='"), 
		Some("'struct'"), Some("'union'"), Some("'tagged'"), Some("'deriving'"), 
		Some("'let'"), Some("'new'"), Some("'<-'"), Some("'match'"), Some("'typeclass'"), 
		Some("'endtypeclass'"), Some("'dependencies'"), Some("'determines'"), 
		Some("'instance'"), Some("'endinstance'"), Some("'endmodule'"), Some("'module'"), 
		Some("'parameter'"), Some("'endmethod'"), Some("'when'"), Some("'if'"), 
		Some("'rule'"), Some("'endrule'"), Some("'endfunction'"), Some("'function'"), 
		Some("'\"BDPI\"'"), Some("'.'"), Some("'+:'"), Some("'-:'"), Some("'?'"), 
		Some("'matches'"), Some("'case'"), Some("'endcase'"), Some("'default'"), 
		Some("'&&&'"), Some("'**'"), Some("'/'"), Some("'%'"), Some("'+'"), Some("'-'"), 
		Some("'<<'"), Some("'>>'"), Some("'<'"), Some("'<='"), Some("'>'"), Some("'>='"), 
		Some("'=='"), Some("'!='"), Some("'&'"), Some("'^'"), Some("'^~'"), Some("'~^'"), 
		Some("'|'"), Some("'&&'"), Some("'||'"), Some("'!'"), Some("'~'"), Some("'~&'"), 
		Some("'~|'"), Some("'''"), Some("'valueOf'"), Some("'valueof'"), Some("'clocked_by'"), 
		Some("'reset_by'"), Some("'\u{2019}'"), Some("'begin'"), Some("'end'"), 
		Some("'action'"), Some("'endaction'"), Some("'actionvalue'"), Some("'endactionvalue'"), 
		Some("'else'"), Some("'while'"), Some("'for'"), Some("'return'"), Some("'.*'"), 
		Some("'(*'"), Some("'*)'"), Some("'provisos'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;116]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, Some("PPTOK"), Some("UpperCaseIdentifier"), 
		Some("LowerCaseIdentifier"), Some("DollarIdentifier"), Some("EscapedOperator"), 
		Some("IntLiteral"), Some("IntPattern"), Some("RealLiteral"), Some("StringLiteral"), 
		Some("WS"), Some("ONE_LINE_COMMENT"), Some("INLINE_COMMENT")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,BSVParserExt, I, BSVParserContextType , dyn BSVListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type BSVTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, BSVParserContextType , dyn BSVListener<'input> + 'a>;

/// Parser for BSV grammar
pub struct BSVParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				BSVParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> BSVParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> BSVParser<'input, I, DefaultErrorStrategy<'input,BSVParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for BSVParser
pub trait BSVParserContext<'input>:
	for<'x> Listenable<dyn BSVListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=BSVParserContextType>
{}

impl<'input> BSVParserContext<'input> for TerminalNode<'input,BSVParserContextType> {}
impl<'input> BSVParserContext<'input> for ErrorNode<'input,BSVParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn BSVParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn BSVListener<'input> + 'input{}

pub struct BSVParserContextType;
antlr_rust::type_id!{BSVParserContextType}

impl<'input> ParserNodeType<'input> for BSVParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn BSVParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct BSVParserExt{
}

impl BSVParserExt{
}


impl<'input> TokenAware<'input> for BSVParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for BSVParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for BSVParserExt{
	fn get_grammar_file_name(&self) -> & str{ "BSV.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn BSVParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					67 => BSVParser::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					72 => BSVParser::<'input,I,_>::binopexpr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					74 => BSVParser::<'input,I,_>::exprprimary_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> BSVParser<'input, I, DefaultErrorStrategy<'input,BSVParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 4)
				}
				1=>{
					recog.precpred(None, 3)
				}
			_ => true
		}
	}
	fn binopexpr_sempred(_localctx: Option<&BinopexprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				2=>{
					recog.precpred(None, 11)
				}
				3=>{
					recog.precpred(None, 10)
				}
				4=>{
					recog.precpred(None, 9)
				}
				5=>{
					recog.precpred(None, 8)
				}
				6=>{
					recog.precpred(None, 7)
				}
				7=>{
					recog.precpred(None, 6)
				}
				8=>{
					recog.precpred(None, 5)
				}
				9=>{
					recog.precpred(None, 4)
				}
				10=>{
					recog.precpred(None, 3)
				}
				11=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn exprprimary_sempred(_localctx: Option<&ExprprimaryContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				12=>{
					recog.precpred(None, 20)
				}
				13=>{
					recog.precpred(None, 11)
				}
				14=>{
					recog.precpred(None, 10)
				}
			_ => true
		}
	}
}
//------------------- packagedef ----------------
pub type PackagedefContextAll<'input> = PackagedefContext<'input>;


pub type PackagedefContext<'input> = BaseParserRuleContext<'input,PackagedefContextExt<'input>>;

#[derive(Clone)]
pub struct PackagedefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for PackagedefContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for PackagedefContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_packagedef(self);
	}
}

impl<'input> CustomRuleContext<'input> for PackagedefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_packagedef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_packagedef }
}
antlr_rust::type_id!{PackagedefContextExt<'a>}

impl<'input> PackagedefContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PackagedefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PackagedefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PackagedefContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<PackagedefContextExt<'input>>{

fn packagedecl(&self) -> Option<Rc<PackagedeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn packagestmt_all(&self) ->  Vec<Rc<PackagestmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn packagestmt(&self, i: usize) -> Option<Rc<PackagestmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn endpackage(&self) -> Option<Rc<EndpackageContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PackagedefContextAttrs<'input> for PackagedefContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn packagedef(&mut self,)
	-> Result<Rc<PackagedefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PackagedefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_packagedef);
        let mut _localctx: Rc<PackagedefContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(227);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__0 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule packagedecl*/
					recog.base.set_state(208);
					recog.packagedecl()?;

					recog.base.set_state(212);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__4) | (1usize << T__8) | (1usize << T__11) | (1usize << T__12) | (1usize << T__18) | (1usize << T__29) | (1usize << T__33) | (1usize << T__37) | (1usize << T__40) | (1usize << T__48))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (T__100 - 101)) | (1usize << (UpperCaseIdentifier - 101)) | (1usize << (LowerCaseIdentifier - 101)) | (1usize << (EscapedOperator - 101)) | (1usize << (IntLiteral - 101)))) != 0) {
						{
						{
						/*InvokeRule packagestmt*/
						recog.base.set_state(209);
						recog.packagestmt()?;

						}
						}
						recog.base.set_state(214);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(216);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__2 {
						{
						/*InvokeRule endpackage*/
						recog.base.set_state(215);
						recog.endpackage()?;

						}
					}

					recog.base.set_state(218);
					recog.base.match_token(EOF,&mut recog.err_handler)?;

					}
				}

			 EOF | T__4 | T__8 | T__11 | T__12 | T__18 | T__29 | T__33 | T__37 | T__40 |
			 T__48 | T__100 | UpperCaseIdentifier | LowerCaseIdentifier | EscapedOperator |
			 IntLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(223);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__4) | (1usize << T__8) | (1usize << T__11) | (1usize << T__12) | (1usize << T__18) | (1usize << T__29) | (1usize << T__33) | (1usize << T__37) | (1usize << T__40) | (1usize << T__48))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (T__100 - 101)) | (1usize << (UpperCaseIdentifier - 101)) | (1usize << (LowerCaseIdentifier - 101)) | (1usize << (EscapedOperator - 101)) | (1usize << (IntLiteral - 101)))) != 0) {
						{
						{
						/*InvokeRule packagestmt*/
						recog.base.set_state(220);
						recog.packagestmt()?;

						}
						}
						recog.base.set_state(225);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(226);
					recog.base.match_token(EOF,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- packagedecl ----------------
pub type PackagedeclContextAll<'input> = PackagedeclContext<'input>;


pub type PackagedeclContext<'input> = BaseParserRuleContext<'input,PackagedeclContextExt<'input>>;

#[derive(Clone)]
pub struct PackagedeclContextExt<'input>{
	pub pkgname: Option<Rc<PackageideContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for PackagedeclContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for PackagedeclContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_packagedecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for PackagedeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_packagedecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_packagedecl }
}
antlr_rust::type_id!{PackagedeclContextExt<'a>}

impl<'input> PackagedeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PackagedeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PackagedeclContextExt{
				pkgname: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait PackagedeclContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<PackagedeclContextExt<'input>>{

fn packageide(&self) -> Option<Rc<PackageideContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PackagedeclContextAttrs<'input> for PackagedeclContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn packagedecl(&mut self,)
	-> Result<Rc<PackagedeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PackagedeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_packagedecl);
        let mut _localctx: Rc<PackagedeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(229);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			/*InvokeRule packageide*/
			recog.base.set_state(230);
			let tmp = recog.packageide()?;
			 cast_mut::<_,PackagedeclContext >(&mut _localctx).pkgname = Some(tmp.clone());
			  

			recog.base.set_state(231);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- endpackage ----------------
pub type EndpackageContextAll<'input> = EndpackageContext<'input>;


pub type EndpackageContext<'input> = BaseParserRuleContext<'input,EndpackageContextExt<'input>>;

#[derive(Clone)]
pub struct EndpackageContextExt<'input>{
	pub pkgname: Option<Rc<PackageideContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for EndpackageContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for EndpackageContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_endpackage(self);
	}
}

impl<'input> CustomRuleContext<'input> for EndpackageContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_endpackage }
	//fn type_rule_index() -> usize where Self: Sized { RULE_endpackage }
}
antlr_rust::type_id!{EndpackageContextExt<'a>}

impl<'input> EndpackageContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EndpackageContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EndpackageContextExt{
				pkgname: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait EndpackageContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<EndpackageContextExt<'input>>{

fn packageide(&self) -> Option<Rc<PackageideContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EndpackageContextAttrs<'input> for EndpackageContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn endpackage(&mut self,)
	-> Result<Rc<EndpackageContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EndpackageContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_endpackage);
        let mut _localctx: Rc<EndpackageContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(233);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			recog.base.set_state(236);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 {
				{
				recog.base.set_state(234);
				recog.base.match_token(T__3,&mut recog.err_handler)?;

				/*InvokeRule packageide*/
				recog.base.set_state(235);
				let tmp = recog.packageide()?;
				 cast_mut::<_,EndpackageContext >(&mut _localctx).pkgname = Some(tmp.clone());
				  

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lowerCaseIdentifier ----------------
pub type LowerCaseIdentifierContextAll<'input> = LowerCaseIdentifierContext<'input>;


pub type LowerCaseIdentifierContext<'input> = BaseParserRuleContext<'input,LowerCaseIdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct LowerCaseIdentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for LowerCaseIdentifierContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for LowerCaseIdentifierContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_lowerCaseIdentifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for LowerCaseIdentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lowerCaseIdentifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lowerCaseIdentifier }
}
antlr_rust::type_id!{LowerCaseIdentifierContextExt<'a>}

impl<'input> LowerCaseIdentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LowerCaseIdentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LowerCaseIdentifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LowerCaseIdentifierContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<LowerCaseIdentifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LowerCaseIdentifier
/// Returns `None` if there is no child corresponding to token LowerCaseIdentifier
fn LowerCaseIdentifier(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
	self.get_token(LowerCaseIdentifier, 0)
}
/// Retrieves first TerminalNode corresponding to token EscapedOperator
/// Returns `None` if there is no child corresponding to token EscapedOperator
fn EscapedOperator(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
	self.get_token(EscapedOperator, 0)
}

}

impl<'input> LowerCaseIdentifierContextAttrs<'input> for LowerCaseIdentifierContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lowerCaseIdentifier(&mut self,)
	-> Result<Rc<LowerCaseIdentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LowerCaseIdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_lowerCaseIdentifier);
        let mut _localctx: Rc<LowerCaseIdentifierContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(238);
			_la = recog.base.input.la(1);
			if { !(_la==LowerCaseIdentifier || _la==EscapedOperator) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- upperCaseIdentifier ----------------
pub type UpperCaseIdentifierContextAll<'input> = UpperCaseIdentifierContext<'input>;


pub type UpperCaseIdentifierContext<'input> = BaseParserRuleContext<'input,UpperCaseIdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct UpperCaseIdentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for UpperCaseIdentifierContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for UpperCaseIdentifierContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_upperCaseIdentifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for UpperCaseIdentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_upperCaseIdentifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_upperCaseIdentifier }
}
antlr_rust::type_id!{UpperCaseIdentifierContextExt<'a>}

impl<'input> UpperCaseIdentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UpperCaseIdentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UpperCaseIdentifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UpperCaseIdentifierContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<UpperCaseIdentifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UpperCaseIdentifier
/// Returns `None` if there is no child corresponding to token UpperCaseIdentifier
fn UpperCaseIdentifier(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
	self.get_token(UpperCaseIdentifier, 0)
}

}

impl<'input> UpperCaseIdentifierContextAttrs<'input> for UpperCaseIdentifierContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn upperCaseIdentifier(&mut self,)
	-> Result<Rc<UpperCaseIdentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UpperCaseIdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_upperCaseIdentifier);
        let mut _localctx: Rc<UpperCaseIdentifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(240);
			recog.base.match_token(UpperCaseIdentifier,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- anyidentifier ----------------
pub type AnyidentifierContextAll<'input> = AnyidentifierContext<'input>;


pub type AnyidentifierContext<'input> = BaseParserRuleContext<'input,AnyidentifierContextExt<'input>>;

#[derive(Clone)]
pub struct AnyidentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for AnyidentifierContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for AnyidentifierContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_anyidentifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnyidentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_anyidentifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_anyidentifier }
}
antlr_rust::type_id!{AnyidentifierContextExt<'a>}

impl<'input> AnyidentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnyidentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnyidentifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnyidentifierContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<AnyidentifierContextExt<'input>>{

fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn upperCaseIdentifier(&self) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnyidentifierContextAttrs<'input> for AnyidentifierContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn anyidentifier(&mut self,)
	-> Result<Rc<AnyidentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnyidentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_anyidentifier);
        let mut _localctx: Rc<AnyidentifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(244);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LowerCaseIdentifier | EscapedOperator 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(242);
					recog.lowerCaseIdentifier()?;

					}
				}

			 UpperCaseIdentifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(243);
					recog.upperCaseIdentifier()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- exportdecl ----------------
pub type ExportdeclContextAll<'input> = ExportdeclContext<'input>;


pub type ExportdeclContext<'input> = BaseParserRuleContext<'input,ExportdeclContextExt<'input>>;

#[derive(Clone)]
pub struct ExportdeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ExportdeclContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ExportdeclContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_exportdecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExportdeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exportdecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exportdecl }
}
antlr_rust::type_id!{ExportdeclContextExt<'a>}

impl<'input> ExportdeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExportdeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExportdeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExportdeclContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ExportdeclContextExt<'input>>{

fn exportitem_all(&self) ->  Vec<Rc<ExportitemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn exportitem(&self, i: usize) -> Option<Rc<ExportitemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExportdeclContextAttrs<'input> for ExportdeclContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn exportdecl(&mut self,)
	-> Result<Rc<ExportdeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExportdeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_exportdecl);
        let mut _localctx: Rc<ExportdeclContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(246);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

			/*InvokeRule exportitem*/
			recog.base.set_state(247);
			recog.exportitem()?;

			recog.base.set_state(252);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(248);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule exportitem*/
				recog.base.set_state(249);
				recog.exportitem()?;

				}
				}
				recog.base.set_state(254);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(255);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- exportitem ----------------
pub type ExportitemContextAll<'input> = ExportitemContext<'input>;


pub type ExportitemContext<'input> = BaseParserRuleContext<'input,ExportitemContextExt<'input>>;

#[derive(Clone)]
pub struct ExportitemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ExportitemContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ExportitemContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_exportitem(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExportitemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exportitem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exportitem }
}
antlr_rust::type_id!{ExportitemContextExt<'a>}

impl<'input> ExportitemContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExportitemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExportitemContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExportitemContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ExportitemContextExt<'input>>{

fn packageide(&self) -> Option<Rc<PackageideContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn anyidentifier(&self) -> Option<Rc<AnyidentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExportitemContextAttrs<'input> for ExportitemContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn exportitem(&mut self,)
	-> Result<Rc<ExportitemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExportitemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_exportitem);
        let mut _localctx: Rc<ExportitemContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(267);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(8,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule packageide*/
					recog.base.set_state(257);
					recog.packageide()?;

					recog.base.set_state(258);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					recog.base.set_state(259);
					recog.base.match_token(T__7,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule anyidentifier*/
					recog.base.set_state(261);
					recog.anyidentifier()?;

					recog.base.set_state(265);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__8 {
						{
						recog.base.set_state(262);
						recog.base.match_token(T__8,&mut recog.err_handler)?;

						recog.base.set_state(263);
						recog.base.match_token(T__9,&mut recog.err_handler)?;

						recog.base.set_state(264);
						recog.base.match_token(T__10,&mut recog.err_handler)?;

						}
					}

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- importdecl ----------------
pub type ImportdeclContextAll<'input> = ImportdeclContext<'input>;


pub type ImportdeclContext<'input> = BaseParserRuleContext<'input,ImportdeclContextExt<'input>>;

#[derive(Clone)]
pub struct ImportdeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ImportdeclContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ImportdeclContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_importdecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportdeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importdecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importdecl }
}
antlr_rust::type_id!{ImportdeclContextExt<'a>}

impl<'input> ImportdeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportdeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportdeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportdeclContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ImportdeclContextExt<'input>>{

fn upperCaseIdentifier_all(&self) ->  Vec<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn upperCaseIdentifier(&self, i: usize) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ImportdeclContextAttrs<'input> for ImportdeclContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importdecl(&mut self,)
	-> Result<Rc<ImportdeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportdeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_importdecl);
        let mut _localctx: Rc<ImportdeclContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(269);
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			recog.base.set_state(273); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule upperCaseIdentifier*/
				recog.base.set_state(270);
				recog.upperCaseIdentifier()?;

				recog.base.set_state(271);
				recog.base.match_token(T__6,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(275); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==UpperCaseIdentifier) {break}
			}
			recog.base.set_state(277);
			recog.base.match_token(T__7,&mut recog.err_handler)?;

			recog.base.set_state(278);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- packagestmt ----------------
pub type PackagestmtContextAll<'input> = PackagestmtContext<'input>;


pub type PackagestmtContext<'input> = BaseParserRuleContext<'input,PackagestmtContextExt<'input>>;

#[derive(Clone)]
pub struct PackagestmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for PackagestmtContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for PackagestmtContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_packagestmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for PackagestmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_packagestmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_packagestmt }
}
antlr_rust::type_id!{PackagestmtContextExt<'a>}

impl<'input> PackagestmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PackagestmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PackagestmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PackagestmtContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<PackagestmtContextExt<'input>>{

fn interfacedecl(&self) -> Option<Rc<InterfacedeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typedefsynonym(&self) -> Option<Rc<TypedefsynonymContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typedefenum(&self) -> Option<Rc<TypedefenumContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typedefstruct(&self) -> Option<Rc<TypedefstructContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typedeftaggedunion(&self) -> Option<Rc<TypedeftaggedunionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeclassdecl(&self) -> Option<Rc<TypeclassdeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeclassinstance(&self) -> Option<Rc<TypeclassinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn externcimport(&self) -> Option<Rc<ExterncimportContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn varbinding(&self) -> Option<Rc<VarbindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functiondef(&self) -> Option<Rc<FunctiondefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn moduledef(&self) -> Option<Rc<ModuledefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn importdecl(&self) -> Option<Rc<ImportdeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn exportdecl(&self) -> Option<Rc<ExportdeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PackagestmtContextAttrs<'input> for PackagestmtContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn packagestmt(&mut self,)
	-> Result<Rc<PackagestmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PackagestmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_packagestmt);
        let mut _localctx: Rc<PackagestmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(293);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule interfacedecl*/
					recog.base.set_state(280);
					recog.interfacedecl()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule typedefsynonym*/
					recog.base.set_state(281);
					recog.typedefsynonym()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule typedefenum*/
					recog.base.set_state(282);
					recog.typedefenum()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule typedefstruct*/
					recog.base.set_state(283);
					recog.typedefstruct()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule typedeftaggedunion*/
					recog.base.set_state(284);
					recog.typedeftaggedunion()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule typeclassdecl*/
					recog.base.set_state(285);
					recog.typeclassdecl()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule typeclassinstance*/
					recog.base.set_state(286);
					recog.typeclassinstance()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule externcimport*/
					recog.base.set_state(287);
					recog.externcimport()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule varbinding*/
					recog.base.set_state(288);
					recog.varbinding()?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule functiondef*/
					recog.base.set_state(289);
					recog.functiondef()?;

					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					/*InvokeRule moduledef*/
					recog.base.set_state(290);
					recog.moduledef()?;

					}
				}
			,
				12 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					/*InvokeRule importdecl*/
					recog.base.set_state(291);
					recog.importdecl()?;

					}
				}
			,
				13 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 13);
					recog.base.enter_outer_alt(None, 13);
					{
					/*InvokeRule exportdecl*/
					recog.base.set_state(292);
					recog.exportdecl()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- packageide ----------------
pub type PackageideContextAll<'input> = PackageideContext<'input>;


pub type PackageideContext<'input> = BaseParserRuleContext<'input,PackageideContextExt<'input>>;

#[derive(Clone)]
pub struct PackageideContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for PackageideContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for PackageideContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_packageide(self);
	}
}

impl<'input> CustomRuleContext<'input> for PackageideContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_packageide }
	//fn type_rule_index() -> usize where Self: Sized { RULE_packageide }
}
antlr_rust::type_id!{PackageideContextExt<'a>}

impl<'input> PackageideContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PackageideContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PackageideContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PackageideContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<PackageideContextExt<'input>>{

fn upperCaseIdentifier(&self) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PackageideContextAttrs<'input> for PackageideContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn packageide(&mut self,)
	-> Result<Rc<PackageideContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PackageideContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_packageide);
        let mut _localctx: Rc<PackageideContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule upperCaseIdentifier*/
			recog.base.set_state(295);
			recog.upperCaseIdentifier()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- interfacedecl ----------------
pub type InterfacedeclContextAll<'input> = InterfacedeclContext<'input>;


pub type InterfacedeclContext<'input> = BaseParserRuleContext<'input,InterfacedeclContextExt<'input>>;

#[derive(Clone)]
pub struct InterfacedeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for InterfacedeclContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for InterfacedeclContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_interfacedecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for InterfacedeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interfacedecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interfacedecl }
}
antlr_rust::type_id!{InterfacedeclContextExt<'a>}

impl<'input> InterfacedeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InterfacedeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InterfacedeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InterfacedeclContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<InterfacedeclContextExt<'input>>{

fn typedeftype(&self) -> Option<Rc<TypedeftypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn interfacememberdecl_all(&self) ->  Vec<Rc<InterfacememberdeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn interfacememberdecl(&self, i: usize) -> Option<Rc<InterfacememberdeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn typeide(&self) -> Option<Rc<TypeideContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InterfacedeclContextAttrs<'input> for InterfacedeclContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interfacedecl(&mut self,)
	-> Result<Rc<InterfacedeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InterfacedeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_interfacedecl);
        let mut _localctx: Rc<InterfacedeclContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(300);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(297);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(302);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(303);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			/*InvokeRule typedeftype*/
			recog.base.set_state(304);
			recog.typedeftype()?;

			recog.base.set_state(305);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			recog.base.set_state(309);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__12 || _la==T__14 || _la==T__100 {
				{
				{
				/*InvokeRule interfacememberdecl*/
				recog.base.set_state(306);
				recog.interfacememberdecl()?;

				}
				}
				recog.base.set_state(311);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(312);
			recog.base.match_token(T__13,&mut recog.err_handler)?;

			recog.base.set_state(315);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 {
				{
				recog.base.set_state(313);
				recog.base.match_token(T__3,&mut recog.err_handler)?;

				/*InvokeRule typeide*/
				recog.base.set_state(314);
				recog.typeide()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- interfacememberdecl ----------------
pub type InterfacememberdeclContextAll<'input> = InterfacememberdeclContext<'input>;


pub type InterfacememberdeclContext<'input> = BaseParserRuleContext<'input,InterfacememberdeclContextExt<'input>>;

#[derive(Clone)]
pub struct InterfacememberdeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for InterfacememberdeclContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for InterfacememberdeclContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_interfacememberdecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for InterfacememberdeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interfacememberdecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interfacememberdecl }
}
antlr_rust::type_id!{InterfacememberdeclContextExt<'a>}

impl<'input> InterfacememberdeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InterfacememberdeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InterfacememberdeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InterfacememberdeclContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<InterfacememberdeclContextExt<'input>>{

fn methodproto(&self) -> Option<Rc<MethodprotoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn subinterfacedecl(&self) -> Option<Rc<SubinterfacedeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InterfacememberdeclContextAttrs<'input> for InterfacememberdeclContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interfacememberdecl(&mut self,)
	-> Result<Rc<InterfacememberdeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InterfacememberdeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_interfacememberdecl);
        let mut _localctx: Rc<InterfacememberdeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(319);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(14,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule methodproto*/
					recog.base.set_state(317);
					recog.methodproto()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule subinterfacedecl*/
					recog.base.set_state(318);
					recog.subinterfacedecl()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- methodproto ----------------
pub type MethodprotoContextAll<'input> = MethodprotoContext<'input>;


pub type MethodprotoContext<'input> = BaseParserRuleContext<'input,MethodprotoContextExt<'input>>;

#[derive(Clone)]
pub struct MethodprotoContextExt<'input>{
	pub name: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for MethodprotoContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for MethodprotoContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_methodproto(self);
	}
}

impl<'input> CustomRuleContext<'input> for MethodprotoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_methodproto }
	//fn type_rule_index() -> usize where Self: Sized { RULE_methodproto }
}
antlr_rust::type_id!{MethodprotoContextExt<'a>}

impl<'input> MethodprotoContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MethodprotoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MethodprotoContextExt{
				name: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait MethodprotoContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<MethodprotoContextExt<'input>>{

fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn provisos(&self) -> Option<Rc<ProvisosContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn methodprotoformals(&self) -> Option<Rc<MethodprotoformalsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MethodprotoContextAttrs<'input> for MethodprotoContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn methodproto(&mut self,)
	-> Result<Rc<MethodprotoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MethodprotoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_methodproto);
        let mut _localctx: Rc<MethodprotoContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(324);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(321);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(326);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(327);
			recog.base.match_token(T__14,&mut recog.err_handler)?;

			/*InvokeRule bsvtype*/
			recog.base.set_state(328);
			recog.bsvtype()?;

			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(329);
			let tmp = recog.lowerCaseIdentifier()?;
			 cast_mut::<_,MethodprotoContext >(&mut _localctx).name = Some(tmp.clone());
			  

			recog.base.set_state(335);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__8 {
				{
				recog.base.set_state(330);
				recog.base.match_token(T__8,&mut recog.err_handler)?;

				recog.base.set_state(332);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==T__8 || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (T__100 - 101)) | (1usize << (UpperCaseIdentifier - 101)) | (1usize << (LowerCaseIdentifier - 101)) | (1usize << (EscapedOperator - 101)) | (1usize << (IntLiteral - 101)))) != 0) {
					{
					/*InvokeRule methodprotoformals*/
					recog.base.set_state(331);
					recog.methodprotoformals()?;

					}
				}

				recog.base.set_state(334);
				recog.base.match_token(T__10,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(338);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__102 {
				{
				/*InvokeRule provisos*/
				recog.base.set_state(337);
				recog.provisos()?;

				}
			}

			recog.base.set_state(340);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- methodprotoformals ----------------
pub type MethodprotoformalsContextAll<'input> = MethodprotoformalsContext<'input>;


pub type MethodprotoformalsContext<'input> = BaseParserRuleContext<'input,MethodprotoformalsContextExt<'input>>;

#[derive(Clone)]
pub struct MethodprotoformalsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for MethodprotoformalsContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for MethodprotoformalsContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_methodprotoformals(self);
	}
}

impl<'input> CustomRuleContext<'input> for MethodprotoformalsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_methodprotoformals }
	//fn type_rule_index() -> usize where Self: Sized { RULE_methodprotoformals }
}
antlr_rust::type_id!{MethodprotoformalsContextExt<'a>}

impl<'input> MethodprotoformalsContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MethodprotoformalsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MethodprotoformalsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MethodprotoformalsContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<MethodprotoformalsContextExt<'input>>{

fn methodprotoformal_all(&self) ->  Vec<Rc<MethodprotoformalContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn methodprotoformal(&self, i: usize) -> Option<Rc<MethodprotoformalContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MethodprotoformalsContextAttrs<'input> for MethodprotoformalsContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn methodprotoformals(&mut self,)
	-> Result<Rc<MethodprotoformalsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MethodprotoformalsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_methodprotoformals);
        let mut _localctx: Rc<MethodprotoformalsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule methodprotoformal*/
			recog.base.set_state(342);
			recog.methodprotoformal()?;

			recog.base.set_state(347);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(343);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule methodprotoformal*/
				recog.base.set_state(344);
				recog.methodprotoformal()?;

				}
				}
				recog.base.set_state(349);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- methodprotoformal ----------------
pub type MethodprotoformalContextAll<'input> = MethodprotoformalContext<'input>;


pub type MethodprotoformalContext<'input> = BaseParserRuleContext<'input,MethodprotoformalContextExt<'input>>;

#[derive(Clone)]
pub struct MethodprotoformalContextExt<'input>{
	pub name: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for MethodprotoformalContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for MethodprotoformalContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_methodprotoformal(self);
	}
}

impl<'input> CustomRuleContext<'input> for MethodprotoformalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_methodprotoformal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_methodprotoformal }
}
antlr_rust::type_id!{MethodprotoformalContextExt<'a>}

impl<'input> MethodprotoformalContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MethodprotoformalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MethodprotoformalContextExt{
				name: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait MethodprotoformalContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<MethodprotoformalContextExt<'input>>{

fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MethodprotoformalContextAttrs<'input> for MethodprotoformalContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn methodprotoformal(&mut self,)
	-> Result<Rc<MethodprotoformalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MethodprotoformalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_methodprotoformal);
        let mut _localctx: Rc<MethodprotoformalContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(353);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(350);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(355);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(357);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule bsvtype*/
					recog.base.set_state(356);
					recog.bsvtype()?;

					}
				}

				_ => {}
			}
			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(359);
			let tmp = recog.lowerCaseIdentifier()?;
			 cast_mut::<_,MethodprotoformalContext >(&mut _localctx).name = Some(tmp.clone());
			  

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- subinterfacedecl ----------------
pub type SubinterfacedeclContextAll<'input> = SubinterfacedeclContext<'input>;


pub type SubinterfacedeclContext<'input> = BaseParserRuleContext<'input,SubinterfacedeclContextExt<'input>>;

#[derive(Clone)]
pub struct SubinterfacedeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for SubinterfacedeclContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for SubinterfacedeclContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_subinterfacedecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubinterfacedeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_subinterfacedecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_subinterfacedecl }
}
antlr_rust::type_id!{SubinterfacedeclContextExt<'a>}

impl<'input> SubinterfacedeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SubinterfacedeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SubinterfacedeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SubinterfacedeclContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<SubinterfacedeclContextExt<'input>>{

fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SubinterfacedeclContextAttrs<'input> for SubinterfacedeclContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn subinterfacedecl(&mut self,)
	-> Result<Rc<SubinterfacedeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SubinterfacedeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_subinterfacedecl);
        let mut _localctx: Rc<SubinterfacedeclContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(364);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(361);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(366);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(367);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			/*InvokeRule bsvtype*/
			recog.base.set_state(368);
			recog.bsvtype()?;

			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(369);
			recog.lowerCaseIdentifier()?;

			recog.base.set_state(370);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typedeftype ----------------
pub type TypedeftypeContextAll<'input> = TypedeftypeContext<'input>;


pub type TypedeftypeContext<'input> = BaseParserRuleContext<'input,TypedeftypeContextExt<'input>>;

#[derive(Clone)]
pub struct TypedeftypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypedeftypeContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypedeftypeContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typedeftype(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypedeftypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typedeftype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typedeftype }
}
antlr_rust::type_id!{TypedeftypeContextExt<'a>}

impl<'input> TypedeftypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypedeftypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypedeftypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypedeftypeContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypedeftypeContextExt<'input>>{

fn typeide(&self) -> Option<Rc<TypeideContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeformals(&self) -> Option<Rc<TypeformalsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypedeftypeContextAttrs<'input> for TypedeftypeContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typedeftype(&mut self,)
	-> Result<Rc<TypedeftypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypedeftypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_typedeftype);
        let mut _localctx: Rc<TypedeftypeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeide*/
			recog.base.set_state(372);
			recog.typeide()?;

			recog.base.set_state(374);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__15 {
				{
				/*InvokeRule typeformals*/
				recog.base.set_state(373);
				recog.typeformals()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeformals ----------------
pub type TypeformalsContextAll<'input> = TypeformalsContext<'input>;


pub type TypeformalsContext<'input> = BaseParserRuleContext<'input,TypeformalsContextExt<'input>>;

#[derive(Clone)]
pub struct TypeformalsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypeformalsContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypeformalsContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typeformals(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeformalsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeformals }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeformals }
}
antlr_rust::type_id!{TypeformalsContextExt<'a>}

impl<'input> TypeformalsContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeformalsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeformalsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeformalsContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypeformalsContextExt<'input>>{

fn typeformal_all(&self) ->  Vec<Rc<TypeformalContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeformal(&self, i: usize) -> Option<Rc<TypeformalContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TypeformalsContextAttrs<'input> for TypeformalsContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeformals(&mut self,)
	-> Result<Rc<TypeformalsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeformalsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_typeformals);
        let mut _localctx: Rc<TypeformalsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(376);
			recog.base.match_token(T__15,&mut recog.err_handler)?;

			recog.base.set_state(377);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule typeformal*/
			recog.base.set_state(378);
			recog.typeformal()?;

			recog.base.set_state(383);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(379);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule typeformal*/
				recog.base.set_state(380);
				recog.typeformal()?;

				}
				}
				recog.base.set_state(385);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(386);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeformal ----------------
pub type TypeformalContextAll<'input> = TypeformalContext<'input>;


pub type TypeformalContext<'input> = BaseParserRuleContext<'input,TypeformalContextExt<'input>>;

#[derive(Clone)]
pub struct TypeformalContextExt<'input>{
	pub numeric: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypeformalContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypeformalContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typeformal(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeformalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeformal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeformal }
}
antlr_rust::type_id!{TypeformalContextExt<'a>}

impl<'input> TypeformalContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeformalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeformalContextExt{
				numeric: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeformalContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypeformalContextExt<'input>>{

fn typeide(&self) -> Option<Rc<TypeideContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypeformalContextAttrs<'input> for TypeformalContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeformal(&mut self,)
	-> Result<Rc<TypeformalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeformalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_typeformal);
        let mut _localctx: Rc<TypeformalContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(389);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__16 {
				{
				recog.base.set_state(388);
				 cast_mut::<_,TypeformalContext >(&mut _localctx).numeric = recog.base.input.lt(1).cloned();
				 
				_la = recog.base.input.la(1);
				if { !(_la==T__16) } {
					let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
					 cast_mut::<_,TypeformalContext >(&mut _localctx).numeric = Some(tmp.clone());
					  

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
			}

			recog.base.set_state(391);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			/*InvokeRule typeide*/
			recog.base.set_state(392);
			recog.typeide()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typedefsynonym ----------------
pub type TypedefsynonymContextAll<'input> = TypedefsynonymContext<'input>;


pub type TypedefsynonymContext<'input> = BaseParserRuleContext<'input,TypedefsynonymContextExt<'input>>;

#[derive(Clone)]
pub struct TypedefsynonymContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypedefsynonymContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypedefsynonymContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typedefsynonym(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypedefsynonymContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typedefsynonym }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typedefsynonym }
}
antlr_rust::type_id!{TypedefsynonymContextExt<'a>}

impl<'input> TypedefsynonymContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypedefsynonymContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypedefsynonymContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypedefsynonymContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypedefsynonymContextExt<'input>>{

fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typedeftype(&self) -> Option<Rc<TypedeftypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TypedefsynonymContextAttrs<'input> for TypedefsynonymContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typedefsynonym(&mut self,)
	-> Result<Rc<TypedefsynonymContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypedefsynonymContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_typedefsynonym);
        let mut _localctx: Rc<TypedefsynonymContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(397);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(394);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(399);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(400);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			/*InvokeRule bsvtype*/
			recog.base.set_state(401);
			recog.bsvtype()?;

			/*InvokeRule typedeftype*/
			recog.base.set_state(402);
			recog.typedeftype()?;

			recog.base.set_state(403);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typedefenum ----------------
pub type TypedefenumContextAll<'input> = TypedefenumContext<'input>;


pub type TypedefenumContext<'input> = BaseParserRuleContext<'input,TypedefenumContextExt<'input>>;

#[derive(Clone)]
pub struct TypedefenumContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypedefenumContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypedefenumContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typedefenum(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypedefenumContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typedefenum }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typedefenum }
}
antlr_rust::type_id!{TypedefenumContextExt<'a>}

impl<'input> TypedefenumContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypedefenumContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypedefenumContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypedefenumContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypedefenumContextExt<'input>>{

fn typedefenumelement_all(&self) ->  Vec<Rc<TypedefenumelementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typedefenumelement(&self, i: usize) -> Option<Rc<TypedefenumelementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn upperCaseIdentifier(&self) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn derives(&self) -> Option<Rc<DerivesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypedefenumContextAttrs<'input> for TypedefenumContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typedefenum(&mut self,)
	-> Result<Rc<TypedefenumContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypedefenumContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_typedefenum);
        let mut _localctx: Rc<TypedefenumContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(405);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			recog.base.set_state(406);
			recog.base.match_token(T__19,&mut recog.err_handler)?;

			recog.base.set_state(407);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			/*InvokeRule typedefenumelement*/
			recog.base.set_state(408);
			recog.typedefenumelement()?;

			recog.base.set_state(413);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(409);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule typedefenumelement*/
				recog.base.set_state(410);
				recog.typedefenumelement()?;

				}
				}
				recog.base.set_state(415);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(416);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			/*InvokeRule upperCaseIdentifier*/
			recog.base.set_state(417);
			recog.upperCaseIdentifier()?;

			recog.base.set_state(419);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__28 {
				{
				/*InvokeRule derives*/
				recog.base.set_state(418);
				recog.derives()?;

				}
			}

			recog.base.set_state(421);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typedefenumelement ----------------
pub type TypedefenumelementContextAll<'input> = TypedefenumelementContext<'input>;


pub type TypedefenumelementContext<'input> = BaseParserRuleContext<'input,TypedefenumelementContextExt<'input>>;

#[derive(Clone)]
pub struct TypedefenumelementContextExt<'input>{
	pub tag: Option<Rc<UpperCaseIdentifierContextAll<'input>>>,
	pub from: Option<TokenType<'input>>,
	pub to: Option<TokenType<'input>>,
	pub tagval: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypedefenumelementContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypedefenumelementContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typedefenumelement(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypedefenumelementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typedefenumelement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typedefenumelement }
}
antlr_rust::type_id!{TypedefenumelementContextExt<'a>}

impl<'input> TypedefenumelementContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypedefenumelementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypedefenumelementContextExt{
				from: None, to: None, tagval: None, 
				tag: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait TypedefenumelementContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypedefenumelementContextExt<'input>>{

fn upperCaseIdentifier(&self) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token IntLiteral in current rule
fn IntLiteral_all(&self) -> Vec<Rc<TerminalNode<'input,BSVParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IntLiteral, starting from 0.
/// Returns `None` if number of children corresponding to token IntLiteral is less or equal than `i`.
fn IntLiteral(&self, i: usize) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
	self.get_token(IntLiteral, i)
}

}

impl<'input> TypedefenumelementContextAttrs<'input> for TypedefenumelementContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typedefenumelement(&mut self,)
	-> Result<Rc<TypedefenumelementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypedefenumelementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_typedefenumelement);
        let mut _localctx: Rc<TypedefenumelementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(440);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(423);
					let tmp = recog.upperCaseIdentifier()?;
					 cast_mut::<_,TypedefenumelementContext >(&mut _localctx).tag = Some(tmp.clone());
					  

					recog.base.set_state(424);
					recog.base.match_token(T__22,&mut recog.err_handler)?;

					recog.base.set_state(425);
					let tmp = recog.base.match_token(IntLiteral,&mut recog.err_handler)?;
					 cast_mut::<_,TypedefenumelementContext >(&mut _localctx).from = Some(tmp.clone());
					  

					recog.base.set_state(428);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__3 {
						{
						recog.base.set_state(426);
						recog.base.match_token(T__3,&mut recog.err_handler)?;

						recog.base.set_state(427);
						let tmp = recog.base.match_token(IntLiteral,&mut recog.err_handler)?;
						 cast_mut::<_,TypedefenumelementContext >(&mut _localctx).to = Some(tmp.clone());
						  

						}
					}

					recog.base.set_state(430);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					recog.base.set_state(433);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__24 {
						{
						recog.base.set_state(431);
						recog.base.match_token(T__24,&mut recog.err_handler)?;

						recog.base.set_state(432);
						let tmp = recog.base.match_token(IntLiteral,&mut recog.err_handler)?;
						 cast_mut::<_,TypedefenumelementContext >(&mut _localctx).tagval = Some(tmp.clone());
						  

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(435);
					let tmp = recog.upperCaseIdentifier()?;
					 cast_mut::<_,TypedefenumelementContext >(&mut _localctx).tag = Some(tmp.clone());
					  

					recog.base.set_state(438);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__24 {
						{
						recog.base.set_state(436);
						recog.base.match_token(T__24,&mut recog.err_handler)?;

						recog.base.set_state(437);
						let tmp = recog.base.match_token(IntLiteral,&mut recog.err_handler)?;
						 cast_mut::<_,TypedefenumelementContext >(&mut _localctx).tagval = Some(tmp.clone());
						  

						}
					}

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typedefstruct ----------------
pub type TypedefstructContextAll<'input> = TypedefstructContext<'input>;


pub type TypedefstructContext<'input> = BaseParserRuleContext<'input,TypedefstructContextExt<'input>>;

#[derive(Clone)]
pub struct TypedefstructContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypedefstructContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypedefstructContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typedefstruct(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypedefstructContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typedefstruct }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typedefstruct }
}
antlr_rust::type_id!{TypedefstructContextExt<'a>}

impl<'input> TypedefstructContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypedefstructContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypedefstructContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypedefstructContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypedefstructContextExt<'input>>{

fn typedeftype(&self) -> Option<Rc<TypedeftypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn structmember_all(&self) ->  Vec<Rc<StructmemberContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn structmember(&self, i: usize) -> Option<Rc<StructmemberContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn derives(&self) -> Option<Rc<DerivesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypedefstructContextAttrs<'input> for TypedefstructContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typedefstruct(&mut self,)
	-> Result<Rc<TypedefstructContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypedefstructContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_typedefstruct);
        let mut _localctx: Rc<TypedefstructContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(445);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(442);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(447);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(448);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			recog.base.set_state(449);
			recog.base.match_token(T__25,&mut recog.err_handler)?;

			recog.base.set_state(450);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			recog.base.set_state(454);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__8 || _la==T__26 || ((((_la - 105)) & !0x3f) == 0 && ((1usize << (_la - 105)) & ((1usize << (UpperCaseIdentifier - 105)) | (1usize << (LowerCaseIdentifier - 105)) | (1usize << (EscapedOperator - 105)) | (1usize << (IntLiteral - 105)))) != 0) {
				{
				{
				/*InvokeRule structmember*/
				recog.base.set_state(451);
				recog.structmember()?;

				}
				}
				recog.base.set_state(456);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(457);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			/*InvokeRule typedeftype*/
			recog.base.set_state(458);
			recog.typedeftype()?;

			recog.base.set_state(460);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__28 {
				{
				/*InvokeRule derives*/
				recog.base.set_state(459);
				recog.derives()?;

				}
			}

			recog.base.set_state(462);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typedeftaggedunion ----------------
pub type TypedeftaggedunionContextAll<'input> = TypedeftaggedunionContext<'input>;


pub type TypedeftaggedunionContext<'input> = BaseParserRuleContext<'input,TypedeftaggedunionContextExt<'input>>;

#[derive(Clone)]
pub struct TypedeftaggedunionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypedeftaggedunionContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypedeftaggedunionContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typedeftaggedunion(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypedeftaggedunionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typedeftaggedunion }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typedeftaggedunion }
}
antlr_rust::type_id!{TypedeftaggedunionContextExt<'a>}

impl<'input> TypedeftaggedunionContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypedeftaggedunionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypedeftaggedunionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypedeftaggedunionContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypedeftaggedunionContextExt<'input>>{

fn typedeftype(&self) -> Option<Rc<TypedeftypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn unionmember_all(&self) ->  Vec<Rc<UnionmemberContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn unionmember(&self, i: usize) -> Option<Rc<UnionmemberContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn derives(&self) -> Option<Rc<DerivesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypedeftaggedunionContextAttrs<'input> for TypedeftaggedunionContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typedeftaggedunion(&mut self,)
	-> Result<Rc<TypedeftaggedunionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypedeftaggedunionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_typedeftaggedunion);
        let mut _localctx: Rc<TypedeftaggedunionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(467);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(464);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(469);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(470);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			recog.base.set_state(471);
			recog.base.match_token(T__26,&mut recog.err_handler)?;

			recog.base.set_state(473);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__27 {
				{
				recog.base.set_state(472);
				recog.base.match_token(T__27,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(475);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			recog.base.set_state(479);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__25) | (1usize << T__26))) != 0) || ((((_la - 105)) & !0x3f) == 0 && ((1usize << (_la - 105)) & ((1usize << (UpperCaseIdentifier - 105)) | (1usize << (LowerCaseIdentifier - 105)) | (1usize << (EscapedOperator - 105)) | (1usize << (IntLiteral - 105)))) != 0) {
				{
				{
				/*InvokeRule unionmember*/
				recog.base.set_state(476);
				recog.unionmember()?;

				}
				}
				recog.base.set_state(481);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(482);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			/*InvokeRule typedeftype*/
			recog.base.set_state(483);
			recog.typedeftype()?;

			recog.base.set_state(485);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__28 {
				{
				/*InvokeRule derives*/
				recog.base.set_state(484);
				recog.derives()?;

				}
			}

			recog.base.set_state(487);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- structmember ----------------
pub type StructmemberContextAll<'input> = StructmemberContext<'input>;


pub type StructmemberContext<'input> = BaseParserRuleContext<'input,StructmemberContextExt<'input>>;

#[derive(Clone)]
pub struct StructmemberContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for StructmemberContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for StructmemberContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_structmember(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructmemberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structmember }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structmember }
}
antlr_rust::type_id!{StructmemberContextExt<'a>}

impl<'input> StructmemberContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StructmemberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructmemberContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StructmemberContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<StructmemberContextExt<'input>>{

fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn subunion(&self) -> Option<Rc<SubunionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StructmemberContextAttrs<'input> for StructmemberContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn structmember(&mut self,)
	-> Result<Rc<StructmemberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructmemberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_structmember);
        let mut _localctx: Rc<StructmemberContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(497);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__8 | UpperCaseIdentifier | LowerCaseIdentifier | EscapedOperator |
			 IntLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule bsvtype*/
					recog.base.set_state(489);
					recog.bsvtype()?;

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(490);
					recog.lowerCaseIdentifier()?;

					recog.base.set_state(491);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

			 T__26 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule subunion*/
					recog.base.set_state(493);
					recog.subunion()?;

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(494);
					recog.lowerCaseIdentifier()?;

					recog.base.set_state(495);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unionmember ----------------
pub type UnionmemberContextAll<'input> = UnionmemberContext<'input>;


pub type UnionmemberContext<'input> = BaseParserRuleContext<'input,UnionmemberContextExt<'input>>;

#[derive(Clone)]
pub struct UnionmemberContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for UnionmemberContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for UnionmemberContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unionmember(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnionmemberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unionmember }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unionmember }
}
antlr_rust::type_id!{UnionmemberContextExt<'a>}

impl<'input> UnionmemberContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnionmemberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnionmemberContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnionmemberContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<UnionmemberContextExt<'input>>{

fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn upperCaseIdentifier(&self) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn substruct(&self) -> Option<Rc<SubstructContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn subunion(&self) -> Option<Rc<SubunionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> UnionmemberContextAttrs<'input> for UnionmemberContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unionmember(&mut self,)
	-> Result<Rc<UnionmemberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnionmemberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_unionmember);
        let mut _localctx: Rc<UnionmemberContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(511);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__8 | UpperCaseIdentifier | LowerCaseIdentifier | EscapedOperator |
			 IntLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule bsvtype*/
					recog.base.set_state(499);
					recog.bsvtype()?;

					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(500);
					recog.upperCaseIdentifier()?;

					recog.base.set_state(501);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

			 T__25 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule substruct*/
					recog.base.set_state(503);
					recog.substruct()?;

					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(504);
					recog.upperCaseIdentifier()?;

					recog.base.set_state(505);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

			 T__26 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule subunion*/
					recog.base.set_state(507);
					recog.subunion()?;

					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(508);
					recog.upperCaseIdentifier()?;

					recog.base.set_state(509);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- substruct ----------------
pub type SubstructContextAll<'input> = SubstructContext<'input>;


pub type SubstructContext<'input> = BaseParserRuleContext<'input,SubstructContextExt<'input>>;

#[derive(Clone)]
pub struct SubstructContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for SubstructContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for SubstructContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_substruct(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubstructContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_substruct }
	//fn type_rule_index() -> usize where Self: Sized { RULE_substruct }
}
antlr_rust::type_id!{SubstructContextExt<'a>}

impl<'input> SubstructContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SubstructContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SubstructContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SubstructContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<SubstructContextExt<'input>>{

fn structmember_all(&self) ->  Vec<Rc<StructmemberContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn structmember(&self, i: usize) -> Option<Rc<StructmemberContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SubstructContextAttrs<'input> for SubstructContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn substruct(&mut self,)
	-> Result<Rc<SubstructContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SubstructContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_substruct);
        let mut _localctx: Rc<SubstructContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(513);
			recog.base.match_token(T__25,&mut recog.err_handler)?;

			recog.base.set_state(514);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			recog.base.set_state(518);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__8 || _la==T__26 || ((((_la - 105)) & !0x3f) == 0 && ((1usize << (_la - 105)) & ((1usize << (UpperCaseIdentifier - 105)) | (1usize << (LowerCaseIdentifier - 105)) | (1usize << (EscapedOperator - 105)) | (1usize << (IntLiteral - 105)))) != 0) {
				{
				{
				/*InvokeRule structmember*/
				recog.base.set_state(515);
				recog.structmember()?;

				}
				}
				recog.base.set_state(520);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(521);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- subunion ----------------
pub type SubunionContextAll<'input> = SubunionContext<'input>;


pub type SubunionContext<'input> = BaseParserRuleContext<'input,SubunionContextExt<'input>>;

#[derive(Clone)]
pub struct SubunionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for SubunionContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for SubunionContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_subunion(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubunionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_subunion }
	//fn type_rule_index() -> usize where Self: Sized { RULE_subunion }
}
antlr_rust::type_id!{SubunionContextExt<'a>}

impl<'input> SubunionContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SubunionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SubunionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SubunionContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<SubunionContextExt<'input>>{

fn unionmember_all(&self) ->  Vec<Rc<UnionmemberContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn unionmember(&self, i: usize) -> Option<Rc<UnionmemberContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SubunionContextAttrs<'input> for SubunionContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn subunion(&mut self,)
	-> Result<Rc<SubunionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SubunionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_subunion);
        let mut _localctx: Rc<SubunionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(523);
			recog.base.match_token(T__26,&mut recog.err_handler)?;

			recog.base.set_state(525);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__27 {
				{
				recog.base.set_state(524);
				recog.base.match_token(T__27,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(527);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			recog.base.set_state(531);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__25) | (1usize << T__26))) != 0) || ((((_la - 105)) & !0x3f) == 0 && ((1usize << (_la - 105)) & ((1usize << (UpperCaseIdentifier - 105)) | (1usize << (LowerCaseIdentifier - 105)) | (1usize << (EscapedOperator - 105)) | (1usize << (IntLiteral - 105)))) != 0) {
				{
				{
				/*InvokeRule unionmember*/
				recog.base.set_state(528);
				recog.unionmember()?;

				}
				}
				recog.base.set_state(533);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(534);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- derives ----------------
pub type DerivesContextAll<'input> = DerivesContext<'input>;


pub type DerivesContext<'input> = BaseParserRuleContext<'input,DerivesContextExt<'input>>;

#[derive(Clone)]
pub struct DerivesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for DerivesContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for DerivesContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_derives(self);
	}
}

impl<'input> CustomRuleContext<'input> for DerivesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_derives }
	//fn type_rule_index() -> usize where Self: Sized { RULE_derives }
}
antlr_rust::type_id!{DerivesContextExt<'a>}

impl<'input> DerivesContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DerivesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DerivesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DerivesContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<DerivesContextExt<'input>>{

fn typeide_all(&self) ->  Vec<Rc<TypeideContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeide(&self, i: usize) -> Option<Rc<TypeideContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DerivesContextAttrs<'input> for DerivesContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn derives(&mut self,)
	-> Result<Rc<DerivesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DerivesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_derives);
        let mut _localctx: Rc<DerivesContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(536);
			recog.base.match_token(T__28,&mut recog.err_handler)?;

			recog.base.set_state(537);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule typeide*/
			recog.base.set_state(538);
			recog.typeide()?;

			recog.base.set_state(543);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(539);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule typeide*/
				recog.base.set_state(540);
				recog.typeide()?;

				}
				}
				recog.base.set_state(545);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(546);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- moduleinst ----------------
pub type ModuleinstContextAll<'input> = ModuleinstContext<'input>;


pub type ModuleinstContext<'input> = BaseParserRuleContext<'input,ModuleinstContextExt<'input>>;

#[derive(Clone)]
pub struct ModuleinstContextExt<'input>{
	pub t: Option<Rc<BsvtypeContextAll<'input>>>,
	pub var: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
	pub rhs: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ModuleinstContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ModuleinstContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_moduleinst(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModuleinstContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_moduleinst }
	//fn type_rule_index() -> usize where Self: Sized { RULE_moduleinst }
}
antlr_rust::type_id!{ModuleinstContextExt<'a>}

impl<'input> ModuleinstContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModuleinstContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModuleinstContextExt{
				t: None, var: None, rhs: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ModuleinstContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ModuleinstContextExt<'input>>{

fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ModuleinstContextAttrs<'input> for ModuleinstContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn moduleinst(&mut self,)
	-> Result<Rc<ModuleinstContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModuleinstContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_moduleinst);
        let mut _localctx: Rc<ModuleinstContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(551);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(548);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(553);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(556);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__29 
				=> {
					{
					recog.base.set_state(554);
					recog.base.match_token(T__29,&mut recog.err_handler)?;

					}
				}

			 T__8 | UpperCaseIdentifier | LowerCaseIdentifier | EscapedOperator |
			 IntLiteral 
				=> {
					{
					/*InvokeRule bsvtype*/
					recog.base.set_state(555);
					let tmp = recog.bsvtype()?;
					 cast_mut::<_,ModuleinstContext >(&mut _localctx).t = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(558);
			let tmp = recog.lowerCaseIdentifier()?;
			 cast_mut::<_,ModuleinstContext >(&mut _localctx).var = Some(tmp.clone());
			  

			recog.base.set_state(559);
			recog.base.match_token(T__24,&mut recog.err_handler)?;

			recog.base.set_state(560);
			recog.base.match_token(T__30,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(561);
			let tmp = recog.expression_rec(0)?;
			 cast_mut::<_,ModuleinstContext >(&mut _localctx).rhs = Some(tmp.clone());
			  

			recog.base.set_state(562);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tuplebind ----------------
pub type TuplebindContextAll<'input> = TuplebindContext<'input>;


pub type TuplebindContext<'input> = BaseParserRuleContext<'input,TuplebindContextExt<'input>>;

#[derive(Clone)]
pub struct TuplebindContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TuplebindContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TuplebindContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_tuplebind(self);
	}
}

impl<'input> CustomRuleContext<'input> for TuplebindContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tuplebind }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tuplebind }
}
antlr_rust::type_id!{TuplebindContextExt<'a>}

impl<'input> TuplebindContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TuplebindContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TuplebindContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TuplebindContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TuplebindContextExt<'input>>{

fn lowerCaseIdentifier_all(&self) ->  Vec<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn lowerCaseIdentifier(&self, i: usize) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TuplebindContextAttrs<'input> for TuplebindContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tuplebind(&mut self,)
	-> Result<Rc<TuplebindContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TuplebindContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_tuplebind);
        let mut _localctx: Rc<TuplebindContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(564);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(565);
			recog.lowerCaseIdentifier()?;

			recog.base.set_state(568); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				recog.base.set_state(566);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule lowerCaseIdentifier*/
				recog.base.set_state(567);
				recog.lowerCaseIdentifier()?;

				}
				}
				recog.base.set_state(570); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__5) {break}
			}
			recog.base.set_state(572);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- varinit ----------------
pub type VarinitContextAll<'input> = VarinitContext<'input>;


pub type VarinitContext<'input> = BaseParserRuleContext<'input,VarinitContextExt<'input>>;

#[derive(Clone)]
pub struct VarinitContextExt<'input>{
	pub var: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
	pub rhs: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for VarinitContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for VarinitContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_varinit(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarinitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_varinit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_varinit }
}
antlr_rust::type_id!{VarinitContextExt<'a>}

impl<'input> VarinitContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VarinitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VarinitContextExt{
				var: None, rhs: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait VarinitContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<VarinitContextExt<'input>>{

fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tuplebind(&self) -> Option<Rc<TuplebindContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VarinitContextAttrs<'input> for VarinitContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn varinit(&mut self,)
	-> Result<Rc<VarinitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VarinitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_varinit);
        let mut _localctx: Rc<VarinitContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(583);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LowerCaseIdentifier | EscapedOperator 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(574);
					let tmp = recog.lowerCaseIdentifier()?;
					 cast_mut::<_,VarinitContext >(&mut _localctx).var = Some(tmp.clone());
					  

					recog.base.set_state(577);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__24 {
						{
						recog.base.set_state(575);
						recog.base.match_token(T__24,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(576);
						let tmp = recog.expression_rec(0)?;
						 cast_mut::<_,VarinitContext >(&mut _localctx).rhs = Some(tmp.clone());
						  

						}
					}

					}
				}

			 T__20 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tuplebind*/
					recog.base.set_state(579);
					recog.tuplebind()?;

					recog.base.set_state(580);
					recog.base.match_token(T__24,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(581);
					let tmp = recog.expression_rec(0)?;
					 cast_mut::<_,VarinitContext >(&mut _localctx).rhs = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- varbinding ----------------
pub type VarbindingContextAll<'input> = VarbindingContext<'input>;


pub type VarbindingContext<'input> = BaseParserRuleContext<'input,VarbindingContextExt<'input>>;

#[derive(Clone)]
pub struct VarbindingContextExt<'input>{
	pub t: Option<Rc<BsvtypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for VarbindingContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for VarbindingContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_varbinding(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarbindingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_varbinding }
	//fn type_rule_index() -> usize where Self: Sized { RULE_varbinding }
}
antlr_rust::type_id!{VarbindingContextExt<'a>}

impl<'input> VarbindingContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VarbindingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VarbindingContextExt{
				t: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait VarbindingContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<VarbindingContextExt<'input>>{

fn varinit_all(&self) ->  Vec<Rc<VarinitContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn varinit(&self, i: usize) -> Option<Rc<VarinitContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VarbindingContextAttrs<'input> for VarbindingContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn varbinding(&mut self,)
	-> Result<Rc<VarbindingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VarbindingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_varbinding);
        let mut _localctx: Rc<VarbindingContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(588);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(585);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(590);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(593);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__29 
				=> {
					{
					recog.base.set_state(591);
					recog.base.match_token(T__29,&mut recog.err_handler)?;

					}
				}

			 T__8 | UpperCaseIdentifier | LowerCaseIdentifier | EscapedOperator |
			 IntLiteral 
				=> {
					{
					/*InvokeRule bsvtype*/
					recog.base.set_state(592);
					let tmp = recog.bsvtype()?;
					 cast_mut::<_,VarbindingContext >(&mut _localctx).t = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			/*InvokeRule varinit*/
			recog.base.set_state(595);
			recog.varinit()?;

			recog.base.set_state(600);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(596);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule varinit*/
				recog.base.set_state(597);
				recog.varinit()?;

				}
				}
				recog.base.set_state(602);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(603);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- actionbinding ----------------
pub type ActionbindingContextAll<'input> = ActionbindingContext<'input>;


pub type ActionbindingContext<'input> = BaseParserRuleContext<'input,ActionbindingContextExt<'input>>;

#[derive(Clone)]
pub struct ActionbindingContextExt<'input>{
	pub t: Option<Rc<BsvtypeContextAll<'input>>>,
	pub var: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
	pub arraydim: Option<Rc<ExpressionContextAll<'input>>>,
	pub rhs: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ActionbindingContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ActionbindingContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_actionbinding(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionbindingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionbinding }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionbinding }
}
antlr_rust::type_id!{ActionbindingContextExt<'a>}

impl<'input> ActionbindingContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionbindingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionbindingContextExt{
				t: None, var: None, arraydim: None, rhs: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionbindingContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ActionbindingContextExt<'input>>{

fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ActionbindingContextAttrs<'input> for ActionbindingContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionbinding(&mut self,)
	-> Result<Rc<ActionbindingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionbindingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_actionbinding);
        let mut _localctx: Rc<ActionbindingContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(608);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(605);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(610);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(613);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__29 
				=> {
					{
					recog.base.set_state(611);
					recog.base.match_token(T__29,&mut recog.err_handler)?;

					}
				}

			 T__8 | UpperCaseIdentifier | LowerCaseIdentifier | EscapedOperator |
			 IntLiteral 
				=> {
					{
					/*InvokeRule bsvtype*/
					recog.base.set_state(612);
					let tmp = recog.bsvtype()?;
					 cast_mut::<_,ActionbindingContext >(&mut _localctx).t = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(615);
			let tmp = recog.lowerCaseIdentifier()?;
			 cast_mut::<_,ActionbindingContext >(&mut _localctx).var = Some(tmp.clone());
			  

			recog.base.set_state(620);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__22 {
				{
				recog.base.set_state(616);
				recog.base.match_token(T__22,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(617);
				let tmp = recog.expression_rec(0)?;
				 cast_mut::<_,ActionbindingContext >(&mut _localctx).arraydim = Some(tmp.clone());
				  

				recog.base.set_state(618);
				recog.base.match_token(T__23,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(622);
			recog.base.match_token(T__31,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(623);
			let tmp = recog.expression_rec(0)?;
			 cast_mut::<_,ActionbindingContext >(&mut _localctx).rhs = Some(tmp.clone());
			  

			recog.base.set_state(624);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- patternbinding ----------------
pub type PatternbindingContextAll<'input> = PatternbindingContext<'input>;


pub type PatternbindingContext<'input> = BaseParserRuleContext<'input,PatternbindingContextExt<'input>>;

#[derive(Clone)]
pub struct PatternbindingContextExt<'input>{
	pub op: Option<TokenType<'input>>,
	pub rhs: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for PatternbindingContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for PatternbindingContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_patternbinding(self);
	}
}

impl<'input> CustomRuleContext<'input> for PatternbindingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternbinding }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternbinding }
}
antlr_rust::type_id!{PatternbindingContextExt<'a>}

impl<'input> PatternbindingContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternbindingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternbindingContextExt{
				op: None, 
				rhs: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternbindingContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<PatternbindingContextExt<'input>>{

fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PatternbindingContextAttrs<'input> for PatternbindingContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternbinding(&mut self,)
	-> Result<Rc<PatternbindingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternbindingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_patternbinding);
        let mut _localctx: Rc<PatternbindingContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(629);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(626);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(631);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(632);
			recog.base.match_token(T__32,&mut recog.err_handler)?;

			/*InvokeRule pattern*/
			recog.base.set_state(633);
			recog.pattern()?;

			recog.base.set_state(634);
			 cast_mut::<_,PatternbindingContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
			 
			_la = recog.base.input.la(1);
			if { !(_la==T__24 || _la==T__31) } {
				let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
				 cast_mut::<_,PatternbindingContext >(&mut _localctx).op = Some(tmp.clone());
				  

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule expression*/
			recog.base.set_state(635);
			let tmp = recog.expression_rec(0)?;
			 cast_mut::<_,PatternbindingContext >(&mut _localctx).rhs = Some(tmp.clone());
			  

			recog.base.set_state(636);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeclassdecl ----------------
pub type TypeclassdeclContextAll<'input> = TypeclassdeclContext<'input>;


pub type TypeclassdeclContext<'input> = BaseParserRuleContext<'input,TypeclassdeclContextExt<'input>>;

#[derive(Clone)]
pub struct TypeclassdeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypeclassdeclContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypeclassdeclContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typeclassdecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeclassdeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeclassdecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeclassdecl }
}
antlr_rust::type_id!{TypeclassdeclContextExt<'a>}

impl<'input> TypeclassdeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeclassdeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeclassdeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeclassdeclContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypeclassdeclContextExt<'input>>{

fn typeclasside_all(&self) ->  Vec<Rc<TypeclassideContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeclasside(&self, i: usize) -> Option<Rc<TypeclassideContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn typeformals(&self) -> Option<Rc<TypeformalsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn provisos(&self) -> Option<Rc<ProvisosContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typedepends(&self) -> Option<Rc<TypedependsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn overloadeddecl_all(&self) ->  Vec<Rc<OverloadeddeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn overloadeddecl(&self, i: usize) -> Option<Rc<OverloadeddeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TypeclassdeclContextAttrs<'input> for TypeclassdeclContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeclassdecl(&mut self,)
	-> Result<Rc<TypeclassdeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeclassdeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_typeclassdecl);
        let mut _localctx: Rc<TypeclassdeclContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(641);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(638);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(643);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(644);
			recog.base.match_token(T__33,&mut recog.err_handler)?;

			/*InvokeRule typeclasside*/
			recog.base.set_state(645);
			recog.typeclasside()?;

			/*InvokeRule typeformals*/
			recog.base.set_state(646);
			recog.typeformals()?;

			recog.base.set_state(648);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__102 {
				{
				/*InvokeRule provisos*/
				recog.base.set_state(647);
				recog.provisos()?;

				}
			}

			recog.base.set_state(651);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__35 {
				{
				/*InvokeRule typedepends*/
				recog.base.set_state(650);
				recog.typedepends()?;

				}
			}

			recog.base.set_state(653);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			recog.base.set_state(657);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__29) | (1usize << T__40) | (1usize << T__48))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (T__100 - 101)) | (1usize << (UpperCaseIdentifier - 101)) | (1usize << (LowerCaseIdentifier - 101)) | (1usize << (EscapedOperator - 101)) | (1usize << (IntLiteral - 101)))) != 0) {
				{
				{
				/*InvokeRule overloadeddecl*/
				recog.base.set_state(654);
				recog.overloadeddecl()?;

				}
				}
				recog.base.set_state(659);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(660);
			recog.base.match_token(T__34,&mut recog.err_handler)?;

			recog.base.set_state(663);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 {
				{
				recog.base.set_state(661);
				recog.base.match_token(T__3,&mut recog.err_handler)?;

				/*InvokeRule typeclasside*/
				recog.base.set_state(662);
				recog.typeclasside()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeclasside ----------------
pub type TypeclassideContextAll<'input> = TypeclassideContext<'input>;


pub type TypeclassideContext<'input> = BaseParserRuleContext<'input,TypeclassideContextExt<'input>>;

#[derive(Clone)]
pub struct TypeclassideContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypeclassideContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypeclassideContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typeclasside(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeclassideContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeclasside }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeclasside }
}
antlr_rust::type_id!{TypeclassideContextExt<'a>}

impl<'input> TypeclassideContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeclassideContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeclassideContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeclassideContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypeclassideContextExt<'input>>{

fn upperCaseIdentifier(&self) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypeclassideContextAttrs<'input> for TypeclassideContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeclasside(&mut self,)
	-> Result<Rc<TypeclassideContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeclassideContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_typeclasside);
        let mut _localctx: Rc<TypeclassideContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule upperCaseIdentifier*/
			recog.base.set_state(665);
			recog.upperCaseIdentifier()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typedepends ----------------
pub type TypedependsContextAll<'input> = TypedependsContext<'input>;


pub type TypedependsContext<'input> = BaseParserRuleContext<'input,TypedependsContextExt<'input>>;

#[derive(Clone)]
pub struct TypedependsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypedependsContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypedependsContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typedepends(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypedependsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typedepends }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typedepends }
}
antlr_rust::type_id!{TypedependsContextExt<'a>}

impl<'input> TypedependsContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypedependsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypedependsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypedependsContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypedependsContextExt<'input>>{

fn typedepend_all(&self) ->  Vec<Rc<TypedependContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typedepend(&self, i: usize) -> Option<Rc<TypedependContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TypedependsContextAttrs<'input> for TypedependsContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typedepends(&mut self,)
	-> Result<Rc<TypedependsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypedependsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_typedepends);
        let mut _localctx: Rc<TypedependsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(667);
			recog.base.match_token(T__35,&mut recog.err_handler)?;

			recog.base.set_state(668);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule typedepend*/
			recog.base.set_state(669);
			recog.typedepend()?;

			recog.base.set_state(674);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(670);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule typedepend*/
				recog.base.set_state(671);
				recog.typedepend()?;

				}
				}
				recog.base.set_state(676);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(677);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typedepend ----------------
pub type TypedependContextAll<'input> = TypedependContext<'input>;


pub type TypedependContext<'input> = BaseParserRuleContext<'input,TypedependContextExt<'input>>;

#[derive(Clone)]
pub struct TypedependContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypedependContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypedependContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typedepend(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypedependContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typedepend }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typedepend }
}
antlr_rust::type_id!{TypedependContextExt<'a>}

impl<'input> TypedependContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypedependContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypedependContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypedependContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypedependContextExt<'input>>{

fn typelist_all(&self) ->  Vec<Rc<TypelistContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typelist(&self, i: usize) -> Option<Rc<TypelistContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TypedependContextAttrs<'input> for TypedependContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typedepend(&mut self,)
	-> Result<Rc<TypedependContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypedependContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_typedepend);
        let mut _localctx: Rc<TypedependContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typelist*/
			recog.base.set_state(679);
			recog.typelist()?;

			recog.base.set_state(680);
			recog.base.match_token(T__36,&mut recog.err_handler)?;

			/*InvokeRule typelist*/
			recog.base.set_state(681);
			recog.typelist()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typelist ----------------
pub type TypelistContextAll<'input> = TypelistContext<'input>;


pub type TypelistContext<'input> = BaseParserRuleContext<'input,TypelistContextExt<'input>>;

#[derive(Clone)]
pub struct TypelistContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypelistContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypelistContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typelist(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypelistContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typelist }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typelist }
}
antlr_rust::type_id!{TypelistContextExt<'a>}

impl<'input> TypelistContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypelistContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypelistContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypelistContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypelistContextExt<'input>>{

fn typeide_all(&self) ->  Vec<Rc<TypeideContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeide(&self, i: usize) -> Option<Rc<TypeideContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TypelistContextAttrs<'input> for TypelistContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typelist(&mut self,)
	-> Result<Rc<TypelistContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypelistContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_typelist);
        let mut _localctx: Rc<TypelistContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(695);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 UpperCaseIdentifier | LowerCaseIdentifier | EscapedOperator 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule typeide*/
					recog.base.set_state(683);
					recog.typeide()?;

					}
				}

			 T__8 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(684);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					/*InvokeRule typeide*/
					recog.base.set_state(685);
					recog.typeide()?;

					recog.base.set_state(690);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__5 {
						{
						{
						recog.base.set_state(686);
						recog.base.match_token(T__5,&mut recog.err_handler)?;

						/*InvokeRule typeide*/
						recog.base.set_state(687);
						recog.typeide()?;

						}
						}
						recog.base.set_state(692);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(693);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- overloadeddecl ----------------
pub type OverloadeddeclContextAll<'input> = OverloadeddeclContext<'input>;


pub type OverloadeddeclContext<'input> = BaseParserRuleContext<'input,OverloadeddeclContextExt<'input>>;

#[derive(Clone)]
pub struct OverloadeddeclContextExt<'input>{
	pub defaultdef: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for OverloadeddeclContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for OverloadeddeclContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_overloadeddecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for OverloadeddeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_overloadeddecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_overloadeddecl }
}
antlr_rust::type_id!{OverloadeddeclContextExt<'a>}

impl<'input> OverloadeddeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OverloadeddeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OverloadeddeclContextExt{
				defaultdef: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait OverloadeddeclContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<OverloadeddeclContextExt<'input>>{

fn functionproto(&self) -> Option<Rc<FunctionprotoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn moduleproto(&self) -> Option<Rc<ModuleprotoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn varbinding(&self) -> Option<Rc<VarbindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OverloadeddeclContextAttrs<'input> for OverloadeddeclContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn overloadeddecl(&mut self,)
	-> Result<Rc<OverloadeddeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OverloadeddeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_overloadeddecl);
        let mut _localctx: Rc<OverloadeddeclContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(700);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(66,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule attributeinstance*/
					recog.base.set_state(697);
					recog.attributeinstance()?;

					}
					} 
				}
				recog.base.set_state(702);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(66,&mut recog.base)?;
			}
			recog.base.set_state(712);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__48 
				=> {
					{
					/*InvokeRule functionproto*/
					recog.base.set_state(703);
					recog.functionproto()?;

					recog.base.set_state(706);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__24 {
						{
						recog.base.set_state(704);
						recog.base.match_token(T__24,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(705);
						let tmp = recog.expression_rec(0)?;
						 cast_mut::<_,OverloadeddeclContext >(&mut _localctx).defaultdef = Some(tmp.clone());
						  

						}
					}

					recog.base.set_state(708);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

			 T__40 
				=> {
					{
					/*InvokeRule moduleproto*/
					recog.base.set_state(710);
					recog.moduleproto()?;

					}
				}

			 T__8 | T__29 | T__100 | UpperCaseIdentifier | LowerCaseIdentifier | EscapedOperator |
			 IntLiteral 
				=> {
					{
					/*InvokeRule varbinding*/
					recog.base.set_state(711);
					recog.varbinding()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tctype ----------------
pub type TctypeContextAll<'input> = TctypeContext<'input>;


pub type TctypeContext<'input> = BaseParserRuleContext<'input,TctypeContextExt<'input>>;

#[derive(Clone)]
pub struct TctypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TctypeContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TctypeContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_tctype(self);
	}
}

impl<'input> CustomRuleContext<'input> for TctypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tctype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tctype }
}
antlr_rust::type_id!{TctypeContextExt<'a>}

impl<'input> TctypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TctypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TctypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TctypeContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TctypeContextExt<'input>>{

fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionproto(&self) -> Option<Rc<FunctionprotoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TctypeContextAttrs<'input> for TctypeContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tctype(&mut self,)
	-> Result<Rc<TctypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TctypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_tctype);
        let mut _localctx: Rc<TctypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(716);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__8 | UpperCaseIdentifier | LowerCaseIdentifier | EscapedOperator |
			 IntLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule bsvtype*/
					recog.base.set_state(714);
					recog.bsvtype()?;

					}
				}

			 T__48 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule functionproto*/
					recog.base.set_state(715);
					recog.functionproto()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeclassinstance ----------------
pub type TypeclassinstanceContextAll<'input> = TypeclassinstanceContext<'input>;


pub type TypeclassinstanceContext<'input> = BaseParserRuleContext<'input,TypeclassinstanceContextExt<'input>>;

#[derive(Clone)]
pub struct TypeclassinstanceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypeclassinstanceContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypeclassinstanceContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typeclassinstance(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeclassinstanceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeclassinstance }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeclassinstance }
}
antlr_rust::type_id!{TypeclassinstanceContextExt<'a>}

impl<'input> TypeclassinstanceContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeclassinstanceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeclassinstanceContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeclassinstanceContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypeclassinstanceContextExt<'input>>{

fn typeclasside_all(&self) ->  Vec<Rc<TypeclassideContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeclasside(&self, i: usize) -> Option<Rc<TypeclassideContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn tctype_all(&self) ->  Vec<Rc<TctypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tctype(&self, i: usize) -> Option<Rc<TctypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn provisos(&self) -> Option<Rc<ProvisosContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn overloadeddef_all(&self) ->  Vec<Rc<OverloadeddefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn overloadeddef(&self, i: usize) -> Option<Rc<OverloadeddefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TypeclassinstanceContextAttrs<'input> for TypeclassinstanceContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeclassinstance(&mut self,)
	-> Result<Rc<TypeclassinstanceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeclassinstanceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_typeclassinstance);
        let mut _localctx: Rc<TypeclassinstanceContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(721);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(718);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(723);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(724);
			recog.base.match_token(T__37,&mut recog.err_handler)?;

			/*InvokeRule typeclasside*/
			recog.base.set_state(725);
			recog.typeclasside()?;

			recog.base.set_state(726);
			recog.base.match_token(T__15,&mut recog.err_handler)?;

			recog.base.set_state(727);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule tctype*/
			recog.base.set_state(728);
			recog.tctype()?;

			recog.base.set_state(733);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(729);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule tctype*/
				recog.base.set_state(730);
				recog.tctype()?;

				}
				}
				recog.base.set_state(735);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(736);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			recog.base.set_state(738);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__102 {
				{
				/*InvokeRule provisos*/
				recog.base.set_state(737);
				recog.provisos()?;

				}
			}

			recog.base.set_state(740);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			recog.base.set_state(744);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__40) | (1usize << T__43) | (1usize << T__48) | (1usize << T__53))) != 0) || ((((_la - 85)) & !0x3f) == 0 && ((1usize << (_la - 85)) & ((1usize << (T__84 - 85)) | (1usize << (T__85 - 85)) | (1usize << (T__86 - 85)) | (1usize << (T__87 - 85)) | (1usize << (T__89 - 85)) | (1usize << (T__93 - 85)) | (1usize << (T__100 - 85)) | (1usize << (UpperCaseIdentifier - 85)) | (1usize << (LowerCaseIdentifier - 85)) | (1usize << (DollarIdentifier - 85)) | (1usize << (EscapedOperator - 85)) | (1usize << (IntLiteral - 85)) | (1usize << (RealLiteral - 85)) | (1usize << (StringLiteral - 85)))) != 0) {
				{
				{
				/*InvokeRule overloadeddef*/
				recog.base.set_state(741);
				recog.overloadeddef()?;

				}
				}
				recog.base.set_state(746);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(747);
			recog.base.match_token(T__38,&mut recog.err_handler)?;

			recog.base.set_state(750);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 {
				{
				recog.base.set_state(748);
				recog.base.match_token(T__3,&mut recog.err_handler)?;

				/*InvokeRule typeclasside*/
				recog.base.set_state(749);
				recog.typeclasside()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- overloadeddef ----------------
pub type OverloadeddefContextAll<'input> = OverloadeddefContext<'input>;


pub type OverloadeddefContext<'input> = BaseParserRuleContext<'input,OverloadeddefContextExt<'input>>;

#[derive(Clone)]
pub struct OverloadeddefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for OverloadeddefContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for OverloadeddefContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_overloadeddef(self);
	}
}

impl<'input> CustomRuleContext<'input> for OverloadeddefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_overloadeddef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_overloadeddef }
}
antlr_rust::type_id!{OverloadeddefContextExt<'a>}

impl<'input> OverloadeddefContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OverloadeddefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OverloadeddefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OverloadeddefContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<OverloadeddefContextExt<'input>>{

fn varassign(&self) -> Option<Rc<VarassignContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functiondef(&self) -> Option<Rc<FunctiondefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn moduledef(&self) -> Option<Rc<ModuledefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OverloadeddefContextAttrs<'input> for OverloadeddefContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn overloadeddef(&mut self,)
	-> Result<Rc<OverloadeddefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OverloadeddefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_overloadeddef);
        let mut _localctx: Rc<OverloadeddefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(755);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(75,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule varassign*/
					recog.base.set_state(752);
					recog.varassign()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule functiondef*/
					recog.base.set_state(753);
					recog.functiondef()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule moduledef*/
					recog.base.set_state(754);
					recog.moduledef()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- moduledef ----------------
pub type ModuledefContextAll<'input> = ModuledefContext<'input>;


pub type ModuledefContext<'input> = BaseParserRuleContext<'input,ModuledefContextExt<'input>>;

#[derive(Clone)]
pub struct ModuledefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ModuledefContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ModuledefContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_moduledef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModuledefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_moduledef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_moduledef }
}
antlr_rust::type_id!{ModuledefContextExt<'a>}

impl<'input> ModuledefContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModuledefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModuledefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModuledefContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ModuledefContextExt<'input>>{

fn moduleproto(&self) -> Option<Rc<ModuleprotoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn modulestmt_all(&self) ->  Vec<Rc<ModulestmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn modulestmt(&self, i: usize) -> Option<Rc<ModulestmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ModuledefContextAttrs<'input> for ModuledefContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn moduledef(&mut self,)
	-> Result<Rc<ModuledefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModuledefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_moduledef);
        let mut _localctx: Rc<ModuledefContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(760);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(757);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(762);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule moduleproto*/
			recog.base.set_state(763);
			recog.moduleproto()?;

			recog.base.set_state(767);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__14) | (1usize << T__20) | (1usize << T__27) | (1usize << T__29) | (1usize << T__32) | (1usize << T__40) | (1usize << T__43) | (1usize << T__44) | (1usize << T__45) | (1usize << T__48) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__91 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__96 - 64)) | (1usize << (T__97 - 64)) | (1usize << (T__98 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
				{
				{
				/*InvokeRule modulestmt*/
				recog.base.set_state(764);
				recog.modulestmt()?;

				}
				}
				recog.base.set_state(769);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(770);
			recog.base.match_token(T__39,&mut recog.err_handler)?;

			recog.base.set_state(773);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 {
				{
				recog.base.set_state(771);
				recog.base.match_token(T__3,&mut recog.err_handler)?;

				/*InvokeRule lowerCaseIdentifier*/
				recog.base.set_state(772);
				recog.lowerCaseIdentifier()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- moduleproto ----------------
pub type ModuleprotoContextAll<'input> = ModuleprotoContext<'input>;


pub type ModuleprotoContext<'input> = BaseParserRuleContext<'input,ModuleprotoContextExt<'input>>;

#[derive(Clone)]
pub struct ModuleprotoContextExt<'input>{
	pub moduleinterface: Option<Rc<BsvtypeContextAll<'input>>>,
	pub name: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
	pub monad: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ModuleprotoContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ModuleprotoContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_moduleproto(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModuleprotoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_moduleproto }
	//fn type_rule_index() -> usize where Self: Sized { RULE_moduleproto }
}
antlr_rust::type_id!{ModuleprotoContextExt<'a>}

impl<'input> ModuleprotoContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModuleprotoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModuleprotoContextExt{
				moduleinterface: None, name: None, monad: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ModuleprotoContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ModuleprotoContextExt<'input>>{

fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn moduleprotoformals(&self) -> Option<Rc<ModuleprotoformalsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn provisos(&self) -> Option<Rc<ProvisosContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ModuleprotoContextAttrs<'input> for ModuleprotoContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn moduleproto(&mut self,)
	-> Result<Rc<ModuleprotoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModuleprotoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_moduleproto);
        let mut _localctx: Rc<ModuleprotoContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(826);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(86,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(775);
					recog.base.match_token(T__40,&mut recog.err_handler)?;

					/*InvokeRule bsvtype*/
					recog.base.set_state(776);
					let tmp = recog.bsvtype()?;
					 cast_mut::<_,ModuleprotoContext >(&mut _localctx).moduleinterface = Some(tmp.clone());
					  

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(777);
					let tmp = recog.lowerCaseIdentifier()?;
					 cast_mut::<_,ModuleprotoContext >(&mut _localctx).name = Some(tmp.clone());
					  

					recog.base.set_state(778);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					recog.base.set_state(780);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__41) | (1usize << T__48))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (T__100 - 101)) | (1usize << (UpperCaseIdentifier - 101)) | (1usize << (LowerCaseIdentifier - 101)) | (1usize << (EscapedOperator - 101)) | (1usize << (IntLiteral - 101)))) != 0) {
						{
						/*InvokeRule moduleprotoformals*/
						recog.base.set_state(779);
						recog.moduleprotoformals()?;

						}
					}

					recog.base.set_state(782);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					recog.base.set_state(784);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__102 {
						{
						/*InvokeRule provisos*/
						recog.base.set_state(783);
						recog.provisos()?;

						}
					}

					recog.base.set_state(786);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(788);
					recog.base.match_token(T__40,&mut recog.err_handler)?;

					recog.base.set_state(793);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__22 {
						{
						recog.base.set_state(789);
						recog.base.match_token(T__22,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(790);
						let tmp = recog.expression_rec(0)?;
						 cast_mut::<_,ModuleprotoContext >(&mut _localctx).monad = Some(tmp.clone());
						  

						recog.base.set_state(791);
						recog.base.match_token(T__23,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(795);
					let tmp = recog.lowerCaseIdentifier()?;
					 cast_mut::<_,ModuleprotoContext >(&mut _localctx).name = Some(tmp.clone());
					  

					recog.base.set_state(796);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					/*InvokeRule bsvtype*/
					recog.base.set_state(797);
					let tmp = recog.bsvtype()?;
					 cast_mut::<_,ModuleprotoContext >(&mut _localctx).moduleinterface = Some(tmp.clone());
					  

					recog.base.set_state(798);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					recog.base.set_state(800);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__102 {
						{
						/*InvokeRule provisos*/
						recog.base.set_state(799);
						recog.provisos()?;

						}
					}

					recog.base.set_state(802);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(804);
					recog.base.match_token(T__40,&mut recog.err_handler)?;

					recog.base.set_state(809);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__22 {
						{
						recog.base.set_state(805);
						recog.base.match_token(T__22,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(806);
						let tmp = recog.expression_rec(0)?;
						 cast_mut::<_,ModuleprotoContext >(&mut _localctx).monad = Some(tmp.clone());
						  

						recog.base.set_state(807);
						recog.base.match_token(T__23,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(811);
					let tmp = recog.lowerCaseIdentifier()?;
					 cast_mut::<_,ModuleprotoContext >(&mut _localctx).name = Some(tmp.clone());
					  

					recog.base.set_state(812);
					recog.base.match_token(T__15,&mut recog.err_handler)?;

					recog.base.set_state(813);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					recog.base.set_state(815);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__41) | (1usize << T__48))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (T__100 - 101)) | (1usize << (UpperCaseIdentifier - 101)) | (1usize << (LowerCaseIdentifier - 101)) | (1usize << (EscapedOperator - 101)) | (1usize << (IntLiteral - 101)))) != 0) {
						{
						/*InvokeRule moduleprotoformals*/
						recog.base.set_state(814);
						recog.moduleprotoformals()?;

						}
					}

					recog.base.set_state(817);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					recog.base.set_state(818);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					/*InvokeRule bsvtype*/
					recog.base.set_state(819);
					let tmp = recog.bsvtype()?;
					 cast_mut::<_,ModuleprotoContext >(&mut _localctx).moduleinterface = Some(tmp.clone());
					  

					recog.base.set_state(820);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					recog.base.set_state(822);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__102 {
						{
						/*InvokeRule provisos*/
						recog.base.set_state(821);
						recog.provisos()?;

						}
					}

					recog.base.set_state(824);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- moduleprotoformals ----------------
pub type ModuleprotoformalsContextAll<'input> = ModuleprotoformalsContext<'input>;


pub type ModuleprotoformalsContext<'input> = BaseParserRuleContext<'input,ModuleprotoformalsContextExt<'input>>;

#[derive(Clone)]
pub struct ModuleprotoformalsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ModuleprotoformalsContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ModuleprotoformalsContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_moduleprotoformals(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModuleprotoformalsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_moduleprotoformals }
	//fn type_rule_index() -> usize where Self: Sized { RULE_moduleprotoformals }
}
antlr_rust::type_id!{ModuleprotoformalsContextExt<'a>}

impl<'input> ModuleprotoformalsContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModuleprotoformalsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModuleprotoformalsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModuleprotoformalsContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ModuleprotoformalsContextExt<'input>>{

fn moduleprotoformal_all(&self) ->  Vec<Rc<ModuleprotoformalContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn moduleprotoformal(&self, i: usize) -> Option<Rc<ModuleprotoformalContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ModuleprotoformalsContextAttrs<'input> for ModuleprotoformalsContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn moduleprotoformals(&mut self,)
	-> Result<Rc<ModuleprotoformalsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModuleprotoformalsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_moduleprotoformals);
        let mut _localctx: Rc<ModuleprotoformalsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule moduleprotoformal*/
			recog.base.set_state(828);
			recog.moduleprotoformal()?;

			recog.base.set_state(833);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(829);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule moduleprotoformal*/
				recog.base.set_state(830);
				recog.moduleprotoformal()?;

				}
				}
				recog.base.set_state(835);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- moduleprotoformal ----------------
pub type ModuleprotoformalContextAll<'input> = ModuleprotoformalContext<'input>;


pub type ModuleprotoformalContext<'input> = BaseParserRuleContext<'input,ModuleprotoformalContextExt<'input>>;

#[derive(Clone)]
pub struct ModuleprotoformalContextExt<'input>{
	pub name: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ModuleprotoformalContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ModuleprotoformalContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_moduleprotoformal(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModuleprotoformalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_moduleprotoformal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_moduleprotoformal }
}
antlr_rust::type_id!{ModuleprotoformalContextExt<'a>}

impl<'input> ModuleprotoformalContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModuleprotoformalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModuleprotoformalContextExt{
				name: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ModuleprotoformalContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ModuleprotoformalContextExt<'input>>{

fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn functionproto(&self) -> Option<Rc<FunctionprotoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ModuleprotoformalContextAttrs<'input> for ModuleprotoformalContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn moduleprotoformal(&mut self,)
	-> Result<Rc<ModuleprotoformalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModuleprotoformalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_moduleprotoformal);
        let mut _localctx: Rc<ModuleprotoformalContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(849);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__8 | T__41 | T__100 | UpperCaseIdentifier | LowerCaseIdentifier | EscapedOperator |
			 IntLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(839);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__100 {
						{
						{
						/*InvokeRule attributeinstance*/
						recog.base.set_state(836);
						recog.attributeinstance()?;

						}
						}
						recog.base.set_state(841);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(843);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__41 {
						{
						recog.base.set_state(842);
						recog.base.match_token(T__41,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule bsvtype*/
					recog.base.set_state(845);
					recog.bsvtype()?;

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(846);
					let tmp = recog.lowerCaseIdentifier()?;
					 cast_mut::<_,ModuleprotoformalContext >(&mut _localctx).name = Some(tmp.clone());
					  

					}
				}

			 T__48 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule functionproto*/
					recog.base.set_state(848);
					recog.functionproto()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- modulestmt ----------------
pub type ModulestmtContextAll<'input> = ModulestmtContext<'input>;


pub type ModulestmtContext<'input> = BaseParserRuleContext<'input,ModulestmtContextExt<'input>>;

#[derive(Clone)]
pub struct ModulestmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ModulestmtContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ModulestmtContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_modulestmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModulestmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_modulestmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_modulestmt }
}
antlr_rust::type_id!{ModulestmtContextExt<'a>}

impl<'input> ModulestmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModulestmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModulestmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModulestmtContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ModulestmtContextExt<'input>>{

fn methoddef(&self) -> Option<Rc<MethoddefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn moduledef(&self) -> Option<Rc<ModuledefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn moduleinst(&self) -> Option<Rc<ModuleinstContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn subinterfacedef(&self) -> Option<Rc<SubinterfacedefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stmt(&self) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ModulestmtContextAttrs<'input> for ModulestmtContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn modulestmt(&mut self,)
	-> Result<Rc<ModulestmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModulestmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_modulestmt);
        let mut _localctx: Rc<ModulestmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(856);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(91,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule methoddef*/
					recog.base.set_state(851);
					recog.methoddef()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule moduledef*/
					recog.base.set_state(852);
					recog.moduledef()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule moduleinst*/
					recog.base.set_state(853);
					recog.moduleinst()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule subinterfacedef*/
					recog.base.set_state(854);
					recog.subinterfacedef()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule stmt*/
					recog.base.set_state(855);
					recog.stmt()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- methoddef ----------------
pub type MethoddefContextAll<'input> = MethoddefContext<'input>;


pub type MethoddefContext<'input> = BaseParserRuleContext<'input,MethoddefContextExt<'input>>;

#[derive(Clone)]
pub struct MethoddefContextExt<'input>{
	pub name: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for MethoddefContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for MethoddefContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_methoddef(self);
	}
}

impl<'input> CustomRuleContext<'input> for MethoddefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_methoddef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_methoddef }
}
antlr_rust::type_id!{MethoddefContextExt<'a>}

impl<'input> MethoddefContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MethoddefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MethoddefContextExt{
				name: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait MethoddefContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<MethoddefContextExt<'input>>{

fn lowerCaseIdentifier_all(&self) ->  Vec<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn lowerCaseIdentifier(&self, i: usize) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn provisos(&self) -> Option<Rc<ProvisosContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn methodcond(&self) -> Option<Rc<MethodcondContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stmt_all(&self) ->  Vec<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn methodformals(&self) -> Option<Rc<MethodformalsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MethoddefContextAttrs<'input> for MethoddefContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn methoddef(&mut self,)
	-> Result<Rc<MethoddefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MethoddefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_methoddef);
        let mut _localctx: Rc<MethoddefContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(907);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(103,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(858);
					recog.base.match_token(T__14,&mut recog.err_handler)?;

					recog.base.set_state(860);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(92,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule bsvtype*/
							recog.base.set_state(859);
							recog.bsvtype()?;

							}
						}

						_ => {}
					}
					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(862);
					let tmp = recog.lowerCaseIdentifier()?;
					 cast_mut::<_,MethoddefContext >(&mut _localctx).name = Some(tmp.clone());
					  

					recog.base.set_state(868);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__8 {
						{
						recog.base.set_state(863);
						recog.base.match_token(T__8,&mut recog.err_handler)?;

						recog.base.set_state(865);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if _la==T__8 || _la==T__48 || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (T__100 - 101)) | (1usize << (UpperCaseIdentifier - 101)) | (1usize << (LowerCaseIdentifier - 101)) | (1usize << (EscapedOperator - 101)) | (1usize << (IntLiteral - 101)))) != 0) {
							{
							/*InvokeRule methodformals*/
							recog.base.set_state(864);
							recog.methodformals()?;

							}
						}

						recog.base.set_state(867);
						recog.base.match_token(T__10,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(871);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__102 {
						{
						/*InvokeRule provisos*/
						recog.base.set_state(870);
						recog.provisos()?;

						}
					}

					recog.base.set_state(874);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__43 || _la==T__44 {
						{
						/*InvokeRule methodcond*/
						recog.base.set_state(873);
						recog.methodcond()?;

						}
					}

					recog.base.set_state(876);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					recog.base.set_state(880);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__29) | (1usize << T__32) | (1usize << T__43) | (1usize << T__44) | (1usize << T__45) | (1usize << T__48) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__91 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__96 - 64)) | (1usize << (T__97 - 64)) | (1usize << (T__98 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
						{
						{
						/*InvokeRule stmt*/
						recog.base.set_state(877);
						recog.stmt()?;

						}
						}
						recog.base.set_state(882);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(883);
					recog.base.match_token(T__42,&mut recog.err_handler)?;

					recog.base.set_state(886);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__3 {
						{
						recog.base.set_state(884);
						recog.base.match_token(T__3,&mut recog.err_handler)?;

						/*InvokeRule lowerCaseIdentifier*/
						recog.base.set_state(885);
						recog.lowerCaseIdentifier()?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(888);
					recog.base.match_token(T__14,&mut recog.err_handler)?;

					recog.base.set_state(890);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(99,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule bsvtype*/
							recog.base.set_state(889);
							recog.bsvtype()?;

							}
						}

						_ => {}
					}
					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(892);
					let tmp = recog.lowerCaseIdentifier()?;
					 cast_mut::<_,MethoddefContext >(&mut _localctx).name = Some(tmp.clone());
					  

					recog.base.set_state(898);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__8 {
						{
						recog.base.set_state(893);
						recog.base.match_token(T__8,&mut recog.err_handler)?;

						recog.base.set_state(895);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if _la==T__8 || _la==T__48 || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (T__100 - 101)) | (1usize << (UpperCaseIdentifier - 101)) | (1usize << (LowerCaseIdentifier - 101)) | (1usize << (EscapedOperator - 101)) | (1usize << (IntLiteral - 101)))) != 0) {
							{
							/*InvokeRule methodformals*/
							recog.base.set_state(894);
							recog.methodformals()?;

							}
						}

						recog.base.set_state(897);
						recog.base.match_token(T__10,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(901);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__43 || _la==T__44 {
						{
						/*InvokeRule methodcond*/
						recog.base.set_state(900);
						recog.methodcond()?;

						}
					}

					recog.base.set_state(903);
					recog.base.match_token(T__24,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(904);
					recog.expression_rec(0)?;

					recog.base.set_state(905);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- methodformals ----------------
pub type MethodformalsContextAll<'input> = MethodformalsContext<'input>;


pub type MethodformalsContext<'input> = BaseParserRuleContext<'input,MethodformalsContextExt<'input>>;

#[derive(Clone)]
pub struct MethodformalsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for MethodformalsContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for MethodformalsContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_methodformals(self);
	}
}

impl<'input> CustomRuleContext<'input> for MethodformalsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_methodformals }
	//fn type_rule_index() -> usize where Self: Sized { RULE_methodformals }
}
antlr_rust::type_id!{MethodformalsContextExt<'a>}

impl<'input> MethodformalsContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MethodformalsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MethodformalsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MethodformalsContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<MethodformalsContextExt<'input>>{

fn methodformal_all(&self) ->  Vec<Rc<MethodformalContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn methodformal(&self, i: usize) -> Option<Rc<MethodformalContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MethodformalsContextAttrs<'input> for MethodformalsContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn methodformals(&mut self,)
	-> Result<Rc<MethodformalsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MethodformalsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_methodformals);
        let mut _localctx: Rc<MethodformalsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule methodformal*/
			recog.base.set_state(909);
			recog.methodformal()?;

			recog.base.set_state(914);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(910);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule methodformal*/
				recog.base.set_state(911);
				recog.methodformal()?;

				}
				}
				recog.base.set_state(916);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- methodformal ----------------
pub type MethodformalContextAll<'input> = MethodformalContext<'input>;


pub type MethodformalContext<'input> = BaseParserRuleContext<'input,MethodformalContextExt<'input>>;

#[derive(Clone)]
pub struct MethodformalContextExt<'input>{
	pub name: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for MethodformalContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for MethodformalContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_methodformal(self);
	}
}

impl<'input> CustomRuleContext<'input> for MethodformalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_methodformal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_methodformal }
}
antlr_rust::type_id!{MethodformalContextExt<'a>}

impl<'input> MethodformalContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MethodformalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MethodformalContextExt{
				name: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait MethodformalContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<MethodformalContextExt<'input>>{

fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionproto(&self) -> Option<Rc<FunctionprotoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MethodformalContextAttrs<'input> for MethodformalContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn methodformal(&mut self,)
	-> Result<Rc<MethodformalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MethodformalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_methodformal);
        let mut _localctx: Rc<MethodformalContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(928);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__8 | T__100 | UpperCaseIdentifier | LowerCaseIdentifier | EscapedOperator |
			 IntLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(920);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__100 {
						{
						{
						/*InvokeRule attributeinstance*/
						recog.base.set_state(917);
						recog.attributeinstance()?;

						}
						}
						recog.base.set_state(922);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(924);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(106,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule bsvtype*/
							recog.base.set_state(923);
							recog.bsvtype()?;

							}
						}

						_ => {}
					}
					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(926);
					let tmp = recog.lowerCaseIdentifier()?;
					 cast_mut::<_,MethodformalContext >(&mut _localctx).name = Some(tmp.clone());
					  

					}
				}

			 T__48 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule functionproto*/
					recog.base.set_state(927);
					recog.functionproto()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- methodcond ----------------
pub type MethodcondContextAll<'input> = MethodcondContext<'input>;


pub type MethodcondContext<'input> = BaseParserRuleContext<'input,MethodcondContextExt<'input>>;

#[derive(Clone)]
pub struct MethodcondContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for MethodcondContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for MethodcondContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_methodcond(self);
	}
}

impl<'input> CustomRuleContext<'input> for MethodcondContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_methodcond }
	//fn type_rule_index() -> usize where Self: Sized { RULE_methodcond }
}
antlr_rust::type_id!{MethodcondContextExt<'a>}

impl<'input> MethodcondContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MethodcondContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MethodcondContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MethodcondContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<MethodcondContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MethodcondContextAttrs<'input> for MethodcondContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn methodcond(&mut self,)
	-> Result<Rc<MethodcondContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MethodcondContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_methodcond);
        let mut _localctx: Rc<MethodcondContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(930);
			_la = recog.base.input.la(1);
			if { !(_la==T__43 || _la==T__44) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(931);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(932);
			recog.expression_rec(0)?;

			recog.base.set_state(933);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- subinterfacedef ----------------
pub type SubinterfacedefContextAll<'input> = SubinterfacedefContext<'input>;


pub type SubinterfacedefContext<'input> = BaseParserRuleContext<'input,SubinterfacedefContextExt<'input>>;

#[derive(Clone)]
pub struct SubinterfacedefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for SubinterfacedefContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for SubinterfacedefContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_subinterfacedef(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubinterfacedefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_subinterfacedef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_subinterfacedef }
}
antlr_rust::type_id!{SubinterfacedefContextExt<'a>}

impl<'input> SubinterfacedefContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SubinterfacedefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SubinterfacedefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SubinterfacedefContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<SubinterfacedefContextExt<'input>>{

fn upperCaseIdentifier(&self) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lowerCaseIdentifier_all(&self) ->  Vec<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn lowerCaseIdentifier(&self, i: usize) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn interfacestmt_all(&self) ->  Vec<Rc<InterfacestmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn interfacestmt(&self, i: usize) -> Option<Rc<InterfacestmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SubinterfacedefContextAttrs<'input> for SubinterfacedefContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn subinterfacedef(&mut self,)
	-> Result<Rc<SubinterfacedefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SubinterfacedefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_subinterfacedef);
        let mut _localctx: Rc<SubinterfacedefContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(959);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(111,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(935);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(936);
					recog.upperCaseIdentifier()?;

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(937);
					recog.lowerCaseIdentifier()?;

					recog.base.set_state(938);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					recog.base.set_state(942);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__14) | (1usize << T__20) | (1usize << T__27) | (1usize << T__29) | (1usize << T__43) | (1usize << T__53))) != 0) || ((((_la - 85)) & !0x3f) == 0 && ((1usize << (_la - 85)) & ((1usize << (T__84 - 85)) | (1usize << (T__85 - 85)) | (1usize << (T__86 - 85)) | (1usize << (T__87 - 85)) | (1usize << (T__89 - 85)) | (1usize << (T__93 - 85)) | (1usize << (T__100 - 85)) | (1usize << (UpperCaseIdentifier - 85)) | (1usize << (LowerCaseIdentifier - 85)) | (1usize << (DollarIdentifier - 85)) | (1usize << (EscapedOperator - 85)) | (1usize << (IntLiteral - 85)) | (1usize << (RealLiteral - 85)) | (1usize << (StringLiteral - 85)))) != 0) {
						{
						{
						/*InvokeRule interfacestmt*/
						recog.base.set_state(939);
						recog.interfacestmt()?;

						}
						}
						recog.base.set_state(944);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(945);
					recog.base.match_token(T__13,&mut recog.err_handler)?;

					recog.base.set_state(948);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__3 {
						{
						recog.base.set_state(946);
						recog.base.match_token(T__3,&mut recog.err_handler)?;

						/*InvokeRule lowerCaseIdentifier*/
						recog.base.set_state(947);
						recog.lowerCaseIdentifier()?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(950);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					recog.base.set_state(952);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==UpperCaseIdentifier {
						{
						/*InvokeRule upperCaseIdentifier*/
						recog.base.set_state(951);
						recog.upperCaseIdentifier()?;

						}
					}

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(954);
					recog.lowerCaseIdentifier()?;

					recog.base.set_state(955);
					recog.base.match_token(T__24,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(956);
					recog.expression_rec(0)?;

					recog.base.set_state(957);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ruledef ----------------
pub type RuledefContextAll<'input> = RuledefContext<'input>;


pub type RuledefContext<'input> = BaseParserRuleContext<'input,RuledefContextExt<'input>>;

#[derive(Clone)]
pub struct RuledefContextExt<'input>{
	pub name: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for RuledefContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for RuledefContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ruledef(self);
	}
}

impl<'input> CustomRuleContext<'input> for RuledefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ruledef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ruledef }
}
antlr_rust::type_id!{RuledefContextExt<'a>}

impl<'input> RuledefContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RuledefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RuledefContextExt{
				name: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait RuledefContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<RuledefContextExt<'input>>{

fn lowerCaseIdentifier_all(&self) ->  Vec<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn lowerCaseIdentifier(&self, i: usize) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn rulecond(&self) -> Option<Rc<RulecondContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stmt_all(&self) ->  Vec<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RuledefContextAttrs<'input> for RuledefContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ruledef(&mut self,)
	-> Result<Rc<RuledefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RuledefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_ruledef);
        let mut _localctx: Rc<RuledefContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(964);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(961);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(966);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(967);
			recog.base.match_token(T__45,&mut recog.err_handler)?;

			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(968);
			let tmp = recog.lowerCaseIdentifier()?;
			 cast_mut::<_,RuledefContext >(&mut _localctx).name = Some(tmp.clone());
			  

			recog.base.set_state(970);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__43) | (1usize << T__44))) != 0) {
				{
				/*InvokeRule rulecond*/
				recog.base.set_state(969);
				recog.rulecond()?;

				}
			}

			recog.base.set_state(972);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			recog.base.set_state(976);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__29) | (1usize << T__32) | (1usize << T__43) | (1usize << T__44) | (1usize << T__45) | (1usize << T__48) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__91 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__96 - 64)) | (1usize << (T__97 - 64)) | (1usize << (T__98 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
				{
				{
				/*InvokeRule stmt*/
				recog.base.set_state(973);
				recog.stmt()?;

				}
				}
				recog.base.set_state(978);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(979);
			recog.base.match_token(T__46,&mut recog.err_handler)?;

			recog.base.set_state(982);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 {
				{
				recog.base.set_state(980);
				recog.base.match_token(T__3,&mut recog.err_handler)?;

				/*InvokeRule lowerCaseIdentifier*/
				recog.base.set_state(981);
				recog.lowerCaseIdentifier()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- rulecond ----------------
pub type RulecondContextAll<'input> = RulecondContext<'input>;


pub type RulecondContext<'input> = BaseParserRuleContext<'input,RulecondContextExt<'input>>;

#[derive(Clone)]
pub struct RulecondContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for RulecondContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for RulecondContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_rulecond(self);
	}
}

impl<'input> CustomRuleContext<'input> for RulecondContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rulecond }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rulecond }
}
antlr_rust::type_id!{RulecondContextExt<'a>}

impl<'input> RulecondContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RulecondContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RulecondContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RulecondContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<RulecondContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RulecondContextAttrs<'input> for RulecondContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rulecond(&mut self,)
	-> Result<Rc<RulecondContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RulecondContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_rulecond);
        let mut _localctx: Rc<RulecondContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(987);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__43 
				=> {
					{
					recog.base.set_state(984);
					recog.base.match_token(T__43,&mut recog.err_handler)?;

					}
				}

			 T__44 
				=> {
					{
					recog.base.set_state(985);
					recog.base.match_token(T__44,&mut recog.err_handler)?;

					}
				}

			 T__8 
				=> {
					{
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(989);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(990);
			recog.expression_rec(0)?;

			recog.base.set_state(991);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functiondef ----------------
pub type FunctiondefContextAll<'input> = FunctiondefContext<'input>;


pub type FunctiondefContext<'input> = BaseParserRuleContext<'input,FunctiondefContextExt<'input>>;

#[derive(Clone)]
pub struct FunctiondefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for FunctiondefContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for FunctiondefContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functiondef(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctiondefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functiondef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functiondef }
}
antlr_rust::type_id!{FunctiondefContextExt<'a>}

impl<'input> FunctiondefContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctiondefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctiondefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctiondefContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<FunctiondefContextExt<'input>>{

fn functionproto(&self) -> Option<Rc<FunctionprotoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn stmt_all(&self) ->  Vec<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctiondefContextAttrs<'input> for FunctiondefContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functiondef(&mut self,)
	-> Result<Rc<FunctiondefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctiondefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_functiondef);
        let mut _localctx: Rc<FunctiondefContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1017);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(120,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(996);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__100 {
						{
						{
						/*InvokeRule attributeinstance*/
						recog.base.set_state(993);
						recog.attributeinstance()?;

						}
						}
						recog.base.set_state(998);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule functionproto*/
					recog.base.set_state(999);
					recog.functionproto()?;

					recog.base.set_state(1000);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					recog.base.set_state(1004);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__29) | (1usize << T__32) | (1usize << T__43) | (1usize << T__44) | (1usize << T__45) | (1usize << T__48) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__91 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__96 - 64)) | (1usize << (T__97 - 64)) | (1usize << (T__98 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
						{
						{
						/*InvokeRule stmt*/
						recog.base.set_state(1001);
						recog.stmt()?;

						}
						}
						recog.base.set_state(1006);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1007);
					recog.base.match_token(T__47,&mut recog.err_handler)?;

					recog.base.set_state(1010);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__3 {
						{
						recog.base.set_state(1008);
						recog.base.match_token(T__3,&mut recog.err_handler)?;

						/*InvokeRule lowerCaseIdentifier*/
						recog.base.set_state(1009);
						recog.lowerCaseIdentifier()?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule functionproto*/
					recog.base.set_state(1012);
					recog.functionproto()?;

					recog.base.set_state(1013);
					recog.base.match_token(T__24,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1014);
					recog.expression_rec(0)?;

					recog.base.set_state(1015);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionproto ----------------
pub type FunctionprotoContextAll<'input> = FunctionprotoContext<'input>;


pub type FunctionprotoContext<'input> = BaseParserRuleContext<'input,FunctionprotoContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionprotoContextExt<'input>{
	pub name: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for FunctionprotoContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for FunctionprotoContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionproto(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionprotoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionproto }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionproto }
}
antlr_rust::type_id!{FunctionprotoContextExt<'a>}

impl<'input> FunctionprotoContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionprotoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionprotoContextExt{
				name: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionprotoContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<FunctionprotoContextExt<'input>>{

fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn provisos(&self) -> Option<Rc<ProvisosContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn methodformals(&self) -> Option<Rc<MethodformalsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctionprotoContextAttrs<'input> for FunctionprotoContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionproto(&mut self,)
	-> Result<Rc<FunctionprotoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionprotoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_functionproto);
        let mut _localctx: Rc<FunctionprotoContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1019);
			recog.base.match_token(T__48,&mut recog.err_handler)?;

			recog.base.set_state(1021);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(121,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule bsvtype*/
					recog.base.set_state(1020);
					recog.bsvtype()?;

					}
				}

				_ => {}
			}
			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(1023);
			let tmp = recog.lowerCaseIdentifier()?;
			 cast_mut::<_,FunctionprotoContext >(&mut _localctx).name = Some(tmp.clone());
			  

			recog.base.set_state(1029);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__8 {
				{
				recog.base.set_state(1024);
				recog.base.match_token(T__8,&mut recog.err_handler)?;

				recog.base.set_state(1026);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==T__8 || _la==T__48 || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (T__100 - 101)) | (1usize << (UpperCaseIdentifier - 101)) | (1usize << (LowerCaseIdentifier - 101)) | (1usize << (EscapedOperator - 101)) | (1usize << (IntLiteral - 101)))) != 0) {
					{
					/*InvokeRule methodformals*/
					recog.base.set_state(1025);
					recog.methodformals()?;

					}
				}

				recog.base.set_state(1028);
				recog.base.match_token(T__10,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1032);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__102 {
				{
				/*InvokeRule provisos*/
				recog.base.set_state(1031);
				recog.provisos()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- externcimport ----------------
pub type ExterncimportContextAll<'input> = ExterncimportContext<'input>;


pub type ExterncimportContext<'input> = BaseParserRuleContext<'input,ExterncimportContextExt<'input>>;

#[derive(Clone)]
pub struct ExterncimportContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ExterncimportContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ExterncimportContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_externcimport(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExterncimportContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_externcimport }
	//fn type_rule_index() -> usize where Self: Sized { RULE_externcimport }
}
antlr_rust::type_id!{ExterncimportContextExt<'a>}

impl<'input> ExterncimportContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExterncimportContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExterncimportContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExterncimportContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ExterncimportContextExt<'input>>{

fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lowerCaseIdentifier_all(&self) ->  Vec<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn lowerCaseIdentifier(&self, i: usize) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn externcfuncargs(&self) -> Option<Rc<ExterncfuncargsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExterncimportContextAttrs<'input> for ExterncimportContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn externcimport(&mut self,)
	-> Result<Rc<ExterncimportContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExterncimportContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_externcimport);
        let mut _localctx: Rc<ExterncimportContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1034);
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			recog.base.set_state(1035);
			recog.base.match_token(T__49,&mut recog.err_handler)?;

			recog.base.set_state(1039);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LowerCaseIdentifier || _la==EscapedOperator {
				{
				/*InvokeRule lowerCaseIdentifier*/
				recog.base.set_state(1036);
				recog.lowerCaseIdentifier()?;

				recog.base.set_state(1037);
				recog.base.match_token(T__24,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1041);
			recog.base.match_token(T__48,&mut recog.err_handler)?;

			/*InvokeRule bsvtype*/
			recog.base.set_state(1042);
			recog.bsvtype()?;

			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(1043);
			recog.lowerCaseIdentifier()?;

			recog.base.set_state(1044);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			recog.base.set_state(1046);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__8 || ((((_la - 105)) & !0x3f) == 0 && ((1usize << (_la - 105)) & ((1usize << (UpperCaseIdentifier - 105)) | (1usize << (LowerCaseIdentifier - 105)) | (1usize << (EscapedOperator - 105)) | (1usize << (IntLiteral - 105)))) != 0) {
				{
				/*InvokeRule externcfuncargs*/
				recog.base.set_state(1045);
				recog.externcfuncargs()?;

				}
			}

			recog.base.set_state(1048);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			recog.base.set_state(1049);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- externcfuncargs ----------------
pub type ExterncfuncargsContextAll<'input> = ExterncfuncargsContext<'input>;


pub type ExterncfuncargsContext<'input> = BaseParserRuleContext<'input,ExterncfuncargsContextExt<'input>>;

#[derive(Clone)]
pub struct ExterncfuncargsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ExterncfuncargsContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ExterncfuncargsContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_externcfuncargs(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExterncfuncargsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_externcfuncargs }
	//fn type_rule_index() -> usize where Self: Sized { RULE_externcfuncargs }
}
antlr_rust::type_id!{ExterncfuncargsContextExt<'a>}

impl<'input> ExterncfuncargsContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExterncfuncargsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExterncfuncargsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExterncfuncargsContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ExterncfuncargsContextExt<'input>>{

fn externcfuncarg_all(&self) ->  Vec<Rc<ExterncfuncargContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn externcfuncarg(&self, i: usize) -> Option<Rc<ExterncfuncargContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExterncfuncargsContextAttrs<'input> for ExterncfuncargsContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn externcfuncargs(&mut self,)
	-> Result<Rc<ExterncfuncargsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExterncfuncargsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_externcfuncargs);
        let mut _localctx: Rc<ExterncfuncargsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule externcfuncarg*/
			recog.base.set_state(1051);
			recog.externcfuncarg()?;

			recog.base.set_state(1056);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(1052);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule externcfuncarg*/
				recog.base.set_state(1053);
				recog.externcfuncarg()?;

				}
				}
				recog.base.set_state(1058);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- externcfuncarg ----------------
pub type ExterncfuncargContextAll<'input> = ExterncfuncargContext<'input>;


pub type ExterncfuncargContext<'input> = BaseParserRuleContext<'input,ExterncfuncargContextExt<'input>>;

#[derive(Clone)]
pub struct ExterncfuncargContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ExterncfuncargContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ExterncfuncargContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_externcfuncarg(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExterncfuncargContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_externcfuncarg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_externcfuncarg }
}
antlr_rust::type_id!{ExterncfuncargContextExt<'a>}

impl<'input> ExterncfuncargContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExterncfuncargContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExterncfuncargContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExterncfuncargContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ExterncfuncargContextExt<'input>>{

fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExterncfuncargContextAttrs<'input> for ExterncfuncargContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn externcfuncarg(&mut self,)
	-> Result<Rc<ExterncfuncargContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExterncfuncargContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_externcfuncarg);
        let mut _localctx: Rc<ExterncfuncargContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule bsvtype*/
			recog.base.set_state(1059);
			recog.bsvtype()?;

			recog.base.set_state(1061);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LowerCaseIdentifier || _la==EscapedOperator {
				{
				/*InvokeRule lowerCaseIdentifier*/
				recog.base.set_state(1060);
				recog.lowerCaseIdentifier()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- varassign ----------------
pub type VarassignContextAll<'input> = VarassignContext<'input>;


pub type VarassignContext<'input> = BaseParserRuleContext<'input,VarassignContextExt<'input>>;

#[derive(Clone)]
pub struct VarassignContextExt<'input>{
	pub op: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for VarassignContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for VarassignContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_varassign(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarassignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_varassign }
	//fn type_rule_index() -> usize where Self: Sized { RULE_varassign }
}
antlr_rust::type_id!{VarassignContextExt<'a>}

impl<'input> VarassignContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VarassignContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VarassignContextExt{
				op: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait VarassignContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<VarassignContextExt<'input>>{

fn lvalue_all(&self) ->  Vec<Rc<LvalueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn lvalue(&self, i: usize) -> Option<Rc<LvalueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> VarassignContextAttrs<'input> for VarassignContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn varassign(&mut self,)
	-> Result<Rc<VarassignContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VarassignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_varassign);
        let mut _localctx: Rc<VarassignContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			recog.base.set_state(1094);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(132,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1066);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(129,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule attributeinstance*/
							recog.base.set_state(1063);
							recog.attributeinstance()?;

							}
							} 
						}
						recog.base.set_state(1068);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(129,&mut recog.base)?;
					}
					/*InvokeRule lvalue*/
					recog.base.set_state(1069);
					recog.lvalue()?;

					recog.base.set_state(1070);
					 cast_mut::<_,VarassignContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
					 
					_la = recog.base.input.la(1);
					if { !(_la==T__24 || _la==T__31) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						 cast_mut::<_,VarassignContext >(&mut _localctx).op = Some(tmp.clone());
						  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expression*/
					recog.base.set_state(1071);
					recog.expression_rec(0)?;

					recog.base.set_state(1072);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1077);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__100 {
						{
						{
						/*InvokeRule attributeinstance*/
						recog.base.set_state(1074);
						recog.attributeinstance()?;

						}
						}
						recog.base.set_state(1079);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1080);
					recog.base.match_token(T__20,&mut recog.err_handler)?;

					/*InvokeRule lvalue*/
					recog.base.set_state(1081);
					recog.lvalue()?;

					recog.base.set_state(1086);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__5 {
						{
						{
						recog.base.set_state(1082);
						recog.base.match_token(T__5,&mut recog.err_handler)?;

						/*InvokeRule lvalue*/
						recog.base.set_state(1083);
						recog.lvalue()?;

						}
						}
						recog.base.set_state(1088);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1089);
					recog.base.match_token(T__21,&mut recog.err_handler)?;

					recog.base.set_state(1090);
					 cast_mut::<_,VarassignContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
					 
					_la = recog.base.input.la(1);
					if { !(_la==T__24 || _la==T__31) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						 cast_mut::<_,VarassignContext >(&mut _localctx).op = Some(tmp.clone());
						  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expression*/
					recog.base.set_state(1091);
					recog.expression_rec(0)?;

					recog.base.set_state(1092);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lvalue ----------------
pub type LvalueContextAll<'input> = LvalueContext<'input>;


pub type LvalueContext<'input> = BaseParserRuleContext<'input,LvalueContextExt<'input>>;

#[derive(Clone)]
pub struct LvalueContextExt<'input>{
	pub index: Option<Rc<ExpressionContextAll<'input>>>,
	pub msb: Option<Rc<ExpressionContextAll<'input>>>,
	pub lsb: Option<Rc<ExpressionContextAll<'input>>>,
	pub widthup: Option<TokenType<'input>>,
	pub widthdown: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for LvalueContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for LvalueContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_lvalue(self);
	}
}

impl<'input> CustomRuleContext<'input> for LvalueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lvalue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lvalue }
}
antlr_rust::type_id!{LvalueContextExt<'a>}

impl<'input> LvalueContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LvalueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LvalueContextExt{
				widthup: None, widthdown: None, 
				index: None, msb: None, lsb: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait LvalueContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<LvalueContextExt<'input>>{

fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn exprprimary(&self) -> Option<Rc<ExprprimaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token IntLiteral
/// Returns `None` if there is no child corresponding to token IntLiteral
fn IntLiteral(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
	self.get_token(IntLiteral, 0)
}

}

impl<'input> LvalueContextAttrs<'input> for LvalueContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lvalue(&mut self,)
	-> Result<Rc<LvalueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LvalueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_lvalue);
        let mut _localctx: Rc<LvalueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1119);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(134,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(1096);
					recog.lowerCaseIdentifier()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule exprprimary*/
					recog.base.set_state(1097);
					recog.exprprimary_rec(0)?;

					recog.base.set_state(1098);
					recog.base.match_token(T__50,&mut recog.err_handler)?;

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(1099);
					recog.lowerCaseIdentifier()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule exprprimary*/
					recog.base.set_state(1101);
					recog.exprprimary_rec(0)?;

					recog.base.set_state(1102);
					recog.base.match_token(T__22,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1103);
					let tmp = recog.expression_rec(0)?;
					 cast_mut::<_,LvalueContext >(&mut _localctx).index = Some(tmp.clone());
					  

					recog.base.set_state(1104);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule exprprimary*/
					recog.base.set_state(1106);
					recog.exprprimary_rec(0)?;

					recog.base.set_state(1107);
					recog.base.match_token(T__22,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1108);
					let tmp = recog.expression_rec(0)?;
					 cast_mut::<_,LvalueContext >(&mut _localctx).msb = Some(tmp.clone());
					  

					recog.base.set_state(1115);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__3 
						=> {
							{
							{
							recog.base.set_state(1109);
							recog.base.match_token(T__3,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1110);
							let tmp = recog.expression_rec(0)?;
							 cast_mut::<_,LvalueContext >(&mut _localctx).lsb = Some(tmp.clone());
							  

							}
							}
						}

					 T__51 
						=> {
							{
							{
							recog.base.set_state(1111);
							recog.base.match_token(T__51,&mut recog.err_handler)?;

							recog.base.set_state(1112);
							let tmp = recog.base.match_token(IntLiteral,&mut recog.err_handler)?;
							 cast_mut::<_,LvalueContext >(&mut _localctx).widthup = Some(tmp.clone());
							  

							}
							}
						}

					 T__52 
						=> {
							{
							{
							recog.base.set_state(1113);
							recog.base.match_token(T__52,&mut recog.err_handler)?;

							recog.base.set_state(1114);
							let tmp = recog.base.match_token(IntLiteral,&mut recog.err_handler)?;
							 cast_mut::<_,LvalueContext >(&mut _localctx).widthdown = Some(tmp.clone());
							  

							}
							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(1117);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- bsvtype ----------------
pub type BsvtypeContextAll<'input> = BsvtypeContext<'input>;


pub type BsvtypeContext<'input> = BaseParserRuleContext<'input,BsvtypeContextExt<'input>>;

#[derive(Clone)]
pub struct BsvtypeContextExt<'input>{
	pub var: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for BsvtypeContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for BsvtypeContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_bsvtype(self);
	}
}

impl<'input> CustomRuleContext<'input> for BsvtypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_bsvtype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_bsvtype }
}
antlr_rust::type_id!{BsvtypeContextExt<'a>}

impl<'input> BsvtypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BsvtypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BsvtypeContextExt{
				var: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait BsvtypeContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<BsvtypeContextExt<'input>>{

fn typeide(&self) -> Option<Rc<TypeideContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn bsvtype_all(&self) ->  Vec<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn bsvtype(&self, i: usize) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typenat(&self) -> Option<Rc<TypenatContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BsvtypeContextAttrs<'input> for BsvtypeContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn bsvtype(&mut self,)
	-> Result<Rc<BsvtypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BsvtypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_bsvtype);
        let mut _localctx: Rc<BsvtypeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1142);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(137,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule typeide*/
					recog.base.set_state(1121);
					recog.typeide()?;

					recog.base.set_state(1134);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__15 {
						{
						recog.base.set_state(1122);
						recog.base.match_token(T__15,&mut recog.err_handler)?;

						recog.base.set_state(1123);
						recog.base.match_token(T__8,&mut recog.err_handler)?;

						/*InvokeRule bsvtype*/
						recog.base.set_state(1124);
						recog.bsvtype()?;

						recog.base.set_state(1129);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==T__5 {
							{
							{
							recog.base.set_state(1125);
							recog.base.match_token(T__5,&mut recog.err_handler)?;

							/*InvokeRule bsvtype*/
							recog.base.set_state(1126);
							recog.bsvtype()?;

							}
							}
							recog.base.set_state(1131);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						recog.base.set_state(1132);
						recog.base.match_token(T__10,&mut recog.err_handler)?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(1136);
					let tmp = recog.lowerCaseIdentifier()?;
					 cast_mut::<_,BsvtypeContext >(&mut _localctx).var = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule typenat*/
					recog.base.set_state(1137);
					recog.typenat()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1138);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					/*InvokeRule bsvtype*/
					recog.base.set_state(1139);
					recog.bsvtype()?;

					recog.base.set_state(1140);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeide ----------------
pub type TypeideContextAll<'input> = TypeideContext<'input>;


pub type TypeideContext<'input> = BaseParserRuleContext<'input,TypeideContextExt<'input>>;

#[derive(Clone)]
pub struct TypeideContextExt<'input>{
	pub pkg: Option<Rc<UpperCaseIdentifierContextAll<'input>>>,
	pub name: Option<Rc<UpperCaseIdentifierContextAll<'input>>>,
	pub typevar: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypeideContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypeideContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typeide(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeideContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeide }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeide }
}
antlr_rust::type_id!{TypeideContextExt<'a>}

impl<'input> TypeideContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeideContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeideContextExt{
				pkg: None, name: None, typevar: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeideContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypeideContextExt<'input>>{

fn upperCaseIdentifier_all(&self) ->  Vec<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn upperCaseIdentifier(&self, i: usize) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypeideContextAttrs<'input> for TypeideContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeide(&mut self,)
	-> Result<Rc<TypeideContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeideContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_typeide);
        let mut _localctx: Rc<TypeideContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			recog.base.set_state(1154);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 UpperCaseIdentifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1149);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(138,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule upperCaseIdentifier*/
							recog.base.set_state(1144);
							let tmp = recog.upperCaseIdentifier()?;
							 cast_mut::<_,TypeideContext >(&mut _localctx).pkg = Some(tmp.clone());
							  

							recog.base.set_state(1145);
							recog.base.match_token(T__6,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(1151);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(138,&mut recog.base)?;
					}
					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(1152);
					let tmp = recog.upperCaseIdentifier()?;
					 cast_mut::<_,TypeideContext >(&mut _localctx).name = Some(tmp.clone());
					  

					}
				}

			 LowerCaseIdentifier | EscapedOperator 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(1153);
					let tmp = recog.lowerCaseIdentifier()?;
					 cast_mut::<_,TypeideContext >(&mut _localctx).typevar = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typenat ----------------
pub type TypenatContextAll<'input> = TypenatContext<'input>;


pub type TypenatContext<'input> = BaseParserRuleContext<'input,TypenatContextExt<'input>>;

#[derive(Clone)]
pub struct TypenatContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TypenatContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypenatContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typenat(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypenatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typenat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typenat }
}
antlr_rust::type_id!{TypenatContextExt<'a>}

impl<'input> TypenatContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypenatContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypenatContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypenatContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TypenatContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IntLiteral
/// Returns `None` if there is no child corresponding to token IntLiteral
fn IntLiteral(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
	self.get_token(IntLiteral, 0)
}

}

impl<'input> TypenatContextAttrs<'input> for TypenatContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typenat(&mut self,)
	-> Result<Rc<TypenatContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypenatContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_typenat);
        let mut _localctx: Rc<TypenatContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1156);
			recog.base.match_token(IntLiteral,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
#[derive(Debug)]
pub enum ExpressionContextAll<'input>{
	OperatorexprContext(OperatorexprContext<'input>),
	CaseexprContext(CaseexprContext<'input>),
	MatchesexprContext(MatchesexprContext<'input>),
	CondexprContext(CondexprContext<'input>),
Error(ExpressionContext<'input>)
}
antlr_rust::type_id!{ExpressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExpressionContextAll<'input>{}

impl<'input> BSVParserContext<'input> for ExpressionContextAll<'input>{}

impl<'input> Deref for ExpressionContextAll<'input>{
	type Target = dyn ExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExpressionContextAll::*;
		match self{
			OperatorexprContext(inner) => inner,
			CaseexprContext(inner) => inner,
			MatchesexprContext(inner) => inner,
			CondexprContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn BSVListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn BSVListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::type_id!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
		ExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExpressionContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{


}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

pub type OperatorexprContext<'input> = BaseParserRuleContext<'input,OperatorexprContextExt<'input>>;

pub trait OperatorexprContextAttrs<'input>: BSVParserContext<'input>{
	fn binopexpr(&self) -> Option<Rc<BinopexprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> OperatorexprContextAttrs<'input> for OperatorexprContext<'input>{}

pub struct OperatorexprContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{OperatorexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for OperatorexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for OperatorexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_operatorexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for OperatorexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for OperatorexprContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for OperatorexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for OperatorexprContext<'input> {}

impl<'input> OperatorexprContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::OperatorexprContext(
				BaseParserRuleContext::copy_from(ctx,OperatorexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CaseexprContext<'input> = BaseParserRuleContext<'input,CaseexprContextExt<'input>>;

pub trait CaseexprContextAttrs<'input>: BSVParserContext<'input>{
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn caseexprdefaultitem(&self) -> Option<Rc<CaseexprdefaultitemContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn caseexpritem_all(&self) ->  Vec<Rc<CaseexpritemContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn caseexpritem(&self, i: usize) -> Option<Rc<CaseexpritemContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn caseexprpatitem_all(&self) ->  Vec<Rc<CaseexprpatitemContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn caseexprpatitem(&self, i: usize) -> Option<Rc<CaseexprpatitemContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> CaseexprContextAttrs<'input> for CaseexprContext<'input>{}

pub struct CaseexprContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{CaseexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for CaseexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for CaseexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_caseexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for CaseexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for CaseexprContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for CaseexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for CaseexprContext<'input> {}

impl<'input> CaseexprContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::CaseexprContext(
				BaseParserRuleContext::copy_from(ctx,CaseexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MatchesexprContext<'input> = BaseParserRuleContext<'input,MatchesexprContextExt<'input>>;

pub trait MatchesexprContextAttrs<'input>: BSVParserContext<'input>{
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn patterncond_all(&self) ->  Vec<Rc<PatterncondContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn patterncond(&self, i: usize) -> Option<Rc<PatterncondContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> MatchesexprContextAttrs<'input> for MatchesexprContext<'input>{}

pub struct MatchesexprContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{MatchesexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for MatchesexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for MatchesexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_matchesexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for MatchesexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for MatchesexprContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for MatchesexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for MatchesexprContext<'input> {}

impl<'input> MatchesexprContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::MatchesexprContext(
				BaseParserRuleContext::copy_from(ctx,MatchesexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CondexprContext<'input> = BaseParserRuleContext<'input,CondexprContextExt<'input>>;

pub trait CondexprContextAttrs<'input>: BSVParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> CondexprContextAttrs<'input> for CondexprContext<'input>{}

pub struct CondexprContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub pred: Option<Rc<ExpressionContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{CondexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for CondexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for CondexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_condexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for CondexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for CondexprContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for CondexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for CondexprContext<'input> {}

impl<'input> CondexprContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::CondexprContext(
				BaseParserRuleContext::copy_from(ctx,CondexprContextExt{
        			pred:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		self.expression_rec(0)
	}

	fn expression_rec(&mut self, _p: isize)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 134, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 134;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1183);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__55 
				=> {
					{
					let mut tmp = CaseexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(1159);
					recog.base.match_token(T__55,&mut recog.err_handler)?;

					recog.base.set_state(1160);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1161);
					recog.expression_rec(0)?;

					recog.base.set_state(1162);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					recog.base.set_state(1175);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__54 
						=> {
							{
							{
							recog.base.set_state(1163);
							recog.base.match_token(T__54,&mut recog.err_handler)?;

							recog.base.set_state(1165); 
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							loop {
								{
								{
								/*InvokeRule caseexprpatitem*/
								recog.base.set_state(1164);
								recog.caseexprpatitem()?;

								}
								}
								recog.base.set_state(1167); 
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__20) | (1usize << T__27) | (1usize << T__50))) != 0) || ((((_la - 100)) & !0x3f) == 0 && ((1usize << (_la - 100)) & ((1usize << (T__99 - 100)) | (1usize << (UpperCaseIdentifier - 100)) | (1usize << (IntLiteral - 100)) | (1usize << (IntPattern - 100)) | (1usize << (RealLiteral - 100)) | (1usize << (StringLiteral - 100)))) != 0)) {break}
							}
							}
							}
						}

					 T__8 | T__12 | T__20 | T__27 | T__43 | T__53 | T__55 | T__56 | T__57 |
					 T__62 | T__63 | T__72 | T__73 | T__74 | T__75 | T__76 | T__79 | T__80 |
					 T__81 | T__82 | T__84 | T__85 | T__86 | T__87 | T__89 | T__93 | T__100 |
					 UpperCaseIdentifier | LowerCaseIdentifier | DollarIdentifier | EscapedOperator |
					 IntLiteral | RealLiteral | StringLiteral 
						=> {
							{
							recog.base.set_state(1172);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__43) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
								{
								{
								/*InvokeRule caseexpritem*/
								recog.base.set_state(1169);
								recog.caseexpritem()?;

								}
								}
								recog.base.set_state(1174);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
							}
							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(1178);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__57 {
						{
						/*InvokeRule caseexprdefaultitem*/
						recog.base.set_state(1177);
						recog.caseexprdefaultitem()?;

						}
					}

					recog.base.set_state(1180);
					recog.base.match_token(T__56,&mut recog.err_handler)?;

					}
				}

			 T__8 | T__12 | T__20 | T__27 | T__43 | T__53 | T__62 | T__63 | T__72 |
			 T__73 | T__74 | T__75 | T__76 | T__79 | T__80 | T__81 | T__82 | T__84 |
			 T__85 | T__86 | T__87 | T__89 | T__93 | T__100 | UpperCaseIdentifier |
			 LowerCaseIdentifier | DollarIdentifier | EscapedOperator | IntLiteral |
			 RealLiteral | StringLiteral 
				=> {
					{
					let mut tmp = OperatorexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule binopexpr*/
					recog.base.set_state(1182);
					recog.binopexpr_rec(0)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1202);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(147,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(1200);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(146,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = CondexprContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							if let ExpressionContextAll::CondexprContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut tmp){
								ctx.pred = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1185);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(1186);
							recog.base.match_token(T__53,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1187);
							recog.expression_rec(0)?;

							recog.base.set_state(1188);
							recog.base.match_token(T__3,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1189);
							recog.expression_rec(5)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = MatchesexprContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1191);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(1192);
							recog.base.match_token(T__54,&mut recog.err_handler)?;

							/*InvokeRule pattern*/
							recog.base.set_state(1193);
							recog.pattern()?;

							recog.base.set_state(1197);
							recog.err_handler.sync(&mut recog.base)?;
							_alt = recog.interpreter.adaptive_predict(145,&mut recog.base)?;
							while { _alt!=2 && _alt!=INVALID_ALT } {
								if _alt==1 {
									{
									{
									/*InvokeRule patterncond*/
									recog.base.set_state(1194);
									recog.patterncond()?;

									}
									} 
								}
								recog.base.set_state(1199);
								recog.err_handler.sync(&mut recog.base)?;
								_alt = recog.interpreter.adaptive_predict(145,&mut recog.base)?;
							}
							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(1204);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(147,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- caseexprpatitem ----------------
pub type CaseexprpatitemContextAll<'input> = CaseexprpatitemContext<'input>;


pub type CaseexprpatitemContext<'input> = BaseParserRuleContext<'input,CaseexprpatitemContextExt<'input>>;

#[derive(Clone)]
pub struct CaseexprpatitemContextExt<'input>{
	pub body: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for CaseexprpatitemContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for CaseexprpatitemContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_caseexprpatitem(self);
	}
}

impl<'input> CustomRuleContext<'input> for CaseexprpatitemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_caseexprpatitem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_caseexprpatitem }
}
antlr_rust::type_id!{CaseexprpatitemContextExt<'a>}

impl<'input> CaseexprpatitemContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CaseexprpatitemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CaseexprpatitemContextExt{
				body: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait CaseexprpatitemContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<CaseexprpatitemContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn patterncond_all(&self) ->  Vec<Rc<PatterncondContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn patterncond(&self, i: usize) -> Option<Rc<PatterncondContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CaseexprpatitemContextAttrs<'input> for CaseexprpatitemContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn caseexprpatitem(&mut self,)
	-> Result<Rc<CaseexprpatitemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CaseexprpatitemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_caseexprpatitem);
        let mut _localctx: Rc<CaseexprpatitemContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule pattern*/
			recog.base.set_state(1205);
			recog.pattern()?;

			recog.base.set_state(1209);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__58 {
				{
				{
				/*InvokeRule patterncond*/
				recog.base.set_state(1206);
				recog.patterncond()?;

				}
				}
				recog.base.set_state(1211);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			recog.base.set_state(1212);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1213);
			let tmp = recog.expression_rec(0)?;
			 cast_mut::<_,CaseexprpatitemContext >(&mut _localctx).body = Some(tmp.clone());
			  

			recog.base.set_state(1214);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- caseexpritem ----------------
pub type CaseexpritemContextAll<'input> = CaseexpritemContext<'input>;


pub type CaseexpritemContext<'input> = BaseParserRuleContext<'input,CaseexpritemContextExt<'input>>;

#[derive(Clone)]
pub struct CaseexpritemContextExt<'input>{
	pub match_: Option<Rc<ExpressionContextAll<'input>>>,
	pub altmatches: Option<Rc<ExpressionContextAll<'input>>>,
	pub body: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for CaseexpritemContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for CaseexpritemContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_caseexpritem(self);
	}
}

impl<'input> CustomRuleContext<'input> for CaseexpritemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_caseexpritem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_caseexpritem }
}
antlr_rust::type_id!{CaseexpritemContextExt<'a>}

impl<'input> CaseexpritemContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CaseexpritemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CaseexpritemContextExt{
				match_: None, altmatches: None, body: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait CaseexpritemContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<CaseexpritemContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CaseexpritemContextAttrs<'input> for CaseexpritemContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn caseexpritem(&mut self,)
	-> Result<Rc<CaseexpritemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CaseexpritemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_caseexpritem);
        let mut _localctx: Rc<CaseexpritemContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(1216);
			let tmp = recog.expression_rec(0)?;
			 cast_mut::<_,CaseexpritemContext >(&mut _localctx).match_ = Some(tmp.clone());
			  

			recog.base.set_state(1221);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(1217);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(1218);
				let tmp = recog.expression_rec(0)?;
				 cast_mut::<_,CaseexpritemContext >(&mut _localctx).altmatches = Some(tmp.clone());
				  

				}
				}
				recog.base.set_state(1223);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1224);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1225);
			let tmp = recog.expression_rec(0)?;
			 cast_mut::<_,CaseexpritemContext >(&mut _localctx).body = Some(tmp.clone());
			  

			recog.base.set_state(1226);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- caseexprdefaultitem ----------------
pub type CaseexprdefaultitemContextAll<'input> = CaseexprdefaultitemContext<'input>;


pub type CaseexprdefaultitemContext<'input> = BaseParserRuleContext<'input,CaseexprdefaultitemContextExt<'input>>;

#[derive(Clone)]
pub struct CaseexprdefaultitemContextExt<'input>{
	pub body: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for CaseexprdefaultitemContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for CaseexprdefaultitemContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_caseexprdefaultitem(self);
	}
}

impl<'input> CustomRuleContext<'input> for CaseexprdefaultitemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_caseexprdefaultitem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_caseexprdefaultitem }
}
antlr_rust::type_id!{CaseexprdefaultitemContextExt<'a>}

impl<'input> CaseexprdefaultitemContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CaseexprdefaultitemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CaseexprdefaultitemContextExt{
				body: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait CaseexprdefaultitemContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<CaseexprdefaultitemContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CaseexprdefaultitemContextAttrs<'input> for CaseexprdefaultitemContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn caseexprdefaultitem(&mut self,)
	-> Result<Rc<CaseexprdefaultitemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CaseexprdefaultitemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_caseexprdefaultitem);
        let mut _localctx: Rc<CaseexprdefaultitemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1228);
			recog.base.match_token(T__57,&mut recog.err_handler)?;

			recog.base.set_state(1229);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1230);
			let tmp = recog.expression_rec(0)?;
			 cast_mut::<_,CaseexprdefaultitemContext >(&mut _localctx).body = Some(tmp.clone());
			  

			recog.base.set_state(1231);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- patterncond ----------------
pub type PatterncondContextAll<'input> = PatterncondContext<'input>;


pub type PatterncondContext<'input> = BaseParserRuleContext<'input,PatterncondContextExt<'input>>;

#[derive(Clone)]
pub struct PatterncondContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for PatterncondContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for PatterncondContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_patterncond(self);
	}
}

impl<'input> CustomRuleContext<'input> for PatterncondContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patterncond }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patterncond }
}
antlr_rust::type_id!{PatterncondContextExt<'a>}

impl<'input> PatterncondContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatterncondContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatterncondContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatterncondContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<PatterncondContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PatterncondContextAttrs<'input> for PatterncondContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patterncond(&mut self,)
	-> Result<Rc<PatterncondContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatterncondContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_patterncond);
        let mut _localctx: Rc<PatterncondContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			recog.base.set_state(1233);
			recog.base.match_token(T__58,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1234);
			recog.expression_rec(0)?;

			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- binopexpr ----------------
pub type BinopexprContextAll<'input> = BinopexprContext<'input>;


pub type BinopexprContext<'input> = BaseParserRuleContext<'input,BinopexprContextExt<'input>>;

#[derive(Clone)]
pub struct BinopexprContextExt<'input>{
	pub left: Option<Rc<BinopexprContextAll<'input>>>,
	pub op: Option<TokenType<'input>>,
	pub right: Option<Rc<BinopexprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for BinopexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for BinopexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_binopexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for BinopexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binopexpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binopexpr }
}
antlr_rust::type_id!{BinopexprContextExt<'a>}

impl<'input> BinopexprContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BinopexprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BinopexprContextExt{
				op: None, 
				left: None, right: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait BinopexprContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<BinopexprContextExt<'input>>{

fn unopexpr(&self) -> Option<Rc<UnopexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn binopexpr_all(&self) ->  Vec<Rc<BinopexprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn binopexpr(&self, i: usize) -> Option<Rc<BinopexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BinopexprContextAttrs<'input> for BinopexprContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  binopexpr(&mut self,)
	-> Result<Rc<BinopexprContextAll<'input>>,ANTLRError> {
		self.binopexpr_rec(0)
	}

	fn binopexpr_rec(&mut self, _p: isize)
	-> Result<Rc<BinopexprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = BinopexprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 144, RULE_binopexpr, _p);
	    let mut _localctx: Rc<BinopexprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 144;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule unopexpr*/
			recog.base.set_state(1237);
			recog.unopexpr()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1271);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(151,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(1269);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(150,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = BinopexprContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,BinopexprContext>(&mut tmp)).left = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_binopexpr);
							_localctx = tmp;
							recog.base.set_state(1239);
							if !({recog.precpred(None, 11)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 11)".to_owned()), None))?;
							}
							recog.base.set_state(1240);
							 cast_mut::<_,BinopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(_la==T__59) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,BinopexprContext >(&mut _localctx).op = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule binopexpr*/
							recog.base.set_state(1241);
							let tmp = recog.binopexpr_rec(12)?;
							 cast_mut::<_,BinopexprContext >(&mut _localctx).right = Some(tmp.clone());
							  

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = BinopexprContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,BinopexprContext>(&mut tmp)).left = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_binopexpr);
							_localctx = tmp;
							recog.base.set_state(1242);
							if !({recog.precpred(None, 10)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 10)".to_owned()), None))?;
							}
							recog.base.set_state(1243);
							 cast_mut::<_,BinopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__7) | (1usize << T__59) | (1usize << T__60) | (1usize << T__61))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,BinopexprContext >(&mut _localctx).op = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule binopexpr*/
							recog.base.set_state(1244);
							let tmp = recog.binopexpr_rec(11)?;
							 cast_mut::<_,BinopexprContext >(&mut _localctx).right = Some(tmp.clone());
							  

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = BinopexprContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,BinopexprContext>(&mut tmp)).left = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_binopexpr);
							_localctx = tmp;
							recog.base.set_state(1245);
							if !({recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							recog.base.set_state(1246);
							 cast_mut::<_,BinopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(_la==T__62 || _la==T__63) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,BinopexprContext >(&mut _localctx).op = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule binopexpr*/
							recog.base.set_state(1247);
							let tmp = recog.binopexpr_rec(10)?;
							 cast_mut::<_,BinopexprContext >(&mut _localctx).right = Some(tmp.clone());
							  

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = BinopexprContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,BinopexprContext>(&mut tmp)).left = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_binopexpr);
							_localctx = tmp;
							recog.base.set_state(1248);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(1249);
							 cast_mut::<_,BinopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(_la==T__64 || _la==T__65) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,BinopexprContext >(&mut _localctx).op = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule binopexpr*/
							recog.base.set_state(1250);
							let tmp = recog.binopexpr_rec(9)?;
							 cast_mut::<_,BinopexprContext >(&mut _localctx).right = Some(tmp.clone());
							  

							}
						}
					,
						5 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = BinopexprContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,BinopexprContext>(&mut tmp)).left = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_binopexpr);
							_localctx = tmp;
							recog.base.set_state(1251);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(1252);
							 cast_mut::<_,BinopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (T__66 - 67)) | (1usize << (T__67 - 67)) | (1usize << (T__68 - 67)) | (1usize << (T__69 - 67)))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,BinopexprContext >(&mut _localctx).op = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule binopexpr*/
							recog.base.set_state(1253);
							let tmp = recog.binopexpr_rec(8)?;
							 cast_mut::<_,BinopexprContext >(&mut _localctx).right = Some(tmp.clone());
							  

							}
						}
					,
						6 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = BinopexprContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,BinopexprContext>(&mut tmp)).left = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_binopexpr);
							_localctx = tmp;
							recog.base.set_state(1254);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(1255);
							 cast_mut::<_,BinopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(_la==T__70 || _la==T__71) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,BinopexprContext >(&mut _localctx).op = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule binopexpr*/
							recog.base.set_state(1256);
							let tmp = recog.binopexpr_rec(7)?;
							 cast_mut::<_,BinopexprContext >(&mut _localctx).right = Some(tmp.clone());
							  

							}
						}
					,
						7 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = BinopexprContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,BinopexprContext>(&mut tmp)).left = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_binopexpr);
							_localctx = tmp;
							recog.base.set_state(1257);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(1258);
							 cast_mut::<_,BinopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(((((_la - 73)) & !0x3f) == 0 && ((1usize << (_la - 73)) & ((1usize << (T__72 - 73)) | (1usize << (T__73 - 73)) | (1usize << (T__74 - 73)) | (1usize << (T__75 - 73)))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,BinopexprContext >(&mut _localctx).op = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule binopexpr*/
							recog.base.set_state(1259);
							let tmp = recog.binopexpr_rec(6)?;
							 cast_mut::<_,BinopexprContext >(&mut _localctx).right = Some(tmp.clone());
							  

							}
						}
					,
						8 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = BinopexprContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,BinopexprContext>(&mut tmp)).left = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_binopexpr);
							_localctx = tmp;
							recog.base.set_state(1260);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(1261);
							 cast_mut::<_,BinopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(_la==T__76) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,BinopexprContext >(&mut _localctx).op = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule binopexpr*/
							recog.base.set_state(1262);
							let tmp = recog.binopexpr_rec(5)?;
							 cast_mut::<_,BinopexprContext >(&mut _localctx).right = Some(tmp.clone());
							  

							}
						}
					,
						9 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = BinopexprContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,BinopexprContext>(&mut tmp)).left = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_binopexpr);
							_localctx = tmp;
							recog.base.set_state(1263);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(1264);
							 cast_mut::<_,BinopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(_la==T__58 || _la==T__77) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,BinopexprContext >(&mut _localctx).op = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule binopexpr*/
							recog.base.set_state(1265);
							let tmp = recog.binopexpr_rec(4)?;
							 cast_mut::<_,BinopexprContext >(&mut _localctx).right = Some(tmp.clone());
							  

							}
						}
					,
						10 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = BinopexprContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,BinopexprContext>(&mut tmp)).left = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_binopexpr);
							_localctx = tmp;
							recog.base.set_state(1266);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(1267);
							 cast_mut::<_,BinopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(_la==T__78) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,BinopexprContext >(&mut _localctx).op = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule binopexpr*/
							recog.base.set_state(1268);
							let tmp = recog.binopexpr_rec(3)?;
							 cast_mut::<_,BinopexprContext >(&mut _localctx).right = Some(tmp.clone());
							  

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(1273);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(151,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- unopexpr ----------------
pub type UnopexprContextAll<'input> = UnopexprContext<'input>;


pub type UnopexprContext<'input> = BaseParserRuleContext<'input,UnopexprContextExt<'input>>;

#[derive(Clone)]
pub struct UnopexprContextExt<'input>{
	pub op: Option<TokenType<'input>>,
	pub right: Option<Rc<ExprprimaryContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for UnopexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for UnopexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unopexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnopexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unopexpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unopexpr }
}
antlr_rust::type_id!{UnopexprContextExt<'a>}

impl<'input> UnopexprContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnopexprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnopexprContextExt{
				op: None, 
				right: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait UnopexprContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<UnopexprContextExt<'input>>{

fn exprprimary(&self) -> Option<Rc<ExprprimaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> UnopexprContextAttrs<'input> for UnopexprContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unopexpr(&mut self,)
	-> Result<Rc<UnopexprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnopexprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_unopexpr);
        let mut _localctx: Rc<UnopexprContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1279);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__72 | T__73 | T__74 | T__75 | T__76 | T__79 | T__80 | T__81 | T__82 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1274);
					 cast_mut::<_,UnopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
					 
					_la = recog.base.input.la(1);
					if { !(((((_la - 73)) & !0x3f) == 0 && ((1usize << (_la - 73)) & ((1usize << (T__72 - 73)) | (1usize << (T__73 - 73)) | (1usize << (T__74 - 73)) | (1usize << (T__75 - 73)) | (1usize << (T__76 - 73)) | (1usize << (T__79 - 73)) | (1usize << (T__80 - 73)) | (1usize << (T__81 - 73)) | (1usize << (T__82 - 73)))) != 0)) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						 cast_mut::<_,UnopexprContext >(&mut _localctx).op = Some(tmp.clone());
						  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule exprprimary*/
					recog.base.set_state(1275);
					recog.exprprimary_rec(0)?;

					}
				}

			 T__62 | T__63 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1276);
					 cast_mut::<_,UnopexprContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
					 
					_la = recog.base.input.la(1);
					if { !(_la==T__62 || _la==T__63) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						 cast_mut::<_,UnopexprContext >(&mut _localctx).op = Some(tmp.clone());
						  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule exprprimary*/
					recog.base.set_state(1277);
					let tmp = recog.exprprimary_rec(0)?;
					 cast_mut::<_,UnopexprContext >(&mut _localctx).right = Some(tmp.clone());
					  

					}
				}

			 T__8 | T__12 | T__20 | T__27 | T__43 | T__53 | T__84 | T__85 | T__86 |
			 T__87 | T__89 | T__93 | T__100 | UpperCaseIdentifier | LowerCaseIdentifier |
			 DollarIdentifier | EscapedOperator | IntLiteral | RealLiteral | StringLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule exprprimary*/
					recog.base.set_state(1278);
					recog.exprprimary_rec(0)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- exprprimary ----------------
#[derive(Debug)]
pub enum ExprprimaryContextAll<'input>{
	BitconcatContext(BitconcatContext<'input>),
	WhenexprContext(WhenexprContext<'input>),
	VarexprContext(VarexprContext<'input>),
	InterfaceexprContext(InterfaceexprContext<'input>),
	BlockexprContext(BlockexprContext<'input>),
	StringliteralContext(StringliteralContext<'input>),
	SyscallexprContext(SyscallexprContext<'input>),
	CallexprContext(CallexprContext<'input>),
	IntliteralContext(IntliteralContext<'input>),
	RealliteralContext(RealliteralContext<'input>),
	ValueofexprContext(ValueofexprContext<'input>),
	CastexprContext(CastexprContext<'input>),
	TypeassertionexprContext(TypeassertionexprContext<'input>),
	ResetbyexprContext(ResetbyexprContext<'input>),
	TaggedunionexprContext(TaggedunionexprContext<'input>),
	ArraysubContext(ArraysubContext<'input>),
	UndefinedexprContext(UndefinedexprContext<'input>),
	ClockedbyexprContext(ClockedbyexprContext<'input>),
	ActionvalueblockexprContext(ActionvalueblockexprContext<'input>),
	FieldexprContext(FieldexprContext<'input>),
	ParenexprContext(ParenexprContext<'input>),
Error(ExprprimaryContext<'input>)
}
antlr_rust::type_id!{ExprprimaryContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExprprimaryContextAll<'input>{}

impl<'input> BSVParserContext<'input> for ExprprimaryContextAll<'input>{}

impl<'input> Deref for ExprprimaryContextAll<'input>{
	type Target = dyn ExprprimaryContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExprprimaryContextAll::*;
		match self{
			BitconcatContext(inner) => inner,
			WhenexprContext(inner) => inner,
			VarexprContext(inner) => inner,
			InterfaceexprContext(inner) => inner,
			BlockexprContext(inner) => inner,
			StringliteralContext(inner) => inner,
			SyscallexprContext(inner) => inner,
			CallexprContext(inner) => inner,
			IntliteralContext(inner) => inner,
			RealliteralContext(inner) => inner,
			ValueofexprContext(inner) => inner,
			CastexprContext(inner) => inner,
			TypeassertionexprContext(inner) => inner,
			ResetbyexprContext(inner) => inner,
			TaggedunionexprContext(inner) => inner,
			ArraysubContext(inner) => inner,
			UndefinedexprContext(inner) => inner,
			ClockedbyexprContext(inner) => inner,
			ActionvalueblockexprContext(inner) => inner,
			FieldexprContext(inner) => inner,
			ParenexprContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ExprprimaryContextAll<'input>{
    fn enter(&self, listener: &mut (dyn BSVListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn BSVListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExprprimaryContext<'input> = BaseParserRuleContext<'input,ExprprimaryContextExt<'input>>;

#[derive(Clone)]
pub struct ExprprimaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ExprprimaryContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ExprprimaryContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExprprimaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}
antlr_rust::type_id!{ExprprimaryContextExt<'a>}

impl<'input> ExprprimaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprprimaryContextAll<'input>> {
		Rc::new(
		ExprprimaryContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprprimaryContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExprprimaryContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ExprprimaryContextExt<'input>>{


}

impl<'input> ExprprimaryContextAttrs<'input> for ExprprimaryContext<'input>{}

pub type BitconcatContext<'input> = BaseParserRuleContext<'input,BitconcatContextExt<'input>>;

pub trait BitconcatContextAttrs<'input>: BSVParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> BitconcatContextAttrs<'input> for BitconcatContext<'input>{}

pub struct BitconcatContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{BitconcatContextExt<'a>}

impl<'input> BSVParserContext<'input> for BitconcatContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for BitconcatContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_bitconcat(self);
	}
}

impl<'input> CustomRuleContext<'input> for BitconcatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for BitconcatContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for BitconcatContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for BitconcatContext<'input> {}

impl<'input> BitconcatContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::BitconcatContext(
				BaseParserRuleContext::copy_from(ctx,BitconcatContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WhenexprContext<'input> = BaseParserRuleContext<'input,WhenexprContextExt<'input>>;

pub trait WhenexprContextAttrs<'input>: BSVParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> WhenexprContextAttrs<'input> for WhenexprContext<'input>{}

pub struct WhenexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{WhenexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for WhenexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for WhenexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_whenexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for WhenexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for WhenexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for WhenexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for WhenexprContext<'input> {}

impl<'input> WhenexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::WhenexprContext(
				BaseParserRuleContext::copy_from(ctx,WhenexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type VarexprContext<'input> = BaseParserRuleContext<'input,VarexprContextExt<'input>>;

pub trait VarexprContextAttrs<'input>: BSVParserContext<'input>{
	fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn upperCaseIdentifier_all(&self) ->  Vec<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn upperCaseIdentifier(&self, i: usize) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> VarexprContextAttrs<'input> for VarexprContext<'input>{}

pub struct VarexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	pub pkg: Option<Rc<UpperCaseIdentifierContextAll<'input>>>,
	pub var: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{VarexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for VarexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for VarexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_varexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for VarexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for VarexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for VarexprContext<'input> {}

impl<'input> VarexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::VarexprContext(
				BaseParserRuleContext::copy_from(ctx,VarexprContextExt{
        			pkg:None, var:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type InterfaceexprContext<'input> = BaseParserRuleContext<'input,InterfaceexprContextExt<'input>>;

pub trait InterfaceexprContextAttrs<'input>: BSVParserContext<'input>{
	fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn interfacestmt_all(&self) ->  Vec<Rc<InterfacestmtContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn interfacestmt(&self, i: usize) -> Option<Rc<InterfacestmtContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn typeide(&self) -> Option<Rc<TypeideContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> InterfaceexprContextAttrs<'input> for InterfaceexprContext<'input>{}

pub struct InterfaceexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{InterfaceexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for InterfaceexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for InterfaceexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_interfaceexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for InterfaceexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for InterfaceexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for InterfaceexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for InterfaceexprContext<'input> {}

impl<'input> InterfaceexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::InterfaceexprContext(
				BaseParserRuleContext::copy_from(ctx,InterfaceexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BlockexprContext<'input> = BaseParserRuleContext<'input,BlockexprContextExt<'input>>;

pub trait BlockexprContextAttrs<'input>: BSVParserContext<'input>{
	fn beginendblock(&self) -> Option<Rc<BeginendblockContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> BlockexprContextAttrs<'input> for BlockexprContext<'input>{}

pub struct BlockexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{BlockexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for BlockexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for BlockexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_blockexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for BlockexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for BlockexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for BlockexprContext<'input> {}

impl<'input> BlockexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::BlockexprContext(
				BaseParserRuleContext::copy_from(ctx,BlockexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StringliteralContext<'input> = BaseParserRuleContext<'input,StringliteralContextExt<'input>>;

pub trait StringliteralContextAttrs<'input>: BSVParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token StringLiteral
	/// Returns `None` if there is no child corresponding to token StringLiteral
	fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
		self.get_token(StringLiteral, 0)
	}
}

impl<'input> StringliteralContextAttrs<'input> for StringliteralContext<'input>{}

pub struct StringliteralContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{StringliteralContextExt<'a>}

impl<'input> BSVParserContext<'input> for StringliteralContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for StringliteralContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_stringliteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringliteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for StringliteralContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for StringliteralContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for StringliteralContext<'input> {}

impl<'input> StringliteralContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::StringliteralContext(
				BaseParserRuleContext::copy_from(ctx,StringliteralContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SyscallexprContext<'input> = BaseParserRuleContext<'input,SyscallexprContextExt<'input>>;

pub trait SyscallexprContextAttrs<'input>: BSVParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token DollarIdentifier
	/// Returns `None` if there is no child corresponding to token DollarIdentifier
	fn DollarIdentifier(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
		self.get_token(DollarIdentifier, 0)
	}
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> SyscallexprContextAttrs<'input> for SyscallexprContext<'input>{}

pub struct SyscallexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	pub fcn: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{SyscallexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for SyscallexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for SyscallexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_syscallexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for SyscallexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for SyscallexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for SyscallexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for SyscallexprContext<'input> {}

impl<'input> SyscallexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::SyscallexprContext(
				BaseParserRuleContext::copy_from(ctx,SyscallexprContextExt{
					fcn:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CallexprContext<'input> = BaseParserRuleContext<'input,CallexprContextExt<'input>>;

pub trait CallexprContextAttrs<'input>: BSVParserContext<'input>{
	fn exprprimary(&self) -> Option<Rc<ExprprimaryContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> CallexprContextAttrs<'input> for CallexprContext<'input>{}

pub struct CallexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	pub fcn: Option<Rc<ExprprimaryContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{CallexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for CallexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for CallexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_callexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for CallexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for CallexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for CallexprContext<'input> {}

impl<'input> CallexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::CallexprContext(
				BaseParserRuleContext::copy_from(ctx,CallexprContextExt{
        			fcn:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntliteralContext<'input> = BaseParserRuleContext<'input,IntliteralContextExt<'input>>;

pub trait IntliteralContextAttrs<'input>: BSVParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IntLiteral
	/// Returns `None` if there is no child corresponding to token IntLiteral
	fn IntLiteral(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
		self.get_token(IntLiteral, 0)
	}
}

impl<'input> IntliteralContextAttrs<'input> for IntliteralContext<'input>{}

pub struct IntliteralContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{IntliteralContextExt<'a>}

impl<'input> BSVParserContext<'input> for IntliteralContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for IntliteralContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_intliteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntliteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for IntliteralContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for IntliteralContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for IntliteralContext<'input> {}

impl<'input> IntliteralContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::IntliteralContext(
				BaseParserRuleContext::copy_from(ctx,IntliteralContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RealliteralContext<'input> = BaseParserRuleContext<'input,RealliteralContextExt<'input>>;

pub trait RealliteralContextAttrs<'input>: BSVParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token RealLiteral
	/// Returns `None` if there is no child corresponding to token RealLiteral
	fn RealLiteral(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
		self.get_token(RealLiteral, 0)
	}
}

impl<'input> RealliteralContextAttrs<'input> for RealliteralContext<'input>{}

pub struct RealliteralContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{RealliteralContextExt<'a>}

impl<'input> BSVParserContext<'input> for RealliteralContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for RealliteralContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_realliteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for RealliteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for RealliteralContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for RealliteralContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for RealliteralContext<'input> {}

impl<'input> RealliteralContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::RealliteralContext(
				BaseParserRuleContext::copy_from(ctx,RealliteralContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ValueofexprContext<'input> = BaseParserRuleContext<'input,ValueofexprContextExt<'input>>;

pub trait ValueofexprContextAttrs<'input>: BSVParserContext<'input>{
	fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ValueofexprContextAttrs<'input> for ValueofexprContext<'input>{}

pub struct ValueofexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ValueofexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for ValueofexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ValueofexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_valueofexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueofexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for ValueofexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for ValueofexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for ValueofexprContext<'input> {}

impl<'input> ValueofexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::ValueofexprContext(
				BaseParserRuleContext::copy_from(ctx,ValueofexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CastexprContext<'input> = BaseParserRuleContext<'input,CastexprContextExt<'input>>;

pub trait CastexprContextAttrs<'input>: BSVParserContext<'input>{
	fn exprprimary(&self) -> Option<Rc<ExprprimaryContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> CastexprContextAttrs<'input> for CastexprContext<'input>{}

pub struct CastexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{CastexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for CastexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for CastexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_castexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for CastexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for CastexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for CastexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for CastexprContext<'input> {}

impl<'input> CastexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::CastexprContext(
				BaseParserRuleContext::copy_from(ctx,CastexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TypeassertionexprContext<'input> = BaseParserRuleContext<'input,TypeassertionexprContextExt<'input>>;

pub trait TypeassertionexprContextAttrs<'input>: BSVParserContext<'input>{
	fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> TypeassertionexprContextAttrs<'input> for TypeassertionexprContext<'input>{}

pub struct TypeassertionexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{TypeassertionexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for TypeassertionexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TypeassertionexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typeassertionexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeassertionexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for TypeassertionexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for TypeassertionexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for TypeassertionexprContext<'input> {}

impl<'input> TypeassertionexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::TypeassertionexprContext(
				BaseParserRuleContext::copy_from(ctx,TypeassertionexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ResetbyexprContext<'input> = BaseParserRuleContext<'input,ResetbyexprContextExt<'input>>;

pub trait ResetbyexprContextAttrs<'input>: BSVParserContext<'input>{
	fn exprprimary(&self) -> Option<Rc<ExprprimaryContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ResetbyexprContextAttrs<'input> for ResetbyexprContext<'input>{}

pub struct ResetbyexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ResetbyexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for ResetbyexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ResetbyexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_resetbyexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ResetbyexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for ResetbyexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for ResetbyexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for ResetbyexprContext<'input> {}

impl<'input> ResetbyexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::ResetbyexprContext(
				BaseParserRuleContext::copy_from(ctx,ResetbyexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaggedunionexprContext<'input> = BaseParserRuleContext<'input,TaggedunionexprContextExt<'input>>;

pub trait TaggedunionexprContextAttrs<'input>: BSVParserContext<'input>{
	fn upperCaseIdentifier_all(&self) ->  Vec<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn upperCaseIdentifier(&self, i: usize) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn exprprimary(&self) -> Option<Rc<ExprprimaryContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn memberbinds(&self) -> Option<Rc<MemberbindsContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaggedunionexprContextAttrs<'input> for TaggedunionexprContext<'input>{}

pub struct TaggedunionexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	pub tag: Option<Rc<UpperCaseIdentifierContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{TaggedunionexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for TaggedunionexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TaggedunionexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_taggedunionexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaggedunionexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for TaggedunionexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for TaggedunionexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for TaggedunionexprContext<'input> {}

impl<'input> TaggedunionexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::TaggedunionexprContext(
				BaseParserRuleContext::copy_from(ctx,TaggedunionexprContextExt{
        			tag:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ArraysubContext<'input> = BaseParserRuleContext<'input,ArraysubContextExt<'input>>;

pub trait ArraysubContextAttrs<'input>: BSVParserContext<'input>{
	fn exprprimary(&self) -> Option<Rc<ExprprimaryContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token IntLiteral
	/// Returns `None` if there is no child corresponding to token IntLiteral
	fn IntLiteral(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
		self.get_token(IntLiteral, 0)
	}
}

impl<'input> ArraysubContextAttrs<'input> for ArraysubContext<'input>{}

pub struct ArraysubContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	pub array: Option<Rc<ExprprimaryContextAll<'input>>>,
	pub msb: Option<Rc<ExpressionContextAll<'input>>>,
	pub lsb: Option<Rc<ExpressionContextAll<'input>>>,
	pub widthup: Option<TokenType<'input>>,
	pub widthdown: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ArraysubContextExt<'a>}

impl<'input> BSVParserContext<'input> for ArraysubContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ArraysubContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_arraysub(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArraysubContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for ArraysubContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for ArraysubContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for ArraysubContext<'input> {}

impl<'input> ArraysubContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::ArraysubContext(
				BaseParserRuleContext::copy_from(ctx,ArraysubContextExt{
					widthup:None, widthdown:None, 
        			array:None, msb:None, lsb:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UndefinedexprContext<'input> = BaseParserRuleContext<'input,UndefinedexprContextExt<'input>>;

pub trait UndefinedexprContextAttrs<'input>: BSVParserContext<'input>{
}

impl<'input> UndefinedexprContextAttrs<'input> for UndefinedexprContext<'input>{}

pub struct UndefinedexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{UndefinedexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for UndefinedexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for UndefinedexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_undefinedexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for UndefinedexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for UndefinedexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for UndefinedexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for UndefinedexprContext<'input> {}

impl<'input> UndefinedexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::UndefinedexprContext(
				BaseParserRuleContext::copy_from(ctx,UndefinedexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ClockedbyexprContext<'input> = BaseParserRuleContext<'input,ClockedbyexprContextExt<'input>>;

pub trait ClockedbyexprContextAttrs<'input>: BSVParserContext<'input>{
	fn exprprimary(&self) -> Option<Rc<ExprprimaryContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ClockedbyexprContextAttrs<'input> for ClockedbyexprContext<'input>{}

pub struct ClockedbyexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ClockedbyexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for ClockedbyexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ClockedbyexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_clockedbyexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ClockedbyexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for ClockedbyexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for ClockedbyexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for ClockedbyexprContext<'input> {}

impl<'input> ClockedbyexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::ClockedbyexprContext(
				BaseParserRuleContext::copy_from(ctx,ClockedbyexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ActionvalueblockexprContext<'input> = BaseParserRuleContext<'input,ActionvalueblockexprContextExt<'input>>;

pub trait ActionvalueblockexprContextAttrs<'input>: BSVParserContext<'input>{
	fn actionvalueblock(&self) -> Option<Rc<ActionvalueblockContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ActionvalueblockexprContextAttrs<'input> for ActionvalueblockexprContext<'input>{}

pub struct ActionvalueblockexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ActionvalueblockexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for ActionvalueblockexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ActionvalueblockexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_actionvalueblockexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionvalueblockexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for ActionvalueblockexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for ActionvalueblockexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for ActionvalueblockexprContext<'input> {}

impl<'input> ActionvalueblockexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::ActionvalueblockexprContext(
				BaseParserRuleContext::copy_from(ctx,ActionvalueblockexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FieldexprContext<'input> = BaseParserRuleContext<'input,FieldexprContextExt<'input>>;

pub trait FieldexprContextAttrs<'input>: BSVParserContext<'input>{
	fn exprprimary(&self) -> Option<Rc<ExprprimaryContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn anyidentifier(&self) -> Option<Rc<AnyidentifierContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> FieldexprContextAttrs<'input> for FieldexprContext<'input>{}

pub struct FieldexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	pub field: Option<Rc<AnyidentifierContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{FieldexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for FieldexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for FieldexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fieldexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for FieldexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for FieldexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for FieldexprContext<'input> {}

impl<'input> FieldexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::FieldexprContext(
				BaseParserRuleContext::copy_from(ctx,FieldexprContextExt{
        			field:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParenexprContext<'input> = BaseParserRuleContext<'input,ParenexprContextExt<'input>>;

pub trait ParenexprContextAttrs<'input>: BSVParserContext<'input>{
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ParenexprContextAttrs<'input> for ParenexprContext<'input>{}

pub struct ParenexprContextExt<'input>{
	base:ExprprimaryContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ParenexprContextExt<'a>}

impl<'input> BSVParserContext<'input> for ParenexprContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ParenexprContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parenexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParenexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprprimary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprprimary }
}

impl<'input> Borrow<ExprprimaryContextExt<'input>> for ParenexprContext<'input>{
	fn borrow(&self) -> &ExprprimaryContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprprimaryContextExt<'input>> for ParenexprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprprimaryContextExt<'input> { &mut self.base }
}

impl<'input> ExprprimaryContextAttrs<'input> for ParenexprContext<'input> {}

impl<'input> ParenexprContextExt<'input>{
	fn new(ctx: &dyn ExprprimaryContextAttrs<'input>) -> Rc<ExprprimaryContextAll<'input>>  {
		Rc::new(
			ExprprimaryContextAll::ParenexprContext(
				BaseParserRuleContext::copy_from(ctx,ParenexprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  exprprimary(&mut self,)
	-> Result<Rc<ExprprimaryContextAll<'input>>,ANTLRError> {
		self.exprprimary_rec(0)
	}

	fn exprprimary_rec(&mut self, _p: isize)
	-> Result<Rc<ExprprimaryContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExprprimaryContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 148, RULE_exprprimary, _p);
	    let mut _localctx: Rc<ExprprimaryContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 148;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1414);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(169,&mut recog.base)? {
				1 =>{
					{
					let mut tmp = ParenexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(1282);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1283);
					recog.expression_rec(0)?;

					recog.base.set_state(1284);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					let mut tmp = CastexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1291);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(153,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule bsvtype*/
							recog.base.set_state(1286);
							recog.bsvtype()?;

							}
						}
					,
						2 =>{
							{
							{
							recog.base.set_state(1287);
							recog.base.match_token(T__8,&mut recog.err_handler)?;

							/*InvokeRule bsvtype*/
							recog.base.set_state(1288);
							recog.bsvtype()?;

							recog.base.set_state(1289);
							recog.base.match_token(T__10,&mut recog.err_handler)?;

							}
							}
						}

						_ => {}
					}
					recog.base.set_state(1293);
					recog.base.match_token(T__83,&mut recog.err_handler)?;

					/*InvokeRule exprprimary*/
					recog.base.set_state(1294);
					recog.exprprimary_rec(19)?;

					}
				}
			,
				3 =>{
					{
					let mut tmp = VarexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1301);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==UpperCaseIdentifier {
						{
						{
						/*InvokeRule upperCaseIdentifier*/
						recog.base.set_state(1296);
						let tmp = recog.upperCaseIdentifier()?;
						if let ExprprimaryContextAll::VarexprContext(ctx) = cast_mut::<_,ExprprimaryContextAll >(&mut _localctx){
						ctx.pkg = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						recog.base.set_state(1297);
						recog.base.match_token(T__6,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(1303);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(1304);
					let tmp = recog.lowerCaseIdentifier()?;
					if let ExprprimaryContextAll::VarexprContext(ctx) = cast_mut::<_,ExprprimaryContextAll >(&mut _localctx){
					ctx.var = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				4 =>{
					{
					let mut tmp = IntliteralContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1305);
					recog.base.match_token(IntLiteral,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					{
					let mut tmp = RealliteralContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1306);
					recog.base.match_token(RealLiteral,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					{
					let mut tmp = StringliteralContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1307);
					recog.base.match_token(StringLiteral,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					{
					let mut tmp = UndefinedexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1308);
					recog.base.match_token(T__53,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					{
					let mut tmp = ValueofexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1309);
					_la = recog.base.input.la(1);
					if { !(_la==T__84 || _la==T__85) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(1310);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					/*InvokeRule bsvtype*/
					recog.base.set_state(1311);
					recog.bsvtype()?;

					recog.base.set_state(1312);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					{
					let mut tmp = BitconcatContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1314);
					recog.base.match_token(T__20,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1315);
					recog.expression_rec(0)?;

					recog.base.set_state(1320);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__5 {
						{
						{
						recog.base.set_state(1316);
						recog.base.match_token(T__5,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(1317);
						recog.expression_rec(0)?;

						}
						}
						recog.base.set_state(1322);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1323);
					recog.base.match_token(T__21,&mut recog.err_handler)?;

					}
				}
			,
				10 =>{
					{
					let mut tmp = WhenexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1325);
					recog.base.match_token(T__43,&mut recog.err_handler)?;

					recog.base.set_state(1326);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					recog.base.set_state(1335);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__43) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(1327);
						recog.expression_rec(0)?;

						recog.base.set_state(1332);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==T__5 {
							{
							{
							recog.base.set_state(1328);
							recog.base.match_token(T__5,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1329);
							recog.expression_rec(0)?;

							}
							}
							recog.base.set_state(1334);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(1337);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}
			,
				11 =>{
					{
					let mut tmp = SyscallexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1338);
					let tmp = recog.base.match_token(DollarIdentifier,&mut recog.err_handler)?;
					if let ExprprimaryContextAll::SyscallexprContext(ctx) = cast_mut::<_,ExprprimaryContextAll >(&mut _localctx){
					ctx.fcn = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(1351);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(160,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(1339);
							recog.base.match_token(T__8,&mut recog.err_handler)?;

							recog.base.set_state(1348);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__43) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
								{
								/*InvokeRule expression*/
								recog.base.set_state(1340);
								recog.expression_rec(0)?;

								recog.base.set_state(1345);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								while _la==T__5 {
									{
									{
									recog.base.set_state(1341);
									recog.base.match_token(T__5,&mut recog.err_handler)?;

									/*InvokeRule expression*/
									recog.base.set_state(1342);
									recog.expression_rec(0)?;

									}
									}
									recog.base.set_state(1347);
									recog.err_handler.sync(&mut recog.base)?;
									_la = recog.base.input.la(1);
								}
								}
							}

							recog.base.set_state(1350);
							recog.base.match_token(T__10,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				12 =>{
					{
					let mut tmp = ClockedbyexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1353);
					recog.base.match_token(T__86,&mut recog.err_handler)?;

					/*InvokeRule exprprimary*/
					recog.base.set_state(1354);
					recog.exprprimary_rec(7)?;

					}
				}
			,
				13 =>{
					{
					let mut tmp = ResetbyexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1355);
					recog.base.match_token(T__87,&mut recog.err_handler)?;

					/*InvokeRule exprprimary*/
					recog.base.set_state(1356);
					recog.exprprimary_rec(6)?;

					}
				}
			,
				14 =>{
					{
					let mut tmp = TypeassertionexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule bsvtype*/
					recog.base.set_state(1357);
					recog.bsvtype()?;

					recog.base.set_state(1358);
					recog.base.match_token(T__88,&mut recog.err_handler)?;

					recog.base.set_state(1374);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__20 
						=> {
							{
							{
							recog.base.set_state(1359);
							recog.base.match_token(T__20,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1360);
							recog.expression_rec(0)?;

							recog.base.set_state(1365);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							while _la==T__5 {
								{
								{
								recog.base.set_state(1361);
								recog.base.match_token(T__5,&mut recog.err_handler)?;

								/*InvokeRule expression*/
								recog.base.set_state(1362);
								recog.expression_rec(0)?;

								}
								}
								recog.base.set_state(1367);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
							}
							recog.base.set_state(1368);
							recog.base.match_token(T__21,&mut recog.err_handler)?;

							}
							}
						}

					 T__8 
						=> {
							{
							{
							recog.base.set_state(1370);
							recog.base.match_token(T__8,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1371);
							recog.expression_rec(0)?;

							recog.base.set_state(1372);
							recog.base.match_token(T__10,&mut recog.err_handler)?;

							}
							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}
			,
				15 =>{
					{
					let mut tmp = TaggedunionexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1377);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__27 {
						{
						recog.base.set_state(1376);
						recog.base.match_token(T__27,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1384);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(164,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule upperCaseIdentifier*/
							recog.base.set_state(1379);
							recog.upperCaseIdentifier()?;

							recog.base.set_state(1380);
							recog.base.match_token(T__6,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(1386);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(164,&mut recog.base)?;
					}
					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(1387);
					let tmp = recog.upperCaseIdentifier()?;
					if let ExprprimaryContextAll::TaggedunionexprContext(ctx) = cast_mut::<_,ExprprimaryContextAll >(&mut _localctx){
					ctx.tag = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(1394);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(165,&mut recog.base)? {
						1 =>{
							{
							{
							recog.base.set_state(1388);
							recog.base.match_token(T__20,&mut recog.err_handler)?;

							/*InvokeRule memberbinds*/
							recog.base.set_state(1389);
							recog.memberbinds()?;

							recog.base.set_state(1390);
							recog.base.match_token(T__21,&mut recog.err_handler)?;

							}
							}
						}
					,
						2 =>{
							{
							/*InvokeRule exprprimary*/
							recog.base.set_state(1392);
							recog.exprprimary_rec(0)?;

							}
						}
					,
						3 =>{
							{
							}
						}

						_ => {}
					}
					}
				}
			,
				16 =>{
					{
					let mut tmp = InterfaceexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(1396);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					/*InvokeRule bsvtype*/
					recog.base.set_state(1397);
					recog.bsvtype()?;

					recog.base.set_state(1399);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__1 {
						{
						recog.base.set_state(1398);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(1404);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__14) | (1usize << T__20) | (1usize << T__27) | (1usize << T__29) | (1usize << T__43) | (1usize << T__53))) != 0) || ((((_la - 85)) & !0x3f) == 0 && ((1usize << (_la - 85)) & ((1usize << (T__84 - 85)) | (1usize << (T__85 - 85)) | (1usize << (T__86 - 85)) | (1usize << (T__87 - 85)) | (1usize << (T__89 - 85)) | (1usize << (T__93 - 85)) | (1usize << (T__100 - 85)) | (1usize << (UpperCaseIdentifier - 85)) | (1usize << (LowerCaseIdentifier - 85)) | (1usize << (DollarIdentifier - 85)) | (1usize << (EscapedOperator - 85)) | (1usize << (IntLiteral - 85)) | (1usize << (RealLiteral - 85)) | (1usize << (StringLiteral - 85)))) != 0) {
						{
						{
						/*InvokeRule interfacestmt*/
						recog.base.set_state(1401);
						recog.interfacestmt()?;

						}
						}
						recog.base.set_state(1406);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1407);
					recog.base.match_token(T__13,&mut recog.err_handler)?;

					recog.base.set_state(1410);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(168,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(1408);
							recog.base.match_token(T__3,&mut recog.err_handler)?;

							/*InvokeRule typeide*/
							recog.base.set_state(1409);
							recog.typeide()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				17 =>{
					{
					let mut tmp = BlockexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule beginendblock*/
					recog.base.set_state(1412);
					recog.beginendblock()?;

					}
				}
			,
				18 =>{
					{
					let mut tmp = ActionvalueblockexprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule actionvalueblock*/
					recog.base.set_state(1413);
					recog.actionvalueblock()?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1447);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(174,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(1445);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(173,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = FieldexprContextExt::new(&**ExprprimaryContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_exprprimary);
							_localctx = tmp;
							recog.base.set_state(1416);
							if !({recog.precpred(None, 20)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 20)".to_owned()), None))?;
							}
							recog.base.set_state(1417);
							recog.base.match_token(T__50,&mut recog.err_handler)?;

							/*InvokeRule anyidentifier*/
							recog.base.set_state(1418);
							let tmp = recog.anyidentifier()?;
							if let ExprprimaryContextAll::FieldexprContext(ctx) = cast_mut::<_,ExprprimaryContextAll >(&mut _localctx){
							ctx.field = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ArraysubContextExt::new(&**ExprprimaryContextExt::new(_parentctx.clone(), _parentState));
							if let ExprprimaryContextAll::ArraysubContext(ctx) = cast_mut::<_,ExprprimaryContextAll >(&mut tmp){
								ctx.array = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_exprprimary);
							_localctx = tmp;
							recog.base.set_state(1419);
							if !({recog.precpred(None, 11)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 11)".to_owned()), None))?;
							}
							recog.base.set_state(1420);
							recog.base.match_token(T__22,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1421);
							let tmp = recog.expression_rec(0)?;
							if let ExprprimaryContextAll::ArraysubContext(ctx) = cast_mut::<_,ExprprimaryContextAll >(&mut _localctx){
							ctx.msb = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							recog.base.set_state(1428);
							recog.err_handler.sync(&mut recog.base)?;
							match recog.base.input.la(1) {
							 T__3 
								=> {
							    	{
							    	{
							    	recog.base.set_state(1422);
							    	recog.base.match_token(T__3,&mut recog.err_handler)?;

							    	/*InvokeRule expression*/
							    	recog.base.set_state(1423);
							    	let tmp = recog.expression_rec(0)?;
							    	if let ExprprimaryContextAll::ArraysubContext(ctx) = cast_mut::<_,ExprprimaryContextAll >(&mut _localctx){
							    	ctx.lsb = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							    	}
							    	}
							    }

							 T__51 
								=> {
							    	{
							    	{
							    	recog.base.set_state(1424);
							    	recog.base.match_token(T__51,&mut recog.err_handler)?;

							    	recog.base.set_state(1425);
							    	let tmp = recog.base.match_token(IntLiteral,&mut recog.err_handler)?;
							    	if let ExprprimaryContextAll::ArraysubContext(ctx) = cast_mut::<_,ExprprimaryContextAll >(&mut _localctx){
							    	ctx.widthup = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							    	}
							    	}
							    }

							 T__52 
								=> {
							    	{
							    	{
							    	recog.base.set_state(1426);
							    	recog.base.match_token(T__52,&mut recog.err_handler)?;

							    	recog.base.set_state(1427);
							    	let tmp = recog.base.match_token(IntLiteral,&mut recog.err_handler)?;
							    	if let ExprprimaryContextAll::ArraysubContext(ctx) = cast_mut::<_,ExprprimaryContextAll >(&mut _localctx){
							    	ctx.widthdown = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							    	}
							    	}
							    }

							 T__23 
								=> {
							    }

								_ => {}
							}
							recog.base.set_state(1430);
							recog.base.match_token(T__23,&mut recog.err_handler)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = CallexprContextExt::new(&**ExprprimaryContextExt::new(_parentctx.clone(), _parentState));
							if let ExprprimaryContextAll::CallexprContext(ctx) = cast_mut::<_,ExprprimaryContextAll >(&mut tmp){
								ctx.fcn = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_exprprimary);
							_localctx = tmp;
							recog.base.set_state(1432);
							if !({recog.precpred(None, 10)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 10)".to_owned()), None))?;
							}
							recog.base.set_state(1433);
							recog.base.match_token(T__8,&mut recog.err_handler)?;

							recog.base.set_state(1442);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__43) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
								{
								/*InvokeRule expression*/
								recog.base.set_state(1434);
								recog.expression_rec(0)?;

								recog.base.set_state(1439);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								while _la==T__5 {
									{
									{
									recog.base.set_state(1435);
									recog.base.match_token(T__5,&mut recog.err_handler)?;

									/*InvokeRule expression*/
									recog.base.set_state(1436);
									recog.expression_rec(0)?;

									}
									}
									recog.base.set_state(1441);
									recog.err_handler.sync(&mut recog.base)?;
									_la = recog.base.input.la(1);
								}
								}
							}

							recog.base.set_state(1444);
							recog.base.match_token(T__10,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(1449);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(174,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- memberbinds ----------------
pub type MemberbindsContextAll<'input> = MemberbindsContext<'input>;


pub type MemberbindsContext<'input> = BaseParserRuleContext<'input,MemberbindsContextExt<'input>>;

#[derive(Clone)]
pub struct MemberbindsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for MemberbindsContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for MemberbindsContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_memberbinds(self);
	}
}

impl<'input> CustomRuleContext<'input> for MemberbindsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_memberbinds }
	//fn type_rule_index() -> usize where Self: Sized { RULE_memberbinds }
}
antlr_rust::type_id!{MemberbindsContextExt<'a>}

impl<'input> MemberbindsContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MemberbindsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MemberbindsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MemberbindsContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<MemberbindsContextExt<'input>>{

fn memberbind_all(&self) ->  Vec<Rc<MemberbindContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn memberbind(&self, i: usize) -> Option<Rc<MemberbindContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MemberbindsContextAttrs<'input> for MemberbindsContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn memberbinds(&mut self,)
	-> Result<Rc<MemberbindsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MemberbindsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_memberbinds);
        let mut _localctx: Rc<MemberbindsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule memberbind*/
			recog.base.set_state(1450);
			recog.memberbind()?;

			recog.base.set_state(1455);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(1451);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule memberbind*/
				recog.base.set_state(1452);
				recog.memberbind()?;

				}
				}
				recog.base.set_state(1457);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- memberbind ----------------
pub type MemberbindContextAll<'input> = MemberbindContext<'input>;


pub type MemberbindContext<'input> = BaseParserRuleContext<'input,MemberbindContextExt<'input>>;

#[derive(Clone)]
pub struct MemberbindContextExt<'input>{
	pub field: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for MemberbindContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for MemberbindContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_memberbind(self);
	}
}

impl<'input> CustomRuleContext<'input> for MemberbindContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_memberbind }
	//fn type_rule_index() -> usize where Self: Sized { RULE_memberbind }
}
antlr_rust::type_id!{MemberbindContextExt<'a>}

impl<'input> MemberbindContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MemberbindContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MemberbindContextExt{
				field: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait MemberbindContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<MemberbindContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MemberbindContextAttrs<'input> for MemberbindContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn memberbind(&mut self,)
	-> Result<Rc<MemberbindContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MemberbindContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_memberbind);
        let mut _localctx: Rc<MemberbindContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(1458);
			let tmp = recog.lowerCaseIdentifier()?;
			 cast_mut::<_,MemberbindContext >(&mut _localctx).field = Some(tmp.clone());
			  

			recog.base.set_state(1459);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1460);
			recog.expression_rec(0)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- interfacestmt ----------------
pub type InterfacestmtContextAll<'input> = InterfacestmtContext<'input>;


pub type InterfacestmtContext<'input> = BaseParserRuleContext<'input,InterfacestmtContextExt<'input>>;

#[derive(Clone)]
pub struct InterfacestmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for InterfacestmtContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for InterfacestmtContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_interfacestmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for InterfacestmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interfacestmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interfacestmt }
}
antlr_rust::type_id!{InterfacestmtContextExt<'a>}

impl<'input> InterfacestmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InterfacestmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InterfacestmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InterfacestmtContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<InterfacestmtContextExt<'input>>{

fn methoddef(&self) -> Option<Rc<MethoddefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn subinterfacedef(&self) -> Option<Rc<SubinterfacedefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn varbinding(&self) -> Option<Rc<VarbindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn varassign(&self) -> Option<Rc<VarassignContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InterfacestmtContextAttrs<'input> for InterfacestmtContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interfacestmt(&mut self,)
	-> Result<Rc<InterfacestmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InterfacestmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_interfacestmt);
        let mut _localctx: Rc<InterfacestmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1466);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(176,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule methoddef*/
					recog.base.set_state(1462);
					recog.methoddef()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule subinterfacedef*/
					recog.base.set_state(1463);
					recog.subinterfacedef()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule varbinding*/
					recog.base.set_state(1464);
					recog.varbinding()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule varassign*/
					recog.base.set_state(1465);
					recog.varassign()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- beginendblock ----------------
pub type BeginendblockContextAll<'input> = BeginendblockContext<'input>;


pub type BeginendblockContext<'input> = BaseParserRuleContext<'input,BeginendblockContextExt<'input>>;

#[derive(Clone)]
pub struct BeginendblockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for BeginendblockContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for BeginendblockContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_beginendblock(self);
	}
}

impl<'input> CustomRuleContext<'input> for BeginendblockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_beginendblock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_beginendblock }
}
antlr_rust::type_id!{BeginendblockContextExt<'a>}

impl<'input> BeginendblockContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BeginendblockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BeginendblockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BeginendblockContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<BeginendblockContextExt<'input>>{

fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn lowerCaseIdentifier_all(&self) ->  Vec<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn lowerCaseIdentifier(&self, i: usize) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn stmt_all(&self) ->  Vec<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BeginendblockContextAttrs<'input> for BeginendblockContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn beginendblock(&mut self,)
	-> Result<Rc<BeginendblockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BeginendblockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_beginendblock);
        let mut _localctx: Rc<BeginendblockContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1471);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(1468);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(1473);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1474);
			recog.base.match_token(T__89,&mut recog.err_handler)?;

			recog.base.set_state(1477);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 {
				{
				recog.base.set_state(1475);
				recog.base.match_token(T__3,&mut recog.err_handler)?;

				/*InvokeRule lowerCaseIdentifier*/
				recog.base.set_state(1476);
				recog.lowerCaseIdentifier()?;

				}
			}

			recog.base.set_state(1482);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__29) | (1usize << T__32) | (1usize << T__43) | (1usize << T__44) | (1usize << T__45) | (1usize << T__48) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__91 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__96 - 64)) | (1usize << (T__97 - 64)) | (1usize << (T__98 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
				{
				{
				/*InvokeRule stmt*/
				recog.base.set_state(1479);
				recog.stmt()?;

				}
				}
				recog.base.set_state(1484);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1485);
			recog.base.match_token(T__90,&mut recog.err_handler)?;

			recog.base.set_state(1488);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(180,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(1486);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(1487);
					recog.lowerCaseIdentifier()?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- actionblock ----------------
pub type ActionblockContextAll<'input> = ActionblockContext<'input>;


pub type ActionblockContext<'input> = BaseParserRuleContext<'input,ActionblockContextExt<'input>>;

#[derive(Clone)]
pub struct ActionblockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ActionblockContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ActionblockContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_actionblock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionblockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionblock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionblock }
}
antlr_rust::type_id!{ActionblockContextExt<'a>}

impl<'input> ActionblockContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionblockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionblockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionblockContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ActionblockContextExt<'input>>{

fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn stmt_all(&self) ->  Vec<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ActionblockContextAttrs<'input> for ActionblockContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionblock(&mut self,)
	-> Result<Rc<ActionblockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionblockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_actionblock);
        let mut _localctx: Rc<ActionblockContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1493);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(1490);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(1495);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1496);
			recog.base.match_token(T__91,&mut recog.err_handler)?;

			recog.base.set_state(1500);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__29) | (1usize << T__32) | (1usize << T__43) | (1usize << T__44) | (1usize << T__45) | (1usize << T__48) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__91 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__96 - 64)) | (1usize << (T__97 - 64)) | (1usize << (T__98 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
				{
				{
				/*InvokeRule stmt*/
				recog.base.set_state(1497);
				recog.stmt()?;

				}
				}
				recog.base.set_state(1502);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1503);
			recog.base.match_token(T__92,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- actionvalueblock ----------------
pub type ActionvalueblockContextAll<'input> = ActionvalueblockContext<'input>;


pub type ActionvalueblockContext<'input> = BaseParserRuleContext<'input,ActionvalueblockContextExt<'input>>;

#[derive(Clone)]
pub struct ActionvalueblockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ActionvalueblockContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ActionvalueblockContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_actionvalueblock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionvalueblockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionvalueblock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionvalueblock }
}
antlr_rust::type_id!{ActionvalueblockContextExt<'a>}

impl<'input> ActionvalueblockContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionvalueblockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionvalueblockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionvalueblockContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ActionvalueblockContextExt<'input>>{

fn attributeinstance_all(&self) ->  Vec<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeinstance(&self, i: usize) -> Option<Rc<AttributeinstanceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn stmt_all(&self) ->  Vec<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ActionvalueblockContextAttrs<'input> for ActionvalueblockContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionvalueblock(&mut self,)
	-> Result<Rc<ActionvalueblockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionvalueblockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_actionvalueblock);
        let mut _localctx: Rc<ActionvalueblockContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1508);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				/*InvokeRule attributeinstance*/
				recog.base.set_state(1505);
				recog.attributeinstance()?;

				}
				}
				recog.base.set_state(1510);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1511);
			recog.base.match_token(T__93,&mut recog.err_handler)?;

			recog.base.set_state(1515);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__29) | (1usize << T__32) | (1usize << T__43) | (1usize << T__44) | (1usize << T__45) | (1usize << T__48) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__91 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__96 - 64)) | (1usize << (T__97 - 64)) | (1usize << (T__98 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
				{
				{
				/*InvokeRule stmt*/
				recog.base.set_state(1512);
				recog.stmt()?;

				}
				}
				recog.base.set_state(1517);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1518);
			recog.base.match_token(T__94,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- regwrite ----------------
pub type RegwriteContextAll<'input> = RegwriteContext<'input>;


pub type RegwriteContext<'input> = BaseParserRuleContext<'input,RegwriteContextExt<'input>>;

#[derive(Clone)]
pub struct RegwriteContextExt<'input>{
	pub lhs: Option<Rc<LvalueContextAll<'input>>>,
	pub rhs: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for RegwriteContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for RegwriteContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_regwrite(self);
	}
}

impl<'input> CustomRuleContext<'input> for RegwriteContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_regwrite }
	//fn type_rule_index() -> usize where Self: Sized { RULE_regwrite }
}
antlr_rust::type_id!{RegwriteContextExt<'a>}

impl<'input> RegwriteContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RegwriteContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RegwriteContextExt{
				lhs: None, rhs: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait RegwriteContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<RegwriteContextExt<'input>>{

fn lvalue(&self) -> Option<Rc<LvalueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RegwriteContextAttrs<'input> for RegwriteContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn regwrite(&mut self,)
	-> Result<Rc<RegwriteContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RegwriteContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_regwrite);
        let mut _localctx: Rc<RegwriteContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule lvalue*/
			recog.base.set_state(1520);
			let tmp = recog.lvalue()?;
			 cast_mut::<_,RegwriteContext >(&mut _localctx).lhs = Some(tmp.clone());
			  

			recog.base.set_state(1521);
			recog.base.match_token(T__67,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1522);
			let tmp = recog.expression_rec(0)?;
			 cast_mut::<_,RegwriteContext >(&mut _localctx).rhs = Some(tmp.clone());
			  

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stmt ----------------
pub type StmtContextAll<'input> = StmtContext<'input>;


pub type StmtContext<'input> = BaseParserRuleContext<'input,StmtContextExt<'input>>;

#[derive(Clone)]
pub struct StmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for StmtContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for StmtContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_stmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}
antlr_rust::type_id!{StmtContextExt<'a>}

impl<'input> StmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StmtContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<StmtContextExt<'input>>{

fn varbinding(&self) -> Option<Rc<VarbindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn actionbinding(&self) -> Option<Rc<ActionbindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn patternbinding(&self) -> Option<Rc<PatternbindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn varassign(&self) -> Option<Rc<VarassignContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functiondef(&self) -> Option<Rc<FunctiondefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ruledef(&self) -> Option<Rc<RuledefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn regwrite(&self) -> Option<Rc<RegwriteContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn beginendblock(&self) -> Option<Rc<BeginendblockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ifstmt(&self) -> Option<Rc<IfstmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn casestmt(&self) -> Option<Rc<CasestmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn forstmt(&self) -> Option<Rc<ForstmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn whilestmt(&self) -> Option<Rc<WhilestmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnstmt(&self) -> Option<Rc<ReturnstmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn actionblock(&self) -> Option<Rc<ActionblockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn actionvalueblock(&self) -> Option<Rc<ActionvalueblockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StmtContextAttrs<'input> for StmtContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stmt(&mut self,)
	-> Result<Rc<StmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_stmt);
        let mut _localctx: Rc<StmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1544);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(185,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule varbinding*/
					recog.base.set_state(1524);
					recog.varbinding()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule actionbinding*/
					recog.base.set_state(1525);
					recog.actionbinding()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule patternbinding*/
					recog.base.set_state(1526);
					recog.patternbinding()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule varassign*/
					recog.base.set_state(1527);
					recog.varassign()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule functiondef*/
					recog.base.set_state(1528);
					recog.functiondef()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule ruledef*/
					recog.base.set_state(1529);
					recog.ruledef()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule regwrite*/
					recog.base.set_state(1530);
					recog.regwrite()?;

					recog.base.set_state(1531);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule beginendblock*/
					recog.base.set_state(1533);
					recog.beginendblock()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule ifstmt*/
					recog.base.set_state(1534);
					recog.ifstmt()?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule casestmt*/
					recog.base.set_state(1535);
					recog.casestmt()?;

					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					/*InvokeRule forstmt*/
					recog.base.set_state(1536);
					recog.forstmt()?;

					}
				}
			,
				12 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					/*InvokeRule whilestmt*/
					recog.base.set_state(1537);
					recog.whilestmt()?;

					}
				}
			,
				13 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 13);
					recog.base.enter_outer_alt(None, 13);
					{
					/*InvokeRule expression*/
					recog.base.set_state(1538);
					recog.expression_rec(0)?;

					recog.base.set_state(1539);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}
			,
				14 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 14);
					recog.base.enter_outer_alt(None, 14);
					{
					/*InvokeRule returnstmt*/
					recog.base.set_state(1541);
					recog.returnstmt()?;

					}
				}
			,
				15 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 15);
					recog.base.enter_outer_alt(None, 15);
					{
					/*InvokeRule actionblock*/
					recog.base.set_state(1542);
					recog.actionblock()?;

					}
				}
			,
				16 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 16);
					recog.base.enter_outer_alt(None, 16);
					{
					/*InvokeRule actionvalueblock*/
					recog.base.set_state(1543);
					recog.actionvalueblock()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ifstmt ----------------
pub type IfstmtContextAll<'input> = IfstmtContext<'input>;


pub type IfstmtContext<'input> = BaseParserRuleContext<'input,IfstmtContextExt<'input>>;

#[derive(Clone)]
pub struct IfstmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for IfstmtContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for IfstmtContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ifstmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for IfstmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ifstmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ifstmt }
}
antlr_rust::type_id!{IfstmtContextExt<'a>}

impl<'input> IfstmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IfstmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IfstmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IfstmtContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<IfstmtContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stmt_all(&self) ->  Vec<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> IfstmtContextAttrs<'input> for IfstmtContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ifstmt(&mut self,)
	-> Result<Rc<IfstmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IfstmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_ifstmt);
        let mut _localctx: Rc<IfstmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1546);
			recog.base.match_token(T__44,&mut recog.err_handler)?;

			recog.base.set_state(1547);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1548);
			recog.expression_rec(0)?;

			recog.base.set_state(1549);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			/*InvokeRule stmt*/
			recog.base.set_state(1550);
			recog.stmt()?;

			recog.base.set_state(1553);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(186,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(1551);
					recog.base.match_token(T__95,&mut recog.err_handler)?;

					/*InvokeRule stmt*/
					recog.base.set_state(1552);
					recog.stmt()?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- casestmt ----------------
pub type CasestmtContextAll<'input> = CasestmtContext<'input>;


pub type CasestmtContext<'input> = BaseParserRuleContext<'input,CasestmtContextExt<'input>>;

#[derive(Clone)]
pub struct CasestmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for CasestmtContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for CasestmtContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_casestmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for CasestmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casestmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casestmt }
}
antlr_rust::type_id!{CasestmtContextExt<'a>}

impl<'input> CasestmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CasestmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CasestmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CasestmtContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<CasestmtContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn casestmtdefaultitem(&self) -> Option<Rc<CasestmtdefaultitemContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn casestmtitem_all(&self) ->  Vec<Rc<CasestmtitemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn casestmtitem(&self, i: usize) -> Option<Rc<CasestmtitemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn casestmtpatitem_all(&self) ->  Vec<Rc<CasestmtpatitemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn casestmtpatitem(&self, i: usize) -> Option<Rc<CasestmtpatitemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CasestmtContextAttrs<'input> for CasestmtContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn casestmt(&mut self,)
	-> Result<Rc<CasestmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CasestmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_casestmt);
        let mut _localctx: Rc<CasestmtContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1555);
			recog.base.match_token(T__55,&mut recog.err_handler)?;

			recog.base.set_state(1556);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1557);
			recog.expression_rec(0)?;

			recog.base.set_state(1558);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			recog.base.set_state(1571);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__54 
				=> {
					{
					{
					recog.base.set_state(1559);
					recog.base.match_token(T__54,&mut recog.err_handler)?;

					recog.base.set_state(1561); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule casestmtpatitem*/
						recog.base.set_state(1560);
						recog.casestmtpatitem()?;

						}
						}
						recog.base.set_state(1563); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__20) | (1usize << T__27) | (1usize << T__50))) != 0) || ((((_la - 100)) & !0x3f) == 0 && ((1usize << (_la - 100)) & ((1usize << (T__99 - 100)) | (1usize << (UpperCaseIdentifier - 100)) | (1usize << (IntLiteral - 100)) | (1usize << (IntPattern - 100)) | (1usize << (RealLiteral - 100)) | (1usize << (StringLiteral - 100)))) != 0)) {break}
					}
					}
					}
				}

			 T__8 | T__12 | T__20 | T__27 | T__43 | T__53 | T__55 | T__56 | T__57 |
			 T__62 | T__63 | T__72 | T__73 | T__74 | T__75 | T__76 | T__79 | T__80 |
			 T__81 | T__82 | T__84 | T__85 | T__86 | T__87 | T__89 | T__93 | T__100 |
			 UpperCaseIdentifier | LowerCaseIdentifier | DollarIdentifier | EscapedOperator |
			 IntLiteral | RealLiteral | StringLiteral 
				=> {
					{
					recog.base.set_state(1568);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__8) | (1usize << T__12) | (1usize << T__20) | (1usize << T__27) | (1usize << T__43) | (1usize << T__53) | (1usize << T__55) | (1usize << T__62))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (T__63 - 64)) | (1usize << (T__72 - 64)) | (1usize << (T__73 - 64)) | (1usize << (T__74 - 64)) | (1usize << (T__75 - 64)) | (1usize << (T__76 - 64)) | (1usize << (T__79 - 64)) | (1usize << (T__80 - 64)) | (1usize << (T__81 - 64)) | (1usize << (T__82 - 64)) | (1usize << (T__84 - 64)) | (1usize << (T__85 - 64)) | (1usize << (T__86 - 64)) | (1usize << (T__87 - 64)) | (1usize << (T__89 - 64)) | (1usize << (T__93 - 64)) | (1usize << (T__100 - 64)) | (1usize << (UpperCaseIdentifier - 64)) | (1usize << (LowerCaseIdentifier - 64)) | (1usize << (DollarIdentifier - 64)) | (1usize << (EscapedOperator - 64)) | (1usize << (IntLiteral - 64)) | (1usize << (RealLiteral - 64)) | (1usize << (StringLiteral - 64)))) != 0) {
						{
						{
						/*InvokeRule casestmtitem*/
						recog.base.set_state(1565);
						recog.casestmtitem()?;

						}
						}
						recog.base.set_state(1570);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(1574);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__57 {
				{
				/*InvokeRule casestmtdefaultitem*/
				recog.base.set_state(1573);
				recog.casestmtdefaultitem()?;

				}
			}

			recog.base.set_state(1576);
			recog.base.match_token(T__56,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- casestmtitem ----------------
pub type CasestmtitemContextAll<'input> = CasestmtitemContext<'input>;


pub type CasestmtitemContext<'input> = BaseParserRuleContext<'input,CasestmtitemContextExt<'input>>;

#[derive(Clone)]
pub struct CasestmtitemContextExt<'input>{
	pub match_: Option<Rc<ExpressionContextAll<'input>>>,
	pub altmatches: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for CasestmtitemContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for CasestmtitemContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_casestmtitem(self);
	}
}

impl<'input> CustomRuleContext<'input> for CasestmtitemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casestmtitem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casestmtitem }
}
antlr_rust::type_id!{CasestmtitemContextExt<'a>}

impl<'input> CasestmtitemContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CasestmtitemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CasestmtitemContextExt{
				match_: None, altmatches: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait CasestmtitemContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<CasestmtitemContextExt<'input>>{

fn stmt(&self) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CasestmtitemContextAttrs<'input> for CasestmtitemContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn casestmtitem(&mut self,)
	-> Result<Rc<CasestmtitemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CasestmtitemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_casestmtitem);
        let mut _localctx: Rc<CasestmtitemContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(1578);
			let tmp = recog.expression_rec(0)?;
			 cast_mut::<_,CasestmtitemContext >(&mut _localctx).match_ = Some(tmp.clone());
			  

			recog.base.set_state(1583);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(1579);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(1580);
				let tmp = recog.expression_rec(0)?;
				 cast_mut::<_,CasestmtitemContext >(&mut _localctx).altmatches = Some(tmp.clone());
				  

				}
				}
				recog.base.set_state(1585);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1586);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			/*InvokeRule stmt*/
			recog.base.set_state(1587);
			recog.stmt()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- casestmtpatitem ----------------
pub type CasestmtpatitemContextAll<'input> = CasestmtpatitemContext<'input>;


pub type CasestmtpatitemContext<'input> = BaseParserRuleContext<'input,CasestmtpatitemContextExt<'input>>;

#[derive(Clone)]
pub struct CasestmtpatitemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for CasestmtpatitemContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for CasestmtpatitemContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_casestmtpatitem(self);
	}
}

impl<'input> CustomRuleContext<'input> for CasestmtpatitemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casestmtpatitem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casestmtpatitem }
}
antlr_rust::type_id!{CasestmtpatitemContextExt<'a>}

impl<'input> CasestmtpatitemContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CasestmtpatitemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CasestmtpatitemContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CasestmtpatitemContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<CasestmtpatitemContextExt<'input>>{

fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stmt(&self) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn patterncond_all(&self) ->  Vec<Rc<PatterncondContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn patterncond(&self, i: usize) -> Option<Rc<PatterncondContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CasestmtpatitemContextAttrs<'input> for CasestmtpatitemContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn casestmtpatitem(&mut self,)
	-> Result<Rc<CasestmtpatitemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CasestmtpatitemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_casestmtpatitem);
        let mut _localctx: Rc<CasestmtpatitemContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule pattern*/
			recog.base.set_state(1589);
			recog.pattern()?;

			recog.base.set_state(1593);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__58 {
				{
				{
				/*InvokeRule patterncond*/
				recog.base.set_state(1590);
				recog.patterncond()?;

				}
				}
				recog.base.set_state(1595);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1596);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			/*InvokeRule stmt*/
			recog.base.set_state(1597);
			recog.stmt()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- casestmtdefaultitem ----------------
pub type CasestmtdefaultitemContextAll<'input> = CasestmtdefaultitemContext<'input>;


pub type CasestmtdefaultitemContext<'input> = BaseParserRuleContext<'input,CasestmtdefaultitemContextExt<'input>>;

#[derive(Clone)]
pub struct CasestmtdefaultitemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for CasestmtdefaultitemContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for CasestmtdefaultitemContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_casestmtdefaultitem(self);
	}
}

impl<'input> CustomRuleContext<'input> for CasestmtdefaultitemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casestmtdefaultitem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casestmtdefaultitem }
}
antlr_rust::type_id!{CasestmtdefaultitemContextExt<'a>}

impl<'input> CasestmtdefaultitemContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CasestmtdefaultitemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CasestmtdefaultitemContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CasestmtdefaultitemContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<CasestmtdefaultitemContextExt<'input>>{

fn stmt(&self) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CasestmtdefaultitemContextAttrs<'input> for CasestmtdefaultitemContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn casestmtdefaultitem(&mut self,)
	-> Result<Rc<CasestmtdefaultitemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CasestmtdefaultitemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_casestmtdefaultitem);
        let mut _localctx: Rc<CasestmtdefaultitemContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1599);
			recog.base.match_token(T__57,&mut recog.err_handler)?;

			recog.base.set_state(1601);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 {
				{
				recog.base.set_state(1600);
				recog.base.match_token(T__3,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule stmt*/
			recog.base.set_state(1603);
			recog.stmt()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- whilestmt ----------------
pub type WhilestmtContextAll<'input> = WhilestmtContext<'input>;


pub type WhilestmtContext<'input> = BaseParserRuleContext<'input,WhilestmtContextExt<'input>>;

#[derive(Clone)]
pub struct WhilestmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for WhilestmtContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for WhilestmtContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_whilestmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for WhilestmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_whilestmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_whilestmt }
}
antlr_rust::type_id!{WhilestmtContextExt<'a>}

impl<'input> WhilestmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WhilestmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WhilestmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WhilestmtContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<WhilestmtContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stmt(&self) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> WhilestmtContextAttrs<'input> for WhilestmtContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn whilestmt(&mut self,)
	-> Result<Rc<WhilestmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WhilestmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_whilestmt);
        let mut _localctx: Rc<WhilestmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1605);
			recog.base.match_token(T__96,&mut recog.err_handler)?;

			recog.base.set_state(1606);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1607);
			recog.expression_rec(0)?;

			recog.base.set_state(1608);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			/*InvokeRule stmt*/
			recog.base.set_state(1609);
			recog.stmt()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forstmt ----------------
pub type ForstmtContextAll<'input> = ForstmtContext<'input>;


pub type ForstmtContext<'input> = BaseParserRuleContext<'input,ForstmtContextExt<'input>>;

#[derive(Clone)]
pub struct ForstmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ForstmtContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ForstmtContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_forstmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForstmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forstmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forstmt }
}
antlr_rust::type_id!{ForstmtContextExt<'a>}

impl<'input> ForstmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForstmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForstmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ForstmtContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ForstmtContextExt<'input>>{

fn forinit(&self) -> Option<Rc<ForinitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fortest(&self) -> Option<Rc<FortestContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn forincr(&self) -> Option<Rc<ForincrContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stmt(&self) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ForstmtContextAttrs<'input> for ForstmtContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forstmt(&mut self,)
	-> Result<Rc<ForstmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForstmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_forstmt);
        let mut _localctx: Rc<ForstmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1611);
			recog.base.match_token(T__97,&mut recog.err_handler)?;

			recog.base.set_state(1612);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule forinit*/
			recog.base.set_state(1613);
			recog.forinit()?;

			recog.base.set_state(1614);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule fortest*/
			recog.base.set_state(1615);
			recog.fortest()?;

			recog.base.set_state(1616);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule forincr*/
			recog.base.set_state(1617);
			recog.forincr()?;

			recog.base.set_state(1618);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			/*InvokeRule stmt*/
			recog.base.set_state(1619);
			recog.stmt()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forinit ----------------
pub type ForinitContextAll<'input> = ForinitContext<'input>;


pub type ForinitContext<'input> = BaseParserRuleContext<'input,ForinitContextExt<'input>>;

#[derive(Clone)]
pub struct ForinitContextExt<'input>{
	pub var: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ForinitContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ForinitContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_forinit(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForinitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forinit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forinit }
}
antlr_rust::type_id!{ForinitContextExt<'a>}

impl<'input> ForinitContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForinitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForinitContextExt{
				var: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ForinitContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ForinitContextExt<'input>>{

fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn simplevardeclassign_all(&self) ->  Vec<Rc<SimplevardeclassignContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn simplevardeclassign(&self, i: usize) -> Option<Rc<SimplevardeclassignContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ForinitContextAttrs<'input> for ForinitContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forinit(&mut self,)
	-> Result<Rc<ForinitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForinitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 180, RULE_forinit);
        let mut _localctx: Rc<ForinitContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule bsvtype*/
			recog.base.set_state(1621);
			recog.bsvtype()?;

			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(1622);
			let tmp = recog.lowerCaseIdentifier()?;
			 cast_mut::<_,ForinitContext >(&mut _localctx).var = Some(tmp.clone());
			  

			recog.base.set_state(1623);
			recog.base.match_token(T__24,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1624);
			recog.expression_rec(0)?;

			recog.base.set_state(1629);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(1625);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule simplevardeclassign*/
				recog.base.set_state(1626);
				recog.simplevardeclassign()?;

				}
				}
				recog.base.set_state(1631);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- simplevardeclassign ----------------
pub type SimplevardeclassignContextAll<'input> = SimplevardeclassignContext<'input>;


pub type SimplevardeclassignContext<'input> = BaseParserRuleContext<'input,SimplevardeclassignContextExt<'input>>;

#[derive(Clone)]
pub struct SimplevardeclassignContextExt<'input>{
	pub var: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for SimplevardeclassignContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for SimplevardeclassignContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_simplevardeclassign(self);
	}
}

impl<'input> CustomRuleContext<'input> for SimplevardeclassignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_simplevardeclassign }
	//fn type_rule_index() -> usize where Self: Sized { RULE_simplevardeclassign }
}
antlr_rust::type_id!{SimplevardeclassignContextExt<'a>}

impl<'input> SimplevardeclassignContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SimplevardeclassignContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SimplevardeclassignContextExt{
				var: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait SimplevardeclassignContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<SimplevardeclassignContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn bsvtype(&self) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SimplevardeclassignContextAttrs<'input> for SimplevardeclassignContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn simplevardeclassign(&mut self,)
	-> Result<Rc<SimplevardeclassignContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SimplevardeclassignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_simplevardeclassign);
        let mut _localctx: Rc<SimplevardeclassignContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1633);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(195,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule bsvtype*/
					recog.base.set_state(1632);
					recog.bsvtype()?;

					}
				}

				_ => {}
			}
			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(1635);
			let tmp = recog.lowerCaseIdentifier()?;
			 cast_mut::<_,SimplevardeclassignContext >(&mut _localctx).var = Some(tmp.clone());
			  

			recog.base.set_state(1636);
			recog.base.match_token(T__24,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1637);
			recog.expression_rec(0)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fortest ----------------
pub type FortestContextAll<'input> = FortestContext<'input>;


pub type FortestContext<'input> = BaseParserRuleContext<'input,FortestContextExt<'input>>;

#[derive(Clone)]
pub struct FortestContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for FortestContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for FortestContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fortest(self);
	}
}

impl<'input> CustomRuleContext<'input> for FortestContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fortest }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fortest }
}
antlr_rust::type_id!{FortestContextExt<'a>}

impl<'input> FortestContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FortestContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FortestContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FortestContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<FortestContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FortestContextAttrs<'input> for FortestContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fortest(&mut self,)
	-> Result<Rc<FortestContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FortestContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_fortest);
        let mut _localctx: Rc<FortestContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(1639);
			recog.expression_rec(0)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forincr ----------------
pub type ForincrContextAll<'input> = ForincrContext<'input>;


pub type ForincrContext<'input> = BaseParserRuleContext<'input,ForincrContextExt<'input>>;

#[derive(Clone)]
pub struct ForincrContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ForincrContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ForincrContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_forincr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForincrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forincr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forincr }
}
antlr_rust::type_id!{ForincrContextExt<'a>}

impl<'input> ForincrContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForincrContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForincrContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ForincrContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ForincrContextExt<'input>>{

fn varincr_all(&self) ->  Vec<Rc<VarincrContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn varincr(&self, i: usize) -> Option<Rc<VarincrContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ForincrContextAttrs<'input> for ForincrContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forincr(&mut self,)
	-> Result<Rc<ForincrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForincrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_forincr);
        let mut _localctx: Rc<ForincrContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule varincr*/
			recog.base.set_state(1641);
			recog.varincr()?;

			recog.base.set_state(1646);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(1642);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule varincr*/
				recog.base.set_state(1643);
				recog.varincr()?;

				}
				}
				recog.base.set_state(1648);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- varincr ----------------
pub type VarincrContextAll<'input> = VarincrContext<'input>;


pub type VarincrContext<'input> = BaseParserRuleContext<'input,VarincrContextExt<'input>>;

#[derive(Clone)]
pub struct VarincrContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for VarincrContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for VarincrContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_varincr(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarincrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_varincr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_varincr }
}
antlr_rust::type_id!{VarincrContextExt<'a>}

impl<'input> VarincrContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VarincrContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VarincrContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VarincrContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<VarincrContextExt<'input>>{

fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VarincrContextAttrs<'input> for VarincrContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn varincr(&mut self,)
	-> Result<Rc<VarincrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VarincrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 188, RULE_varincr);
        let mut _localctx: Rc<VarincrContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule lowerCaseIdentifier*/
			recog.base.set_state(1649);
			recog.lowerCaseIdentifier()?;

			recog.base.set_state(1650);
			recog.base.match_token(T__24,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1651);
			recog.expression_rec(0)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- returnstmt ----------------
pub type ReturnstmtContextAll<'input> = ReturnstmtContext<'input>;


pub type ReturnstmtContext<'input> = BaseParserRuleContext<'input,ReturnstmtContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnstmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ReturnstmtContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ReturnstmtContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_returnstmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnstmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnstmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnstmt }
}
antlr_rust::type_id!{ReturnstmtContextExt<'a>}

impl<'input> ReturnstmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnstmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnstmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnstmtContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ReturnstmtContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReturnstmtContextAttrs<'input> for ReturnstmtContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnstmt(&mut self,)
	-> Result<Rc<ReturnstmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnstmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 190, RULE_returnstmt);
        let mut _localctx: Rc<ReturnstmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1653);
			recog.base.match_token(T__98,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1654);
			recog.expression_rec(0)?;

			recog.base.set_state(1655);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pattern ----------------
pub type PatternContextAll<'input> = PatternContext<'input>;


pub type PatternContext<'input> = BaseParserRuleContext<'input,PatternContextExt<'input>>;

#[derive(Clone)]
pub struct PatternContextExt<'input>{
	pub var: Option<Rc<LowerCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for PatternContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for PatternContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_pattern(self);
	}
}

impl<'input> CustomRuleContext<'input> for PatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pattern }
}
antlr_rust::type_id!{PatternContextExt<'a>}

impl<'input> PatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternContextExt{
				var: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<PatternContextExt<'input>>{

fn lowerCaseIdentifier(&self) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constantpattern(&self) -> Option<Rc<ConstantpatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn taggedunionpattern(&self) -> Option<Rc<TaggedunionpatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tuplepattern(&self) -> Option<Rc<TuplepatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PatternContextAttrs<'input> for PatternContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pattern(&mut self,)
	-> Result<Rc<PatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 192, RULE_pattern);
        let mut _localctx: Rc<PatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1667);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__50 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1657);
					recog.base.match_token(T__50,&mut recog.err_handler)?;

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(1658);
					let tmp = recog.lowerCaseIdentifier()?;
					 cast_mut::<_,PatternContext >(&mut _localctx).var = Some(tmp.clone());
					  

					}
				}

			 T__99 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1659);
					recog.base.match_token(T__99,&mut recog.err_handler)?;

					}
				}

			 IntLiteral | IntPattern | RealLiteral | StringLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule constantpattern*/
					recog.base.set_state(1660);
					recog.constantpattern()?;

					}
				}

			 T__27 | UpperCaseIdentifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule taggedunionpattern*/
					recog.base.set_state(1661);
					recog.taggedunionpattern()?;

					}
				}

			 T__20 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule tuplepattern*/
					recog.base.set_state(1662);
					recog.tuplepattern()?;

					}
				}

			 T__8 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(1663);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(1664);
					recog.pattern()?;

					recog.base.set_state(1665);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- constantpattern ----------------
pub type ConstantpatternContextAll<'input> = ConstantpatternContext<'input>;


pub type ConstantpatternContext<'input> = BaseParserRuleContext<'input,ConstantpatternContextExt<'input>>;

#[derive(Clone)]
pub struct ConstantpatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ConstantpatternContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ConstantpatternContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_constantpattern(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstantpatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constantpattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constantpattern }
}
antlr_rust::type_id!{ConstantpatternContextExt<'a>}

impl<'input> ConstantpatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstantpatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstantpatternContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstantpatternContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ConstantpatternContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IntLiteral
/// Returns `None` if there is no child corresponding to token IntLiteral
fn IntLiteral(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
	self.get_token(IntLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token IntPattern
/// Returns `None` if there is no child corresponding to token IntPattern
fn IntPattern(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
	self.get_token(IntPattern, 0)
}
/// Retrieves first TerminalNode corresponding to token RealLiteral
/// Returns `None` if there is no child corresponding to token RealLiteral
fn RealLiteral(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
	self.get_token(RealLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token StringLiteral
/// Returns `None` if there is no child corresponding to token StringLiteral
fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input,BSVParserContextType>>> where Self:Sized{
	self.get_token(StringLiteral, 0)
}

}

impl<'input> ConstantpatternContextAttrs<'input> for ConstantpatternContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constantpattern(&mut self,)
	-> Result<Rc<ConstantpatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstantpatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 194, RULE_constantpattern);
        let mut _localctx: Rc<ConstantpatternContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1669);
			_la = recog.base.input.la(1);
			if { !(((((_la - 109)) & !0x3f) == 0 && ((1usize << (_la - 109)) & ((1usize << (IntLiteral - 109)) | (1usize << (IntPattern - 109)) | (1usize << (RealLiteral - 109)) | (1usize << (StringLiteral - 109)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- taggedunionpattern ----------------
pub type TaggedunionpatternContextAll<'input> = TaggedunionpatternContext<'input>;


pub type TaggedunionpatternContext<'input> = BaseParserRuleContext<'input,TaggedunionpatternContextExt<'input>>;

#[derive(Clone)]
pub struct TaggedunionpatternContextExt<'input>{
	pub tag: Option<Rc<UpperCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TaggedunionpatternContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TaggedunionpatternContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_taggedunionpattern(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaggedunionpatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taggedunionpattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taggedunionpattern }
}
antlr_rust::type_id!{TaggedunionpatternContextExt<'a>}

impl<'input> TaggedunionpatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TaggedunionpatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TaggedunionpatternContextExt{
				tag: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait TaggedunionpatternContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TaggedunionpatternContextExt<'input>>{

fn upperCaseIdentifier(&self) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pattern_all(&self) ->  Vec<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pattern(&self, i: usize) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn lowerCaseIdentifier_all(&self) ->  Vec<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn lowerCaseIdentifier(&self, i: usize) -> Option<Rc<LowerCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TaggedunionpatternContextAttrs<'input> for TaggedunionpatternContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn taggedunionpattern(&mut self,)
	-> Result<Rc<TaggedunionpatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TaggedunionpatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 196, RULE_taggedunionpattern);
        let mut _localctx: Rc<TaggedunionpatternContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1695);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(201,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1672);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__27 {
						{
						recog.base.set_state(1671);
						recog.base.match_token(T__27,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(1674);
					let tmp = recog.upperCaseIdentifier()?;
					 cast_mut::<_,TaggedunionpatternContext >(&mut _localctx).tag = Some(tmp.clone());
					  

					recog.base.set_state(1676);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(199,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule pattern*/
							recog.base.set_state(1675);
							recog.pattern()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(1678);
					let tmp = recog.upperCaseIdentifier()?;
					 cast_mut::<_,TaggedunionpatternContext >(&mut _localctx).tag = Some(tmp.clone());
					  

					recog.base.set_state(1679);
					recog.base.match_token(T__20,&mut recog.err_handler)?;

					/*InvokeRule lowerCaseIdentifier*/
					recog.base.set_state(1680);
					recog.lowerCaseIdentifier()?;

					recog.base.set_state(1681);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(1682);
					recog.pattern()?;

					recog.base.set_state(1690);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__5 {
						{
						{
						recog.base.set_state(1683);
						recog.base.match_token(T__5,&mut recog.err_handler)?;

						/*InvokeRule lowerCaseIdentifier*/
						recog.base.set_state(1684);
						recog.lowerCaseIdentifier()?;

						recog.base.set_state(1685);
						recog.base.match_token(T__3,&mut recog.err_handler)?;

						/*InvokeRule pattern*/
						recog.base.set_state(1686);
						recog.pattern()?;

						}
						}
						recog.base.set_state(1692);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1693);
					recog.base.match_token(T__21,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tuplepattern ----------------
pub type TuplepatternContextAll<'input> = TuplepatternContext<'input>;


pub type TuplepatternContext<'input> = BaseParserRuleContext<'input,TuplepatternContextExt<'input>>;

#[derive(Clone)]
pub struct TuplepatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for TuplepatternContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for TuplepatternContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_tuplepattern(self);
	}
}

impl<'input> CustomRuleContext<'input> for TuplepatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tuplepattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tuplepattern }
}
antlr_rust::type_id!{TuplepatternContextExt<'a>}

impl<'input> TuplepatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TuplepatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TuplepatternContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TuplepatternContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<TuplepatternContextExt<'input>>{

fn pattern_all(&self) ->  Vec<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pattern(&self, i: usize) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TuplepatternContextAttrs<'input> for TuplepatternContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tuplepattern(&mut self,)
	-> Result<Rc<TuplepatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TuplepatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 198, RULE_tuplepattern);
        let mut _localctx: Rc<TuplepatternContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1697);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			/*InvokeRule pattern*/
			recog.base.set_state(1698);
			recog.pattern()?;

			recog.base.set_state(1703);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(1699);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule pattern*/
				recog.base.set_state(1700);
				recog.pattern()?;

				}
				}
				recog.base.set_state(1705);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1706);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- attributeinstance ----------------
pub type AttributeinstanceContextAll<'input> = AttributeinstanceContext<'input>;


pub type AttributeinstanceContext<'input> = BaseParserRuleContext<'input,AttributeinstanceContextExt<'input>>;

#[derive(Clone)]
pub struct AttributeinstanceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for AttributeinstanceContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for AttributeinstanceContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_attributeinstance(self);
	}
}

impl<'input> CustomRuleContext<'input> for AttributeinstanceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attributeinstance }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attributeinstance }
}
antlr_rust::type_id!{AttributeinstanceContextExt<'a>}

impl<'input> AttributeinstanceContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AttributeinstanceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AttributeinstanceContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AttributeinstanceContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<AttributeinstanceContextExt<'input>>{

fn attrspec_all(&self) ->  Vec<Rc<AttrspecContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attrspec(&self, i: usize) -> Option<Rc<AttrspecContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AttributeinstanceContextAttrs<'input> for AttributeinstanceContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn attributeinstance(&mut self,)
	-> Result<Rc<AttributeinstanceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AttributeinstanceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 200, RULE_attributeinstance);
        let mut _localctx: Rc<AttributeinstanceContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1708);
			recog.base.match_token(T__100,&mut recog.err_handler)?;

			/*InvokeRule attrspec*/
			recog.base.set_state(1709);
			recog.attrspec()?;

			recog.base.set_state(1714);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(1710);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule attrspec*/
				recog.base.set_state(1711);
				recog.attrspec()?;

				}
				}
				recog.base.set_state(1716);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1717);
			recog.base.match_token(T__101,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- attrspec ----------------
pub type AttrspecContextAll<'input> = AttrspecContext<'input>;


pub type AttrspecContext<'input> = BaseParserRuleContext<'input,AttrspecContextExt<'input>>;

#[derive(Clone)]
pub struct AttrspecContextExt<'input>{
	pub attrname: Option<Rc<AnyidentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for AttrspecContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for AttrspecContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_attrspec(self);
	}
}

impl<'input> CustomRuleContext<'input> for AttrspecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attrspec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attrspec }
}
antlr_rust::type_id!{AttrspecContextExt<'a>}

impl<'input> AttrspecContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AttrspecContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AttrspecContextExt{
				attrname: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait AttrspecContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<AttrspecContextExt<'input>>{

fn anyidentifier(&self) -> Option<Rc<AnyidentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AttrspecContextAttrs<'input> for AttrspecContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn attrspec(&mut self,)
	-> Result<Rc<AttrspecContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AttrspecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 202, RULE_attrspec);
        let mut _localctx: Rc<AttrspecContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule anyidentifier*/
			recog.base.set_state(1719);
			let tmp = recog.anyidentifier()?;
			 cast_mut::<_,AttrspecContext >(&mut _localctx).attrname = Some(tmp.clone());
			  

			recog.base.set_state(1722);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__24 {
				{
				recog.base.set_state(1720);
				recog.base.match_token(T__24,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(1721);
				recog.expression_rec(0)?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- provisos ----------------
pub type ProvisosContextAll<'input> = ProvisosContext<'input>;


pub type ProvisosContext<'input> = BaseParserRuleContext<'input,ProvisosContextExt<'input>>;

#[derive(Clone)]
pub struct ProvisosContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ProvisosContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ProvisosContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_provisos(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProvisosContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_provisos }
	//fn type_rule_index() -> usize where Self: Sized { RULE_provisos }
}
antlr_rust::type_id!{ProvisosContextExt<'a>}

impl<'input> ProvisosContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProvisosContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProvisosContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProvisosContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ProvisosContextExt<'input>>{

fn proviso_all(&self) ->  Vec<Rc<ProvisoContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn proviso(&self, i: usize) -> Option<Rc<ProvisoContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProvisosContextAttrs<'input> for ProvisosContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn provisos(&mut self,)
	-> Result<Rc<ProvisosContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProvisosContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 204, RULE_provisos);
        let mut _localctx: Rc<ProvisosContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1724);
			recog.base.match_token(T__102,&mut recog.err_handler)?;

			recog.base.set_state(1725);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule proviso*/
			recog.base.set_state(1726);
			recog.proviso()?;

			recog.base.set_state(1731);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(1727);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule proviso*/
				recog.base.set_state(1728);
				recog.proviso()?;

				}
				}
				recog.base.set_state(1733);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1734);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- proviso ----------------
pub type ProvisoContextAll<'input> = ProvisoContext<'input>;


pub type ProvisoContext<'input> = BaseParserRuleContext<'input,ProvisoContextExt<'input>>;

#[derive(Clone)]
pub struct ProvisoContextExt<'input>{
	pub pkg: Option<Rc<UpperCaseIdentifierContextAll<'input>>>,
	pub var: Option<Rc<UpperCaseIdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> BSVParserContext<'input> for ProvisoContext<'input>{}

impl<'input,'a> Listenable<dyn BSVListener<'input> + 'a> for ProvisoContext<'input>{
	fn enter(&self,listener: &mut (dyn BSVListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_proviso(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProvisoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = BSVParserContextType;
	fn get_rule_index(&self) -> usize { RULE_proviso }
	//fn type_rule_index() -> usize where Self: Sized { RULE_proviso }
}
antlr_rust::type_id!{ProvisoContextExt<'a>}

impl<'input> ProvisoContextExt<'input>{
	fn new(parent: Option<Rc<dyn BSVParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProvisoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProvisoContextExt{
				pkg: None, var: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ProvisoContextAttrs<'input>: BSVParserContext<'input> + BorrowMut<ProvisoContextExt<'input>>{

fn bsvtype_all(&self) ->  Vec<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn bsvtype(&self, i: usize) -> Option<Rc<BsvtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn upperCaseIdentifier_all(&self) ->  Vec<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn upperCaseIdentifier(&self, i: usize) -> Option<Rc<UpperCaseIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProvisoContextAttrs<'input> for ProvisoContext<'input>{}

impl<'input, I, H> BSVParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn proviso(&mut self,)
	-> Result<Rc<ProvisoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProvisoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 206, RULE_proviso);
        let mut _localctx: Rc<ProvisoContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1739);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(206,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule upperCaseIdentifier*/
					recog.base.set_state(1736);
					let tmp = recog.upperCaseIdentifier()?;
					 cast_mut::<_,ProvisoContext >(&mut _localctx).pkg = Some(tmp.clone());
					  

					recog.base.set_state(1737);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			/*InvokeRule upperCaseIdentifier*/
			recog.base.set_state(1741);
			let tmp = recog.upperCaseIdentifier()?;
			 cast_mut::<_,ProvisoContext >(&mut _localctx).var = Some(tmp.clone());
			  

			recog.base.set_state(1742);
			recog.base.match_token(T__15,&mut recog.err_handler)?;

			recog.base.set_state(1743);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule bsvtype*/
			recog.base.set_state(1744);
			recog.bsvtype()?;

			recog.base.set_state(1749);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__5 {
				{
				{
				recog.base.set_state(1745);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule bsvtype*/
				recog.base.set_state(1746);
				recog.bsvtype()?;

				}
				}
				recog.base.set_state(1751);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1752);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x75\u{6dd}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x04\x55\x09\x55\x04\x56\x09\
	\x56\x04\x57\x09\x57\x04\x58\x09\x58\x04\x59\x09\x59\x04\x5a\x09\x5a\x04\
	\x5b\x09\x5b\x04\x5c\x09\x5c\x04\x5d\x09\x5d\x04\x5e\x09\x5e\x04\x5f\x09\
	\x5f\x04\x60\x09\x60\x04\x61\x09\x61\x04\x62\x09\x62\x04\x63\x09\x63\x04\
	\x64\x09\x64\x04\x65\x09\x65\x04\x66\x09\x66\x04\x67\x09\x67\x04\x68\x09\
	\x68\x04\x69\x09\x69\x03\x02\x03\x02\x07\x02\u{d5}\x0a\x02\x0c\x02\x0e\x02\
	\u{d8}\x0b\x02\x03\x02\x05\x02\u{db}\x0a\x02\x03\x02\x03\x02\x03\x02\x07\
	\x02\u{e0}\x0a\x02\x0c\x02\x0e\x02\u{e3}\x0b\x02\x03\x02\x05\x02\u{e6}\x0a\
	\x02\x03\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x05\x04\u{ef}\
	\x0a\x04\x03\x05\x03\x05\x03\x06\x03\x06\x03\x07\x03\x07\x05\x07\u{f7}\x0a\
	\x07\x03\x08\x03\x08\x03\x08\x03\x08\x07\x08\u{fd}\x0a\x08\x0c\x08\x0e\x08\
	\u{100}\x0b\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\
	\x03\x09\x03\x09\x03\x09\x05\x09\u{10c}\x0a\x09\x05\x09\u{10e}\x0a\x09\x03\
	\x0a\x03\x0a\x03\x0a\x03\x0a\x06\x0a\u{114}\x0a\x0a\x0d\x0a\x0e\x0a\u{115}\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{128}\
	\x0a\x0b\x03\x0c\x03\x0c\x03\x0d\x07\x0d\u{12d}\x0a\x0d\x0c\x0d\x0e\x0d\
	\u{130}\x0b\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x07\x0d\u{136}\x0a\x0d\x0c\
	\x0d\x0e\x0d\u{139}\x0b\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{13e}\x0a\x0d\
	\x03\x0e\x03\x0e\x05\x0e\u{142}\x0a\x0e\x03\x0f\x07\x0f\u{145}\x0a\x0f\x0c\
	\x0f\x0e\x0f\u{148}\x0b\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x05\
	\x0f\u{14f}\x0a\x0f\x03\x0f\x05\x0f\u{152}\x0a\x0f\x03\x0f\x05\x0f\u{155}\
	\x0a\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x10\x07\x10\u{15c}\x0a\x10\
	\x0c\x10\x0e\x10\u{15f}\x0b\x10\x03\x11\x07\x11\u{162}\x0a\x11\x0c\x11\x0e\
	\x11\u{165}\x0b\x11\x03\x11\x05\x11\u{168}\x0a\x11\x03\x11\x03\x11\x03\x12\
	\x07\x12\u{16d}\x0a\x12\x0c\x12\x0e\x12\u{170}\x0b\x12\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x13\x03\x13\x05\x13\u{179}\x0a\x13\x03\x14\x03\
	\x14\x03\x14\x03\x14\x03\x14\x07\x14\u{180}\x0a\x14\x0c\x14\x0e\x14\u{183}\
	\x0b\x14\x03\x14\x03\x14\x03\x15\x05\x15\u{188}\x0a\x15\x03\x15\x03\x15\
	\x03\x15\x03\x16\x07\x16\u{18e}\x0a\x16\x0c\x16\x0e\x16\u{191}\x0b\x16\x03\
	\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x17\x03\x17\x03\x17\x03\x17\x03\
	\x17\x03\x17\x07\x17\u{19e}\x0a\x17\x0c\x17\x0e\x17\u{1a1}\x0b\x17\x03\x17\
	\x03\x17\x03\x17\x05\x17\u{1a6}\x0a\x17\x03\x17\x03\x17\x03\x18\x03\x18\
	\x03\x18\x03\x18\x03\x18\x05\x18\u{1af}\x0a\x18\x03\x18\x03\x18\x03\x18\
	\x05\x18\u{1b4}\x0a\x18\x03\x18\x03\x18\x03\x18\x05\x18\u{1b9}\x0a\x18\x05\
	\x18\u{1bb}\x0a\x18\x03\x19\x07\x19\u{1be}\x0a\x19\x0c\x19\x0e\x19\u{1c1}\
	\x0b\x19\x03\x19\x03\x19\x03\x19\x03\x19\x07\x19\u{1c7}\x0a\x19\x0c\x19\
	\x0e\x19\u{1ca}\x0b\x19\x03\x19\x03\x19\x03\x19\x05\x19\u{1cf}\x0a\x19\x03\
	\x19\x03\x19\x03\x1a\x07\x1a\u{1d4}\x0a\x1a\x0c\x1a\x0e\x1a\u{1d7}\x0b\x1a\
	\x03\x1a\x03\x1a\x03\x1a\x05\x1a\u{1dc}\x0a\x1a\x03\x1a\x03\x1a\x07\x1a\
	\u{1e0}\x0a\x1a\x0c\x1a\x0e\x1a\u{1e3}\x0b\x1a\x03\x1a\x03\x1a\x03\x1a\x05\
	\x1a\u{1e8}\x0a\x1a\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\
	\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\u{1f4}\x0a\x1b\x03\x1c\x03\x1c\x03\
	\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\
	\x1c\x05\x1c\u{202}\x0a\x1c\x03\x1d\x03\x1d\x03\x1d\x07\x1d\u{207}\x0a\x1d\
	\x0c\x1d\x0e\x1d\u{20a}\x0b\x1d\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x05\x1e\
	\u{210}\x0a\x1e\x03\x1e\x03\x1e\x07\x1e\u{214}\x0a\x1e\x0c\x1e\x0e\x1e\u{217}\
	\x0b\x1e\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x07\x1f\
	\u{220}\x0a\x1f\x0c\x1f\x0e\x1f\u{223}\x0b\x1f\x03\x1f\x03\x1f\x03\x20\x07\
	\x20\u{228}\x0a\x20\x0c\x20\x0e\x20\u{22b}\x0b\x20\x03\x20\x03\x20\x05\x20\
	\u{22f}\x0a\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x21\
	\x03\x21\x03\x21\x03\x21\x06\x21\u{23b}\x0a\x21\x0d\x21\x0e\x21\u{23c}\x03\
	\x21\x03\x21\x03\x22\x03\x22\x03\x22\x05\x22\u{244}\x0a\x22\x03\x22\x03\
	\x22\x03\x22\x03\x22\x05\x22\u{24a}\x0a\x22\x03\x23\x07\x23\u{24d}\x0a\x23\
	\x0c\x23\x0e\x23\u{250}\x0b\x23\x03\x23\x03\x23\x05\x23\u{254}\x0a\x23\x03\
	\x23\x03\x23\x03\x23\x07\x23\u{259}\x0a\x23\x0c\x23\x0e\x23\u{25c}\x0b\x23\
	\x03\x23\x03\x23\x03\x24\x07\x24\u{261}\x0a\x24\x0c\x24\x0e\x24\u{264}\x0b\
	\x24\x03\x24\x03\x24\x05\x24\u{268}\x0a\x24\x03\x24\x03\x24\x03\x24\x03\
	\x24\x03\x24\x05\x24\u{26f}\x0a\x24\x03\x24\x03\x24\x03\x24\x03\x24\x03\
	\x25\x07\x25\u{276}\x0a\x25\x0c\x25\x0e\x25\u{279}\x0b\x25\x03\x25\x03\x25\
	\x03\x25\x03\x25\x03\x25\x03\x25\x03\x26\x07\x26\u{282}\x0a\x26\x0c\x26\
	\x0e\x26\u{285}\x0b\x26\x03\x26\x03\x26\x03\x26\x03\x26\x05\x26\u{28b}\x0a\
	\x26\x03\x26\x05\x26\u{28e}\x0a\x26\x03\x26\x03\x26\x07\x26\u{292}\x0a\x26\
	\x0c\x26\x0e\x26\u{295}\x0b\x26\x03\x26\x03\x26\x03\x26\x05\x26\u{29a}\x0a\
	\x26\x03\x27\x03\x27\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x07\x28\u{2a3}\
	\x0a\x28\x0c\x28\x0e\x28\u{2a6}\x0b\x28\x03\x28\x03\x28\x03\x29\x03\x29\
	\x03\x29\x03\x29\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x07\x2a\u{2b3}\
	\x0a\x2a\x0c\x2a\x0e\x2a\u{2b6}\x0b\x2a\x03\x2a\x03\x2a\x05\x2a\u{2ba}\x0a\
	\x2a\x03\x2b\x07\x2b\u{2bd}\x0a\x2b\x0c\x2b\x0e\x2b\u{2c0}\x0b\x2b\x03\x2b\
	\x03\x2b\x03\x2b\x05\x2b\u{2c5}\x0a\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\
	\x05\x2b\u{2cb}\x0a\x2b\x03\x2c\x03\x2c\x05\x2c\u{2cf}\x0a\x2c\x03\x2d\x07\
	\x2d\u{2d2}\x0a\x2d\x0c\x2d\x0e\x2d\u{2d5}\x0b\x2d\x03\x2d\x03\x2d\x03\x2d\
	\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x07\x2d\u{2de}\x0a\x2d\x0c\x2d\x0e\x2d\
	\u{2e1}\x0b\x2d\x03\x2d\x03\x2d\x05\x2d\u{2e5}\x0a\x2d\x03\x2d\x03\x2d\x07\
	\x2d\u{2e9}\x0a\x2d\x0c\x2d\x0e\x2d\u{2ec}\x0b\x2d\x03\x2d\x03\x2d\x03\x2d\
	\x05\x2d\u{2f1}\x0a\x2d\x03\x2e\x03\x2e\x03\x2e\x05\x2e\u{2f6}\x0a\x2e\x03\
	\x2f\x07\x2f\u{2f9}\x0a\x2f\x0c\x2f\x0e\x2f\u{2fc}\x0b\x2f\x03\x2f\x03\x2f\
	\x07\x2f\u{300}\x0a\x2f\x0c\x2f\x0e\x2f\u{303}\x0b\x2f\x03\x2f\x03\x2f\x03\
	\x2f\x05\x2f\u{308}\x0a\x2f\x03\x30\x03\x30\x03\x30\x03\x30\x03\x30\x05\
	\x30\u{30f}\x0a\x30\x03\x30\x03\x30\x05\x30\u{313}\x0a\x30\x03\x30\x03\x30\
	\x03\x30\x03\x30\x03\x30\x03\x30\x03\x30\x05\x30\u{31c}\x0a\x30\x03\x30\
	\x03\x30\x03\x30\x03\x30\x03\x30\x05\x30\u{323}\x0a\x30\x03\x30\x03\x30\
	\x03\x30\x03\x30\x03\x30\x03\x30\x03\x30\x05\x30\u{32c}\x0a\x30\x03\x30\
	\x03\x30\x03\x30\x03\x30\x05\x30\u{332}\x0a\x30\x03\x30\x03\x30\x03\x30\
	\x03\x30\x03\x30\x05\x30\u{339}\x0a\x30\x03\x30\x03\x30\x05\x30\u{33d}\x0a\
	\x30\x03\x31\x03\x31\x03\x31\x07\x31\u{342}\x0a\x31\x0c\x31\x0e\x31\u{345}\
	\x0b\x31\x03\x32\x07\x32\u{348}\x0a\x32\x0c\x32\x0e\x32\u{34b}\x0b\x32\x03\
	\x32\x05\x32\u{34e}\x0a\x32\x03\x32\x03\x32\x03\x32\x03\x32\x05\x32\u{354}\
	\x0a\x32\x03\x33\x03\x33\x03\x33\x03\x33\x03\x33\x05\x33\u{35b}\x0a\x33\
	\x03\x34\x03\x34\x05\x34\u{35f}\x0a\x34\x03\x34\x03\x34\x03\x34\x05\x34\
	\u{364}\x0a\x34\x03\x34\x05\x34\u{367}\x0a\x34\x03\x34\x05\x34\u{36a}\x0a\
	\x34\x03\x34\x05\x34\u{36d}\x0a\x34\x03\x34\x03\x34\x07\x34\u{371}\x0a\x34\
	\x0c\x34\x0e\x34\u{374}\x0b\x34\x03\x34\x03\x34\x03\x34\x05\x34\u{379}\x0a\
	\x34\x03\x34\x03\x34\x05\x34\u{37d}\x0a\x34\x03\x34\x03\x34\x03\x34\x05\
	\x34\u{382}\x0a\x34\x03\x34\x05\x34\u{385}\x0a\x34\x03\x34\x05\x34\u{388}\
	\x0a\x34\x03\x34\x03\x34\x03\x34\x03\x34\x05\x34\u{38e}\x0a\x34\x03\x35\
	\x03\x35\x03\x35\x07\x35\u{393}\x0a\x35\x0c\x35\x0e\x35\u{396}\x0b\x35\x03\
	\x36\x07\x36\u{399}\x0a\x36\x0c\x36\x0e\x36\u{39c}\x0b\x36\x03\x36\x05\x36\
	\u{39f}\x0a\x36\x03\x36\x03\x36\x05\x36\u{3a3}\x0a\x36\x03\x37\x03\x37\x03\
	\x37\x03\x37\x03\x37\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x07\x38\u{3af}\
	\x0a\x38\x0c\x38\x0e\x38\u{3b2}\x0b\x38\x03\x38\x03\x38\x03\x38\x05\x38\
	\u{3b7}\x0a\x38\x03\x38\x03\x38\x05\x38\u{3bb}\x0a\x38\x03\x38\x03\x38\x03\
	\x38\x03\x38\x03\x38\x05\x38\u{3c2}\x0a\x38\x03\x39\x07\x39\u{3c5}\x0a\x39\
	\x0c\x39\x0e\x39\u{3c8}\x0b\x39\x03\x39\x03\x39\x03\x39\x05\x39\u{3cd}\x0a\
	\x39\x03\x39\x03\x39\x07\x39\u{3d1}\x0a\x39\x0c\x39\x0e\x39\u{3d4}\x0b\x39\
	\x03\x39\x03\x39\x03\x39\x05\x39\u{3d9}\x0a\x39\x03\x3a\x03\x3a\x03\x3a\
	\x05\x3a\u{3de}\x0a\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3b\x07\x3b\
	\u{3e5}\x0a\x3b\x0c\x3b\x0e\x3b\u{3e8}\x0b\x3b\x03\x3b\x03\x3b\x03\x3b\x07\
	\x3b\u{3ed}\x0a\x3b\x0c\x3b\x0e\x3b\u{3f0}\x0b\x3b\x03\x3b\x03\x3b\x03\x3b\
	\x05\x3b\u{3f5}\x0a\x3b\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x05\x3b\
	\u{3fc}\x0a\x3b\x03\x3c\x03\x3c\x05\x3c\u{400}\x0a\x3c\x03\x3c\x03\x3c\x03\
	\x3c\x05\x3c\u{405}\x0a\x3c\x03\x3c\x05\x3c\u{408}\x0a\x3c\x03\x3c\x05\x3c\
	\u{40b}\x0a\x3c\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x05\x3d\u{412}\x0a\
	\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x05\x3d\u{419}\x0a\x3d\x03\
	\x3d\x03\x3d\x03\x3d\x03\x3e\x03\x3e\x03\x3e\x07\x3e\u{421}\x0a\x3e\x0c\
	\x3e\x0e\x3e\u{424}\x0b\x3e\x03\x3f\x03\x3f\x05\x3f\u{428}\x0a\x3f\x03\x40\
	\x07\x40\u{42b}\x0a\x40\x0c\x40\x0e\x40\u{42e}\x0b\x40\x03\x40\x03\x40\x03\
	\x40\x03\x40\x03\x40\x03\x40\x07\x40\u{436}\x0a\x40\x0c\x40\x0e\x40\u{439}\
	\x0b\x40\x03\x40\x03\x40\x03\x40\x03\x40\x07\x40\u{43f}\x0a\x40\x0c\x40\
	\x0e\x40\u{442}\x0b\x40\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x05\x40\
	\u{449}\x0a\x40\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\
	\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\
	\x03\x41\x03\x41\x03\x41\x05\x41\u{45e}\x0a\x41\x03\x41\x03\x41\x05\x41\
	\u{462}\x0a\x41\x03\x42\x03\x42\x03\x42\x03\x42\x03\x42\x03\x42\x07\x42\
	\u{46a}\x0a\x42\x0c\x42\x0e\x42\u{46d}\x0b\x42\x03\x42\x03\x42\x05\x42\u{471}\
	\x0a\x42\x03\x42\x03\x42\x03\x42\x03\x42\x03\x42\x03\x42\x05\x42\u{479}\
	\x0a\x42\x03\x43\x03\x43\x03\x43\x07\x43\u{47e}\x0a\x43\x0c\x43\x0e\x43\
	\u{481}\x0b\x43\x03\x43\x03\x43\x05\x43\u{485}\x0a\x43\x03\x44\x03\x44\x03\
	\x45\x03\x45\x03\x45\x03\x45\x03\x45\x03\x45\x03\x45\x06\x45\u{490}\x0a\
	\x45\x0d\x45\x0e\x45\u{491}\x03\x45\x07\x45\u{495}\x0a\x45\x0c\x45\x0e\x45\
	\u{498}\x0b\x45\x05\x45\u{49a}\x0a\x45\x03\x45\x05\x45\u{49d}\x0a\x45\x03\
	\x45\x03\x45\x03\x45\x05\x45\u{4a2}\x0a\x45\x03\x45\x03\x45\x03\x45\x03\
	\x45\x03\x45\x03\x45\x03\x45\x03\x45\x03\x45\x03\x45\x07\x45\u{4ae}\x0a\
	\x45\x0c\x45\x0e\x45\u{4b1}\x0b\x45\x07\x45\u{4b3}\x0a\x45\x0c\x45\x0e\x45\
	\u{4b6}\x0b\x45\x03\x46\x03\x46\x07\x46\u{4ba}\x0a\x46\x0c\x46\x0e\x46\u{4bd}\
	\x0b\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x47\x03\x47\x03\x47\x07\x47\
	\u{4c6}\x0a\x47\x0c\x47\x0e\x47\u{4c9}\x0b\x47\x03\x47\x03\x47\x03\x47\x03\
	\x47\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x49\x03\x49\x03\x49\x03\
	\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\
	\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\
	\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\
	\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x07\x4a\u{4f8}\x0a\x4a\x0c\
	\x4a\x0e\x4a\u{4fb}\x0b\x4a\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x05\
	\x4b\u{502}\x0a\x4b\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\
	\x4c\x03\x4c\x03\x4c\x03\x4c\x05\x4c\u{50e}\x0a\x4c\x03\x4c\x03\x4c\x03\
	\x4c\x03\x4c\x03\x4c\x03\x4c\x07\x4c\u{516}\x0a\x4c\x0c\x4c\x0e\x4c\u{519}\
	\x0b\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\
	\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x07\x4c\u{529}\x0a\x4c\
	\x0c\x4c\x0e\x4c\u{52c}\x0b\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\
	\x03\x4c\x03\x4c\x07\x4c\u{535}\x0a\x4c\x0c\x4c\x0e\x4c\u{538}\x0b\x4c\x05\
	\x4c\u{53a}\x0a\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x07\
	\x4c\u{542}\x0a\x4c\x0c\x4c\x0e\x4c\u{545}\x0b\x4c\x05\x4c\u{547}\x0a\x4c\
	\x03\x4c\x05\x4c\u{54a}\x0a\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\
	\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x07\x4c\u{556}\x0a\x4c\x0c\x4c\
	\x0e\x4c\u{559}\x0b\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\
	\x05\x4c\u{561}\x0a\x4c\x03\x4c\x05\x4c\u{564}\x0a\x4c\x03\x4c\x03\x4c\x03\
	\x4c\x07\x4c\u{569}\x0a\x4c\x0c\x4c\x0e\x4c\u{56c}\x0b\x4c\x03\x4c\x03\x4c\
	\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x05\x4c\u{575}\x0a\x4c\x03\x4c\
	\x03\x4c\x03\x4c\x05\x4c\u{57a}\x0a\x4c\x03\x4c\x07\x4c\u{57d}\x0a\x4c\x0c\
	\x4c\x0e\x4c\u{580}\x0b\x4c\x03\x4c\x03\x4c\x03\x4c\x05\x4c\u{585}\x0a\x4c\
	\x03\x4c\x03\x4c\x05\x4c\u{589}\x0a\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\
	\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x05\x4c\
	\u{597}\x0a\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\
	\x07\x4c\u{5a0}\x0a\x4c\x0c\x4c\x0e\x4c\u{5a3}\x0b\x4c\x05\x4c\u{5a5}\x0a\
	\x4c\x03\x4c\x07\x4c\u{5a8}\x0a\x4c\x0c\x4c\x0e\x4c\u{5ab}\x0b\x4c\x03\x4d\
	\x03\x4d\x03\x4d\x07\x4d\u{5b0}\x0a\x4d\x0c\x4d\x0e\x4d\u{5b3}\x0b\x4d\x03\
	\x4e\x03\x4e\x03\x4e\x03\x4e\x03\x4f\x03\x4f\x03\x4f\x03\x4f\x05\x4f\u{5bd}\
	\x0a\x4f\x03\x50\x07\x50\u{5c0}\x0a\x50\x0c\x50\x0e\x50\u{5c3}\x0b\x50\x03\
	\x50\x03\x50\x03\x50\x05\x50\u{5c8}\x0a\x50\x03\x50\x07\x50\u{5cb}\x0a\x50\
	\x0c\x50\x0e\x50\u{5ce}\x0b\x50\x03\x50\x03\x50\x03\x50\x05\x50\u{5d3}\x0a\
	\x50\x03\x51\x07\x51\u{5d6}\x0a\x51\x0c\x51\x0e\x51\u{5d9}\x0b\x51\x03\x51\
	\x03\x51\x07\x51\u{5dd}\x0a\x51\x0c\x51\x0e\x51\u{5e0}\x0b\x51\x03\x51\x03\
	\x51\x03\x52\x07\x52\u{5e5}\x0a\x52\x0c\x52\x0e\x52\u{5e8}\x0b\x52\x03\x52\
	\x03\x52\x07\x52\u{5ec}\x0a\x52\x0c\x52\x0e\x52\u{5ef}\x0b\x52\x03\x52\x03\
	\x52\x03\x53\x03\x53\x03\x53\x03\x53\x03\x54\x03\x54\x03\x54\x03\x54\x03\
	\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\
	\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x05\x54\u{60b}\x0a\
	\x54\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x05\x55\u{614}\
	\x0a\x55\x03\x56\x03\x56\x03\x56\x03\x56\x03\x56\x03\x56\x06\x56\u{61c}\
	\x0a\x56\x0d\x56\x0e\x56\u{61d}\x03\x56\x07\x56\u{621}\x0a\x56\x0c\x56\x0e\
	\x56\u{624}\x0b\x56\x05\x56\u{626}\x0a\x56\x03\x56\x05\x56\u{629}\x0a\x56\
	\x03\x56\x03\x56\x03\x57\x03\x57\x03\x57\x07\x57\u{630}\x0a\x57\x0c\x57\
	\x0e\x57\u{633}\x0b\x57\x03\x57\x03\x57\x03\x57\x03\x58\x03\x58\x07\x58\
	\u{63a}\x0a\x58\x0c\x58\x0e\x58\u{63d}\x0b\x58\x03\x58\x03\x58\x03\x58\x03\
	\x59\x03\x59\x05\x59\u{644}\x0a\x59\x03\x59\x03\x59\x03\x5a\x03\x5a\x03\
	\x5a\x03\x5a\x03\x5a\x03\x5a\x03\x5b\x03\x5b\x03\x5b\x03\x5b\x03\x5b\x03\
	\x5b\x03\x5b\x03\x5b\x03\x5b\x03\x5b\x03\x5c\x03\x5c\x03\x5c\x03\x5c\x03\
	\x5c\x03\x5c\x07\x5c\u{65e}\x0a\x5c\x0c\x5c\x0e\x5c\u{661}\x0b\x5c\x03\x5d\
	\x05\x5d\u{664}\x0a\x5d\x03\x5d\x03\x5d\x03\x5d\x03\x5d\x03\x5e\x03\x5e\
	\x03\x5f\x03\x5f\x03\x5f\x07\x5f\u{66f}\x0a\x5f\x0c\x5f\x0e\x5f\u{672}\x0b\
	\x5f\x03\x60\x03\x60\x03\x60\x03\x60\x03\x61\x03\x61\x03\x61\x03\x61\x03\
	\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\
	\x62\x05\x62\u{686}\x0a\x62\x03\x63\x03\x63\x03\x64\x05\x64\u{68b}\x0a\x64\
	\x03\x64\x03\x64\x05\x64\u{68f}\x0a\x64\x03\x64\x03\x64\x03\x64\x03\x64\
	\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x07\x64\u{69b}\x0a\x64\
	\x0c\x64\x0e\x64\u{69e}\x0b\x64\x03\x64\x03\x64\x05\x64\u{6a2}\x0a\x64\x03\
	\x65\x03\x65\x03\x65\x03\x65\x07\x65\u{6a8}\x0a\x65\x0c\x65\x0e\x65\u{6ab}\
	\x0b\x65\x03\x65\x03\x65\x03\x66\x03\x66\x03\x66\x03\x66\x07\x66\u{6b3}\
	\x0a\x66\x0c\x66\x0e\x66\u{6b6}\x0b\x66\x03\x66\x03\x66\x03\x67\x03\x67\
	\x03\x67\x05\x67\u{6bd}\x0a\x67\x03\x68\x03\x68\x03\x68\x03\x68\x03\x68\
	\x07\x68\u{6c4}\x0a\x68\x0c\x68\x0e\x68\u{6c7}\x0b\x68\x03\x68\x03\x68\x03\
	\x69\x03\x69\x03\x69\x05\x69\u{6ce}\x0a\x69\x03\x69\x03\x69\x03\x69\x03\
	\x69\x03\x69\x03\x69\x07\x69\u{6d6}\x0a\x69\x0c\x69\x0e\x69\u{6d9}\x0b\x69\
	\x03\x69\x03\x69\x03\x69\x02\x05\u{88}\u{92}\u{96}\x6a\x02\x04\x06\x08\x0a\
	\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\
	\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\
	\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\x76\
	\x78\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\u{90}\u{92}\
	\u{94}\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\u{a8}\u{aa}\
	\u{ac}\u{ae}\u{b0}\u{b2}\u{b4}\u{b6}\u{b8}\u{ba}\u{bc}\u{be}\u{c0}\u{c2}\
	\u{c4}\u{c6}\u{c8}\u{ca}\u{cc}\u{ce}\u{d0}\x02\x13\x04\x02\x6c\x6c\x6e\x6e\
	\x03\x02\x13\x13\x04\x02\x1b\x1b\x22\x22\x03\x02\x2e\x2f\x03\x02\x3e\x3e\
	\x04\x02\x0a\x0a\x3e\x40\x03\x02\x41\x42\x03\x02\x43\x44\x03\x02\x45\x48\
	\x03\x02\x49\x4a\x03\x02\x4b\x4e\x03\x02\x4f\x4f\x04\x02\x3d\x3d\x50\x50\
	\x03\x02\x51\x51\x04\x02\x4b\x4f\x52\x55\x03\x02\x57\x58\x03\x02\x6f\x72\
	\x02\u{78d}\x02\u{e5}\x03\x02\x02\x02\x04\u{e7}\x03\x02\x02\x02\x06\u{eb}\
	\x03\x02\x02\x02\x08\u{f0}\x03\x02\x02\x02\x0a\u{f2}\x03\x02\x02\x02\x0c\
	\u{f6}\x03\x02\x02\x02\x0e\u{f8}\x03\x02\x02\x02\x10\u{10d}\x03\x02\x02\
	\x02\x12\u{10f}\x03\x02\x02\x02\x14\u{127}\x03\x02\x02\x02\x16\u{129}\x03\
	\x02\x02\x02\x18\u{12e}\x03\x02\x02\x02\x1a\u{141}\x03\x02\x02\x02\x1c\u{146}\
	\x03\x02\x02\x02\x1e\u{158}\x03\x02\x02\x02\x20\u{163}\x03\x02\x02\x02\x22\
	\u{16e}\x03\x02\x02\x02\x24\u{176}\x03\x02\x02\x02\x26\u{17a}\x03\x02\x02\
	\x02\x28\u{187}\x03\x02\x02\x02\x2a\u{18f}\x03\x02\x02\x02\x2c\u{197}\x03\
	\x02\x02\x02\x2e\u{1ba}\x03\x02\x02\x02\x30\u{1bf}\x03\x02\x02\x02\x32\u{1d5}\
	\x03\x02\x02\x02\x34\u{1f3}\x03\x02\x02\x02\x36\u{201}\x03\x02\x02\x02\x38\
	\u{203}\x03\x02\x02\x02\x3a\u{20d}\x03\x02\x02\x02\x3c\u{21a}\x03\x02\x02\
	\x02\x3e\u{229}\x03\x02\x02\x02\x40\u{236}\x03\x02\x02\x02\x42\u{249}\x03\
	\x02\x02\x02\x44\u{24e}\x03\x02\x02\x02\x46\u{262}\x03\x02\x02\x02\x48\u{277}\
	\x03\x02\x02\x02\x4a\u{283}\x03\x02\x02\x02\x4c\u{29b}\x03\x02\x02\x02\x4e\
	\u{29d}\x03\x02\x02\x02\x50\u{2a9}\x03\x02\x02\x02\x52\u{2b9}\x03\x02\x02\
	\x02\x54\u{2be}\x03\x02\x02\x02\x56\u{2ce}\x03\x02\x02\x02\x58\u{2d3}\x03\
	\x02\x02\x02\x5a\u{2f5}\x03\x02\x02\x02\x5c\u{2fa}\x03\x02\x02\x02\x5e\u{33c}\
	\x03\x02\x02\x02\x60\u{33e}\x03\x02\x02\x02\x62\u{353}\x03\x02\x02\x02\x64\
	\u{35a}\x03\x02\x02\x02\x66\u{38d}\x03\x02\x02\x02\x68\u{38f}\x03\x02\x02\
	\x02\x6a\u{3a2}\x03\x02\x02\x02\x6c\u{3a4}\x03\x02\x02\x02\x6e\u{3c1}\x03\
	\x02\x02\x02\x70\u{3c6}\x03\x02\x02\x02\x72\u{3dd}\x03\x02\x02\x02\x74\u{3fb}\
	\x03\x02\x02\x02\x76\u{3fd}\x03\x02\x02\x02\x78\u{40c}\x03\x02\x02\x02\x7a\
	\u{41d}\x03\x02\x02\x02\x7c\u{425}\x03\x02\x02\x02\x7e\u{448}\x03\x02\x02\
	\x02\u{80}\u{461}\x03\x02\x02\x02\u{82}\u{478}\x03\x02\x02\x02\u{84}\u{484}\
	\x03\x02\x02\x02\u{86}\u{486}\x03\x02\x02\x02\u{88}\u{4a1}\x03\x02\x02\x02\
	\u{8a}\u{4b7}\x03\x02\x02\x02\u{8c}\u{4c2}\x03\x02\x02\x02\u{8e}\u{4ce}\
	\x03\x02\x02\x02\u{90}\u{4d3}\x03\x02\x02\x02\u{92}\u{4d6}\x03\x02\x02\x02\
	\u{94}\u{501}\x03\x02\x02\x02\u{96}\u{588}\x03\x02\x02\x02\u{98}\u{5ac}\
	\x03\x02\x02\x02\u{9a}\u{5b4}\x03\x02\x02\x02\u{9c}\u{5bc}\x03\x02\x02\x02\
	\u{9e}\u{5c1}\x03\x02\x02\x02\u{a0}\u{5d7}\x03\x02\x02\x02\u{a2}\u{5e6}\
	\x03\x02\x02\x02\u{a4}\u{5f2}\x03\x02\x02\x02\u{a6}\u{60a}\x03\x02\x02\x02\
	\u{a8}\u{60c}\x03\x02\x02\x02\u{aa}\u{615}\x03\x02\x02\x02\u{ac}\u{62c}\
	\x03\x02\x02\x02\u{ae}\u{637}\x03\x02\x02\x02\u{b0}\u{641}\x03\x02\x02\x02\
	\u{b2}\u{647}\x03\x02\x02\x02\u{b4}\u{64d}\x03\x02\x02\x02\u{b6}\u{657}\
	\x03\x02\x02\x02\u{b8}\u{663}\x03\x02\x02\x02\u{ba}\u{669}\x03\x02\x02\x02\
	\u{bc}\u{66b}\x03\x02\x02\x02\u{be}\u{673}\x03\x02\x02\x02\u{c0}\u{677}\
	\x03\x02\x02\x02\u{c2}\u{685}\x03\x02\x02\x02\u{c4}\u{687}\x03\x02\x02\x02\
	\u{c6}\u{6a1}\x03\x02\x02\x02\u{c8}\u{6a3}\x03\x02\x02\x02\u{ca}\u{6ae}\
	\x03\x02\x02\x02\u{cc}\u{6b9}\x03\x02\x02\x02\u{ce}\u{6be}\x03\x02\x02\x02\
	\u{d0}\u{6cd}\x03\x02\x02\x02\u{d2}\u{d6}\x05\x04\x03\x02\u{d3}\u{d5}\x05\
	\x14\x0b\x02\u{d4}\u{d3}\x03\x02\x02\x02\u{d5}\u{d8}\x03\x02\x02\x02\u{d6}\
	\u{d4}\x03\x02\x02\x02\u{d6}\u{d7}\x03\x02\x02\x02\u{d7}\u{da}\x03\x02\x02\
	\x02\u{d8}\u{d6}\x03\x02\x02\x02\u{d9}\u{db}\x05\x06\x04\x02\u{da}\u{d9}\
	\x03\x02\x02\x02\u{da}\u{db}\x03\x02\x02\x02\u{db}\u{dc}\x03\x02\x02\x02\
	\u{dc}\u{dd}\x07\x02\x02\x03\u{dd}\u{e6}\x03\x02\x02\x02\u{de}\u{e0}\x05\
	\x14\x0b\x02\u{df}\u{de}\x03\x02\x02\x02\u{e0}\u{e3}\x03\x02\x02\x02\u{e1}\
	\u{df}\x03\x02\x02\x02\u{e1}\u{e2}\x03\x02\x02\x02\u{e2}\u{e4}\x03\x02\x02\
	\x02\u{e3}\u{e1}\x03\x02\x02\x02\u{e4}\u{e6}\x07\x02\x02\x03\u{e5}\u{d2}\
	\x03\x02\x02\x02\u{e5}\u{e1}\x03\x02\x02\x02\u{e6}\x03\x03\x02\x02\x02\u{e7}\
	\u{e8}\x07\x03\x02\x02\u{e8}\u{e9}\x05\x16\x0c\x02\u{e9}\u{ea}\x07\x04\x02\
	\x02\u{ea}\x05\x03\x02\x02\x02\u{eb}\u{ee}\x07\x05\x02\x02\u{ec}\u{ed}\x07\
	\x06\x02\x02\u{ed}\u{ef}\x05\x16\x0c\x02\u{ee}\u{ec}\x03\x02\x02\x02\u{ee}\
	\u{ef}\x03\x02\x02\x02\u{ef}\x07\x03\x02\x02\x02\u{f0}\u{f1}\x09\x02\x02\
	\x02\u{f1}\x09\x03\x02\x02\x02\u{f2}\u{f3}\x07\x6b\x02\x02\u{f3}\x0b\x03\
	\x02\x02\x02\u{f4}\u{f7}\x05\x08\x05\x02\u{f5}\u{f7}\x05\x0a\x06\x02\u{f6}\
	\u{f4}\x03\x02\x02\x02\u{f6}\u{f5}\x03\x02\x02\x02\u{f7}\x0d\x03\x02\x02\
	\x02\u{f8}\u{f9}\x07\x07\x02\x02\u{f9}\u{fe}\x05\x10\x09\x02\u{fa}\u{fb}\
	\x07\x08\x02\x02\u{fb}\u{fd}\x05\x10\x09\x02\u{fc}\u{fa}\x03\x02\x02\x02\
	\u{fd}\u{100}\x03\x02\x02\x02\u{fe}\u{fc}\x03\x02\x02\x02\u{fe}\u{ff}\x03\
	\x02\x02\x02\u{ff}\u{101}\x03\x02\x02\x02\u{100}\u{fe}\x03\x02\x02\x02\u{101}\
	\u{102}\x07\x04\x02\x02\u{102}\x0f\x03\x02\x02\x02\u{103}\u{104}\x05\x16\
	\x0c\x02\u{104}\u{105}\x07\x09\x02\x02\u{105}\u{106}\x07\x0a\x02\x02\u{106}\
	\u{10e}\x03\x02\x02\x02\u{107}\u{10b}\x05\x0c\x07\x02\u{108}\u{109}\x07\
	\x0b\x02\x02\u{109}\u{10a}\x07\x0c\x02\x02\u{10a}\u{10c}\x07\x0d\x02\x02\
	\u{10b}\u{108}\x03\x02\x02\x02\u{10b}\u{10c}\x03\x02\x02\x02\u{10c}\u{10e}\
	\x03\x02\x02\x02\u{10d}\u{103}\x03\x02\x02\x02\u{10d}\u{107}\x03\x02\x02\
	\x02\u{10e}\x11\x03\x02\x02\x02\u{10f}\u{113}\x07\x0e\x02\x02\u{110}\u{111}\
	\x05\x0a\x06\x02\u{111}\u{112}\x07\x09\x02\x02\u{112}\u{114}\x03\x02\x02\
	\x02\u{113}\u{110}\x03\x02\x02\x02\u{114}\u{115}\x03\x02\x02\x02\u{115}\
	\u{113}\x03\x02\x02\x02\u{115}\u{116}\x03\x02\x02\x02\u{116}\u{117}\x03\
	\x02\x02\x02\u{117}\u{118}\x07\x0a\x02\x02\u{118}\u{119}\x07\x04\x02\x02\
	\u{119}\x13\x03\x02\x02\x02\u{11a}\u{128}\x05\x18\x0d\x02\u{11b}\u{128}\
	\x05\x2a\x16\x02\u{11c}\u{128}\x05\x2c\x17\x02\u{11d}\u{128}\x05\x30\x19\
	\x02\u{11e}\u{128}\x05\x32\x1a\x02\u{11f}\u{128}\x05\x4a\x26\x02\u{120}\
	\u{128}\x05\x58\x2d\x02\u{121}\u{128}\x05\x78\x3d\x02\u{122}\u{128}\x05\
	\x44\x23\x02\u{123}\u{128}\x05\x74\x3b\x02\u{124}\u{128}\x05\x5c\x2f\x02\
	\u{125}\u{128}\x05\x12\x0a\x02\u{126}\u{128}\x05\x0e\x08\x02\u{127}\u{11a}\
	\x03\x02\x02\x02\u{127}\u{11b}\x03\x02\x02\x02\u{127}\u{11c}\x03\x02\x02\
	\x02\u{127}\u{11d}\x03\x02\x02\x02\u{127}\u{11e}\x03\x02\x02\x02\u{127}\
	\u{11f}\x03\x02\x02\x02\u{127}\u{120}\x03\x02\x02\x02\u{127}\u{121}\x03\
	\x02\x02\x02\u{127}\u{122}\x03\x02\x02\x02\u{127}\u{123}\x03\x02\x02\x02\
	\u{127}\u{124}\x03\x02\x02\x02\u{127}\u{125}\x03\x02\x02\x02\u{127}\u{126}\
	\x03\x02\x02\x02\u{128}\x15\x03\x02\x02\x02\u{129}\u{12a}\x05\x0a\x06\x02\
	\u{12a}\x17\x03\x02\x02\x02\u{12b}\u{12d}\x05\u{ca}\x66\x02\u{12c}\u{12b}\
	\x03\x02\x02\x02\u{12d}\u{130}\x03\x02\x02\x02\u{12e}\u{12c}\x03\x02\x02\
	\x02\u{12e}\u{12f}\x03\x02\x02\x02\u{12f}\u{131}\x03\x02\x02\x02\u{130}\
	\u{12e}\x03\x02\x02\x02\u{131}\u{132}\x07\x0f\x02\x02\u{132}\u{133}\x05\
	\x24\x13\x02\u{133}\u{137}\x07\x04\x02\x02\u{134}\u{136}\x05\x1a\x0e\x02\
	\u{135}\u{134}\x03\x02\x02\x02\u{136}\u{139}\x03\x02\x02\x02\u{137}\u{135}\
	\x03\x02\x02\x02\u{137}\u{138}\x03\x02\x02\x02\u{138}\u{13a}\x03\x02\x02\
	\x02\u{139}\u{137}\x03\x02\x02\x02\u{13a}\u{13d}\x07\x10\x02\x02\u{13b}\
	\u{13c}\x07\x06\x02\x02\u{13c}\u{13e}\x05\u{84}\x43\x02\u{13d}\u{13b}\x03\
	\x02\x02\x02\u{13d}\u{13e}\x03\x02\x02\x02\u{13e}\x19\x03\x02\x02\x02\u{13f}\
	\u{142}\x05\x1c\x0f\x02\u{140}\u{142}\x05\x22\x12\x02\u{141}\u{13f}\x03\
	\x02\x02\x02\u{141}\u{140}\x03\x02\x02\x02\u{142}\x1b\x03\x02\x02\x02\u{143}\
	\u{145}\x05\u{ca}\x66\x02\u{144}\u{143}\x03\x02\x02\x02\u{145}\u{148}\x03\
	\x02\x02\x02\u{146}\u{144}\x03\x02\x02\x02\u{146}\u{147}\x03\x02\x02\x02\
	\u{147}\u{149}\x03\x02\x02\x02\u{148}\u{146}\x03\x02\x02\x02\u{149}\u{14a}\
	\x07\x11\x02\x02\u{14a}\u{14b}\x05\u{82}\x42\x02\u{14b}\u{151}\x05\x08\x05\
	\x02\u{14c}\u{14e}\x07\x0b\x02\x02\u{14d}\u{14f}\x05\x1e\x10\x02\u{14e}\
	\u{14d}\x03\x02\x02\x02\u{14e}\u{14f}\x03\x02\x02\x02\u{14f}\u{150}\x03\
	\x02\x02\x02\u{150}\u{152}\x07\x0d\x02\x02\u{151}\u{14c}\x03\x02\x02\x02\
	\u{151}\u{152}\x03\x02\x02\x02\u{152}\u{154}\x03\x02\x02\x02\u{153}\u{155}\
	\x05\u{ce}\x68\x02\u{154}\u{153}\x03\x02\x02\x02\u{154}\u{155}\x03\x02\x02\
	\x02\u{155}\u{156}\x03\x02\x02\x02\u{156}\u{157}\x07\x04\x02\x02\u{157}\
	\x1d\x03\x02\x02\x02\u{158}\u{15d}\x05\x20\x11\x02\u{159}\u{15a}\x07\x08\
	\x02\x02\u{15a}\u{15c}\x05\x20\x11\x02\u{15b}\u{159}\x03\x02\x02\x02\u{15c}\
	\u{15f}\x03\x02\x02\x02\u{15d}\u{15b}\x03\x02\x02\x02\u{15d}\u{15e}\x03\
	\x02\x02\x02\u{15e}\x1f\x03\x02\x02\x02\u{15f}\u{15d}\x03\x02\x02\x02\u{160}\
	\u{162}\x05\u{ca}\x66\x02\u{161}\u{160}\x03\x02\x02\x02\u{162}\u{165}\x03\
	\x02\x02\x02\u{163}\u{161}\x03\x02\x02\x02\u{163}\u{164}\x03\x02\x02\x02\
	\u{164}\u{167}\x03\x02\x02\x02\u{165}\u{163}\x03\x02\x02\x02\u{166}\u{168}\
	\x05\u{82}\x42\x02\u{167}\u{166}\x03\x02\x02\x02\u{167}\u{168}\x03\x02\x02\
	\x02\u{168}\u{169}\x03\x02\x02\x02\u{169}\u{16a}\x05\x08\x05\x02\u{16a}\
	\x21\x03\x02\x02\x02\u{16b}\u{16d}\x05\u{ca}\x66\x02\u{16c}\u{16b}\x03\x02\
	\x02\x02\u{16d}\u{170}\x03\x02\x02\x02\u{16e}\u{16c}\x03\x02\x02\x02\u{16e}\
	\u{16f}\x03\x02\x02\x02\u{16f}\u{171}\x03\x02\x02\x02\u{170}\u{16e}\x03\
	\x02\x02\x02\u{171}\u{172}\x07\x0f\x02\x02\u{172}\u{173}\x05\u{82}\x42\x02\
	\u{173}\u{174}\x05\x08\x05\x02\u{174}\u{175}\x07\x04\x02\x02\u{175}\x23\
	\x03\x02\x02\x02\u{176}\u{178}\x05\u{84}\x43\x02\u{177}\u{179}\x05\x26\x14\
	\x02\u{178}\u{177}\x03\x02\x02\x02\u{178}\u{179}\x03\x02\x02\x02\u{179}\
	\x25\x03\x02\x02\x02\u{17a}\u{17b}\x07\x12\x02\x02\u{17b}\u{17c}\x07\x0b\
	\x02\x02\u{17c}\u{181}\x05\x28\x15\x02\u{17d}\u{17e}\x07\x08\x02\x02\u{17e}\
	\u{180}\x05\x28\x15\x02\u{17f}\u{17d}\x03\x02\x02\x02\u{180}\u{183}\x03\
	\x02\x02\x02\u{181}\u{17f}\x03\x02\x02\x02\u{181}\u{182}\x03\x02\x02\x02\
	\u{182}\u{184}\x03\x02\x02\x02\u{183}\u{181}\x03\x02\x02\x02\u{184}\u{185}\
	\x07\x0d\x02\x02\u{185}\x27\x03\x02\x02\x02\u{186}\u{188}\x09\x03\x02\x02\
	\u{187}\u{186}\x03\x02\x02\x02\u{187}\u{188}\x03\x02\x02\x02\u{188}\u{189}\
	\x03\x02\x02\x02\u{189}\u{18a}\x07\x14\x02\x02\u{18a}\u{18b}\x05\u{84}\x43\
	\x02\u{18b}\x29\x03\x02\x02\x02\u{18c}\u{18e}\x05\u{ca}\x66\x02\u{18d}\u{18c}\
	\x03\x02\x02\x02\u{18e}\u{191}\x03\x02\x02\x02\u{18f}\u{18d}\x03\x02\x02\
	\x02\u{18f}\u{190}\x03\x02\x02\x02\u{190}\u{192}\x03\x02\x02\x02\u{191}\
	\u{18f}\x03\x02\x02\x02\u{192}\u{193}\x07\x15\x02\x02\u{193}\u{194}\x05\
	\u{82}\x42\x02\u{194}\u{195}\x05\x24\x13\x02\u{195}\u{196}\x07\x04\x02\x02\
	\u{196}\x2b\x03\x02\x02\x02\u{197}\u{198}\x07\x15\x02\x02\u{198}\u{199}\
	\x07\x16\x02\x02\u{199}\u{19a}\x07\x17\x02\x02\u{19a}\u{19f}\x05\x2e\x18\
	\x02\u{19b}\u{19c}\x07\x08\x02\x02\u{19c}\u{19e}\x05\x2e\x18\x02\u{19d}\
	\u{19b}\x03\x02\x02\x02\u{19e}\u{1a1}\x03\x02\x02\x02\u{19f}\u{19d}\x03\
	\x02\x02\x02\u{19f}\u{1a0}\x03\x02\x02\x02\u{1a0}\u{1a2}\x03\x02\x02\x02\
	\u{1a1}\u{19f}\x03\x02\x02\x02\u{1a2}\u{1a3}\x07\x18\x02\x02\u{1a3}\u{1a5}\
	\x05\x0a\x06\x02\u{1a4}\u{1a6}\x05\x3c\x1f\x02\u{1a5}\u{1a4}\x03\x02\x02\
	\x02\u{1a5}\u{1a6}\x03\x02\x02\x02\u{1a6}\u{1a7}\x03\x02\x02\x02\u{1a7}\
	\u{1a8}\x07\x04\x02\x02\u{1a8}\x2d\x03\x02\x02\x02\u{1a9}\u{1aa}\x05\x0a\
	\x06\x02\u{1aa}\u{1ab}\x07\x19\x02\x02\u{1ab}\u{1ae}\x07\x6f\x02\x02\u{1ac}\
	\u{1ad}\x07\x06\x02\x02\u{1ad}\u{1af}\x07\x6f\x02\x02\u{1ae}\u{1ac}\x03\
	\x02\x02\x02\u{1ae}\u{1af}\x03\x02\x02\x02\u{1af}\u{1b0}\x03\x02\x02\x02\
	\u{1b0}\u{1b3}\x07\x1a\x02\x02\u{1b1}\u{1b2}\x07\x1b\x02\x02\u{1b2}\u{1b4}\
	\x07\x6f\x02\x02\u{1b3}\u{1b1}\x03\x02\x02\x02\u{1b3}\u{1b4}\x03\x02\x02\
	\x02\u{1b4}\u{1bb}\x03\x02\x02\x02\u{1b5}\u{1b8}\x05\x0a\x06\x02\u{1b6}\
	\u{1b7}\x07\x1b\x02\x02\u{1b7}\u{1b9}\x07\x6f\x02\x02\u{1b8}\u{1b6}\x03\
	\x02\x02\x02\u{1b8}\u{1b9}\x03\x02\x02\x02\u{1b9}\u{1bb}\x03\x02\x02\x02\
	\u{1ba}\u{1a9}\x03\x02\x02\x02\u{1ba}\u{1b5}\x03\x02\x02\x02\u{1bb}\x2f\
	\x03\x02\x02\x02\u{1bc}\u{1be}\x05\u{ca}\x66\x02\u{1bd}\u{1bc}\x03\x02\x02\
	\x02\u{1be}\u{1c1}\x03\x02\x02\x02\u{1bf}\u{1bd}\x03\x02\x02\x02\u{1bf}\
	\u{1c0}\x03\x02\x02\x02\u{1c0}\u{1c2}\x03\x02\x02\x02\u{1c1}\u{1bf}\x03\
	\x02\x02\x02\u{1c2}\u{1c3}\x07\x15\x02\x02\u{1c3}\u{1c4}\x07\x1c\x02\x02\
	\u{1c4}\u{1c8}\x07\x17\x02\x02\u{1c5}\u{1c7}\x05\x34\x1b\x02\u{1c6}\u{1c5}\
	\x03\x02\x02\x02\u{1c7}\u{1ca}\x03\x02\x02\x02\u{1c8}\u{1c6}\x03\x02\x02\
	\x02\u{1c8}\u{1c9}\x03\x02\x02\x02\u{1c9}\u{1cb}\x03\x02\x02\x02\u{1ca}\
	\u{1c8}\x03\x02\x02\x02\u{1cb}\u{1cc}\x07\x18\x02\x02\u{1cc}\u{1ce}\x05\
	\x24\x13\x02\u{1cd}\u{1cf}\x05\x3c\x1f\x02\u{1ce}\u{1cd}\x03\x02\x02\x02\
	\u{1ce}\u{1cf}\x03\x02\x02\x02\u{1cf}\u{1d0}\x03\x02\x02\x02\u{1d0}\u{1d1}\
	\x07\x04\x02\x02\u{1d1}\x31\x03\x02\x02\x02\u{1d2}\u{1d4}\x05\u{ca}\x66\
	\x02\u{1d3}\u{1d2}\x03\x02\x02\x02\u{1d4}\u{1d7}\x03\x02\x02\x02\u{1d5}\
	\u{1d3}\x03\x02\x02\x02\u{1d5}\u{1d6}\x03\x02\x02\x02\u{1d6}\u{1d8}\x03\
	\x02\x02\x02\u{1d7}\u{1d5}\x03\x02\x02\x02\u{1d8}\u{1d9}\x07\x15\x02\x02\
	\u{1d9}\u{1db}\x07\x1d\x02\x02\u{1da}\u{1dc}\x07\x1e\x02\x02\u{1db}\u{1da}\
	\x03\x02\x02\x02\u{1db}\u{1dc}\x03\x02\x02\x02\u{1dc}\u{1dd}\x03\x02\x02\
	\x02\u{1dd}\u{1e1}\x07\x17\x02\x02\u{1de}\u{1e0}\x05\x36\x1c\x02\u{1df}\
	\u{1de}\x03\x02\x02\x02\u{1e0}\u{1e3}\x03\x02\x02\x02\u{1e1}\u{1df}\x03\
	\x02\x02\x02\u{1e1}\u{1e2}\x03\x02\x02\x02\u{1e2}\u{1e4}\x03\x02\x02\x02\
	\u{1e3}\u{1e1}\x03\x02\x02\x02\u{1e4}\u{1e5}\x07\x18\x02\x02\u{1e5}\u{1e7}\
	\x05\x24\x13\x02\u{1e6}\u{1e8}\x05\x3c\x1f\x02\u{1e7}\u{1e6}\x03\x02\x02\
	\x02\u{1e7}\u{1e8}\x03\x02\x02\x02\u{1e8}\u{1e9}\x03\x02\x02\x02\u{1e9}\
	\u{1ea}\x07\x04\x02\x02\u{1ea}\x33\x03\x02\x02\x02\u{1eb}\u{1ec}\x05\u{82}\
	\x42\x02\u{1ec}\u{1ed}\x05\x08\x05\x02\u{1ed}\u{1ee}\x07\x04\x02\x02\u{1ee}\
	\u{1f4}\x03\x02\x02\x02\u{1ef}\u{1f0}\x05\x3a\x1e\x02\u{1f0}\u{1f1}\x05\
	\x08\x05\x02\u{1f1}\u{1f2}\x07\x04\x02\x02\u{1f2}\u{1f4}\x03\x02\x02\x02\
	\u{1f3}\u{1eb}\x03\x02\x02\x02\u{1f3}\u{1ef}\x03\x02\x02\x02\u{1f4}\x35\
	\x03\x02\x02\x02\u{1f5}\u{1f6}\x05\u{82}\x42\x02\u{1f6}\u{1f7}\x05\x0a\x06\
	\x02\u{1f7}\u{1f8}\x07\x04\x02\x02\u{1f8}\u{202}\x03\x02\x02\x02\u{1f9}\
	\u{1fa}\x05\x38\x1d\x02\u{1fa}\u{1fb}\x05\x0a\x06\x02\u{1fb}\u{1fc}\x07\
	\x04\x02\x02\u{1fc}\u{202}\x03\x02\x02\x02\u{1fd}\u{1fe}\x05\x3a\x1e\x02\
	\u{1fe}\u{1ff}\x05\x0a\x06\x02\u{1ff}\u{200}\x07\x04\x02\x02\u{200}\u{202}\
	\x03\x02\x02\x02\u{201}\u{1f5}\x03\x02\x02\x02\u{201}\u{1f9}\x03\x02\x02\
	\x02\u{201}\u{1fd}\x03\x02\x02\x02\u{202}\x37\x03\x02\x02\x02\u{203}\u{204}\
	\x07\x1c\x02\x02\u{204}\u{208}\x07\x17\x02\x02\u{205}\u{207}\x05\x34\x1b\
	\x02\u{206}\u{205}\x03\x02\x02\x02\u{207}\u{20a}\x03\x02\x02\x02\u{208}\
	\u{206}\x03\x02\x02\x02\u{208}\u{209}\x03\x02\x02\x02\u{209}\u{20b}\x03\
	\x02\x02\x02\u{20a}\u{208}\x03\x02\x02\x02\u{20b}\u{20c}\x07\x18\x02\x02\
	\u{20c}\x39\x03\x02\x02\x02\u{20d}\u{20f}\x07\x1d\x02\x02\u{20e}\u{210}\
	\x07\x1e\x02\x02\u{20f}\u{20e}\x03\x02\x02\x02\u{20f}\u{210}\x03\x02\x02\
	\x02\u{210}\u{211}\x03\x02\x02\x02\u{211}\u{215}\x07\x17\x02\x02\u{212}\
	\u{214}\x05\x36\x1c\x02\u{213}\u{212}\x03\x02\x02\x02\u{214}\u{217}\x03\
	\x02\x02\x02\u{215}\u{213}\x03\x02\x02\x02\u{215}\u{216}\x03\x02\x02\x02\
	\u{216}\u{218}\x03\x02\x02\x02\u{217}\u{215}\x03\x02\x02\x02\u{218}\u{219}\
	\x07\x18\x02\x02\u{219}\x3b\x03\x02\x02\x02\u{21a}\u{21b}\x07\x1f\x02\x02\
	\u{21b}\u{21c}\x07\x0b\x02\x02\u{21c}\u{221}\x05\u{84}\x43\x02\u{21d}\u{21e}\
	\x07\x08\x02\x02\u{21e}\u{220}\x05\u{84}\x43\x02\u{21f}\u{21d}\x03\x02\x02\
	\x02\u{220}\u{223}\x03\x02\x02\x02\u{221}\u{21f}\x03\x02\x02\x02\u{221}\
	\u{222}\x03\x02\x02\x02\u{222}\u{224}\x03\x02\x02\x02\u{223}\u{221}\x03\
	\x02\x02\x02\u{224}\u{225}\x07\x0d\x02\x02\u{225}\x3d\x03\x02\x02\x02\u{226}\
	\u{228}\x05\u{ca}\x66\x02\u{227}\u{226}\x03\x02\x02\x02\u{228}\u{22b}\x03\
	\x02\x02\x02\u{229}\u{227}\x03\x02\x02\x02\u{229}\u{22a}\x03\x02\x02\x02\
	\u{22a}\u{22e}\x03\x02\x02\x02\u{22b}\u{229}\x03\x02\x02\x02\u{22c}\u{22f}\
	\x07\x20\x02\x02\u{22d}\u{22f}\x05\u{82}\x42\x02\u{22e}\u{22c}\x03\x02\x02\
	\x02\u{22e}\u{22d}\x03\x02\x02\x02\u{22f}\u{230}\x03\x02\x02\x02\u{230}\
	\u{231}\x05\x08\x05\x02\u{231}\u{232}\x07\x1b\x02\x02\u{232}\u{233}\x07\
	\x21\x02\x02\u{233}\u{234}\x05\u{88}\x45\x02\u{234}\u{235}\x07\x04\x02\x02\
	\u{235}\x3f\x03\x02\x02\x02\u{236}\u{237}\x07\x17\x02\x02\u{237}\u{23a}\
	\x05\x08\x05\x02\u{238}\u{239}\x07\x08\x02\x02\u{239}\u{23b}\x05\x08\x05\
	\x02\u{23a}\u{238}\x03\x02\x02\x02\u{23b}\u{23c}\x03\x02\x02\x02\u{23c}\
	\u{23a}\x03\x02\x02\x02\u{23c}\u{23d}\x03\x02\x02\x02\u{23d}\u{23e}\x03\
	\x02\x02\x02\u{23e}\u{23f}\x07\x18\x02\x02\u{23f}\x41\x03\x02\x02\x02\u{240}\
	\u{243}\x05\x08\x05\x02\u{241}\u{242}\x07\x1b\x02\x02\u{242}\u{244}\x05\
	\u{88}\x45\x02\u{243}\u{241}\x03\x02\x02\x02\u{243}\u{244}\x03\x02\x02\x02\
	\u{244}\u{24a}\x03\x02\x02\x02\u{245}\u{246}\x05\x40\x21\x02\u{246}\u{247}\
	\x07\x1b\x02\x02\u{247}\u{248}\x05\u{88}\x45\x02\u{248}\u{24a}\x03\x02\x02\
	\x02\u{249}\u{240}\x03\x02\x02\x02\u{249}\u{245}\x03\x02\x02\x02\u{24a}\
	\x43\x03\x02\x02\x02\u{24b}\u{24d}\x05\u{ca}\x66\x02\u{24c}\u{24b}\x03\x02\
	\x02\x02\u{24d}\u{250}\x03\x02\x02\x02\u{24e}\u{24c}\x03\x02\x02\x02\u{24e}\
	\u{24f}\x03\x02\x02\x02\u{24f}\u{253}\x03\x02\x02\x02\u{250}\u{24e}\x03\
	\x02\x02\x02\u{251}\u{254}\x07\x20\x02\x02\u{252}\u{254}\x05\u{82}\x42\x02\
	\u{253}\u{251}\x03\x02\x02\x02\u{253}\u{252}\x03\x02\x02\x02\u{254}\u{255}\
	\x03\x02\x02\x02\u{255}\u{25a}\x05\x42\x22\x02\u{256}\u{257}\x07\x08\x02\
	\x02\u{257}\u{259}\x05\x42\x22\x02\u{258}\u{256}\x03\x02\x02\x02\u{259}\
	\u{25c}\x03\x02\x02\x02\u{25a}\u{258}\x03\x02\x02\x02\u{25a}\u{25b}\x03\
	\x02\x02\x02\u{25b}\u{25d}\x03\x02\x02\x02\u{25c}\u{25a}\x03\x02\x02\x02\
	\u{25d}\u{25e}\x07\x04\x02\x02\u{25e}\x45\x03\x02\x02\x02\u{25f}\u{261}\
	\x05\u{ca}\x66\x02\u{260}\u{25f}\x03\x02\x02\x02\u{261}\u{264}\x03\x02\x02\
	\x02\u{262}\u{260}\x03\x02\x02\x02\u{262}\u{263}\x03\x02\x02\x02\u{263}\
	\u{267}\x03\x02\x02\x02\u{264}\u{262}\x03\x02\x02\x02\u{265}\u{268}\x07\
	\x20\x02\x02\u{266}\u{268}\x05\u{82}\x42\x02\u{267}\u{265}\x03\x02\x02\x02\
	\u{267}\u{266}\x03\x02\x02\x02\u{268}\u{269}\x03\x02\x02\x02\u{269}\u{26e}\
	\x05\x08\x05\x02\u{26a}\u{26b}\x07\x19\x02\x02\u{26b}\u{26c}\x05\u{88}\x45\
	\x02\u{26c}\u{26d}\x07\x1a\x02\x02\u{26d}\u{26f}\x03\x02\x02\x02\u{26e}\
	\u{26a}\x03\x02\x02\x02\u{26e}\u{26f}\x03\x02\x02\x02\u{26f}\u{270}\x03\
	\x02\x02\x02\u{270}\u{271}\x07\x22\x02\x02\u{271}\u{272}\x05\u{88}\x45\x02\
	\u{272}\u{273}\x07\x04\x02\x02\u{273}\x47\x03\x02\x02\x02\u{274}\u{276}\
	\x05\u{ca}\x66\x02\u{275}\u{274}\x03\x02\x02\x02\u{276}\u{279}\x03\x02\x02\
	\x02\u{277}\u{275}\x03\x02\x02\x02\u{277}\u{278}\x03\x02\x02\x02\u{278}\
	\u{27a}\x03\x02\x02\x02\u{279}\u{277}\x03\x02\x02\x02\u{27a}\u{27b}\x07\
	\x23\x02\x02\u{27b}\u{27c}\x05\u{c2}\x62\x02\u{27c}\u{27d}\x09\x04\x02\x02\
	\u{27d}\u{27e}\x05\u{88}\x45\x02\u{27e}\u{27f}\x07\x04\x02\x02\u{27f}\x49\
	\x03\x02\x02\x02\u{280}\u{282}\x05\u{ca}\x66\x02\u{281}\u{280}\x03\x02\x02\
	\x02\u{282}\u{285}\x03\x02\x02\x02\u{283}\u{281}\x03\x02\x02\x02\u{283}\
	\u{284}\x03\x02\x02\x02\u{284}\u{286}\x03\x02\x02\x02\u{285}\u{283}\x03\
	\x02\x02\x02\u{286}\u{287}\x07\x24\x02\x02\u{287}\u{288}\x05\x4c\x27\x02\
	\u{288}\u{28a}\x05\x26\x14\x02\u{289}\u{28b}\x05\u{ce}\x68\x02\u{28a}\u{289}\
	\x03\x02\x02\x02\u{28a}\u{28b}\x03\x02\x02\x02\u{28b}\u{28d}\x03\x02\x02\
	\x02\u{28c}\u{28e}\x05\x4e\x28\x02\u{28d}\u{28c}\x03\x02\x02\x02\u{28d}\
	\u{28e}\x03\x02\x02\x02\u{28e}\u{28f}\x03\x02\x02\x02\u{28f}\u{293}\x07\
	\x04\x02\x02\u{290}\u{292}\x05\x54\x2b\x02\u{291}\u{290}\x03\x02\x02\x02\
	\u{292}\u{295}\x03\x02\x02\x02\u{293}\u{291}\x03\x02\x02\x02\u{293}\u{294}\
	\x03\x02\x02\x02\u{294}\u{296}\x03\x02\x02\x02\u{295}\u{293}\x03\x02\x02\
	\x02\u{296}\u{299}\x07\x25\x02\x02\u{297}\u{298}\x07\x06\x02\x02\u{298}\
	\u{29a}\x05\x4c\x27\x02\u{299}\u{297}\x03\x02\x02\x02\u{299}\u{29a}\x03\
	\x02\x02\x02\u{29a}\x4b\x03\x02\x02\x02\u{29b}\u{29c}\x05\x0a\x06\x02\u{29c}\
	\x4d\x03\x02\x02\x02\u{29d}\u{29e}\x07\x26\x02\x02\u{29e}\u{29f}\x07\x0b\
	\x02\x02\u{29f}\u{2a4}\x05\x50\x29\x02\u{2a0}\u{2a1}\x07\x08\x02\x02\u{2a1}\
	\u{2a3}\x05\x50\x29\x02\u{2a2}\u{2a0}\x03\x02\x02\x02\u{2a3}\u{2a6}\x03\
	\x02\x02\x02\u{2a4}\u{2a2}\x03\x02\x02\x02\u{2a4}\u{2a5}\x03\x02\x02\x02\
	\u{2a5}\u{2a7}\x03\x02\x02\x02\u{2a6}\u{2a4}\x03\x02\x02\x02\u{2a7}\u{2a8}\
	\x07\x0d\x02\x02\u{2a8}\x4f\x03\x02\x02\x02\u{2a9}\u{2aa}\x05\x52\x2a\x02\
	\u{2aa}\u{2ab}\x07\x27\x02\x02\u{2ab}\u{2ac}\x05\x52\x2a\x02\u{2ac}\x51\
	\x03\x02\x02\x02\u{2ad}\u{2ba}\x05\u{84}\x43\x02\u{2ae}\u{2af}\x07\x0b\x02\
	\x02\u{2af}\u{2b4}\x05\u{84}\x43\x02\u{2b0}\u{2b1}\x07\x08\x02\x02\u{2b1}\
	\u{2b3}\x05\u{84}\x43\x02\u{2b2}\u{2b0}\x03\x02\x02\x02\u{2b3}\u{2b6}\x03\
	\x02\x02\x02\u{2b4}\u{2b2}\x03\x02\x02\x02\u{2b4}\u{2b5}\x03\x02\x02\x02\
	\u{2b5}\u{2b7}\x03\x02\x02\x02\u{2b6}\u{2b4}\x03\x02\x02\x02\u{2b7}\u{2b8}\
	\x07\x0d\x02\x02\u{2b8}\u{2ba}\x03\x02\x02\x02\u{2b9}\u{2ad}\x03\x02\x02\
	\x02\u{2b9}\u{2ae}\x03\x02\x02\x02\u{2ba}\x53\x03\x02\x02\x02\u{2bb}\u{2bd}\
	\x05\u{ca}\x66\x02\u{2bc}\u{2bb}\x03\x02\x02\x02\u{2bd}\u{2c0}\x03\x02\x02\
	\x02\u{2be}\u{2bc}\x03\x02\x02\x02\u{2be}\u{2bf}\x03\x02\x02\x02\u{2bf}\
	\u{2ca}\x03\x02\x02\x02\u{2c0}\u{2be}\x03\x02\x02\x02\u{2c1}\u{2c4}\x05\
	\x76\x3c\x02\u{2c2}\u{2c3}\x07\x1b\x02\x02\u{2c3}\u{2c5}\x05\u{88}\x45\x02\
	\u{2c4}\u{2c2}\x03\x02\x02\x02\u{2c4}\u{2c5}\x03\x02\x02\x02\u{2c5}\u{2c6}\
	\x03\x02\x02\x02\u{2c6}\u{2c7}\x07\x04\x02\x02\u{2c7}\u{2cb}\x03\x02\x02\
	\x02\u{2c8}\u{2cb}\x05\x5e\x30\x02\u{2c9}\u{2cb}\x05\x44\x23\x02\u{2ca}\
	\u{2c1}\x03\x02\x02\x02\u{2ca}\u{2c8}\x03\x02\x02\x02\u{2ca}\u{2c9}\x03\
	\x02\x02\x02\u{2cb}\x55\x03\x02\x02\x02\u{2cc}\u{2cf}\x05\u{82}\x42\x02\
	\u{2cd}\u{2cf}\x05\x76\x3c\x02\u{2ce}\u{2cc}\x03\x02\x02\x02\u{2ce}\u{2cd}\
	\x03\x02\x02\x02\u{2cf}\x57\x03\x02\x02\x02\u{2d0}\u{2d2}\x05\u{ca}\x66\
	\x02\u{2d1}\u{2d0}\x03\x02\x02\x02\u{2d2}\u{2d5}\x03\x02\x02\x02\u{2d3}\
	\u{2d1}\x03\x02\x02\x02\u{2d3}\u{2d4}\x03\x02\x02\x02\u{2d4}\u{2d6}\x03\
	\x02\x02\x02\u{2d5}\u{2d3}\x03\x02\x02\x02\u{2d6}\u{2d7}\x07\x28\x02\x02\
	\u{2d7}\u{2d8}\x05\x4c\x27\x02\u{2d8}\u{2d9}\x07\x12\x02\x02\u{2d9}\u{2da}\
	\x07\x0b\x02\x02\u{2da}\u{2df}\x05\x56\x2c\x02\u{2db}\u{2dc}\x07\x08\x02\
	\x02\u{2dc}\u{2de}\x05\x56\x2c\x02\u{2dd}\u{2db}\x03\x02\x02\x02\u{2de}\
	\u{2e1}\x03\x02\x02\x02\u{2df}\u{2dd}\x03\x02\x02\x02\u{2df}\u{2e0}\x03\
	\x02\x02\x02\u{2e0}\u{2e2}\x03\x02\x02\x02\u{2e1}\u{2df}\x03\x02\x02\x02\
	\u{2e2}\u{2e4}\x07\x0d\x02\x02\u{2e3}\u{2e5}\x05\u{ce}\x68\x02\u{2e4}\u{2e3}\
	\x03\x02\x02\x02\u{2e4}\u{2e5}\x03\x02\x02\x02\u{2e5}\u{2e6}\x03\x02\x02\
	\x02\u{2e6}\u{2ea}\x07\x04\x02\x02\u{2e7}\u{2e9}\x05\x5a\x2e\x02\u{2e8}\
	\u{2e7}\x03\x02\x02\x02\u{2e9}\u{2ec}\x03\x02\x02\x02\u{2ea}\u{2e8}\x03\
	\x02\x02\x02\u{2ea}\u{2eb}\x03\x02\x02\x02\u{2eb}\u{2ed}\x03\x02\x02\x02\
	\u{2ec}\u{2ea}\x03\x02\x02\x02\u{2ed}\u{2f0}\x07\x29\x02\x02\u{2ee}\u{2ef}\
	\x07\x06\x02\x02\u{2ef}\u{2f1}\x05\x4c\x27\x02\u{2f0}\u{2ee}\x03\x02\x02\
	\x02\u{2f0}\u{2f1}\x03\x02\x02\x02\u{2f1}\x59\x03\x02\x02\x02\u{2f2}\u{2f6}\
	\x05\x7e\x40\x02\u{2f3}\u{2f6}\x05\x74\x3b\x02\u{2f4}\u{2f6}\x05\x5c\x2f\
	\x02\u{2f5}\u{2f2}\x03\x02\x02\x02\u{2f5}\u{2f3}\x03\x02\x02\x02\u{2f5}\
	\u{2f4}\x03\x02\x02\x02\u{2f6}\x5b\x03\x02\x02\x02\u{2f7}\u{2f9}\x05\u{ca}\
	\x66\x02\u{2f8}\u{2f7}\x03\x02\x02\x02\u{2f9}\u{2fc}\x03\x02\x02\x02\u{2fa}\
	\u{2f8}\x03\x02\x02\x02\u{2fa}\u{2fb}\x03\x02\x02\x02\u{2fb}\u{2fd}\x03\
	\x02\x02\x02\u{2fc}\u{2fa}\x03\x02\x02\x02\u{2fd}\u{301}\x05\x5e\x30\x02\
	\u{2fe}\u{300}\x05\x64\x33\x02\u{2ff}\u{2fe}\x03\x02\x02\x02\u{300}\u{303}\
	\x03\x02\x02\x02\u{301}\u{2ff}\x03\x02\x02\x02\u{301}\u{302}\x03\x02\x02\
	\x02\u{302}\u{304}\x03\x02\x02\x02\u{303}\u{301}\x03\x02\x02\x02\u{304}\
	\u{307}\x07\x2a\x02\x02\u{305}\u{306}\x07\x06\x02\x02\u{306}\u{308}\x05\
	\x08\x05\x02\u{307}\u{305}\x03\x02\x02\x02\u{307}\u{308}\x03\x02\x02\x02\
	\u{308}\x5d\x03\x02\x02\x02\u{309}\u{30a}\x07\x2b\x02\x02\u{30a}\u{30b}\
	\x05\u{82}\x42\x02\u{30b}\u{30c}\x05\x08\x05\x02\u{30c}\u{30e}\x07\x0b\x02\
	\x02\u{30d}\u{30f}\x05\x60\x31\x02\u{30e}\u{30d}\x03\x02\x02\x02\u{30e}\
	\u{30f}\x03\x02\x02\x02\u{30f}\u{310}\x03\x02\x02\x02\u{310}\u{312}\x07\
	\x0d\x02\x02\u{311}\u{313}\x05\u{ce}\x68\x02\u{312}\u{311}\x03\x02\x02\x02\
	\u{312}\u{313}\x03\x02\x02\x02\u{313}\u{314}\x03\x02\x02\x02\u{314}\u{315}\
	\x07\x04\x02\x02\u{315}\u{33d}\x03\x02\x02\x02\u{316}\u{31b}\x07\x2b\x02\
	\x02\u{317}\u{318}\x07\x19\x02\x02\u{318}\u{319}\x05\u{88}\x45\x02\u{319}\
	\u{31a}\x07\x1a\x02\x02\u{31a}\u{31c}\x03\x02\x02\x02\u{31b}\u{317}\x03\
	\x02\x02\x02\u{31b}\u{31c}\x03\x02\x02\x02\u{31c}\u{31d}\x03\x02\x02\x02\
	\u{31d}\u{31e}\x05\x08\x05\x02\u{31e}\u{31f}\x07\x0b\x02\x02\u{31f}\u{320}\
	\x05\u{82}\x42\x02\u{320}\u{322}\x07\x0d\x02\x02\u{321}\u{323}\x05\u{ce}\
	\x68\x02\u{322}\u{321}\x03\x02\x02\x02\u{322}\u{323}\x03\x02\x02\x02\u{323}\
	\u{324}\x03\x02\x02\x02\u{324}\u{325}\x07\x04\x02\x02\u{325}\u{33d}\x03\
	\x02\x02\x02\u{326}\u{32b}\x07\x2b\x02\x02\u{327}\u{328}\x07\x19\x02\x02\
	\u{328}\u{329}\x05\u{88}\x45\x02\u{329}\u{32a}\x07\x1a\x02\x02\u{32a}\u{32c}\
	\x03\x02\x02\x02\u{32b}\u{327}\x03\x02\x02\x02\u{32b}\u{32c}\x03\x02\x02\
	\x02\u{32c}\u{32d}\x03\x02\x02\x02\u{32d}\u{32e}\x05\x08\x05\x02\u{32e}\
	\u{32f}\x07\x12\x02\x02\u{32f}\u{331}\x07\x0b\x02\x02\u{330}\u{332}\x05\
	\x60\x31\x02\u{331}\u{330}\x03\x02\x02\x02\u{331}\u{332}\x03\x02\x02\x02\
	\u{332}\u{333}\x03\x02\x02\x02\u{333}\u{334}\x07\x0d\x02\x02\u{334}\u{335}\
	\x07\x0b\x02\x02\u{335}\u{336}\x05\u{82}\x42\x02\u{336}\u{338}\x07\x0d\x02\
	\x02\u{337}\u{339}\x05\u{ce}\x68\x02\u{338}\u{337}\x03\x02\x02\x02\u{338}\
	\u{339}\x03\x02\x02\x02\u{339}\u{33a}\x03\x02\x02\x02\u{33a}\u{33b}\x07\
	\x04\x02\x02\u{33b}\u{33d}\x03\x02\x02\x02\u{33c}\u{309}\x03\x02\x02\x02\
	\u{33c}\u{316}\x03\x02\x02\x02\u{33c}\u{326}\x03\x02\x02\x02\u{33d}\x5f\
	\x03\x02\x02\x02\u{33e}\u{343}\x05\x62\x32\x02\u{33f}\u{340}\x07\x08\x02\
	\x02\u{340}\u{342}\x05\x62\x32\x02\u{341}\u{33f}\x03\x02\x02\x02\u{342}\
	\u{345}\x03\x02\x02\x02\u{343}\u{341}\x03\x02\x02\x02\u{343}\u{344}\x03\
	\x02\x02\x02\u{344}\x61\x03\x02\x02\x02\u{345}\u{343}\x03\x02\x02\x02\u{346}\
	\u{348}\x05\u{ca}\x66\x02\u{347}\u{346}\x03\x02\x02\x02\u{348}\u{34b}\x03\
	\x02\x02\x02\u{349}\u{347}\x03\x02\x02\x02\u{349}\u{34a}\x03\x02\x02\x02\
	\u{34a}\u{34d}\x03\x02\x02\x02\u{34b}\u{349}\x03\x02\x02\x02\u{34c}\u{34e}\
	\x07\x2c\x02\x02\u{34d}\u{34c}\x03\x02\x02\x02\u{34d}\u{34e}\x03\x02\x02\
	\x02\u{34e}\u{34f}\x03\x02\x02\x02\u{34f}\u{350}\x05\u{82}\x42\x02\u{350}\
	\u{351}\x05\x08\x05\x02\u{351}\u{354}\x03\x02\x02\x02\u{352}\u{354}\x05\
	\x76\x3c\x02\u{353}\u{349}\x03\x02\x02\x02\u{353}\u{352}\x03\x02\x02\x02\
	\u{354}\x63\x03\x02\x02\x02\u{355}\u{35b}\x05\x66\x34\x02\u{356}\u{35b}\
	\x05\x5c\x2f\x02\u{357}\u{35b}\x05\x3e\x20\x02\u{358}\u{35b}\x05\x6e\x38\
	\x02\u{359}\u{35b}\x05\u{a6}\x54\x02\u{35a}\u{355}\x03\x02\x02\x02\u{35a}\
	\u{356}\x03\x02\x02\x02\u{35a}\u{357}\x03\x02\x02\x02\u{35a}\u{358}\x03\
	\x02\x02\x02\u{35a}\u{359}\x03\x02\x02\x02\u{35b}\x65\x03\x02\x02\x02\u{35c}\
	\u{35e}\x07\x11\x02\x02\u{35d}\u{35f}\x05\u{82}\x42\x02\u{35e}\u{35d}\x03\
	\x02\x02\x02\u{35e}\u{35f}\x03\x02\x02\x02\u{35f}\u{360}\x03\x02\x02\x02\
	\u{360}\u{366}\x05\x08\x05\x02\u{361}\u{363}\x07\x0b\x02\x02\u{362}\u{364}\
	\x05\x68\x35\x02\u{363}\u{362}\x03\x02\x02\x02\u{363}\u{364}\x03\x02\x02\
	\x02\u{364}\u{365}\x03\x02\x02\x02\u{365}\u{367}\x07\x0d\x02\x02\u{366}\
	\u{361}\x03\x02\x02\x02\u{366}\u{367}\x03\x02\x02\x02\u{367}\u{369}\x03\
	\x02\x02\x02\u{368}\u{36a}\x05\u{ce}\x68\x02\u{369}\u{368}\x03\x02\x02\x02\
	\u{369}\u{36a}\x03\x02\x02\x02\u{36a}\u{36c}\x03\x02\x02\x02\u{36b}\u{36d}\
	\x05\x6c\x37\x02\u{36c}\u{36b}\x03\x02\x02\x02\u{36c}\u{36d}\x03\x02\x02\
	\x02\u{36d}\u{36e}\x03\x02\x02\x02\u{36e}\u{372}\x07\x04\x02\x02\u{36f}\
	\u{371}\x05\u{a6}\x54\x02\u{370}\u{36f}\x03\x02\x02\x02\u{371}\u{374}\x03\
	\x02\x02\x02\u{372}\u{370}\x03\x02\x02\x02\u{372}\u{373}\x03\x02\x02\x02\
	\u{373}\u{375}\x03\x02\x02\x02\u{374}\u{372}\x03\x02\x02\x02\u{375}\u{378}\
	\x07\x2d\x02\x02\u{376}\u{377}\x07\x06\x02\x02\u{377}\u{379}\x05\x08\x05\
	\x02\u{378}\u{376}\x03\x02\x02\x02\u{378}\u{379}\x03\x02\x02\x02\u{379}\
	\u{38e}\x03\x02\x02\x02\u{37a}\u{37c}\x07\x11\x02\x02\u{37b}\u{37d}\x05\
	\u{82}\x42\x02\u{37c}\u{37b}\x03\x02\x02\x02\u{37c}\u{37d}\x03\x02\x02\x02\
	\u{37d}\u{37e}\x03\x02\x02\x02\u{37e}\u{384}\x05\x08\x05\x02\u{37f}\u{381}\
	\x07\x0b\x02\x02\u{380}\u{382}\x05\x68\x35\x02\u{381}\u{380}\x03\x02\x02\
	\x02\u{381}\u{382}\x03\x02\x02\x02\u{382}\u{383}\x03\x02\x02\x02\u{383}\
	\u{385}\x07\x0d\x02\x02\u{384}\u{37f}\x03\x02\x02\x02\u{384}\u{385}\x03\
	\x02\x02\x02\u{385}\u{387}\x03\x02\x02\x02\u{386}\u{388}\x05\x6c\x37\x02\
	\u{387}\u{386}\x03\x02\x02\x02\u{387}\u{388}\x03\x02\x02\x02\u{388}\u{389}\
	\x03\x02\x02\x02\u{389}\u{38a}\x07\x1b\x02\x02\u{38a}\u{38b}\x05\u{88}\x45\
	\x02\u{38b}\u{38c}\x07\x04\x02\x02\u{38c}\u{38e}\x03\x02\x02\x02\u{38d}\
	\u{35c}\x03\x02\x02\x02\u{38d}\u{37a}\x03\x02\x02\x02\u{38e}\x67\x03\x02\
	\x02\x02\u{38f}\u{394}\x05\x6a\x36\x02\u{390}\u{391}\x07\x08\x02\x02\u{391}\
	\u{393}\x05\x6a\x36\x02\u{392}\u{390}\x03\x02\x02\x02\u{393}\u{396}\x03\
	\x02\x02\x02\u{394}\u{392}\x03\x02\x02\x02\u{394}\u{395}\x03\x02\x02\x02\
	\u{395}\x69\x03\x02\x02\x02\u{396}\u{394}\x03\x02\x02\x02\u{397}\u{399}\
	\x05\u{ca}\x66\x02\u{398}\u{397}\x03\x02\x02\x02\u{399}\u{39c}\x03\x02\x02\
	\x02\u{39a}\u{398}\x03\x02\x02\x02\u{39a}\u{39b}\x03\x02\x02\x02\u{39b}\
	\u{39e}\x03\x02\x02\x02\u{39c}\u{39a}\x03\x02\x02\x02\u{39d}\u{39f}\x05\
	\u{82}\x42\x02\u{39e}\u{39d}\x03\x02\x02\x02\u{39e}\u{39f}\x03\x02\x02\x02\
	\u{39f}\u{3a0}\x03\x02\x02\x02\u{3a0}\u{3a3}\x05\x08\x05\x02\u{3a1}\u{3a3}\
	\x05\x76\x3c\x02\u{3a2}\u{39a}\x03\x02\x02\x02\u{3a2}\u{3a1}\x03\x02\x02\
	\x02\u{3a3}\x6b\x03\x02\x02\x02\u{3a4}\u{3a5}\x09\x05\x02\x02\u{3a5}\u{3a6}\
	\x07\x0b\x02\x02\u{3a6}\u{3a7}\x05\u{88}\x45\x02\u{3a7}\u{3a8}\x07\x0d\x02\
	\x02\u{3a8}\x6d\x03\x02\x02\x02\u{3a9}\u{3aa}\x07\x0f\x02\x02\u{3aa}\u{3ab}\
	\x05\x0a\x06\x02\u{3ab}\u{3ac}\x05\x08\x05\x02\u{3ac}\u{3b0}\x07\x04\x02\
	\x02\u{3ad}\u{3af}\x05\u{9c}\x4f\x02\u{3ae}\u{3ad}\x03\x02\x02\x02\u{3af}\
	\u{3b2}\x03\x02\x02\x02\u{3b0}\u{3ae}\x03\x02\x02\x02\u{3b0}\u{3b1}\x03\
	\x02\x02\x02\u{3b1}\u{3b3}\x03\x02\x02\x02\u{3b2}\u{3b0}\x03\x02\x02\x02\
	\u{3b3}\u{3b6}\x07\x10\x02\x02\u{3b4}\u{3b5}\x07\x06\x02\x02\u{3b5}\u{3b7}\
	\x05\x08\x05\x02\u{3b6}\u{3b4}\x03\x02\x02\x02\u{3b6}\u{3b7}\x03\x02\x02\
	\x02\u{3b7}\u{3c2}\x03\x02\x02\x02\u{3b8}\u{3ba}\x07\x0f\x02\x02\u{3b9}\
	\u{3bb}\x05\x0a\x06\x02\u{3ba}\u{3b9}\x03\x02\x02\x02\u{3ba}\u{3bb}\x03\
	\x02\x02\x02\u{3bb}\u{3bc}\x03\x02\x02\x02\u{3bc}\u{3bd}\x05\x08\x05\x02\
	\u{3bd}\u{3be}\x07\x1b\x02\x02\u{3be}\u{3bf}\x05\u{88}\x45\x02\u{3bf}\u{3c0}\
	\x07\x04\x02\x02\u{3c0}\u{3c2}\x03\x02\x02\x02\u{3c1}\u{3a9}\x03\x02\x02\
	\x02\u{3c1}\u{3b8}\x03\x02\x02\x02\u{3c2}\x6f\x03\x02\x02\x02\u{3c3}\u{3c5}\
	\x05\u{ca}\x66\x02\u{3c4}\u{3c3}\x03\x02\x02\x02\u{3c5}\u{3c8}\x03\x02\x02\
	\x02\u{3c6}\u{3c4}\x03\x02\x02\x02\u{3c6}\u{3c7}\x03\x02\x02\x02\u{3c7}\
	\u{3c9}\x03\x02\x02\x02\u{3c8}\u{3c6}\x03\x02\x02\x02\u{3c9}\u{3ca}\x07\
	\x30\x02\x02\u{3ca}\u{3cc}\x05\x08\x05\x02\u{3cb}\u{3cd}\x05\x72\x3a\x02\
	\u{3cc}\u{3cb}\x03\x02\x02\x02\u{3cc}\u{3cd}\x03\x02\x02\x02\u{3cd}\u{3ce}\
	\x03\x02\x02\x02\u{3ce}\u{3d2}\x07\x04\x02\x02\u{3cf}\u{3d1}\x05\u{a6}\x54\
	\x02\u{3d0}\u{3cf}\x03\x02\x02\x02\u{3d1}\u{3d4}\x03\x02\x02\x02\u{3d2}\
	\u{3d0}\x03\x02\x02\x02\u{3d2}\u{3d3}\x03\x02\x02\x02\u{3d3}\u{3d5}\x03\
	\x02\x02\x02\u{3d4}\u{3d2}\x03\x02\x02\x02\u{3d5}\u{3d8}\x07\x31\x02\x02\
	\u{3d6}\u{3d7}\x07\x06\x02\x02\u{3d7}\u{3d9}\x05\x08\x05\x02\u{3d8}\u{3d6}\
	\x03\x02\x02\x02\u{3d8}\u{3d9}\x03\x02\x02\x02\u{3d9}\x71\x03\x02\x02\x02\
	\u{3da}\u{3de}\x07\x2e\x02\x02\u{3db}\u{3de}\x07\x2f\x02\x02\u{3dc}\u{3de}\
	\x03\x02\x02\x02\u{3dd}\u{3da}\x03\x02\x02\x02\u{3dd}\u{3db}\x03\x02\x02\
	\x02\u{3dd}\u{3dc}\x03\x02\x02\x02\u{3de}\u{3df}\x03\x02\x02\x02\u{3df}\
	\u{3e0}\x07\x0b\x02\x02\u{3e0}\u{3e1}\x05\u{88}\x45\x02\u{3e1}\u{3e2}\x07\
	\x0d\x02\x02\u{3e2}\x73\x03\x02\x02\x02\u{3e3}\u{3e5}\x05\u{ca}\x66\x02\
	\u{3e4}\u{3e3}\x03\x02\x02\x02\u{3e5}\u{3e8}\x03\x02\x02\x02\u{3e6}\u{3e4}\
	\x03\x02\x02\x02\u{3e6}\u{3e7}\x03\x02\x02\x02\u{3e7}\u{3e9}\x03\x02\x02\
	\x02\u{3e8}\u{3e6}\x03\x02\x02\x02\u{3e9}\u{3ea}\x05\x76\x3c\x02\u{3ea}\
	\u{3ee}\x07\x04\x02\x02\u{3eb}\u{3ed}\x05\u{a6}\x54\x02\u{3ec}\u{3eb}\x03\
	\x02\x02\x02\u{3ed}\u{3f0}\x03\x02\x02\x02\u{3ee}\u{3ec}\x03\x02\x02\x02\
	\u{3ee}\u{3ef}\x03\x02\x02\x02\u{3ef}\u{3f1}\x03\x02\x02\x02\u{3f0}\u{3ee}\
	\x03\x02\x02\x02\u{3f1}\u{3f4}\x07\x32\x02\x02\u{3f2}\u{3f3}\x07\x06\x02\
	\x02\u{3f3}\u{3f5}\x05\x08\x05\x02\u{3f4}\u{3f2}\x03\x02\x02\x02\u{3f4}\
	\u{3f5}\x03\x02\x02\x02\u{3f5}\u{3fc}\x03\x02\x02\x02\u{3f6}\u{3f7}\x05\
	\x76\x3c\x02\u{3f7}\u{3f8}\x07\x1b\x02\x02\u{3f8}\u{3f9}\x05\u{88}\x45\x02\
	\u{3f9}\u{3fa}\x07\x04\x02\x02\u{3fa}\u{3fc}\x03\x02\x02\x02\u{3fb}\u{3e6}\
	\x03\x02\x02\x02\u{3fb}\u{3f6}\x03\x02\x02\x02\u{3fc}\x75\x03\x02\x02\x02\
	\u{3fd}\u{3ff}\x07\x33\x02\x02\u{3fe}\u{400}\x05\u{82}\x42\x02\u{3ff}\u{3fe}\
	\x03\x02\x02\x02\u{3ff}\u{400}\x03\x02\x02\x02\u{400}\u{401}\x03\x02\x02\
	\x02\u{401}\u{407}\x05\x08\x05\x02\u{402}\u{404}\x07\x0b\x02\x02\u{403}\
	\u{405}\x05\x68\x35\x02\u{404}\u{403}\x03\x02\x02\x02\u{404}\u{405}\x03\
	\x02\x02\x02\u{405}\u{406}\x03\x02\x02\x02\u{406}\u{408}\x07\x0d\x02\x02\
	\u{407}\u{402}\x03\x02\x02\x02\u{407}\u{408}\x03\x02\x02\x02\u{408}\u{40a}\
	\x03\x02\x02\x02\u{409}\u{40b}\x05\u{ce}\x68\x02\u{40a}\u{409}\x03\x02\x02\
	\x02\u{40a}\u{40b}\x03\x02\x02\x02\u{40b}\x77\x03\x02\x02\x02\u{40c}\u{40d}\
	\x07\x0e\x02\x02\u{40d}\u{411}\x07\x34\x02\x02\u{40e}\u{40f}\x05\x08\x05\
	\x02\u{40f}\u{410}\x07\x1b\x02\x02\u{410}\u{412}\x03\x02\x02\x02\u{411}\
	\u{40e}\x03\x02\x02\x02\u{411}\u{412}\x03\x02\x02\x02\u{412}\u{413}\x03\
	\x02\x02\x02\u{413}\u{414}\x07\x33\x02\x02\u{414}\u{415}\x05\u{82}\x42\x02\
	\u{415}\u{416}\x05\x08\x05\x02\u{416}\u{418}\x07\x0b\x02\x02\u{417}\u{419}\
	\x05\x7a\x3e\x02\u{418}\u{417}\x03\x02\x02\x02\u{418}\u{419}\x03\x02\x02\
	\x02\u{419}\u{41a}\x03\x02\x02\x02\u{41a}\u{41b}\x07\x0d\x02\x02\u{41b}\
	\u{41c}\x07\x04\x02\x02\u{41c}\x79\x03\x02\x02\x02\u{41d}\u{422}\x05\x7c\
	\x3f\x02\u{41e}\u{41f}\x07\x08\x02\x02\u{41f}\u{421}\x05\x7c\x3f\x02\u{420}\
	\u{41e}\x03\x02\x02\x02\u{421}\u{424}\x03\x02\x02\x02\u{422}\u{420}\x03\
	\x02\x02\x02\u{422}\u{423}\x03\x02\x02\x02\u{423}\x7b\x03\x02\x02\x02\u{424}\
	\u{422}\x03\x02\x02\x02\u{425}\u{427}\x05\u{82}\x42\x02\u{426}\u{428}\x05\
	\x08\x05\x02\u{427}\u{426}\x03\x02\x02\x02\u{427}\u{428}\x03\x02\x02\x02\
	\u{428}\x7d\x03\x02\x02\x02\u{429}\u{42b}\x05\u{ca}\x66\x02\u{42a}\u{429}\
	\x03\x02\x02\x02\u{42b}\u{42e}\x03\x02\x02\x02\u{42c}\u{42a}\x03\x02\x02\
	\x02\u{42c}\u{42d}\x03\x02\x02\x02\u{42d}\u{42f}\x03\x02\x02\x02\u{42e}\
	\u{42c}\x03\x02\x02\x02\u{42f}\u{430}\x05\u{80}\x41\x02\u{430}\u{431}\x09\
	\x04\x02\x02\u{431}\u{432}\x05\u{88}\x45\x02\u{432}\u{433}\x07\x04\x02\x02\
	\u{433}\u{449}\x03\x02\x02\x02\u{434}\u{436}\x05\u{ca}\x66\x02\u{435}\u{434}\
	\x03\x02\x02\x02\u{436}\u{439}\x03\x02\x02\x02\u{437}\u{435}\x03\x02\x02\
	\x02\u{437}\u{438}\x03\x02\x02\x02\u{438}\u{43a}\x03\x02\x02\x02\u{439}\
	\u{437}\x03\x02\x02\x02\u{43a}\u{43b}\x07\x17\x02\x02\u{43b}\u{440}\x05\
	\u{80}\x41\x02\u{43c}\u{43d}\x07\x08\x02\x02\u{43d}\u{43f}\x05\u{80}\x41\
	\x02\u{43e}\u{43c}\x03\x02\x02\x02\u{43f}\u{442}\x03\x02\x02\x02\u{440}\
	\u{43e}\x03\x02\x02\x02\u{440}\u{441}\x03\x02\x02\x02\u{441}\u{443}\x03\
	\x02\x02\x02\u{442}\u{440}\x03\x02\x02\x02\u{443}\u{444}\x07\x18\x02\x02\
	\u{444}\u{445}\x09\x04\x02\x02\u{445}\u{446}\x05\u{88}\x45\x02\u{446}\u{447}\
	\x07\x04\x02\x02\u{447}\u{449}\x03\x02\x02\x02\u{448}\u{42c}\x03\x02\x02\
	\x02\u{448}\u{437}\x03\x02\x02\x02\u{449}\x7f\x03\x02\x02\x02\u{44a}\u{462}\
	\x05\x08\x05\x02\u{44b}\u{44c}\x05\u{96}\x4c\x02\u{44c}\u{44d}\x07\x35\x02\
	\x02\u{44d}\u{44e}\x05\x08\x05\x02\u{44e}\u{462}\x03\x02\x02\x02\u{44f}\
	\u{450}\x05\u{96}\x4c\x02\u{450}\u{451}\x07\x19\x02\x02\u{451}\u{452}\x05\
	\u{88}\x45\x02\u{452}\u{453}\x07\x1a\x02\x02\u{453}\u{462}\x03\x02\x02\x02\
	\u{454}\u{455}\x05\u{96}\x4c\x02\u{455}\u{456}\x07\x19\x02\x02\u{456}\u{45d}\
	\x05\u{88}\x45\x02\u{457}\u{458}\x07\x06\x02\x02\u{458}\u{45e}\x05\u{88}\
	\x45\x02\u{459}\u{45a}\x07\x36\x02\x02\u{45a}\u{45e}\x07\x6f\x02\x02\u{45b}\
	\u{45c}\x07\x37\x02\x02\u{45c}\u{45e}\x07\x6f\x02\x02\u{45d}\u{457}\x03\
	\x02\x02\x02\u{45d}\u{459}\x03\x02\x02\x02\u{45d}\u{45b}\x03\x02\x02\x02\
	\u{45e}\u{45f}\x03\x02\x02\x02\u{45f}\u{460}\x07\x1a\x02\x02\u{460}\u{462}\
	\x03\x02\x02\x02\u{461}\u{44a}\x03\x02\x02\x02\u{461}\u{44b}\x03\x02\x02\
	\x02\u{461}\u{44f}\x03\x02\x02\x02\u{461}\u{454}\x03\x02\x02\x02\u{462}\
	\u{81}\x03\x02\x02\x02\u{463}\u{470}\x05\u{84}\x43\x02\u{464}\u{465}\x07\
	\x12\x02\x02\u{465}\u{466}\x07\x0b\x02\x02\u{466}\u{46b}\x05\u{82}\x42\x02\
	\u{467}\u{468}\x07\x08\x02\x02\u{468}\u{46a}\x05\u{82}\x42\x02\u{469}\u{467}\
	\x03\x02\x02\x02\u{46a}\u{46d}\x03\x02\x02\x02\u{46b}\u{469}\x03\x02\x02\
	\x02\u{46b}\u{46c}\x03\x02\x02\x02\u{46c}\u{46e}\x03\x02\x02\x02\u{46d}\
	\u{46b}\x03\x02\x02\x02\u{46e}\u{46f}\x07\x0d\x02\x02\u{46f}\u{471}\x03\
	\x02\x02\x02\u{470}\u{464}\x03\x02\x02\x02\u{470}\u{471}\x03\x02\x02\x02\
	\u{471}\u{479}\x03\x02\x02\x02\u{472}\u{479}\x05\x08\x05\x02\u{473}\u{479}\
	\x05\u{86}\x44\x02\u{474}\u{475}\x07\x0b\x02\x02\u{475}\u{476}\x05\u{82}\
	\x42\x02\u{476}\u{477}\x07\x0d\x02\x02\u{477}\u{479}\x03\x02\x02\x02\u{478}\
	\u{463}\x03\x02\x02\x02\u{478}\u{472}\x03\x02\x02\x02\u{478}\u{473}\x03\
	\x02\x02\x02\u{478}\u{474}\x03\x02\x02\x02\u{479}\u{83}\x03\x02\x02\x02\
	\u{47a}\u{47b}\x05\x0a\x06\x02\u{47b}\u{47c}\x07\x09\x02\x02\u{47c}\u{47e}\
	\x03\x02\x02\x02\u{47d}\u{47a}\x03\x02\x02\x02\u{47e}\u{481}\x03\x02\x02\
	\x02\u{47f}\u{47d}\x03\x02\x02\x02\u{47f}\u{480}\x03\x02\x02\x02\u{480}\
	\u{482}\x03\x02\x02\x02\u{481}\u{47f}\x03\x02\x02\x02\u{482}\u{485}\x05\
	\x0a\x06\x02\u{483}\u{485}\x05\x08\x05\x02\u{484}\u{47f}\x03\x02\x02\x02\
	\u{484}\u{483}\x03\x02\x02\x02\u{485}\u{85}\x03\x02\x02\x02\u{486}\u{487}\
	\x07\x6f\x02\x02\u{487}\u{87}\x03\x02\x02\x02\u{488}\u{489}\x08\x45\x01\
	\x02\u{489}\u{48a}\x07\x3a\x02\x02\u{48a}\u{48b}\x07\x0b\x02\x02\u{48b}\
	\u{48c}\x05\u{88}\x45\x02\u{48c}\u{499}\x07\x0d\x02\x02\u{48d}\u{48f}\x07\
	\x39\x02\x02\u{48e}\u{490}\x05\u{8a}\x46\x02\u{48f}\u{48e}\x03\x02\x02\x02\
	\u{490}\u{491}\x03\x02\x02\x02\u{491}\u{48f}\x03\x02\x02\x02\u{491}\u{492}\
	\x03\x02\x02\x02\u{492}\u{49a}\x03\x02\x02\x02\u{493}\u{495}\x05\u{8c}\x47\
	\x02\u{494}\u{493}\x03\x02\x02\x02\u{495}\u{498}\x03\x02\x02\x02\u{496}\
	\u{494}\x03\x02\x02\x02\u{496}\u{497}\x03\x02\x02\x02\u{497}\u{49a}\x03\
	\x02\x02\x02\u{498}\u{496}\x03\x02\x02\x02\u{499}\u{48d}\x03\x02\x02\x02\
	\u{499}\u{496}\x03\x02\x02\x02\u{49a}\u{49c}\x03\x02\x02\x02\u{49b}\u{49d}\
	\x05\u{8e}\x48\x02\u{49c}\u{49b}\x03\x02\x02\x02\u{49c}\u{49d}\x03\x02\x02\
	\x02\u{49d}\u{49e}\x03\x02\x02\x02\u{49e}\u{49f}\x07\x3b\x02\x02\u{49f}\
	\u{4a2}\x03\x02\x02\x02\u{4a0}\u{4a2}\x05\u{92}\x4a\x02\u{4a1}\u{488}\x03\
	\x02\x02\x02\u{4a1}\u{4a0}\x03\x02\x02\x02\u{4a2}\u{4b4}\x03\x02\x02\x02\
	\u{4a3}\u{4a4}\x0c\x06\x02\x02\u{4a4}\u{4a5}\x07\x38\x02\x02\u{4a5}\u{4a6}\
	\x05\u{88}\x45\x02\u{4a6}\u{4a7}\x07\x06\x02\x02\u{4a7}\u{4a8}\x05\u{88}\
	\x45\x07\u{4a8}\u{4b3}\x03\x02\x02\x02\u{4a9}\u{4aa}\x0c\x05\x02\x02\u{4aa}\
	\u{4ab}\x07\x39\x02\x02\u{4ab}\u{4af}\x05\u{c2}\x62\x02\u{4ac}\u{4ae}\x05\
	\u{90}\x49\x02\u{4ad}\u{4ac}\x03\x02\x02\x02\u{4ae}\u{4b1}\x03\x02\x02\x02\
	\u{4af}\u{4ad}\x03\x02\x02\x02\u{4af}\u{4b0}\x03\x02\x02\x02\u{4b0}\u{4b3}\
	\x03\x02\x02\x02\u{4b1}\u{4af}\x03\x02\x02\x02\u{4b2}\u{4a3}\x03\x02\x02\
	\x02\u{4b2}\u{4a9}\x03\x02\x02\x02\u{4b3}\u{4b6}\x03\x02\x02\x02\u{4b4}\
	\u{4b2}\x03\x02\x02\x02\u{4b4}\u{4b5}\x03\x02\x02\x02\u{4b5}\u{89}\x03\x02\
	\x02\x02\u{4b6}\u{4b4}\x03\x02\x02\x02\u{4b7}\u{4bb}\x05\u{c2}\x62\x02\u{4b8}\
	\u{4ba}\x05\u{90}\x49\x02\u{4b9}\u{4b8}\x03\x02\x02\x02\u{4ba}\u{4bd}\x03\
	\x02\x02\x02\u{4bb}\u{4b9}\x03\x02\x02\x02\u{4bb}\u{4bc}\x03\x02\x02\x02\
	\u{4bc}\u{4be}\x03\x02\x02\x02\u{4bd}\u{4bb}\x03\x02\x02\x02\u{4be}\u{4bf}\
	\x07\x06\x02\x02\u{4bf}\u{4c0}\x05\u{88}\x45\x02\u{4c0}\u{4c1}\x07\x04\x02\
	\x02\u{4c1}\u{8b}\x03\x02\x02\x02\u{4c2}\u{4c7}\x05\u{88}\x45\x02\u{4c3}\
	\u{4c4}\x07\x08\x02\x02\u{4c4}\u{4c6}\x05\u{88}\x45\x02\u{4c5}\u{4c3}\x03\
	\x02\x02\x02\u{4c6}\u{4c9}\x03\x02\x02\x02\u{4c7}\u{4c5}\x03\x02\x02\x02\
	\u{4c7}\u{4c8}\x03\x02\x02\x02\u{4c8}\u{4ca}\x03\x02\x02\x02\u{4c9}\u{4c7}\
	\x03\x02\x02\x02\u{4ca}\u{4cb}\x07\x06\x02\x02\u{4cb}\u{4cc}\x05\u{88}\x45\
	\x02\u{4cc}\u{4cd}\x07\x04\x02\x02\u{4cd}\u{8d}\x03\x02\x02\x02\u{4ce}\u{4cf}\
	\x07\x3c\x02\x02\u{4cf}\u{4d0}\x07\x06\x02\x02\u{4d0}\u{4d1}\x05\u{88}\x45\
	\x02\u{4d1}\u{4d2}\x07\x04\x02\x02\u{4d2}\u{8f}\x03\x02\x02\x02\u{4d3}\u{4d4}\
	\x07\x3d\x02\x02\u{4d4}\u{4d5}\x05\u{88}\x45\x02\u{4d5}\u{91}\x03\x02\x02\
	\x02\u{4d6}\u{4d7}\x08\x4a\x01\x02\u{4d7}\u{4d8}\x05\u{94}\x4b\x02\u{4d8}\
	\u{4f9}\x03\x02\x02\x02\u{4d9}\u{4da}\x0c\x0d\x02\x02\u{4da}\u{4db}\x09\
	\x06\x02\x02\u{4db}\u{4f8}\x05\u{92}\x4a\x0e\u{4dc}\u{4dd}\x0c\x0c\x02\x02\
	\u{4dd}\u{4de}\x09\x07\x02\x02\u{4de}\u{4f8}\x05\u{92}\x4a\x0d\u{4df}\u{4e0}\
	\x0c\x0b\x02\x02\u{4e0}\u{4e1}\x09\x08\x02\x02\u{4e1}\u{4f8}\x05\u{92}\x4a\
	\x0c\u{4e2}\u{4e3}\x0c\x0a\x02\x02\u{4e3}\u{4e4}\x09\x09\x02\x02\u{4e4}\
	\u{4f8}\x05\u{92}\x4a\x0b\u{4e5}\u{4e6}\x0c\x09\x02\x02\u{4e6}\u{4e7}\x09\
	\x0a\x02\x02\u{4e7}\u{4f8}\x05\u{92}\x4a\x0a\u{4e8}\u{4e9}\x0c\x08\x02\x02\
	\u{4e9}\u{4ea}\x09\x0b\x02\x02\u{4ea}\u{4f8}\x05\u{92}\x4a\x09\u{4eb}\u{4ec}\
	\x0c\x07\x02\x02\u{4ec}\u{4ed}\x09\x0c\x02\x02\u{4ed}\u{4f8}\x05\u{92}\x4a\
	\x08\u{4ee}\u{4ef}\x0c\x06\x02\x02\u{4ef}\u{4f0}\x09\x0d\x02\x02\u{4f0}\
	\u{4f8}\x05\u{92}\x4a\x07\u{4f1}\u{4f2}\x0c\x05\x02\x02\u{4f2}\u{4f3}\x09\
	\x0e\x02\x02\u{4f3}\u{4f8}\x05\u{92}\x4a\x06\u{4f4}\u{4f5}\x0c\x04\x02\x02\
	\u{4f5}\u{4f6}\x09\x0f\x02\x02\u{4f6}\u{4f8}\x05\u{92}\x4a\x05\u{4f7}\u{4d9}\
	\x03\x02\x02\x02\u{4f7}\u{4dc}\x03\x02\x02\x02\u{4f7}\u{4df}\x03\x02\x02\
	\x02\u{4f7}\u{4e2}\x03\x02\x02\x02\u{4f7}\u{4e5}\x03\x02\x02\x02\u{4f7}\
	\u{4e8}\x03\x02\x02\x02\u{4f7}\u{4eb}\x03\x02\x02\x02\u{4f7}\u{4ee}\x03\
	\x02\x02\x02\u{4f7}\u{4f1}\x03\x02\x02\x02\u{4f7}\u{4f4}\x03\x02\x02\x02\
	\u{4f8}\u{4fb}\x03\x02\x02\x02\u{4f9}\u{4f7}\x03\x02\x02\x02\u{4f9}\u{4fa}\
	\x03\x02\x02\x02\u{4fa}\u{93}\x03\x02\x02\x02\u{4fb}\u{4f9}\x03\x02\x02\
	\x02\u{4fc}\u{4fd}\x09\x10\x02\x02\u{4fd}\u{502}\x05\u{96}\x4c\x02\u{4fe}\
	\u{4ff}\x09\x08\x02\x02\u{4ff}\u{502}\x05\u{96}\x4c\x02\u{500}\u{502}\x05\
	\u{96}\x4c\x02\u{501}\u{4fc}\x03\x02\x02\x02\u{501}\u{4fe}\x03\x02\x02\x02\
	\u{501}\u{500}\x03\x02\x02\x02\u{502}\u{95}\x03\x02\x02\x02\u{503}\u{504}\
	\x08\x4c\x01\x02\u{504}\u{505}\x07\x0b\x02\x02\u{505}\u{506}\x05\u{88}\x45\
	\x02\u{506}\u{507}\x07\x0d\x02\x02\u{507}\u{589}\x03\x02\x02\x02\u{508}\
	\u{50e}\x05\u{82}\x42\x02\u{509}\u{50a}\x07\x0b\x02\x02\u{50a}\u{50b}\x05\
	\u{82}\x42\x02\u{50b}\u{50c}\x07\x0d\x02\x02\u{50c}\u{50e}\x03\x02\x02\x02\
	\u{50d}\u{508}\x03\x02\x02\x02\u{50d}\u{509}\x03\x02\x02\x02\u{50e}\u{50f}\
	\x03\x02\x02\x02\u{50f}\u{510}\x07\x56\x02\x02\u{510}\u{511}\x05\u{96}\x4c\
	\x15\u{511}\u{589}\x03\x02\x02\x02\u{512}\u{513}\x05\x0a\x06\x02\u{513}\
	\u{514}\x07\x09\x02\x02\u{514}\u{516}\x03\x02\x02\x02\u{515}\u{512}\x03\
	\x02\x02\x02\u{516}\u{519}\x03\x02\x02\x02\u{517}\u{515}\x03\x02\x02\x02\
	\u{517}\u{518}\x03\x02\x02\x02\u{518}\u{51a}\x03\x02\x02\x02\u{519}\u{517}\
	\x03\x02\x02\x02\u{51a}\u{589}\x05\x08\x05\x02\u{51b}\u{589}\x07\x6f\x02\
	\x02\u{51c}\u{589}\x07\x71\x02\x02\u{51d}\u{589}\x07\x72\x02\x02\u{51e}\
	\u{589}\x07\x38\x02\x02\u{51f}\u{520}\x09\x11\x02\x02\u{520}\u{521}\x07\
	\x0b\x02\x02\u{521}\u{522}\x05\u{82}\x42\x02\u{522}\u{523}\x07\x0d\x02\x02\
	\u{523}\u{589}\x03\x02\x02\x02\u{524}\u{525}\x07\x17\x02\x02\u{525}\u{52a}\
	\x05\u{88}\x45\x02\u{526}\u{527}\x07\x08\x02\x02\u{527}\u{529}\x05\u{88}\
	\x45\x02\u{528}\u{526}\x03\x02\x02\x02\u{529}\u{52c}\x03\x02\x02\x02\u{52a}\
	\u{528}\x03\x02\x02\x02\u{52a}\u{52b}\x03\x02\x02\x02\u{52b}\u{52d}\x03\
	\x02\x02\x02\u{52c}\u{52a}\x03\x02\x02\x02\u{52d}\u{52e}\x07\x18\x02\x02\
	\u{52e}\u{589}\x03\x02\x02\x02\u{52f}\u{530}\x07\x2e\x02\x02\u{530}\u{539}\
	\x07\x0b\x02\x02\u{531}\u{536}\x05\u{88}\x45\x02\u{532}\u{533}\x07\x08\x02\
	\x02\u{533}\u{535}\x05\u{88}\x45\x02\u{534}\u{532}\x03\x02\x02\x02\u{535}\
	\u{538}\x03\x02\x02\x02\u{536}\u{534}\x03\x02\x02\x02\u{536}\u{537}\x03\
	\x02\x02\x02\u{537}\u{53a}\x03\x02\x02\x02\u{538}\u{536}\x03\x02\x02\x02\
	\u{539}\u{531}\x03\x02\x02\x02\u{539}\u{53a}\x03\x02\x02\x02\u{53a}\u{53b}\
	\x03\x02\x02\x02\u{53b}\u{589}\x07\x0d\x02\x02\u{53c}\u{549}\x07\x6d\x02\
	\x02\u{53d}\u{546}\x07\x0b\x02\x02\u{53e}\u{543}\x05\u{88}\x45\x02\u{53f}\
	\u{540}\x07\x08\x02\x02\u{540}\u{542}\x05\u{88}\x45\x02\u{541}\u{53f}\x03\
	\x02\x02\x02\u{542}\u{545}\x03\x02\x02\x02\u{543}\u{541}\x03\x02\x02\x02\
	\u{543}\u{544}\x03\x02\x02\x02\u{544}\u{547}\x03\x02\x02\x02\u{545}\u{543}\
	\x03\x02\x02\x02\u{546}\u{53e}\x03\x02\x02\x02\u{546}\u{547}\x03\x02\x02\
	\x02\u{547}\u{548}\x03\x02\x02\x02\u{548}\u{54a}\x07\x0d\x02\x02\u{549}\
	\u{53d}\x03\x02\x02\x02\u{549}\u{54a}\x03\x02\x02\x02\u{54a}\u{589}\x03\
	\x02\x02\x02\u{54b}\u{54c}\x07\x59\x02\x02\u{54c}\u{589}\x05\u{96}\x4c\x09\
	\u{54d}\u{54e}\x07\x5a\x02\x02\u{54e}\u{589}\x05\u{96}\x4c\x08\u{54f}\u{550}\
	\x05\u{82}\x42\x02\u{550}\u{560}\x07\x5b\x02\x02\u{551}\u{552}\x07\x17\x02\
	\x02\u{552}\u{557}\x05\u{88}\x45\x02\u{553}\u{554}\x07\x08\x02\x02\u{554}\
	\u{556}\x05\u{88}\x45\x02\u{555}\u{553}\x03\x02\x02\x02\u{556}\u{559}\x03\
	\x02\x02\x02\u{557}\u{555}\x03\x02\x02\x02\u{557}\u{558}\x03\x02\x02\x02\
	\u{558}\u{55a}\x03\x02\x02\x02\u{559}\u{557}\x03\x02\x02\x02\u{55a}\u{55b}\
	\x07\x18\x02\x02\u{55b}\u{561}\x03\x02\x02\x02\u{55c}\u{55d}\x07\x0b\x02\
	\x02\u{55d}\u{55e}\x05\u{88}\x45\x02\u{55e}\u{55f}\x07\x0d\x02\x02\u{55f}\
	\u{561}\x03\x02\x02\x02\u{560}\u{551}\x03\x02\x02\x02\u{560}\u{55c}\x03\
	\x02\x02\x02\u{561}\u{589}\x03\x02\x02\x02\u{562}\u{564}\x07\x1e\x02\x02\
	\u{563}\u{562}\x03\x02\x02\x02\u{563}\u{564}\x03\x02\x02\x02\u{564}\u{56a}\
	\x03\x02\x02\x02\u{565}\u{566}\x05\x0a\x06\x02\u{566}\u{567}\x07\x09\x02\
	\x02\u{567}\u{569}\x03\x02\x02\x02\u{568}\u{565}\x03\x02\x02\x02\u{569}\
	\u{56c}\x03\x02\x02\x02\u{56a}\u{568}\x03\x02\x02\x02\u{56a}\u{56b}\x03\
	\x02\x02\x02\u{56b}\u{56d}\x03\x02\x02\x02\u{56c}\u{56a}\x03\x02\x02\x02\
	\u{56d}\u{574}\x05\x0a\x06\x02\u{56e}\u{56f}\x07\x17\x02\x02\u{56f}\u{570}\
	\x05\u{98}\x4d\x02\u{570}\u{571}\x07\x18\x02\x02\u{571}\u{575}\x03\x02\x02\
	\x02\u{572}\u{575}\x05\u{96}\x4c\x02\u{573}\u{575}\x03\x02\x02\x02\u{574}\
	\u{56e}\x03\x02\x02\x02\u{574}\u{572}\x03\x02\x02\x02\u{574}\u{573}\x03\
	\x02\x02\x02\u{575}\u{589}\x03\x02\x02\x02\u{576}\u{577}\x07\x0f\x02\x02\
	\u{577}\u{579}\x05\u{82}\x42\x02\u{578}\u{57a}\x07\x04\x02\x02\u{579}\u{578}\
	\x03\x02\x02\x02\u{579}\u{57a}\x03\x02\x02\x02\u{57a}\u{57e}\x03\x02\x02\
	\x02\u{57b}\u{57d}\x05\u{9c}\x4f\x02\u{57c}\u{57b}\x03\x02\x02\x02\u{57d}\
	\u{580}\x03\x02\x02\x02\u{57e}\u{57c}\x03\x02\x02\x02\u{57e}\u{57f}\x03\
	\x02\x02\x02\u{57f}\u{581}\x03\x02\x02\x02\u{580}\u{57e}\x03\x02\x02\x02\
	\u{581}\u{584}\x07\x10\x02\x02\u{582}\u{583}\x07\x06\x02\x02\u{583}\u{585}\
	\x05\u{84}\x43\x02\u{584}\u{582}\x03\x02\x02\x02\u{584}\u{585}\x03\x02\x02\
	\x02\u{585}\u{589}\x03\x02\x02\x02\u{586}\u{589}\x05\u{9e}\x50\x02\u{587}\
	\u{589}\x05\u{a2}\x52\x02\u{588}\u{503}\x03\x02\x02\x02\u{588}\u{50d}\x03\
	\x02\x02\x02\u{588}\u{517}\x03\x02\x02\x02\u{588}\u{51b}\x03\x02\x02\x02\
	\u{588}\u{51c}\x03\x02\x02\x02\u{588}\u{51d}\x03\x02\x02\x02\u{588}\u{51e}\
	\x03\x02\x02\x02\u{588}\u{51f}\x03\x02\x02\x02\u{588}\u{524}\x03\x02\x02\
	\x02\u{588}\u{52f}\x03\x02\x02\x02\u{588}\u{53c}\x03\x02\x02\x02\u{588}\
	\u{54b}\x03\x02\x02\x02\u{588}\u{54d}\x03\x02\x02\x02\u{588}\u{54f}\x03\
	\x02\x02\x02\u{588}\u{563}\x03\x02\x02\x02\u{588}\u{576}\x03\x02\x02\x02\
	\u{588}\u{586}\x03\x02\x02\x02\u{588}\u{587}\x03\x02\x02\x02\u{589}\u{5a9}\
	\x03\x02\x02\x02\u{58a}\u{58b}\x0c\x16\x02\x02\u{58b}\u{58c}\x07\x35\x02\
	\x02\u{58c}\u{5a8}\x05\x0c\x07\x02\u{58d}\u{58e}\x0c\x0d\x02\x02\u{58e}\
	\u{58f}\x07\x19\x02\x02\u{58f}\u{596}\x05\u{88}\x45\x02\u{590}\u{591}\x07\
	\x06\x02\x02\u{591}\u{597}\x05\u{88}\x45\x02\u{592}\u{593}\x07\x36\x02\x02\
	\u{593}\u{597}\x07\x6f\x02\x02\u{594}\u{595}\x07\x37\x02\x02\u{595}\u{597}\
	\x07\x6f\x02\x02\u{596}\u{590}\x03\x02\x02\x02\u{596}\u{592}\x03\x02\x02\
	\x02\u{596}\u{594}\x03\x02\x02\x02\u{596}\u{597}\x03\x02\x02\x02\u{597}\
	\u{598}\x03\x02\x02\x02\u{598}\u{599}\x07\x1a\x02\x02\u{599}\u{5a8}\x03\
	\x02\x02\x02\u{59a}\u{59b}\x0c\x0c\x02\x02\u{59b}\u{5a4}\x07\x0b\x02\x02\
	\u{59c}\u{5a1}\x05\u{88}\x45\x02\u{59d}\u{59e}\x07\x08\x02\x02\u{59e}\u{5a0}\
	\x05\u{88}\x45\x02\u{59f}\u{59d}\x03\x02\x02\x02\u{5a0}\u{5a3}\x03\x02\x02\
	\x02\u{5a1}\u{59f}\x03\x02\x02\x02\u{5a1}\u{5a2}\x03\x02\x02\x02\u{5a2}\
	\u{5a5}\x03\x02\x02\x02\u{5a3}\u{5a1}\x03\x02\x02\x02\u{5a4}\u{59c}\x03\
	\x02\x02\x02\u{5a4}\u{5a5}\x03\x02\x02\x02\u{5a5}\u{5a6}\x03\x02\x02\x02\
	\u{5a6}\u{5a8}\x07\x0d\x02\x02\u{5a7}\u{58a}\x03\x02\x02\x02\u{5a7}\u{58d}\
	\x03\x02\x02\x02\u{5a7}\u{59a}\x03\x02\x02\x02\u{5a8}\u{5ab}\x03\x02\x02\
	\x02\u{5a9}\u{5a7}\x03\x02\x02\x02\u{5a9}\u{5aa}\x03\x02\x02\x02\u{5aa}\
	\u{97}\x03\x02\x02\x02\u{5ab}\u{5a9}\x03\x02\x02\x02\u{5ac}\u{5b1}\x05\u{9a}\
	\x4e\x02\u{5ad}\u{5ae}\x07\x08\x02\x02\u{5ae}\u{5b0}\x05\u{9a}\x4e\x02\u{5af}\
	\u{5ad}\x03\x02\x02\x02\u{5b0}\u{5b3}\x03\x02\x02\x02\u{5b1}\u{5af}\x03\
	\x02\x02\x02\u{5b1}\u{5b2}\x03\x02\x02\x02\u{5b2}\u{99}\x03\x02\x02\x02\
	\u{5b3}\u{5b1}\x03\x02\x02\x02\u{5b4}\u{5b5}\x05\x08\x05\x02\u{5b5}\u{5b6}\
	\x07\x06\x02\x02\u{5b6}\u{5b7}\x05\u{88}\x45\x02\u{5b7}\u{9b}\x03\x02\x02\
	\x02\u{5b8}\u{5bd}\x05\x66\x34\x02\u{5b9}\u{5bd}\x05\x6e\x38\x02\u{5ba}\
	\u{5bd}\x05\x44\x23\x02\u{5bb}\u{5bd}\x05\x7e\x40\x02\u{5bc}\u{5b8}\x03\
	\x02\x02\x02\u{5bc}\u{5b9}\x03\x02\x02\x02\u{5bc}\u{5ba}\x03\x02\x02\x02\
	\u{5bc}\u{5bb}\x03\x02\x02\x02\u{5bd}\u{9d}\x03\x02\x02\x02\u{5be}\u{5c0}\
	\x05\u{ca}\x66\x02\u{5bf}\u{5be}\x03\x02\x02\x02\u{5c0}\u{5c3}\x03\x02\x02\
	\x02\u{5c1}\u{5bf}\x03\x02\x02\x02\u{5c1}\u{5c2}\x03\x02\x02\x02\u{5c2}\
	\u{5c4}\x03\x02\x02\x02\u{5c3}\u{5c1}\x03\x02\x02\x02\u{5c4}\u{5c7}\x07\
	\x5c\x02\x02\u{5c5}\u{5c6}\x07\x06\x02\x02\u{5c6}\u{5c8}\x05\x08\x05\x02\
	\u{5c7}\u{5c5}\x03\x02\x02\x02\u{5c7}\u{5c8}\x03\x02\x02\x02\u{5c8}\u{5cc}\
	\x03\x02\x02\x02\u{5c9}\u{5cb}\x05\u{a6}\x54\x02\u{5ca}\u{5c9}\x03\x02\x02\
	\x02\u{5cb}\u{5ce}\x03\x02\x02\x02\u{5cc}\u{5ca}\x03\x02\x02\x02\u{5cc}\
	\u{5cd}\x03\x02\x02\x02\u{5cd}\u{5cf}\x03\x02\x02\x02\u{5ce}\u{5cc}\x03\
	\x02\x02\x02\u{5cf}\u{5d2}\x07\x5d\x02\x02\u{5d0}\u{5d1}\x07\x06\x02\x02\
	\u{5d1}\u{5d3}\x05\x08\x05\x02\u{5d2}\u{5d0}\x03\x02\x02\x02\u{5d2}\u{5d3}\
	\x03\x02\x02\x02\u{5d3}\u{9f}\x03\x02\x02\x02\u{5d4}\u{5d6}\x05\u{ca}\x66\
	\x02\u{5d5}\u{5d4}\x03\x02\x02\x02\u{5d6}\u{5d9}\x03\x02\x02\x02\u{5d7}\
	\u{5d5}\x03\x02\x02\x02\u{5d7}\u{5d8}\x03\x02\x02\x02\u{5d8}\u{5da}\x03\
	\x02\x02\x02\u{5d9}\u{5d7}\x03\x02\x02\x02\u{5da}\u{5de}\x07\x5e\x02\x02\
	\u{5db}\u{5dd}\x05\u{a6}\x54\x02\u{5dc}\u{5db}\x03\x02\x02\x02\u{5dd}\u{5e0}\
	\x03\x02\x02\x02\u{5de}\u{5dc}\x03\x02\x02\x02\u{5de}\u{5df}\x03\x02\x02\
	\x02\u{5df}\u{5e1}\x03\x02\x02\x02\u{5e0}\u{5de}\x03\x02\x02\x02\u{5e1}\
	\u{5e2}\x07\x5f\x02\x02\u{5e2}\u{a1}\x03\x02\x02\x02\u{5e3}\u{5e5}\x05\u{ca}\
	\x66\x02\u{5e4}\u{5e3}\x03\x02\x02\x02\u{5e5}\u{5e8}\x03\x02\x02\x02\u{5e6}\
	\u{5e4}\x03\x02\x02\x02\u{5e6}\u{5e7}\x03\x02\x02\x02\u{5e7}\u{5e9}\x03\
	\x02\x02\x02\u{5e8}\u{5e6}\x03\x02\x02\x02\u{5e9}\u{5ed}\x07\x60\x02\x02\
	\u{5ea}\u{5ec}\x05\u{a6}\x54\x02\u{5eb}\u{5ea}\x03\x02\x02\x02\u{5ec}\u{5ef}\
	\x03\x02\x02\x02\u{5ed}\u{5eb}\x03\x02\x02\x02\u{5ed}\u{5ee}\x03\x02\x02\
	\x02\u{5ee}\u{5f0}\x03\x02\x02\x02\u{5ef}\u{5ed}\x03\x02\x02\x02\u{5f0}\
	\u{5f1}\x07\x61\x02\x02\u{5f1}\u{a3}\x03\x02\x02\x02\u{5f2}\u{5f3}\x05\u{80}\
	\x41\x02\u{5f3}\u{5f4}\x07\x46\x02\x02\u{5f4}\u{5f5}\x05\u{88}\x45\x02\u{5f5}\
	\u{a5}\x03\x02\x02\x02\u{5f6}\u{60b}\x05\x44\x23\x02\u{5f7}\u{60b}\x05\x46\
	\x24\x02\u{5f8}\u{60b}\x05\x48\x25\x02\u{5f9}\u{60b}\x05\x7e\x40\x02\u{5fa}\
	\u{60b}\x05\x74\x3b\x02\u{5fb}\u{60b}\x05\x70\x39\x02\u{5fc}\u{5fd}\x05\
	\u{a4}\x53\x02\u{5fd}\u{5fe}\x07\x04\x02\x02\u{5fe}\u{60b}\x03\x02\x02\x02\
	\u{5ff}\u{60b}\x05\u{9e}\x50\x02\u{600}\u{60b}\x05\u{a8}\x55\x02\u{601}\
	\u{60b}\x05\u{aa}\x56\x02\u{602}\u{60b}\x05\u{b4}\x5b\x02\u{603}\u{60b}\
	\x05\u{b2}\x5a\x02\u{604}\u{605}\x05\u{88}\x45\x02\u{605}\u{606}\x07\x04\
	\x02\x02\u{606}\u{60b}\x03\x02\x02\x02\u{607}\u{60b}\x05\u{c0}\x61\x02\u{608}\
	\u{60b}\x05\u{a0}\x51\x02\u{609}\u{60b}\x05\u{a2}\x52\x02\u{60a}\u{5f6}\
	\x03\x02\x02\x02\u{60a}\u{5f7}\x03\x02\x02\x02\u{60a}\u{5f8}\x03\x02\x02\
	\x02\u{60a}\u{5f9}\x03\x02\x02\x02\u{60a}\u{5fa}\x03\x02\x02\x02\u{60a}\
	\u{5fb}\x03\x02\x02\x02\u{60a}\u{5fc}\x03\x02\x02\x02\u{60a}\u{5ff}\x03\
	\x02\x02\x02\u{60a}\u{600}\x03\x02\x02\x02\u{60a}\u{601}\x03\x02\x02\x02\
	\u{60a}\u{602}\x03\x02\x02\x02\u{60a}\u{603}\x03\x02\x02\x02\u{60a}\u{604}\
	\x03\x02\x02\x02\u{60a}\u{607}\x03\x02\x02\x02\u{60a}\u{608}\x03\x02\x02\
	\x02\u{60a}\u{609}\x03\x02\x02\x02\u{60b}\u{a7}\x03\x02\x02\x02\u{60c}\u{60d}\
	\x07\x2f\x02\x02\u{60d}\u{60e}\x07\x0b\x02\x02\u{60e}\u{60f}\x05\u{88}\x45\
	\x02\u{60f}\u{610}\x07\x0d\x02\x02\u{610}\u{613}\x05\u{a6}\x54\x02\u{611}\
	\u{612}\x07\x62\x02\x02\u{612}\u{614}\x05\u{a6}\x54\x02\u{613}\u{611}\x03\
	\x02\x02\x02\u{613}\u{614}\x03\x02\x02\x02\u{614}\u{a9}\x03\x02\x02\x02\
	\u{615}\u{616}\x07\x3a\x02\x02\u{616}\u{617}\x07\x0b\x02\x02\u{617}\u{618}\
	\x05\u{88}\x45\x02\u{618}\u{625}\x07\x0d\x02\x02\u{619}\u{61b}\x07\x39\x02\
	\x02\u{61a}\u{61c}\x05\u{ae}\x58\x02\u{61b}\u{61a}\x03\x02\x02\x02\u{61c}\
	\u{61d}\x03\x02\x02\x02\u{61d}\u{61b}\x03\x02\x02\x02\u{61d}\u{61e}\x03\
	\x02\x02\x02\u{61e}\u{626}\x03\x02\x02\x02\u{61f}\u{621}\x05\u{ac}\x57\x02\
	\u{620}\u{61f}\x03\x02\x02\x02\u{621}\u{624}\x03\x02\x02\x02\u{622}\u{620}\
	\x03\x02\x02\x02\u{622}\u{623}\x03\x02\x02\x02\u{623}\u{626}\x03\x02\x02\
	\x02\u{624}\u{622}\x03\x02\x02\x02\u{625}\u{619}\x03\x02\x02\x02\u{625}\
	\u{622}\x03\x02\x02\x02\u{626}\u{628}\x03\x02\x02\x02\u{627}\u{629}\x05\
	\u{b0}\x59\x02\u{628}\u{627}\x03\x02\x02\x02\u{628}\u{629}\x03\x02\x02\x02\
	\u{629}\u{62a}\x03\x02\x02\x02\u{62a}\u{62b}\x07\x3b\x02\x02\u{62b}\u{ab}\
	\x03\x02\x02\x02\u{62c}\u{631}\x05\u{88}\x45\x02\u{62d}\u{62e}\x07\x08\x02\
	\x02\u{62e}\u{630}\x05\u{88}\x45\x02\u{62f}\u{62d}\x03\x02\x02\x02\u{630}\
	\u{633}\x03\x02\x02\x02\u{631}\u{62f}\x03\x02\x02\x02\u{631}\u{632}\x03\
	\x02\x02\x02\u{632}\u{634}\x03\x02\x02\x02\u{633}\u{631}\x03\x02\x02\x02\
	\u{634}\u{635}\x07\x06\x02\x02\u{635}\u{636}\x05\u{a6}\x54\x02\u{636}\u{ad}\
	\x03\x02\x02\x02\u{637}\u{63b}\x05\u{c2}\x62\x02\u{638}\u{63a}\x05\u{90}\
	\x49\x02\u{639}\u{638}\x03\x02\x02\x02\u{63a}\u{63d}\x03\x02\x02\x02\u{63b}\
	\u{639}\x03\x02\x02\x02\u{63b}\u{63c}\x03\x02\x02\x02\u{63c}\u{63e}\x03\
	\x02\x02\x02\u{63d}\u{63b}\x03\x02\x02\x02\u{63e}\u{63f}\x07\x06\x02\x02\
	\u{63f}\u{640}\x05\u{a6}\x54\x02\u{640}\u{af}\x03\x02\x02\x02\u{641}\u{643}\
	\x07\x3c\x02\x02\u{642}\u{644}\x07\x06\x02\x02\u{643}\u{642}\x03\x02\x02\
	\x02\u{643}\u{644}\x03\x02\x02\x02\u{644}\u{645}\x03\x02\x02\x02\u{645}\
	\u{646}\x05\u{a6}\x54\x02\u{646}\u{b1}\x03\x02\x02\x02\u{647}\u{648}\x07\
	\x63\x02\x02\u{648}\u{649}\x07\x0b\x02\x02\u{649}\u{64a}\x05\u{88}\x45\x02\
	\u{64a}\u{64b}\x07\x0d\x02\x02\u{64b}\u{64c}\x05\u{a6}\x54\x02\u{64c}\u{b3}\
	\x03\x02\x02\x02\u{64d}\u{64e}\x07\x64\x02\x02\u{64e}\u{64f}\x07\x0b\x02\
	\x02\u{64f}\u{650}\x05\u{b6}\x5c\x02\u{650}\u{651}\x07\x04\x02\x02\u{651}\
	\u{652}\x05\u{ba}\x5e\x02\u{652}\u{653}\x07\x04\x02\x02\u{653}\u{654}\x05\
	\u{bc}\x5f\x02\u{654}\u{655}\x07\x0d\x02\x02\u{655}\u{656}\x05\u{a6}\x54\
	\x02\u{656}\u{b5}\x03\x02\x02\x02\u{657}\u{658}\x05\u{82}\x42\x02\u{658}\
	\u{659}\x05\x08\x05\x02\u{659}\u{65a}\x07\x1b\x02\x02\u{65a}\u{65f}\x05\
	\u{88}\x45\x02\u{65b}\u{65c}\x07\x08\x02\x02\u{65c}\u{65e}\x05\u{b8}\x5d\
	\x02\u{65d}\u{65b}\x03\x02\x02\x02\u{65e}\u{661}\x03\x02\x02\x02\u{65f}\
	\u{65d}\x03\x02\x02\x02\u{65f}\u{660}\x03\x02\x02\x02\u{660}\u{b7}\x03\x02\
	\x02\x02\u{661}\u{65f}\x03\x02\x02\x02\u{662}\u{664}\x05\u{82}\x42\x02\u{663}\
	\u{662}\x03\x02\x02\x02\u{663}\u{664}\x03\x02\x02\x02\u{664}\u{665}\x03\
	\x02\x02\x02\u{665}\u{666}\x05\x08\x05\x02\u{666}\u{667}\x07\x1b\x02\x02\
	\u{667}\u{668}\x05\u{88}\x45\x02\u{668}\u{b9}\x03\x02\x02\x02\u{669}\u{66a}\
	\x05\u{88}\x45\x02\u{66a}\u{bb}\x03\x02\x02\x02\u{66b}\u{670}\x05\u{be}\
	\x60\x02\u{66c}\u{66d}\x07\x08\x02\x02\u{66d}\u{66f}\x05\u{be}\x60\x02\u{66e}\
	\u{66c}\x03\x02\x02\x02\u{66f}\u{672}\x03\x02\x02\x02\u{670}\u{66e}\x03\
	\x02\x02\x02\u{670}\u{671}\x03\x02\x02\x02\u{671}\u{bd}\x03\x02\x02\x02\
	\u{672}\u{670}\x03\x02\x02\x02\u{673}\u{674}\x05\x08\x05\x02\u{674}\u{675}\
	\x07\x1b\x02\x02\u{675}\u{676}\x05\u{88}\x45\x02\u{676}\u{bf}\x03\x02\x02\
	\x02\u{677}\u{678}\x07\x65\x02\x02\u{678}\u{679}\x05\u{88}\x45\x02\u{679}\
	\u{67a}\x07\x04\x02\x02\u{67a}\u{c1}\x03\x02\x02\x02\u{67b}\u{67c}\x07\x35\
	\x02\x02\u{67c}\u{686}\x05\x08\x05\x02\u{67d}\u{686}\x07\x66\x02\x02\u{67e}\
	\u{686}\x05\u{c4}\x63\x02\u{67f}\u{686}\x05\u{c6}\x64\x02\u{680}\u{686}\
	\x05\u{c8}\x65\x02\u{681}\u{682}\x07\x0b\x02\x02\u{682}\u{683}\x05\u{c2}\
	\x62\x02\u{683}\u{684}\x07\x0d\x02\x02\u{684}\u{686}\x03\x02\x02\x02\u{685}\
	\u{67b}\x03\x02\x02\x02\u{685}\u{67d}\x03\x02\x02\x02\u{685}\u{67e}\x03\
	\x02\x02\x02\u{685}\u{67f}\x03\x02\x02\x02\u{685}\u{680}\x03\x02\x02\x02\
	\u{685}\u{681}\x03\x02\x02\x02\u{686}\u{c3}\x03\x02\x02\x02\u{687}\u{688}\
	\x09\x12\x02\x02\u{688}\u{c5}\x03\x02\x02\x02\u{689}\u{68b}\x07\x1e\x02\
	\x02\u{68a}\u{689}\x03\x02\x02\x02\u{68a}\u{68b}\x03\x02\x02\x02\u{68b}\
	\u{68c}\x03\x02\x02\x02\u{68c}\u{68e}\x05\x0a\x06\x02\u{68d}\u{68f}\x05\
	\u{c2}\x62\x02\u{68e}\u{68d}\x03\x02\x02\x02\u{68e}\u{68f}\x03\x02\x02\x02\
	\u{68f}\u{6a2}\x03\x02\x02\x02\u{690}\u{691}\x05\x0a\x06\x02\u{691}\u{692}\
	\x07\x17\x02\x02\u{692}\u{693}\x05\x08\x05\x02\u{693}\u{694}\x07\x06\x02\
	\x02\u{694}\u{69c}\x05\u{c2}\x62\x02\u{695}\u{696}\x07\x08\x02\x02\u{696}\
	\u{697}\x05\x08\x05\x02\u{697}\u{698}\x07\x06\x02\x02\u{698}\u{699}\x05\
	\u{c2}\x62\x02\u{699}\u{69b}\x03\x02\x02\x02\u{69a}\u{695}\x03\x02\x02\x02\
	\u{69b}\u{69e}\x03\x02\x02\x02\u{69c}\u{69a}\x03\x02\x02\x02\u{69c}\u{69d}\
	\x03\x02\x02\x02\u{69d}\u{69f}\x03\x02\x02\x02\u{69e}\u{69c}\x03\x02\x02\
	\x02\u{69f}\u{6a0}\x07\x18\x02\x02\u{6a0}\u{6a2}\x03\x02\x02\x02\u{6a1}\
	\u{68a}\x03\x02\x02\x02\u{6a1}\u{690}\x03\x02\x02\x02\u{6a2}\u{c7}\x03\x02\
	\x02\x02\u{6a3}\u{6a4}\x07\x17\x02\x02\u{6a4}\u{6a9}\x05\u{c2}\x62\x02\u{6a5}\
	\u{6a6}\x07\x08\x02\x02\u{6a6}\u{6a8}\x05\u{c2}\x62\x02\u{6a7}\u{6a5}\x03\
	\x02\x02\x02\u{6a8}\u{6ab}\x03\x02\x02\x02\u{6a9}\u{6a7}\x03\x02\x02\x02\
	\u{6a9}\u{6aa}\x03\x02\x02\x02\u{6aa}\u{6ac}\x03\x02\x02\x02\u{6ab}\u{6a9}\
	\x03\x02\x02\x02\u{6ac}\u{6ad}\x07\x18\x02\x02\u{6ad}\u{c9}\x03\x02\x02\
	\x02\u{6ae}\u{6af}\x07\x67\x02\x02\u{6af}\u{6b4}\x05\u{cc}\x67\x02\u{6b0}\
	\u{6b1}\x07\x08\x02\x02\u{6b1}\u{6b3}\x05\u{cc}\x67\x02\u{6b2}\u{6b0}\x03\
	\x02\x02\x02\u{6b3}\u{6b6}\x03\x02\x02\x02\u{6b4}\u{6b2}\x03\x02\x02\x02\
	\u{6b4}\u{6b5}\x03\x02\x02\x02\u{6b5}\u{6b7}\x03\x02\x02\x02\u{6b6}\u{6b4}\
	\x03\x02\x02\x02\u{6b7}\u{6b8}\x07\x68\x02\x02\u{6b8}\u{cb}\x03\x02\x02\
	\x02\u{6b9}\u{6bc}\x05\x0c\x07\x02\u{6ba}\u{6bb}\x07\x1b\x02\x02\u{6bb}\
	\u{6bd}\x05\u{88}\x45\x02\u{6bc}\u{6ba}\x03\x02\x02\x02\u{6bc}\u{6bd}\x03\
	\x02\x02\x02\u{6bd}\u{cd}\x03\x02\x02\x02\u{6be}\u{6bf}\x07\x69\x02\x02\
	\u{6bf}\u{6c0}\x07\x0b\x02\x02\u{6c0}\u{6c5}\x05\u{d0}\x69\x02\u{6c1}\u{6c2}\
	\x07\x08\x02\x02\u{6c2}\u{6c4}\x05\u{d0}\x69\x02\u{6c3}\u{6c1}\x03\x02\x02\
	\x02\u{6c4}\u{6c7}\x03\x02\x02\x02\u{6c5}\u{6c3}\x03\x02\x02\x02\u{6c5}\
	\u{6c6}\x03\x02\x02\x02\u{6c6}\u{6c8}\x03\x02\x02\x02\u{6c7}\u{6c5}\x03\
	\x02\x02\x02\u{6c8}\u{6c9}\x07\x0d\x02\x02\u{6c9}\u{cf}\x03\x02\x02\x02\
	\u{6ca}\u{6cb}\x05\x0a\x06\x02\u{6cb}\u{6cc}\x07\x09\x02\x02\u{6cc}\u{6ce}\
	\x03\x02\x02\x02\u{6cd}\u{6ca}\x03\x02\x02\x02\u{6cd}\u{6ce}\x03\x02\x02\
	\x02\u{6ce}\u{6cf}\x03\x02\x02\x02\u{6cf}\u{6d0}\x05\x0a\x06\x02\u{6d0}\
	\u{6d1}\x07\x12\x02\x02\u{6d1}\u{6d2}\x07\x0b\x02\x02\u{6d2}\u{6d7}\x05\
	\u{82}\x42\x02\u{6d3}\u{6d4}\x07\x08\x02\x02\u{6d4}\u{6d6}\x05\u{82}\x42\
	\x02\u{6d5}\u{6d3}\x03\x02\x02\x02\u{6d6}\u{6d9}\x03\x02\x02\x02\u{6d7}\
	\u{6d5}\x03\x02\x02\x02\u{6d7}\u{6d8}\x03\x02\x02\x02\u{6d8}\u{6da}\x03\
	\x02\x02\x02\u{6d9}\u{6d7}\x03\x02\x02\x02\u{6da}\u{6db}\x07\x0d\x02\x02\
	\u{6db}\u{d1}\x03\x02\x02\x02\u{d2}\u{d6}\u{da}\u{e1}\u{e5}\u{ee}\u{f6}\
	\u{fe}\u{10b}\u{10d}\u{115}\u{127}\u{12e}\u{137}\u{13d}\u{141}\u{146}\u{14e}\
	\u{151}\u{154}\u{15d}\u{163}\u{167}\u{16e}\u{178}\u{181}\u{187}\u{18f}\u{19f}\
	\u{1a5}\u{1ae}\u{1b3}\u{1b8}\u{1ba}\u{1bf}\u{1c8}\u{1ce}\u{1d5}\u{1db}\u{1e1}\
	\u{1e7}\u{1f3}\u{201}\u{208}\u{20f}\u{215}\u{221}\u{229}\u{22e}\u{23c}\u{243}\
	\u{249}\u{24e}\u{253}\u{25a}\u{262}\u{267}\u{26e}\u{277}\u{283}\u{28a}\u{28d}\
	\u{293}\u{299}\u{2a4}\u{2b4}\u{2b9}\u{2be}\u{2c4}\u{2ca}\u{2ce}\u{2d3}\u{2df}\
	\u{2e4}\u{2ea}\u{2f0}\u{2f5}\u{2fa}\u{301}\u{307}\u{30e}\u{312}\u{31b}\u{322}\
	\u{32b}\u{331}\u{338}\u{33c}\u{343}\u{349}\u{34d}\u{353}\u{35a}\u{35e}\u{363}\
	\u{366}\u{369}\u{36c}\u{372}\u{378}\u{37c}\u{381}\u{384}\u{387}\u{38d}\u{394}\
	\u{39a}\u{39e}\u{3a2}\u{3b0}\u{3b6}\u{3ba}\u{3c1}\u{3c6}\u{3cc}\u{3d2}\u{3d8}\
	\u{3dd}\u{3e6}\u{3ee}\u{3f4}\u{3fb}\u{3ff}\u{404}\u{407}\u{40a}\u{411}\u{418}\
	\u{422}\u{427}\u{42c}\u{437}\u{440}\u{448}\u{45d}\u{461}\u{46b}\u{470}\u{478}\
	\u{47f}\u{484}\u{491}\u{496}\u{499}\u{49c}\u{4a1}\u{4af}\u{4b2}\u{4b4}\u{4bb}\
	\u{4c7}\u{4f7}\u{4f9}\u{501}\u{50d}\u{517}\u{52a}\u{536}\u{539}\u{543}\u{546}\
	\u{549}\u{557}\u{560}\u{563}\u{56a}\u{574}\u{579}\u{57e}\u{584}\u{588}\u{596}\
	\u{5a1}\u{5a4}\u{5a7}\u{5a9}\u{5b1}\u{5bc}\u{5c1}\u{5c7}\u{5cc}\u{5d2}\u{5d7}\
	\u{5de}\u{5e6}\u{5ed}\u{60a}\u{613}\u{61d}\u{622}\u{625}\u{628}\u{631}\u{63b}\
	\u{643}\u{65f}\u{663}\u{670}\u{685}\u{68a}\u{68e}\u{69c}\u{6a1}\u{6a9}\u{6b4}\
	\u{6bc}\u{6c5}\u{6cd}\u{6d7}";

