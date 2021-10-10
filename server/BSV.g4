grammar BSV;

identifier: LOWER_IDENTIFIER;

any_identifier: identifier | UPPER_IDENTIFIER;

top: (exportDecl | importDecl | packageStmt | r_package)*;
r_package:
	'package' package_identifier ';' (
		exportDecl
		| importDecl
		| packageStmt
	)* 'endpackage' (':' package_identifier)?;

non_package: exportDecl* | importDecl* | packageStmt*;

exportDecl: 'export' exportItem (',' exportItem)* ';';
exportItem:
	package_identifier '::' '*'
	| any_identifier ('(' '..' ')')?;

importDecl: 'import' importItem (',' importItem)* ';';
importItem: package_identifier '::' '*';
packageStmt:
	module_def
	| interfaceDecl
	| typeDef
	| variable_declaration
	| variable_assignment
	| function_def
	| typeclassDef
	| typeclassInstanceDef
	| externModuleImport
	| externCImport;

package_identifier: UPPER_IDENTIFIER;

data_type: primary_type;

numeric_type: INT_LITERAL;

primary_type:
	type_identifier ('#' '(' data_type (',' data_type)* ')')?
	| numeric_type
	| 'bit' '[' numeric_type ':' numeric_type ']'; // SV Compatibility, Obselete

type_identifier: (pkg = UPPER_IDENTIFIER '::')* name = UPPER_IDENTIFIER
	| 'SizeOf'
	| typevar = identifier;

// Interface declaration

interfaceDecl:
	attribute_instances 'interface' typeDefType ';' interfaceMemberDecl* 'endinterface' (
		':' type_identifier
	)?;

typeDefType: type_identifier typeFormals?;
typeFormals: '#' '(' typeFormal (',' typeFormal)* ')';
typeFormal:
	'numeric'? 'type'? (type_identifier); // | typeDefType);

interfaceMemberDecl: methodProto | subinterfaceDecl;

methodProto:
	attribute_instances 'method' method_type name = identifier (
		'(' methodProtoFormals? ')'
	)? ';';

methodProtoFormals: methodProtoFormal ( ',' methodProtoFormal)*;
methodProtoFormal:
	attribute_instances data_type? name = identifier;

subinterfaceDecl:
	attribute_instances 'interface' typeDefType identifier ';'; //strange

module_def:
	attribute_instances module_proto (moduleStmt)* //strange
	'endmodule' (':' identifier)?;

module_proto:
	'module' ('[' monad = expression ']')? name = identifier (
		'#' '(' moduleprotoformals? ')'
	)? '(' moduleinterface = data_type ')' provisos? ';';

moduleprotoformals: moduleprotoformal (',' moduleprotoformal)*;
moduleprotoformal:
	attribute_instances 'parameter'? data_type name = identifier
	| function_proto;

moduleFormalArgs:
	attribute_instances data_type
	| attribute_instances (
		data_type identifier '[]'?
	) (
		',' attribute_instances (
			data_type identifier '[]'?
		)
	)*;

moduleStmt:
	method_def
	| subinterfaceDef
	| ruleDef
	| varDo
	| varDeclDo
	| expression ';'
	| returnStmt
	| variable_declaration
	| variable_assignment
	| function_def
	| module_def
	| beginEndStmt_moduleStmt
	| if_moduleStmt
	| case_moduleStmt
	| for_moduleStmt
	| while_moduleStmt;

method_type:
	data_type?
	| 'Action'
	| 'ActionValue' '#' '(' data_type (',' data_type)* ')';

method_def: // implementation
	'method' method_type identifier ('(' methodFormals? ')')? implicitCond? ';' (
		functionBody
		| actionStmt*
		| actionValueStmt*
		// FIXME?
	) 'endmethod' (':' identifier)?
	| 'method' method_type identifier ('(' methodFormals? ')')? implicitCond? '=' expression ';';

implicitCond: 'if'? '(' cond_predicate ')';
methodFormals: methodFormal (',' methodFormal)*;
methodFormal: data_type? identifier;

// subinterfaces
subinterfaceDef:
	'interface' UPPER_IDENTIFIER identifier ';' interfaceStmt* 'endinterface' (
		':' identifier
	)?
	| 'interface' data_type? identifier '=' expression ';';

interfaceStmt: method_def | subinterfaceDef | expressionStmt;

expressionStmt:
	variable_declaration
	| variable_assignment
	| function_def
	| beginEndStmt_expressionStmt
	| if_expressionStmt
	| case_expressionStmt
	| for_expressionStmt
	| while_expressionStmt;

// Rules in module definitions

ruleDef:
	attribute_instances 'rule' identifier ruleCond? ';' ruleBody 'endrule' (
		':' identifier
	)?;

ruleCond: 'if'? '(' cond_predicate ')';
ruleBody: actionStmt*;

// User-defined types

typeDef:
	typedefSynonym
	| typedefEnum
	| typedefStruct
	| typedefTaggedUnion;

typedefSynonym: 'typedef' data_type typeDefType ';';
typedefEnum:
	'typedef' 'enum' '{' typedefEnumElements '}' UPPER_IDENTIFIER derives? ';';
typedefEnumElements:
	typedefEnumElement (',' typedefEnumElement)*;
typedefEnumElement:
	UPPER_IDENTIFIER ('=' INT_LITERAL)?
	| UPPER_IDENTIFIER '[' INT_LITERAL ']' ('=' INT_LITERAL)?
	| UPPER_IDENTIFIER '[' INT_LITERAL ':' INT_LITERAL ']' (
		'=' INT_LITERAL
	)?;

typedefStruct:
	'typedef' 'struct' '{' structMember* '}' typeDefType derives? ';';

typedefTaggedUnion:
	'typedef' 'union' 'tagged' '{' unionMember* '}' typeDefType derives? ';';

structMember:
	data_type identifier ';'
	| subUnion identifier ';';

unionMember:
	data_type UPPER_IDENTIFIER ';'
	| subStruct UPPER_IDENTIFIER ';'
	| subUnion UPPER_IDENTIFIER ';'
	| 'void' UPPER_IDENTIFIER ';';

subStruct: 'struct' '{' structMember* '}';

subUnion: 'union' 'tagged' '{' unionMember* '}';

variable_declaration:
	attribute_instances ('let' | t = data_type) (
		(variable_initialization (',' variable_initialization)*)
	) ';'
	| 'match ';

patternbinding:
	attribute_instances 'match' pattern op = '=' rhs = expression ';'
	| attribute_instances 'match' pattern op = '<-' action = expression ';';

variable_initialization:
	variable_assignment
	| var = identifier op = '<-' action = expression; // actionvalue or module_inst

variable_assignment: var = identifier op = '=' rhs = expression;

lvalue:
	identifier
	| primary_expression '.' identifier
	| primary_expression '[' index = expression ']'
	| primary_expression '[' msb = expression (
		(':' lsb = expression)
		| ('+:' widthup = INT_LITERAL)
		| ('-:' widthdown = INT_LITERAL)
	) ']';

regWrite: lhs = lvalue '<=' rhs = expression;

// Begin-end statements
beginEndStmt_functionBodyStmt:
	'begin' (':' identifier)? functionBodyStmt* 'end' (
		':' identifier
	)?;

beginEndStmt_actionStmt:
	'begin' (':' identifier)? actionStmt* 'end' (':' identifier)?;

beginEndStmt_actionValueStmt:
	'begin' (':' identifier)? actionValueStmt* 'end' (
		':' identifier
	)?;

beginEndStmt_moduleStmt:
	'begin' (':' identifier)? moduleStmt* 'end' (':' identifier)?;

beginEndStmt_expressionStmt:
	'begin' (':' identifier)? expressionStmt* 'end' (
		':' identifier
	)?;

// beginEndStmt[args] : 'begin' (':' identifier)? stmt[args]* 'end' (':' identifier )?;

// Conditional statements
if_functionBodyStmt:
	'if' '(' cond_predicate ')' functionBodyStmt (
		'else' functionBodyStmt
	)?;

if_actionStmt:
	'if' '(' cond_predicate ')' actionStmt ('else' actionStmt)?;

if_actionValueStmt:
	'if' '(' cond_predicate ')' actionValueStmt (
		'else' actionValueStmt
	)?;

if_moduleStmt:
	'if' '(' cond_predicate ')' moduleStmt ('else' moduleStmt)?;

if_expressionStmt:
	'if' '(' cond_predicate ')' expressionStmt (
		'else' expressionStmt
	)?;

case_actionStmt:
	'case' '(' expression ')' caseItem_actionStmt* defaultItem_actionStmt? 'endcase'
	| 'case' '(' expression ')' 'matches' casePatItem_actionStmt* defaultItem_actionStmt? 'endcase';

case_actionValueStmt:
	'case' '(' expression ')' caseItem_actionValueStmt* defaultItem_actionValueStmt? 'endcase'
	| 'case' '(' expression ')' 'matches' casePatItem_actionValueStmt* defaultItem_actionValueStmt?
		'endcase';

case_moduleStmt:
	'case' '(' expression ')' caseItem_moduleStmt* defaultItem_moduleStmt? 'endcase'
	| 'case' '(' expression ')' 'matches' casePatItem_moduleStmt* defaultItem_moduleStmt? 'endcase';

case_expressionStmt:
	'case' '(' expression ')' caseItem_expressionStmt* defaultItem_expressionStmt? 'endcase'
	| 'case' '(' expression ')' 'matches' casePatItem_expressionStmt* defaultItem_expressionStmt?
		'endcase';

// caseItem[args]      : expression (',' expression) ':' stmt[args];

caseItem_functionBodyStmt:
	expression (',' expression)* ':' functionBodyStmt;
caseItem_actionStmt:
	expression (',' expression)* ':' actionStmt;
caseItem_actionValueStmt:
	expression (',' expression)* ':' actionValueStmt;
caseItem_moduleStmt:
	expression (',' expression)* ':' moduleStmt;
caseItem_expressionStmt:
	expression (',' expression)* ':' expressionStmt;

// defaultItem[args]   : 'default' ':'? stmt[args];
defaultItem_functionBodyStmt: 'default' ':'? functionBodyStmt;
defaultItem_actionStmt: 'default' ':'? actionStmt;
defaultItem_actionValueStmt: 'default' ':'? actionValueStmt;
defaultItem_moduleStmt: 'default' ':'? moduleStmt;
defaultItem_expressionStmt: 'default' ':'? expressionStmt;

// while loops while[args] : 'while' '(' expression ')' stmt[args];
while_functionBodyStmt:
	'while' '(' expression ')' functionBodyStmt;

while_actionStmt: 'while' '(' expression ')' actionStmt;

while_actionValueStmt:
	'while' '(' expression ')' actionValueStmt;

while_moduleStmt: 'while' '(' expression ')' moduleStmt;

while_expressionStmt: 'while' '(' expression ')' expressionStmt;

// for loops for[args] : 'for' '(' forInit ';' forTest ';' forIncr ')' stmt[args];
for_functionBodyStmt:
	'for' '(' forInit ';' forTest ';' forIncr ')' functionBodyStmt;

for_actionStmt:
	'for' '(' forInit ';' forTest ';' forIncr ')' actionStmt;

for_actionValueStmt:
	'for' '(' forInit ';' forTest ';' forIncr ')' actionValueStmt;

for_moduleStmt:
	'for' '(' forInit ';' forTest ';' forIncr ')' moduleStmt;

for_expressionStmt:
	'for' '(' forInit ';' forTest ';' forIncr ')' expressionStmt;

simpleVarDeclAssign:
	data_type? identifier '=' expression; // only in for loop
forInit:
	data_type? identifier '=' expression (
		',' simpleVarDeclAssign
	)*;
forTest: expression;
forIncr: varIncr (',' varIncr)*;
varIncr: identifier '=' expression;

function_def:
	attribute_instances function_proto functionBody 'endfunction' (
		':' identifier
	)?
	| 'function' data_type? (identifier | OPERATOR_IDENTIFIER) '(' function_formals? ')' provisos?
		'=' expression ';';
//strange

function_proto:
	'function' data_type? (identifier | OPERATOR_IDENTIFIER) ('(' function_formals? ')')? provisos? ';'; // too strange

function_formals: functionFormal (',' functionFormal)*;
functionFormal:
	data_type? identifier ('(' function_formals? ')')?; // strange

functionBody:
	actionBlock
	| actionValueBlock
	| functionBodyStmt*;

functionBodyStmt: returnStmt | expressionStmt;

returnStmt: 'return' expression ';';

// expression
expression:
	pred = expression '?' expression ':' expression	# condExpr
	| expression 'matches' pattern patterncond*		# matchesexpr
	| 'case' '(' expression ')' (
		('matches' caseexprpatitem+)
		| caseexpritem*
	) caseexprdefaultitem? 'endcase'	# caseexpr
	| binopexpr							# operatorexpr;

caseexprpatitem:
	(pattern patterncond*) ':' body = expression ';';
caseexpritem:
	match_ = expression (',' altmatches = expression)* ':' body = expression ';';
caseexprdefaultitem: 'default' ':' body = expression ';';

patterncond: ('&&&' expression);

unary_operator:
	'+'
	| '-'
	| '!'
	| '~'
	| '&'
	| '~&'
	| '|'
	| '~|'
	| '^'
	| '~^'
	| '^~';

binary_operator:
	'+'
	| '-'
	| '*'
	| '/'
	| '%'
	| '=='
	| '!='
	| '!=='
	| '&&'
	| '&&&'
	| '||'
	| '**'
	| '<'
	| '<='
	| '>'
	| '>='
	| '&'
	| '|'
	| '^'
	| '~^'
	| '^~'
	| '>>'
	| '<<';
binopexpr:
	left = binopexpr op = binary_operator right = binopexpr
	| unopexpr;
unopexpr:
	op = (
		'!'
		| '~'
		| '&'
		| '~&'
		| '|'
		| '~|'
		| '^'
		| '^~'
		| '~^'
	) primary_expression
	| op = ('+' | '-') right = primary_expression
	| primary_expression;

constant_expression: (unary_operator)? constant_primary
	| constant_expression binary_operator constant_expression
	| constant_expression '?' constant_expression ':' constant_expression;

constant_primary:
	INT_LITERAL
	| REAL_LITERAL
	| STRING_LITERAL
	| '?'
	| ('valueOf' | 'valueof') '(' data_type ')';

bit_concatination: '{' expression (',' expression)* '}';

primary_expression:
	'(' expression ')'												# parenexpr
	| primary_expression '.' field = any_identifier					# fieldexpr
	| (data_type | ( '(' data_type ')')) '\'' primary_expression	# castexpr
	| (pkg = UPPER_IDENTIFIER '::')* var = identifier				# varexpr
	| constant_expression											# constexpr
	| bit_concatination												# bitconcat
	| array = primary_expression '[' msb = constant_expression (
		(':' lsb = constant_expression)
	)? ']'																# bitselect
	| fcn = primary_expression '(' (expression (',' expression)*)? ')'	# callexpr
	| 'when' '(' (expression (',' expression)*)? ')'					# whenexpr
	| fcn = SYSTEM_TASK ('(' (expression (',' expression)*)? ')')?		# syscallexpr
	| 'clocked_by' primary_expression									# clockedbyexpr
	| 'reset_by' primary_expression										# resetbyexpr
	| data_type '’' (
		('{' expression (',' expression)* '}')
		| ( '(' expression ')')
	) # typeassertionexpr
	/* MIT BSV: optional tagged */
	| 'tagged'? (UPPER_IDENTIFIER '::')* tag = UPPER_IDENTIFIER (
		('{' memberbinds '}')
		| primary_expression
		|
	) # taggedunionexpr
	| 'interface' data_type (';')? (interfacestmt)* 'endinterface' (
		':' type_identifier
	)?					# interfaceexpr
	| block_expression	# blockexpr;

memberbinds: memberbind (',' memberbind)*;
memberbind: field = identifier ':' expression;

interfacestmt:
	method_def
	| subinterfaceDef
	| variable_declaration
	| variable_assignment;

cond_predicate: (expression | (expression 'matches' pattern)) (
		'&&&' (expression | (expression 'matches' pattern))
	)*;
exprOrCondPattern: expression | expression 'matches' pattern;

block_expression:
	'begin' (':' identifier)? expressionStmt* expression 'end' (
		':' identifier
	)?;

actionBlock: 'action' actionStmt* 'endaction';

actionStmt:
	regWrite
	| varDo
	| varDeclDo
	| expression ';'
	| actionBlock
	| expressionStmt //??
	| function_def
	| module_def
	| variable_declaration
	| beginEndStmt_actionStmt
	| if_actionStmt
	| case_actionStmt
	| for_actionStmt
	| while_actionStmt;

actionValueBlock:
	'actionvalue' actionValueStmt* 'endactionvalue';

actionValueStmt:
	regWrite
	| varDo
	| varDeclDo
	| actionValueBlock // strange
	| expression? ';' // strange
	| returnStmt
	| variable_declaration
	| variable_assignment
	| function_def
	| module_def
	| beginEndStmt_actionValueStmt
	| if_actionValueStmt
	| case_actionValueStmt
	| for_actionValueStmt
	| while_actionValueStmt;

varDeclDo:
	attribute_instances data_type identifier '<-' expression ';';
varDo: attribute_instances identifier '<-' expression ';';

typeAssertion:
	data_type '\'' bit_concatination
	| data_type '\'' '(' expression ')';

structExpr:
	UPPER_IDENTIFIER '{' (memberBind (',' memberBind)*)? '}'; // undoc
memberBind: identifier ':' expression;

taggedUnionExpr:
	'tagged' UPPER_IDENTIFIER '{' memberBind (',' memberBind) '}'
	| 'tagged' UPPER_IDENTIFIER primary_expression;

interfaceExpr:
	'interface' data_type ';'? interfaceStmt* 'endinterface' (
		':' UPPER_IDENTIFIER
	)?;

rulesExpr:
	attribute_instances 'rules' (':' identifier)? rulesStmt* // strange why
	'endrules' (':' identifier)?;

rulesStmt: ruleDef | expressionStmt;

// Pattern matching
pattern:
	'.' identifier
	| '.*'
	| constantPattern
	| taggedUnionPattern
	| structPattern
	| tuplePattern
	| '(' pattern ')'; //undoc

constantPattern:
	INT_LITERAL
	| REAL_LITERAL
	| STRING_LITERAL
	| UPPER_IDENTIFIER;

taggedUnionPattern: 'tagged' UPPER_IDENTIFIER pattern?;
structPattern:
	'tagged' UPPER_IDENTIFIER '{' identifier ':' pattern (
		',' identifier ':' pattern
	)* '}';
tuplePattern: '{' pattern (',' pattern)* '}';

// casePatItem[args]   : pattern ( '&&&' expression)? ':' stmt[args];
casePatItem_functionBodyStmt:
	pattern ('&&&' expression)? ':' functionBodyStmt;
casePatItem_actionStmt:
	pattern ('&&&' expression)? ':' actionStmt;
casePatItem_actionValueStmt:
	pattern ('&&&' expression)? ':' actionValueStmt;
casePatItem_moduleStmt:
	pattern ('&&&' expression)? ':' moduleStmt;
casePatItem_expressionStmt:
	pattern ('&&&' expression)? ':' expressionStmt;

caseExpr:
	'case' '(' expression ')' 'matches' caseExprItem* 'endcase';

caseExprItem:
	pattern ('&&&' expression)? ':' expression
	| 'default' ':'? expression;

// attributes
attribute_instance:
	'(*' attribute_spec (',' attribute_spec)* '*)';
attribute_instances: attribute_instance*;

attribute_spec: attrName ('=' expression)?;
attrName: identifier | UPPER_IDENTIFIER;

// Type classes (overloading groups) and provisos
provisos: 'provisos' '(' proviso (',' proviso)* ')';
proviso:
	UPPER_IDENTIFIER '#' '(' data_type (',' data_type)* ')';

typeclassDef:
	'typeclass' typeclassIde typeFormals provisos? typedepends? ';' overloadedDef* 'endtypeclass' (
		':' UPPER_IDENTIFIER
	)?;

typeclassIde: UPPER_IDENTIFIER;
typelist:
	type_identifier
	| '(' type_identifier (',' type_identifier)* ')';

typedepends:
	'dependencies' '(' typedepend (',' typedepend)* ')';
typedepend: typelist 'determines' typelist;

overloadedDef:
	function_proto
	| variable_declaration
	| module_proto // strange
	| module_def //strange
	| function_def; // strange

typeclassInstanceDef:
	'instance' typeclassIde '#' '(' data_type (',' data_type)* ')' provisos? ';' (
		variable_assignment
		| function_def
		| module_def
	)* 'endinstance' (':' typeclassIde)?;

derives: 'deriving' '(' typeclassIde (',' typeclassIde)* ')';

externModuleImport:
	'import' '"BVI"' (identifier '=')? module_proto moduleStmt* importBVIStmt* 'endmodule' (
		':' identifier
	)?;

importBVIStmt:
	parameterBVIStmt
	| methodBVIStmt
	| portBVIStmt
	| inputClockBVIStmt
	| defaultClockBVIStmt
	| outputClockBVIStmt
	| inputResetBVIStmt
	| defaultResetBVIStmt
	| noResetBVIStmt
	| outputResetBVIStmt
	| ancestorBVIStmt
	| sameFamilyBVIStmt
	| scheduleBVIStmt
	| pathBVIStmt
	| interfaceBVIStmt
	| inoutBVIStmt;

enabled_sel: ('enable' '(' portId ')');
ready_sel: ('ready' '(' portId ')');
clocked_by_sel: ('clocked_by' '(' clockId ')');
reset_by_sel: ('reset_by' '(' resetId ')');

parameterBVIStmt: 'parameter' identifier '=' expression ';';
methodBVIStmt:
	'method' portId? identifier ('(' (portId (',' portId)*)? ')')? (
		enabled_sel
		| ready_sel
		| clocked_by_sel
		| reset_by_sel
	)* ';';
portBVIStmt:
	'port' identifier (clocked_by_sel | reset_by_sel)* '=' expression ';';

inputClockBVIStmt:
	'input_clock' identifier? '(' portsDef? ')' '=' expression ';';
portsDef: portId (',' attribute_instances portId)?;
portId: attribute_instance? identifier;

defaultClockBVIStmt:
	'default_clock' identifier? ('(' portsDef? ')')? (
		'=' expression
	)? ';';
outputClockBVIStmt:
	'output_clock' identifier '(' portsDef? ')' ';';

inputResetBVIStmt:
	'input_reset' identifier? ('(' portId? ')')? clocked_by_sel? '=' expression ';';
clockId: identifier;
defaultResetBVIStmt:
	'default_reset' identifier ';'
	| 'default_reset' identifier? ('(' portId? ')')? clocked_by_sel? (
		'=' expression
	)? ';';

outputResetBVIStmt:
	'output_reset' identifier ('(' portId? ')')? clocked_by_sel? ';';
ancestorBVIStmt: 'ancestor' '(' clockId ',' clockId ')' ';';
sameFamilyBVIStmt:
	'same_family' '(' clockId ',' clockId ')' ';';
scheduleBVIStmt:
	'schedule' (
		'(' identifier (',' identifier)* ')'
		| identifier
	) operatorId (
		'(' identifier (',' identifier)* ')'
		| identifier
	) ';';
operatorId: 'CF' | 'SB' | 'SBR' | 'C';

pathBVIStmt: 'path' '(' portId ',' portId ')' ';';
interfaceBVIStmt:
	'interface' typeDefType type_identifier? ';' //strange
	interfaceBVIMembDecl* 'endinterface' (':' type_identifier)?;

interfaceBVIMembDecl: methodBVIStmt | interfaceBVIStmt;

inoutBVIStmt:
	'inout' portId (clocked_by_sel | reset_by_sel)* '=' expression ';'
	| 'ifc_inout' identifier '(' portId ')' (
		clocked_by_sel
		| reset_by_sel
	)* ';';

resetId: identifier;
noResetBVIStmt: 'no_reset' ';';

// Embedding C in a BSV Design
externCImport:
	'import' '"BDPI"' (identifier '=')? 'function' data_type identifier '(' cFuncArgs? ')' provisos?
		';';
cFuncArgs: cFuncArg (',' cFuncArg)*;
cFuncArg: data_type identifier?;

// fsm
fsmStmt:
	exprFsmStmt
	| seqFsmStmt
	| parFsmStmt
	| ifFsmStmt
	| whileFsmStmt
	| repeatFsmStmt
	| forFsmStmt
	| returnFsmStmt;

exprFsmStmt: regWrite ';' | expression ';';

seqFsmStmt: 'seq' fsmStmt fsmStmt* 'endseq';
parFsmStmt: 'par' fsmStmt fsmStmt* 'endpar';
ifFsmStmt: 'if' expression fsmStmt ('else' fsmStmt)?;
whileFsmStmt: 'while' '(' expression ')' loopBodyFsmStmt;
forFsmStmt:
	'for' '(' fsmStmt ';' expression ';' fsmStmt ')' loopBodyFsmStmt;
returnFsmStmt: 'return' ';';
repeatFsmStmt: 'repeat' '(' expression ')' loopBodyFsmStmt;
loopBodyFsmStmt: fsmStmt | 'break' ';' | 'continue' ';';

LINE_COMMENT: '//' ~[\r\n]* -> skip;

BLOCK_COMMENT: '/*' .*? '*/' -> skip;

WHITESPACE: [ \t]+ -> skip;

NEWLINE: ( '\r' '\n'? | '\n') -> skip;

fragment EscapeSequence:
	SimpleEscapeSequence
	| OctalEscapeSequence
	| HexadecimalEscapeSequence;

fragment SimpleEscapeSequence: '\\' ['"abfnrtv\\];
fragment OctalEscapeSequence:
	'\\' OctalDigit OctalDigit? OctalDigit?;
fragment HexadecimalEscapeSequence: '\\x' HexadecimalDigit+;
fragment SChar:
	~["\\\r\n]
	| EscapeSequence
	| '\\\n' // Added line
	| '\\\r\n'; // Added line

fragment SIGN: '+' | '-';
fragment DIGIT: [0-9];
fragment OctalDigit: [0-7];
fragment HexadecimalDigit: [0-9a-fA-F];
fragment DEC_DIGITS: DIGIT+;
fragment DEC_DIGIT_UNDERSCORE: [0-9_]+;
fragment HEX_DIGIT_UNDERSCORE: [0-9a-fA-F_]+;
fragment OCT_DIGIT_UNDERSCORE: [0-7_];
fragment BIN_DIGIT_UNDERSCORE: [01_];

fragment SCharSequence: SChar+;

fragment FileName: ( IdentifierNondigit | DIGIT)+;
fragment IdentifierNondigit: Nondigit;
fragment Nondigit: [a-zA-Z_$];

UPPER_IDENTIFIER: [A-Z][a-zA-Z0-9_]*;
LOWER_IDENTIFIER: [a-z_][a-zA-Z0-9_]*;
OPERATOR_IDENTIFIER: [\\][-=+_*&^%$#@!~<>?/|]+;
SYSTEM_TASK: [$][a-z][a-zA-Z0-9_$]*;

STRING_LITERAL: '"' SCharSequence? '"';

fragment BaseLiteral: ('\'d' | '\'D') DEC_DIGIT_UNDERSCORE
	| ('\'h' | '\'H') HEX_DIGIT_UNDERSCORE+
	| ('\'o' | '\'O') OCT_DIGIT_UNDERSCORE+
	| ('\'b' | '\'B') BIN_DIGIT_UNDERSCORE+;

fragment BIT_WIDTH: DEC_DIGITS;

fragment SizedIntLiteral: BIT_WIDTH BaseLiteral;

fragment UnsizedIntLiteral:
	/*Sign?*/ BaseLiteral // ignore sign this stage
	| /*Sign?*/ DECIMAL;

// ([1-9][0-9]*)? ('\'' [hdob]?)? [0-9a-fA-F_]+
INT_LITERAL:
	'\'0'
	| '\'1'
	| SizedIntLiteral
	| UnsizedIntLiteral;

DECIMAL: DEC_DIGITS;

fragment EXP: [eE];

REAL_LITERAL:
	DECIMAL ('.' DEC_DIGIT_UNDERSCORE)? EXP SIGN? DEC_DIGIT_UNDERSCORE
	| DECIMAL '.' DEC_DIGIT_UNDERSCORE;

COMPILER_DIRECTIVE: (
		'`include' STRING_LITERAL
		| '`include' '<' FileName WHITESPACE* '>'
		| '`include' MACRO_INVOCATION
		| '`line' LineNumber '"' FileName '"' Level
		| '`define' MACRO_NAME ('(' MACRO_FORMALS ')')? MACRO_TEXT
		| '`undef' MACRO_NAME
		| '`resetall'
		| '`ifdef' MACRO_NAME
		| '`ifndef' MACRO_NAME
		| '`elsif' MACRO_NAME
		| '`else'
		| '`endif'
	) -> skip;

fragment LineNumber: DEC_DIGITS;
fragment Level: '0' | '1' | '2';
fragment MACRO_NAME: LOWER_IDENTIFIER;
fragment MACRO_ACTUALS: SUBST_TEXT (',' SUBST_TEXT)*;
fragment MACRO_FORMALS:
	LOWER_IDENTIFIER (',' LOWER_IDENTIFIER)*;
fragment SUBST_TEXT: [^,]*;
fragment MACRO_TEXT: [.]*;

MACRO_INVOCATION: '\'' MACRO_NAME ('(' MACRO_ACTUALS ')')?;