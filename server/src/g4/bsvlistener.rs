#![allow(nonstandard_style)]
// Generated from BSV.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::bsvparser::*;

pub trait BSVListener<'input> : ParseTreeListener<'input,BSVParserContextType>{

/**
 * Enter a parse tree produced by {@link BSVParser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#any_identifier}.
 * @param ctx the parse tree
 */
fn enter_any_identifier(&mut self, _ctx: &Any_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#any_identifier}.
 * @param ctx the parse tree
 */
fn exit_any_identifier(&mut self, _ctx: &Any_identifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#top}.
 * @param ctx the parse tree
 */
fn enter_top(&mut self, _ctx: &TopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#top}.
 * @param ctx the parse tree
 */
fn exit_top(&mut self, _ctx: &TopContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#r_package}.
 * @param ctx the parse tree
 */
fn enter_r_package(&mut self, _ctx: &R_packageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#r_package}.
 * @param ctx the parse tree
 */
fn exit_r_package(&mut self, _ctx: &R_packageContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#non_package}.
 * @param ctx the parse tree
 */
fn enter_non_package(&mut self, _ctx: &Non_packageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#non_package}.
 * @param ctx the parse tree
 */
fn exit_non_package(&mut self, _ctx: &Non_packageContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#exportDecl}.
 * @param ctx the parse tree
 */
fn enter_exportDecl(&mut self, _ctx: &ExportDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#exportDecl}.
 * @param ctx the parse tree
 */
fn exit_exportDecl(&mut self, _ctx: &ExportDeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#exportItem}.
 * @param ctx the parse tree
 */
fn enter_exportItem(&mut self, _ctx: &ExportItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#exportItem}.
 * @param ctx the parse tree
 */
fn exit_exportItem(&mut self, _ctx: &ExportItemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#importDecl}.
 * @param ctx the parse tree
 */
fn enter_importDecl(&mut self, _ctx: &ImportDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#importDecl}.
 * @param ctx the parse tree
 */
fn exit_importDecl(&mut self, _ctx: &ImportDeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#importItem}.
 * @param ctx the parse tree
 */
fn enter_importItem(&mut self, _ctx: &ImportItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#importItem}.
 * @param ctx the parse tree
 */
fn exit_importItem(&mut self, _ctx: &ImportItemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#packageStmt}.
 * @param ctx the parse tree
 */
fn enter_packageStmt(&mut self, _ctx: &PackageStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#packageStmt}.
 * @param ctx the parse tree
 */
fn exit_packageStmt(&mut self, _ctx: &PackageStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#package_identifier}.
 * @param ctx the parse tree
 */
fn enter_package_identifier(&mut self, _ctx: &Package_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#package_identifier}.
 * @param ctx the parse tree
 */
fn exit_package_identifier(&mut self, _ctx: &Package_identifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#data_type}.
 * @param ctx the parse tree
 */
fn enter_data_type(&mut self, _ctx: &Data_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#data_type}.
 * @param ctx the parse tree
 */
fn exit_data_type(&mut self, _ctx: &Data_typeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#numeric_type}.
 * @param ctx the parse tree
 */
fn enter_numeric_type(&mut self, _ctx: &Numeric_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#numeric_type}.
 * @param ctx the parse tree
 */
fn exit_numeric_type(&mut self, _ctx: &Numeric_typeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#primary_type}.
 * @param ctx the parse tree
 */
fn enter_primary_type(&mut self, _ctx: &Primary_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#primary_type}.
 * @param ctx the parse tree
 */
fn exit_primary_type(&mut self, _ctx: &Primary_typeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#type_identifier}.
 * @param ctx the parse tree
 */
fn enter_type_identifier(&mut self, _ctx: &Type_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#type_identifier}.
 * @param ctx the parse tree
 */
fn exit_type_identifier(&mut self, _ctx: &Type_identifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#interfaceDecl}.
 * @param ctx the parse tree
 */
fn enter_interfaceDecl(&mut self, _ctx: &InterfaceDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#interfaceDecl}.
 * @param ctx the parse tree
 */
fn exit_interfaceDecl(&mut self, _ctx: &InterfaceDeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeDefType}.
 * @param ctx the parse tree
 */
fn enter_typeDefType(&mut self, _ctx: &TypeDefTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeDefType}.
 * @param ctx the parse tree
 */
fn exit_typeDefType(&mut self, _ctx: &TypeDefTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeFormals}.
 * @param ctx the parse tree
 */
fn enter_typeFormals(&mut self, _ctx: &TypeFormalsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeFormals}.
 * @param ctx the parse tree
 */
fn exit_typeFormals(&mut self, _ctx: &TypeFormalsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeFormal}.
 * @param ctx the parse tree
 */
fn enter_typeFormal(&mut self, _ctx: &TypeFormalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeFormal}.
 * @param ctx the parse tree
 */
fn exit_typeFormal(&mut self, _ctx: &TypeFormalContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#interfaceMemberDecl}.
 * @param ctx the parse tree
 */
fn enter_interfaceMemberDecl(&mut self, _ctx: &InterfaceMemberDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#interfaceMemberDecl}.
 * @param ctx the parse tree
 */
fn exit_interfaceMemberDecl(&mut self, _ctx: &InterfaceMemberDeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodProto}.
 * @param ctx the parse tree
 */
fn enter_methodProto(&mut self, _ctx: &MethodProtoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodProto}.
 * @param ctx the parse tree
 */
fn exit_methodProto(&mut self, _ctx: &MethodProtoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodProtoFormals}.
 * @param ctx the parse tree
 */
fn enter_methodProtoFormals(&mut self, _ctx: &MethodProtoFormalsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodProtoFormals}.
 * @param ctx the parse tree
 */
fn exit_methodProtoFormals(&mut self, _ctx: &MethodProtoFormalsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodProtoFormal}.
 * @param ctx the parse tree
 */
fn enter_methodProtoFormal(&mut self, _ctx: &MethodProtoFormalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodProtoFormal}.
 * @param ctx the parse tree
 */
fn exit_methodProtoFormal(&mut self, _ctx: &MethodProtoFormalContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#subinterfaceDecl}.
 * @param ctx the parse tree
 */
fn enter_subinterfaceDecl(&mut self, _ctx: &SubinterfaceDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#subinterfaceDecl}.
 * @param ctx the parse tree
 */
fn exit_subinterfaceDecl(&mut self, _ctx: &SubinterfaceDeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#module_def}.
 * @param ctx the parse tree
 */
fn enter_module_def(&mut self, _ctx: &Module_defContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#module_def}.
 * @param ctx the parse tree
 */
fn exit_module_def(&mut self, _ctx: &Module_defContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#module_proto}.
 * @param ctx the parse tree
 */
fn enter_module_proto(&mut self, _ctx: &Module_protoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#module_proto}.
 * @param ctx the parse tree
 */
fn exit_module_proto(&mut self, _ctx: &Module_protoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#moduleprotoformals}.
 * @param ctx the parse tree
 */
fn enter_moduleprotoformals(&mut self, _ctx: &ModuleprotoformalsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#moduleprotoformals}.
 * @param ctx the parse tree
 */
fn exit_moduleprotoformals(&mut self, _ctx: &ModuleprotoformalsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#moduleprotoformal}.
 * @param ctx the parse tree
 */
fn enter_moduleprotoformal(&mut self, _ctx: &ModuleprotoformalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#moduleprotoformal}.
 * @param ctx the parse tree
 */
fn exit_moduleprotoformal(&mut self, _ctx: &ModuleprotoformalContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#moduleFormalArgs}.
 * @param ctx the parse tree
 */
fn enter_moduleFormalArgs(&mut self, _ctx: &ModuleFormalArgsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#moduleFormalArgs}.
 * @param ctx the parse tree
 */
fn exit_moduleFormalArgs(&mut self, _ctx: &ModuleFormalArgsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#moduleStmt}.
 * @param ctx the parse tree
 */
fn enter_moduleStmt(&mut self, _ctx: &ModuleStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#moduleStmt}.
 * @param ctx the parse tree
 */
fn exit_moduleStmt(&mut self, _ctx: &ModuleStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#method_type}.
 * @param ctx the parse tree
 */
fn enter_method_type(&mut self, _ctx: &Method_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#method_type}.
 * @param ctx the parse tree
 */
fn exit_method_type(&mut self, _ctx: &Method_typeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#method_def}.
 * @param ctx the parse tree
 */
fn enter_method_def(&mut self, _ctx: &Method_defContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#method_def}.
 * @param ctx the parse tree
 */
fn exit_method_def(&mut self, _ctx: &Method_defContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#implicitCond}.
 * @param ctx the parse tree
 */
fn enter_implicitCond(&mut self, _ctx: &ImplicitCondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#implicitCond}.
 * @param ctx the parse tree
 */
fn exit_implicitCond(&mut self, _ctx: &ImplicitCondContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodFormals}.
 * @param ctx the parse tree
 */
fn enter_methodFormals(&mut self, _ctx: &MethodFormalsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodFormals}.
 * @param ctx the parse tree
 */
fn exit_methodFormals(&mut self, _ctx: &MethodFormalsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodFormal}.
 * @param ctx the parse tree
 */
fn enter_methodFormal(&mut self, _ctx: &MethodFormalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodFormal}.
 * @param ctx the parse tree
 */
fn exit_methodFormal(&mut self, _ctx: &MethodFormalContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#subinterfaceDef}.
 * @param ctx the parse tree
 */
fn enter_subinterfaceDef(&mut self, _ctx: &SubinterfaceDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#subinterfaceDef}.
 * @param ctx the parse tree
 */
fn exit_subinterfaceDef(&mut self, _ctx: &SubinterfaceDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#interfaceStmt}.
 * @param ctx the parse tree
 */
fn enter_interfaceStmt(&mut self, _ctx: &InterfaceStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#interfaceStmt}.
 * @param ctx the parse tree
 */
fn exit_interfaceStmt(&mut self, _ctx: &InterfaceStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#expressionStmt}.
 * @param ctx the parse tree
 */
fn enter_expressionStmt(&mut self, _ctx: &ExpressionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#expressionStmt}.
 * @param ctx the parse tree
 */
fn exit_expressionStmt(&mut self, _ctx: &ExpressionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#ruleDef}.
 * @param ctx the parse tree
 */
fn enter_ruleDef(&mut self, _ctx: &RuleDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#ruleDef}.
 * @param ctx the parse tree
 */
fn exit_ruleDef(&mut self, _ctx: &RuleDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#ruleCond}.
 * @param ctx the parse tree
 */
fn enter_ruleCond(&mut self, _ctx: &RuleCondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#ruleCond}.
 * @param ctx the parse tree
 */
fn exit_ruleCond(&mut self, _ctx: &RuleCondContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#ruleBody}.
 * @param ctx the parse tree
 */
fn enter_ruleBody(&mut self, _ctx: &RuleBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#ruleBody}.
 * @param ctx the parse tree
 */
fn exit_ruleBody(&mut self, _ctx: &RuleBodyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeDef}.
 * @param ctx the parse tree
 */
fn enter_typeDef(&mut self, _ctx: &TypeDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeDef}.
 * @param ctx the parse tree
 */
fn exit_typeDef(&mut self, _ctx: &TypeDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedefSynonym}.
 * @param ctx the parse tree
 */
fn enter_typedefSynonym(&mut self, _ctx: &TypedefSynonymContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedefSynonym}.
 * @param ctx the parse tree
 */
fn exit_typedefSynonym(&mut self, _ctx: &TypedefSynonymContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedefEnum}.
 * @param ctx the parse tree
 */
fn enter_typedefEnum(&mut self, _ctx: &TypedefEnumContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedefEnum}.
 * @param ctx the parse tree
 */
fn exit_typedefEnum(&mut self, _ctx: &TypedefEnumContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedefEnumElements}.
 * @param ctx the parse tree
 */
fn enter_typedefEnumElements(&mut self, _ctx: &TypedefEnumElementsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedefEnumElements}.
 * @param ctx the parse tree
 */
fn exit_typedefEnumElements(&mut self, _ctx: &TypedefEnumElementsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedefEnumElement}.
 * @param ctx the parse tree
 */
fn enter_typedefEnumElement(&mut self, _ctx: &TypedefEnumElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedefEnumElement}.
 * @param ctx the parse tree
 */
fn exit_typedefEnumElement(&mut self, _ctx: &TypedefEnumElementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedefStruct}.
 * @param ctx the parse tree
 */
fn enter_typedefStruct(&mut self, _ctx: &TypedefStructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedefStruct}.
 * @param ctx the parse tree
 */
fn exit_typedefStruct(&mut self, _ctx: &TypedefStructContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedefTaggedUnion}.
 * @param ctx the parse tree
 */
fn enter_typedefTaggedUnion(&mut self, _ctx: &TypedefTaggedUnionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedefTaggedUnion}.
 * @param ctx the parse tree
 */
fn exit_typedefTaggedUnion(&mut self, _ctx: &TypedefTaggedUnionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#structMember}.
 * @param ctx the parse tree
 */
fn enter_structMember(&mut self, _ctx: &StructMemberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#structMember}.
 * @param ctx the parse tree
 */
fn exit_structMember(&mut self, _ctx: &StructMemberContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#unionMember}.
 * @param ctx the parse tree
 */
fn enter_unionMember(&mut self, _ctx: &UnionMemberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#unionMember}.
 * @param ctx the parse tree
 */
fn exit_unionMember(&mut self, _ctx: &UnionMemberContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#subStruct}.
 * @param ctx the parse tree
 */
fn enter_subStruct(&mut self, _ctx: &SubStructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#subStruct}.
 * @param ctx the parse tree
 */
fn exit_subStruct(&mut self, _ctx: &SubStructContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#subUnion}.
 * @param ctx the parse tree
 */
fn enter_subUnion(&mut self, _ctx: &SubUnionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#subUnion}.
 * @param ctx the parse tree
 */
fn exit_subUnion(&mut self, _ctx: &SubUnionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#variable_declaration}.
 * @param ctx the parse tree
 */
fn enter_variable_declaration(&mut self, _ctx: &Variable_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#variable_declaration}.
 * @param ctx the parse tree
 */
fn exit_variable_declaration(&mut self, _ctx: &Variable_declarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#patternbinding}.
 * @param ctx the parse tree
 */
fn enter_patternbinding(&mut self, _ctx: &PatternbindingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#patternbinding}.
 * @param ctx the parse tree
 */
fn exit_patternbinding(&mut self, _ctx: &PatternbindingContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#variable_initialization}.
 * @param ctx the parse tree
 */
fn enter_variable_initialization(&mut self, _ctx: &Variable_initializationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#variable_initialization}.
 * @param ctx the parse tree
 */
fn exit_variable_initialization(&mut self, _ctx: &Variable_initializationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#variable_assignment}.
 * @param ctx the parse tree
 */
fn enter_variable_assignment(&mut self, _ctx: &Variable_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#variable_assignment}.
 * @param ctx the parse tree
 */
fn exit_variable_assignment(&mut self, _ctx: &Variable_assignmentContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#lvalue}.
 * @param ctx the parse tree
 */
fn enter_lvalue(&mut self, _ctx: &LvalueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#lvalue}.
 * @param ctx the parse tree
 */
fn exit_lvalue(&mut self, _ctx: &LvalueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#regWrite}.
 * @param ctx the parse tree
 */
fn enter_regWrite(&mut self, _ctx: &RegWriteContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#regWrite}.
 * @param ctx the parse tree
 */
fn exit_regWrite(&mut self, _ctx: &RegWriteContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#beginEndStmt_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn enter_beginEndStmt_functionBodyStmt(&mut self, _ctx: &BeginEndStmt_functionBodyStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#beginEndStmt_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn exit_beginEndStmt_functionBodyStmt(&mut self, _ctx: &BeginEndStmt_functionBodyStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#beginEndStmt_actionStmt}.
 * @param ctx the parse tree
 */
fn enter_beginEndStmt_actionStmt(&mut self, _ctx: &BeginEndStmt_actionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#beginEndStmt_actionStmt}.
 * @param ctx the parse tree
 */
fn exit_beginEndStmt_actionStmt(&mut self, _ctx: &BeginEndStmt_actionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#beginEndStmt_actionValueStmt}.
 * @param ctx the parse tree
 */
fn enter_beginEndStmt_actionValueStmt(&mut self, _ctx: &BeginEndStmt_actionValueStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#beginEndStmt_actionValueStmt}.
 * @param ctx the parse tree
 */
fn exit_beginEndStmt_actionValueStmt(&mut self, _ctx: &BeginEndStmt_actionValueStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#beginEndStmt_moduleStmt}.
 * @param ctx the parse tree
 */
fn enter_beginEndStmt_moduleStmt(&mut self, _ctx: &BeginEndStmt_moduleStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#beginEndStmt_moduleStmt}.
 * @param ctx the parse tree
 */
fn exit_beginEndStmt_moduleStmt(&mut self, _ctx: &BeginEndStmt_moduleStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#beginEndStmt_expressionStmt}.
 * @param ctx the parse tree
 */
fn enter_beginEndStmt_expressionStmt(&mut self, _ctx: &BeginEndStmt_expressionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#beginEndStmt_expressionStmt}.
 * @param ctx the parse tree
 */
fn exit_beginEndStmt_expressionStmt(&mut self, _ctx: &BeginEndStmt_expressionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#if_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn enter_if_functionBodyStmt(&mut self, _ctx: &If_functionBodyStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#if_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn exit_if_functionBodyStmt(&mut self, _ctx: &If_functionBodyStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#if_actionStmt}.
 * @param ctx the parse tree
 */
fn enter_if_actionStmt(&mut self, _ctx: &If_actionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#if_actionStmt}.
 * @param ctx the parse tree
 */
fn exit_if_actionStmt(&mut self, _ctx: &If_actionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#if_actionValueStmt}.
 * @param ctx the parse tree
 */
fn enter_if_actionValueStmt(&mut self, _ctx: &If_actionValueStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#if_actionValueStmt}.
 * @param ctx the parse tree
 */
fn exit_if_actionValueStmt(&mut self, _ctx: &If_actionValueStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#if_moduleStmt}.
 * @param ctx the parse tree
 */
fn enter_if_moduleStmt(&mut self, _ctx: &If_moduleStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#if_moduleStmt}.
 * @param ctx the parse tree
 */
fn exit_if_moduleStmt(&mut self, _ctx: &If_moduleStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#if_expressionStmt}.
 * @param ctx the parse tree
 */
fn enter_if_expressionStmt(&mut self, _ctx: &If_expressionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#if_expressionStmt}.
 * @param ctx the parse tree
 */
fn exit_if_expressionStmt(&mut self, _ctx: &If_expressionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#case_actionStmt}.
 * @param ctx the parse tree
 */
fn enter_case_actionStmt(&mut self, _ctx: &Case_actionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#case_actionStmt}.
 * @param ctx the parse tree
 */
fn exit_case_actionStmt(&mut self, _ctx: &Case_actionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#case_actionValueStmt}.
 * @param ctx the parse tree
 */
fn enter_case_actionValueStmt(&mut self, _ctx: &Case_actionValueStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#case_actionValueStmt}.
 * @param ctx the parse tree
 */
fn exit_case_actionValueStmt(&mut self, _ctx: &Case_actionValueStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#case_moduleStmt}.
 * @param ctx the parse tree
 */
fn enter_case_moduleStmt(&mut self, _ctx: &Case_moduleStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#case_moduleStmt}.
 * @param ctx the parse tree
 */
fn exit_case_moduleStmt(&mut self, _ctx: &Case_moduleStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#case_expressionStmt}.
 * @param ctx the parse tree
 */
fn enter_case_expressionStmt(&mut self, _ctx: &Case_expressionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#case_expressionStmt}.
 * @param ctx the parse tree
 */
fn exit_case_expressionStmt(&mut self, _ctx: &Case_expressionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#caseItem_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn enter_caseItem_functionBodyStmt(&mut self, _ctx: &CaseItem_functionBodyStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#caseItem_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn exit_caseItem_functionBodyStmt(&mut self, _ctx: &CaseItem_functionBodyStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#caseItem_actionStmt}.
 * @param ctx the parse tree
 */
fn enter_caseItem_actionStmt(&mut self, _ctx: &CaseItem_actionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#caseItem_actionStmt}.
 * @param ctx the parse tree
 */
fn exit_caseItem_actionStmt(&mut self, _ctx: &CaseItem_actionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#caseItem_actionValueStmt}.
 * @param ctx the parse tree
 */
fn enter_caseItem_actionValueStmt(&mut self, _ctx: &CaseItem_actionValueStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#caseItem_actionValueStmt}.
 * @param ctx the parse tree
 */
fn exit_caseItem_actionValueStmt(&mut self, _ctx: &CaseItem_actionValueStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#caseItem_moduleStmt}.
 * @param ctx the parse tree
 */
fn enter_caseItem_moduleStmt(&mut self, _ctx: &CaseItem_moduleStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#caseItem_moduleStmt}.
 * @param ctx the parse tree
 */
fn exit_caseItem_moduleStmt(&mut self, _ctx: &CaseItem_moduleStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#caseItem_expressionStmt}.
 * @param ctx the parse tree
 */
fn enter_caseItem_expressionStmt(&mut self, _ctx: &CaseItem_expressionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#caseItem_expressionStmt}.
 * @param ctx the parse tree
 */
fn exit_caseItem_expressionStmt(&mut self, _ctx: &CaseItem_expressionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#defaultItem_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn enter_defaultItem_functionBodyStmt(&mut self, _ctx: &DefaultItem_functionBodyStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#defaultItem_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn exit_defaultItem_functionBodyStmt(&mut self, _ctx: &DefaultItem_functionBodyStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#defaultItem_actionStmt}.
 * @param ctx the parse tree
 */
fn enter_defaultItem_actionStmt(&mut self, _ctx: &DefaultItem_actionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#defaultItem_actionStmt}.
 * @param ctx the parse tree
 */
fn exit_defaultItem_actionStmt(&mut self, _ctx: &DefaultItem_actionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#defaultItem_actionValueStmt}.
 * @param ctx the parse tree
 */
fn enter_defaultItem_actionValueStmt(&mut self, _ctx: &DefaultItem_actionValueStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#defaultItem_actionValueStmt}.
 * @param ctx the parse tree
 */
fn exit_defaultItem_actionValueStmt(&mut self, _ctx: &DefaultItem_actionValueStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#defaultItem_moduleStmt}.
 * @param ctx the parse tree
 */
fn enter_defaultItem_moduleStmt(&mut self, _ctx: &DefaultItem_moduleStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#defaultItem_moduleStmt}.
 * @param ctx the parse tree
 */
fn exit_defaultItem_moduleStmt(&mut self, _ctx: &DefaultItem_moduleStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#defaultItem_expressionStmt}.
 * @param ctx the parse tree
 */
fn enter_defaultItem_expressionStmt(&mut self, _ctx: &DefaultItem_expressionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#defaultItem_expressionStmt}.
 * @param ctx the parse tree
 */
fn exit_defaultItem_expressionStmt(&mut self, _ctx: &DefaultItem_expressionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#while_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn enter_while_functionBodyStmt(&mut self, _ctx: &While_functionBodyStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#while_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn exit_while_functionBodyStmt(&mut self, _ctx: &While_functionBodyStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#while_actionStmt}.
 * @param ctx the parse tree
 */
fn enter_while_actionStmt(&mut self, _ctx: &While_actionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#while_actionStmt}.
 * @param ctx the parse tree
 */
fn exit_while_actionStmt(&mut self, _ctx: &While_actionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#while_actionValueStmt}.
 * @param ctx the parse tree
 */
fn enter_while_actionValueStmt(&mut self, _ctx: &While_actionValueStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#while_actionValueStmt}.
 * @param ctx the parse tree
 */
fn exit_while_actionValueStmt(&mut self, _ctx: &While_actionValueStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#while_moduleStmt}.
 * @param ctx the parse tree
 */
fn enter_while_moduleStmt(&mut self, _ctx: &While_moduleStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#while_moduleStmt}.
 * @param ctx the parse tree
 */
fn exit_while_moduleStmt(&mut self, _ctx: &While_moduleStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#while_expressionStmt}.
 * @param ctx the parse tree
 */
fn enter_while_expressionStmt(&mut self, _ctx: &While_expressionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#while_expressionStmt}.
 * @param ctx the parse tree
 */
fn exit_while_expressionStmt(&mut self, _ctx: &While_expressionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#for_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn enter_for_functionBodyStmt(&mut self, _ctx: &For_functionBodyStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#for_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn exit_for_functionBodyStmt(&mut self, _ctx: &For_functionBodyStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#for_actionStmt}.
 * @param ctx the parse tree
 */
fn enter_for_actionStmt(&mut self, _ctx: &For_actionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#for_actionStmt}.
 * @param ctx the parse tree
 */
fn exit_for_actionStmt(&mut self, _ctx: &For_actionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#for_actionValueStmt}.
 * @param ctx the parse tree
 */
fn enter_for_actionValueStmt(&mut self, _ctx: &For_actionValueStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#for_actionValueStmt}.
 * @param ctx the parse tree
 */
fn exit_for_actionValueStmt(&mut self, _ctx: &For_actionValueStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#for_moduleStmt}.
 * @param ctx the parse tree
 */
fn enter_for_moduleStmt(&mut self, _ctx: &For_moduleStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#for_moduleStmt}.
 * @param ctx the parse tree
 */
fn exit_for_moduleStmt(&mut self, _ctx: &For_moduleStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#for_expressionStmt}.
 * @param ctx the parse tree
 */
fn enter_for_expressionStmt(&mut self, _ctx: &For_expressionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#for_expressionStmt}.
 * @param ctx the parse tree
 */
fn exit_for_expressionStmt(&mut self, _ctx: &For_expressionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#simpleVarDeclAssign}.
 * @param ctx the parse tree
 */
fn enter_simpleVarDeclAssign(&mut self, _ctx: &SimpleVarDeclAssignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#simpleVarDeclAssign}.
 * @param ctx the parse tree
 */
fn exit_simpleVarDeclAssign(&mut self, _ctx: &SimpleVarDeclAssignContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#forInit}.
 * @param ctx the parse tree
 */
fn enter_forInit(&mut self, _ctx: &ForInitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#forInit}.
 * @param ctx the parse tree
 */
fn exit_forInit(&mut self, _ctx: &ForInitContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#forTest}.
 * @param ctx the parse tree
 */
fn enter_forTest(&mut self, _ctx: &ForTestContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#forTest}.
 * @param ctx the parse tree
 */
fn exit_forTest(&mut self, _ctx: &ForTestContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#forIncr}.
 * @param ctx the parse tree
 */
fn enter_forIncr(&mut self, _ctx: &ForIncrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#forIncr}.
 * @param ctx the parse tree
 */
fn exit_forIncr(&mut self, _ctx: &ForIncrContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#varIncr}.
 * @param ctx the parse tree
 */
fn enter_varIncr(&mut self, _ctx: &VarIncrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#varIncr}.
 * @param ctx the parse tree
 */
fn exit_varIncr(&mut self, _ctx: &VarIncrContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#function_def}.
 * @param ctx the parse tree
 */
fn enter_function_def(&mut self, _ctx: &Function_defContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#function_def}.
 * @param ctx the parse tree
 */
fn exit_function_def(&mut self, _ctx: &Function_defContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#function_proto}.
 * @param ctx the parse tree
 */
fn enter_function_proto(&mut self, _ctx: &Function_protoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#function_proto}.
 * @param ctx the parse tree
 */
fn exit_function_proto(&mut self, _ctx: &Function_protoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#function_formals}.
 * @param ctx the parse tree
 */
fn enter_function_formals(&mut self, _ctx: &Function_formalsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#function_formals}.
 * @param ctx the parse tree
 */
fn exit_function_formals(&mut self, _ctx: &Function_formalsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#functionFormal}.
 * @param ctx the parse tree
 */
fn enter_functionFormal(&mut self, _ctx: &FunctionFormalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#functionFormal}.
 * @param ctx the parse tree
 */
fn exit_functionFormal(&mut self, _ctx: &FunctionFormalContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#functionBody}.
 * @param ctx the parse tree
 */
fn enter_functionBody(&mut self, _ctx: &FunctionBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#functionBody}.
 * @param ctx the parse tree
 */
fn exit_functionBody(&mut self, _ctx: &FunctionBodyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#functionBodyStmt}.
 * @param ctx the parse tree
 */
fn enter_functionBodyStmt(&mut self, _ctx: &FunctionBodyStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#functionBodyStmt}.
 * @param ctx the parse tree
 */
fn exit_functionBodyStmt(&mut self, _ctx: &FunctionBodyStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#returnStmt}.
 * @param ctx the parse tree
 */
fn enter_returnStmt(&mut self, _ctx: &ReturnStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#returnStmt}.
 * @param ctx the parse tree
 */
fn exit_returnStmt(&mut self, _ctx: &ReturnStmtContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code operatorexpr}
 * labeled alternative in {@link BSVParser#expression}.
 * @param ctx the parse tree
 */
fn enter_operatorexpr(&mut self, _ctx: &OperatorexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code operatorexpr}
 * labeled alternative in {@link BSVParser#expression}.
 * @param ctx the parse tree
 */
fn exit_operatorexpr(&mut self, _ctx: &OperatorexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code caseexpr}
 * labeled alternative in {@link BSVParser#expression}.
 * @param ctx the parse tree
 */
fn enter_caseexpr(&mut self, _ctx: &CaseexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code caseexpr}
 * labeled alternative in {@link BSVParser#expression}.
 * @param ctx the parse tree
 */
fn exit_caseexpr(&mut self, _ctx: &CaseexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code condExpr}
 * labeled alternative in {@link BSVParser#expression}.
 * @param ctx the parse tree
 */
fn enter_condExpr(&mut self, _ctx: &CondExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code condExpr}
 * labeled alternative in {@link BSVParser#expression}.
 * @param ctx the parse tree
 */
fn exit_condExpr(&mut self, _ctx: &CondExprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code matchesexpr}
 * labeled alternative in {@link BSVParser#expression}.
 * @param ctx the parse tree
 */
fn enter_matchesexpr(&mut self, _ctx: &MatchesexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code matchesexpr}
 * labeled alternative in {@link BSVParser#expression}.
 * @param ctx the parse tree
 */
fn exit_matchesexpr(&mut self, _ctx: &MatchesexprContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#caseexprpatitem}.
 * @param ctx the parse tree
 */
fn enter_caseexprpatitem(&mut self, _ctx: &CaseexprpatitemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#caseexprpatitem}.
 * @param ctx the parse tree
 */
fn exit_caseexprpatitem(&mut self, _ctx: &CaseexprpatitemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#caseexpritem}.
 * @param ctx the parse tree
 */
fn enter_caseexpritem(&mut self, _ctx: &CaseexpritemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#caseexpritem}.
 * @param ctx the parse tree
 */
fn exit_caseexpritem(&mut self, _ctx: &CaseexpritemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#caseexprdefaultitem}.
 * @param ctx the parse tree
 */
fn enter_caseexprdefaultitem(&mut self, _ctx: &CaseexprdefaultitemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#caseexprdefaultitem}.
 * @param ctx the parse tree
 */
fn exit_caseexprdefaultitem(&mut self, _ctx: &CaseexprdefaultitemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#patterncond}.
 * @param ctx the parse tree
 */
fn enter_patterncond(&mut self, _ctx: &PatterncondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#patterncond}.
 * @param ctx the parse tree
 */
fn exit_patterncond(&mut self, _ctx: &PatterncondContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#unary_operator}.
 * @param ctx the parse tree
 */
fn enter_unary_operator(&mut self, _ctx: &Unary_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#unary_operator}.
 * @param ctx the parse tree
 */
fn exit_unary_operator(&mut self, _ctx: &Unary_operatorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#binary_operator}.
 * @param ctx the parse tree
 */
fn enter_binary_operator(&mut self, _ctx: &Binary_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#binary_operator}.
 * @param ctx the parse tree
 */
fn exit_binary_operator(&mut self, _ctx: &Binary_operatorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#binopexpr}.
 * @param ctx the parse tree
 */
fn enter_binopexpr(&mut self, _ctx: &BinopexprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#binopexpr}.
 * @param ctx the parse tree
 */
fn exit_binopexpr(&mut self, _ctx: &BinopexprContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#unopexpr}.
 * @param ctx the parse tree
 */
fn enter_unopexpr(&mut self, _ctx: &UnopexprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#unopexpr}.
 * @param ctx the parse tree
 */
fn exit_unopexpr(&mut self, _ctx: &UnopexprContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#constant_expression}.
 * @param ctx the parse tree
 */
fn enter_constant_expression(&mut self, _ctx: &Constant_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#constant_expression}.
 * @param ctx the parse tree
 */
fn exit_constant_expression(&mut self, _ctx: &Constant_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#constant_primary}.
 * @param ctx the parse tree
 */
fn enter_constant_primary(&mut self, _ctx: &Constant_primaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#constant_primary}.
 * @param ctx the parse tree
 */
fn exit_constant_primary(&mut self, _ctx: &Constant_primaryContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#bit_concatination}.
 * @param ctx the parse tree
 */
fn enter_bit_concatination(&mut self, _ctx: &Bit_concatinationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#bit_concatination}.
 * @param ctx the parse tree
 */
fn exit_bit_concatination(&mut self, _ctx: &Bit_concatinationContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code bitconcat}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_bitconcat(&mut self, _ctx: &BitconcatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code bitconcat}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_bitconcat(&mut self, _ctx: &BitconcatContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code whenexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_whenexpr(&mut self, _ctx: &WhenexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code whenexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_whenexpr(&mut self, _ctx: &WhenexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code varexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_varexpr(&mut self, _ctx: &VarexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code varexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_varexpr(&mut self, _ctx: &VarexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code interfaceexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_interfaceexpr(&mut self, _ctx: &InterfaceexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code interfaceexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_interfaceexpr(&mut self, _ctx: &InterfaceexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code blockexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_blockexpr(&mut self, _ctx: &BlockexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code blockexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_blockexpr(&mut self, _ctx: &BlockexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code constexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_constexpr(&mut self, _ctx: &ConstexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code constexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_constexpr(&mut self, _ctx: &ConstexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code syscallexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_syscallexpr(&mut self, _ctx: &SyscallexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code syscallexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_syscallexpr(&mut self, _ctx: &SyscallexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code callexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_callexpr(&mut self, _ctx: &CallexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code callexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_callexpr(&mut self, _ctx: &CallexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code castexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_castexpr(&mut self, _ctx: &CastexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code castexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_castexpr(&mut self, _ctx: &CastexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code typeassertionexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_typeassertionexpr(&mut self, _ctx: &TypeassertionexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code typeassertionexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_typeassertionexpr(&mut self, _ctx: &TypeassertionexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code bitselect}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_bitselect(&mut self, _ctx: &BitselectContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code bitselect}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_bitselect(&mut self, _ctx: &BitselectContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code resetbyexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_resetbyexpr(&mut self, _ctx: &ResetbyexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code resetbyexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_resetbyexpr(&mut self, _ctx: &ResetbyexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code taggedunionexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_taggedunionexpr(&mut self, _ctx: &TaggedunionexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taggedunionexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_taggedunionexpr(&mut self, _ctx: &TaggedunionexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code clockedbyexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_clockedbyexpr(&mut self, _ctx: &ClockedbyexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code clockedbyexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_clockedbyexpr(&mut self, _ctx: &ClockedbyexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code fieldexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_fieldexpr(&mut self, _ctx: &FieldexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fieldexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_fieldexpr(&mut self, _ctx: &FieldexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code parenexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_parenexpr(&mut self, _ctx: &ParenexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parenexpr}
 * labeled alternative in {@link BSVParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_parenexpr(&mut self, _ctx: &ParenexprContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#memberbinds}.
 * @param ctx the parse tree
 */
fn enter_memberbinds(&mut self, _ctx: &MemberbindsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#memberbinds}.
 * @param ctx the parse tree
 */
fn exit_memberbinds(&mut self, _ctx: &MemberbindsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#memberbind}.
 * @param ctx the parse tree
 */
fn enter_memberbind(&mut self, _ctx: &MemberbindContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#memberbind}.
 * @param ctx the parse tree
 */
fn exit_memberbind(&mut self, _ctx: &MemberbindContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#interfacestmt}.
 * @param ctx the parse tree
 */
fn enter_interfacestmt(&mut self, _ctx: &InterfacestmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#interfacestmt}.
 * @param ctx the parse tree
 */
fn exit_interfacestmt(&mut self, _ctx: &InterfacestmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#cond_predicate}.
 * @param ctx the parse tree
 */
fn enter_cond_predicate(&mut self, _ctx: &Cond_predicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#cond_predicate}.
 * @param ctx the parse tree
 */
fn exit_cond_predicate(&mut self, _ctx: &Cond_predicateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#exprOrCondPattern}.
 * @param ctx the parse tree
 */
fn enter_exprOrCondPattern(&mut self, _ctx: &ExprOrCondPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#exprOrCondPattern}.
 * @param ctx the parse tree
 */
fn exit_exprOrCondPattern(&mut self, _ctx: &ExprOrCondPatternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#block_expression}.
 * @param ctx the parse tree
 */
fn enter_block_expression(&mut self, _ctx: &Block_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#block_expression}.
 * @param ctx the parse tree
 */
fn exit_block_expression(&mut self, _ctx: &Block_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#actionBlock}.
 * @param ctx the parse tree
 */
fn enter_actionBlock(&mut self, _ctx: &ActionBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#actionBlock}.
 * @param ctx the parse tree
 */
fn exit_actionBlock(&mut self, _ctx: &ActionBlockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#actionStmt}.
 * @param ctx the parse tree
 */
fn enter_actionStmt(&mut self, _ctx: &ActionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#actionStmt}.
 * @param ctx the parse tree
 */
fn exit_actionStmt(&mut self, _ctx: &ActionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#actionValueBlock}.
 * @param ctx the parse tree
 */
fn enter_actionValueBlock(&mut self, _ctx: &ActionValueBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#actionValueBlock}.
 * @param ctx the parse tree
 */
fn exit_actionValueBlock(&mut self, _ctx: &ActionValueBlockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#actionValueStmt}.
 * @param ctx the parse tree
 */
fn enter_actionValueStmt(&mut self, _ctx: &ActionValueStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#actionValueStmt}.
 * @param ctx the parse tree
 */
fn exit_actionValueStmt(&mut self, _ctx: &ActionValueStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#varDeclDo}.
 * @param ctx the parse tree
 */
fn enter_varDeclDo(&mut self, _ctx: &VarDeclDoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#varDeclDo}.
 * @param ctx the parse tree
 */
fn exit_varDeclDo(&mut self, _ctx: &VarDeclDoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#varDo}.
 * @param ctx the parse tree
 */
fn enter_varDo(&mut self, _ctx: &VarDoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#varDo}.
 * @param ctx the parse tree
 */
fn exit_varDo(&mut self, _ctx: &VarDoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeAssertion}.
 * @param ctx the parse tree
 */
fn enter_typeAssertion(&mut self, _ctx: &TypeAssertionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeAssertion}.
 * @param ctx the parse tree
 */
fn exit_typeAssertion(&mut self, _ctx: &TypeAssertionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#structExpr}.
 * @param ctx the parse tree
 */
fn enter_structExpr(&mut self, _ctx: &StructExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#structExpr}.
 * @param ctx the parse tree
 */
fn exit_structExpr(&mut self, _ctx: &StructExprContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#memberBind}.
 * @param ctx the parse tree
 */
fn enter_memberBind(&mut self, _ctx: &MemberBindContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#memberBind}.
 * @param ctx the parse tree
 */
fn exit_memberBind(&mut self, _ctx: &MemberBindContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#taggedUnionExpr}.
 * @param ctx the parse tree
 */
fn enter_taggedUnionExpr(&mut self, _ctx: &TaggedUnionExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#taggedUnionExpr}.
 * @param ctx the parse tree
 */
fn exit_taggedUnionExpr(&mut self, _ctx: &TaggedUnionExprContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#interfaceExpr}.
 * @param ctx the parse tree
 */
fn enter_interfaceExpr(&mut self, _ctx: &InterfaceExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#interfaceExpr}.
 * @param ctx the parse tree
 */
fn exit_interfaceExpr(&mut self, _ctx: &InterfaceExprContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#rulesExpr}.
 * @param ctx the parse tree
 */
fn enter_rulesExpr(&mut self, _ctx: &RulesExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#rulesExpr}.
 * @param ctx the parse tree
 */
fn exit_rulesExpr(&mut self, _ctx: &RulesExprContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#rulesStmt}.
 * @param ctx the parse tree
 */
fn enter_rulesStmt(&mut self, _ctx: &RulesStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#rulesStmt}.
 * @param ctx the parse tree
 */
fn exit_rulesStmt(&mut self, _ctx: &RulesStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#pattern}.
 * @param ctx the parse tree
 */
fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#pattern}.
 * @param ctx the parse tree
 */
fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#constantPattern}.
 * @param ctx the parse tree
 */
fn enter_constantPattern(&mut self, _ctx: &ConstantPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#constantPattern}.
 * @param ctx the parse tree
 */
fn exit_constantPattern(&mut self, _ctx: &ConstantPatternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#taggedUnionPattern}.
 * @param ctx the parse tree
 */
fn enter_taggedUnionPattern(&mut self, _ctx: &TaggedUnionPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#taggedUnionPattern}.
 * @param ctx the parse tree
 */
fn exit_taggedUnionPattern(&mut self, _ctx: &TaggedUnionPatternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#structPattern}.
 * @param ctx the parse tree
 */
fn enter_structPattern(&mut self, _ctx: &StructPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#structPattern}.
 * @param ctx the parse tree
 */
fn exit_structPattern(&mut self, _ctx: &StructPatternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#tuplePattern}.
 * @param ctx the parse tree
 */
fn enter_tuplePattern(&mut self, _ctx: &TuplePatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#tuplePattern}.
 * @param ctx the parse tree
 */
fn exit_tuplePattern(&mut self, _ctx: &TuplePatternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#casePatItem_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn enter_casePatItem_functionBodyStmt(&mut self, _ctx: &CasePatItem_functionBodyStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#casePatItem_functionBodyStmt}.
 * @param ctx the parse tree
 */
fn exit_casePatItem_functionBodyStmt(&mut self, _ctx: &CasePatItem_functionBodyStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#casePatItem_actionStmt}.
 * @param ctx the parse tree
 */
fn enter_casePatItem_actionStmt(&mut self, _ctx: &CasePatItem_actionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#casePatItem_actionStmt}.
 * @param ctx the parse tree
 */
fn exit_casePatItem_actionStmt(&mut self, _ctx: &CasePatItem_actionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#casePatItem_actionValueStmt}.
 * @param ctx the parse tree
 */
fn enter_casePatItem_actionValueStmt(&mut self, _ctx: &CasePatItem_actionValueStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#casePatItem_actionValueStmt}.
 * @param ctx the parse tree
 */
fn exit_casePatItem_actionValueStmt(&mut self, _ctx: &CasePatItem_actionValueStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#casePatItem_moduleStmt}.
 * @param ctx the parse tree
 */
fn enter_casePatItem_moduleStmt(&mut self, _ctx: &CasePatItem_moduleStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#casePatItem_moduleStmt}.
 * @param ctx the parse tree
 */
fn exit_casePatItem_moduleStmt(&mut self, _ctx: &CasePatItem_moduleStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#casePatItem_expressionStmt}.
 * @param ctx the parse tree
 */
fn enter_casePatItem_expressionStmt(&mut self, _ctx: &CasePatItem_expressionStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#casePatItem_expressionStmt}.
 * @param ctx the parse tree
 */
fn exit_casePatItem_expressionStmt(&mut self, _ctx: &CasePatItem_expressionStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#caseExpr}.
 * @param ctx the parse tree
 */
fn enter_caseExpr(&mut self, _ctx: &CaseExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#caseExpr}.
 * @param ctx the parse tree
 */
fn exit_caseExpr(&mut self, _ctx: &CaseExprContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#caseExprItem}.
 * @param ctx the parse tree
 */
fn enter_caseExprItem(&mut self, _ctx: &CaseExprItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#caseExprItem}.
 * @param ctx the parse tree
 */
fn exit_caseExprItem(&mut self, _ctx: &CaseExprItemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#attribute_instance}.
 * @param ctx the parse tree
 */
fn enter_attribute_instance(&mut self, _ctx: &Attribute_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#attribute_instance}.
 * @param ctx the parse tree
 */
fn exit_attribute_instance(&mut self, _ctx: &Attribute_instanceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#attribute_instances}.
 * @param ctx the parse tree
 */
fn enter_attribute_instances(&mut self, _ctx: &Attribute_instancesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#attribute_instances}.
 * @param ctx the parse tree
 */
fn exit_attribute_instances(&mut self, _ctx: &Attribute_instancesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#attribute_spec}.
 * @param ctx the parse tree
 */
fn enter_attribute_spec(&mut self, _ctx: &Attribute_specContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#attribute_spec}.
 * @param ctx the parse tree
 */
fn exit_attribute_spec(&mut self, _ctx: &Attribute_specContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#attrName}.
 * @param ctx the parse tree
 */
fn enter_attrName(&mut self, _ctx: &AttrNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#attrName}.
 * @param ctx the parse tree
 */
fn exit_attrName(&mut self, _ctx: &AttrNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#provisos}.
 * @param ctx the parse tree
 */
fn enter_provisos(&mut self, _ctx: &ProvisosContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#provisos}.
 * @param ctx the parse tree
 */
fn exit_provisos(&mut self, _ctx: &ProvisosContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#proviso}.
 * @param ctx the parse tree
 */
fn enter_proviso(&mut self, _ctx: &ProvisoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#proviso}.
 * @param ctx the parse tree
 */
fn exit_proviso(&mut self, _ctx: &ProvisoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeclassDef}.
 * @param ctx the parse tree
 */
fn enter_typeclassDef(&mut self, _ctx: &TypeclassDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeclassDef}.
 * @param ctx the parse tree
 */
fn exit_typeclassDef(&mut self, _ctx: &TypeclassDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeclassIde}.
 * @param ctx the parse tree
 */
fn enter_typeclassIde(&mut self, _ctx: &TypeclassIdeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeclassIde}.
 * @param ctx the parse tree
 */
fn exit_typeclassIde(&mut self, _ctx: &TypeclassIdeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typelist}.
 * @param ctx the parse tree
 */
fn enter_typelist(&mut self, _ctx: &TypelistContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typelist}.
 * @param ctx the parse tree
 */
fn exit_typelist(&mut self, _ctx: &TypelistContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedepends}.
 * @param ctx the parse tree
 */
fn enter_typedepends(&mut self, _ctx: &TypedependsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedepends}.
 * @param ctx the parse tree
 */
fn exit_typedepends(&mut self, _ctx: &TypedependsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedepend}.
 * @param ctx the parse tree
 */
fn enter_typedepend(&mut self, _ctx: &TypedependContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedepend}.
 * @param ctx the parse tree
 */
fn exit_typedepend(&mut self, _ctx: &TypedependContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#overloadedDef}.
 * @param ctx the parse tree
 */
fn enter_overloadedDef(&mut self, _ctx: &OverloadedDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#overloadedDef}.
 * @param ctx the parse tree
 */
fn exit_overloadedDef(&mut self, _ctx: &OverloadedDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeclassInstanceDef}.
 * @param ctx the parse tree
 */
fn enter_typeclassInstanceDef(&mut self, _ctx: &TypeclassInstanceDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeclassInstanceDef}.
 * @param ctx the parse tree
 */
fn exit_typeclassInstanceDef(&mut self, _ctx: &TypeclassInstanceDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#derives}.
 * @param ctx the parse tree
 */
fn enter_derives(&mut self, _ctx: &DerivesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#derives}.
 * @param ctx the parse tree
 */
fn exit_derives(&mut self, _ctx: &DerivesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#externModuleImport}.
 * @param ctx the parse tree
 */
fn enter_externModuleImport(&mut self, _ctx: &ExternModuleImportContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#externModuleImport}.
 * @param ctx the parse tree
 */
fn exit_externModuleImport(&mut self, _ctx: &ExternModuleImportContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#importBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_importBVIStmt(&mut self, _ctx: &ImportBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#importBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_importBVIStmt(&mut self, _ctx: &ImportBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#enabled_sel}.
 * @param ctx the parse tree
 */
fn enter_enabled_sel(&mut self, _ctx: &Enabled_selContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#enabled_sel}.
 * @param ctx the parse tree
 */
fn exit_enabled_sel(&mut self, _ctx: &Enabled_selContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#ready_sel}.
 * @param ctx the parse tree
 */
fn enter_ready_sel(&mut self, _ctx: &Ready_selContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#ready_sel}.
 * @param ctx the parse tree
 */
fn exit_ready_sel(&mut self, _ctx: &Ready_selContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#clocked_by_sel}.
 * @param ctx the parse tree
 */
fn enter_clocked_by_sel(&mut self, _ctx: &Clocked_by_selContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#clocked_by_sel}.
 * @param ctx the parse tree
 */
fn exit_clocked_by_sel(&mut self, _ctx: &Clocked_by_selContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#reset_by_sel}.
 * @param ctx the parse tree
 */
fn enter_reset_by_sel(&mut self, _ctx: &Reset_by_selContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#reset_by_sel}.
 * @param ctx the parse tree
 */
fn exit_reset_by_sel(&mut self, _ctx: &Reset_by_selContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#parameterBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_parameterBVIStmt(&mut self, _ctx: &ParameterBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#parameterBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_parameterBVIStmt(&mut self, _ctx: &ParameterBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_methodBVIStmt(&mut self, _ctx: &MethodBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_methodBVIStmt(&mut self, _ctx: &MethodBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#portBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_portBVIStmt(&mut self, _ctx: &PortBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#portBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_portBVIStmt(&mut self, _ctx: &PortBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#inputClockBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_inputClockBVIStmt(&mut self, _ctx: &InputClockBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#inputClockBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_inputClockBVIStmt(&mut self, _ctx: &InputClockBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#portsDef}.
 * @param ctx the parse tree
 */
fn enter_portsDef(&mut self, _ctx: &PortsDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#portsDef}.
 * @param ctx the parse tree
 */
fn exit_portsDef(&mut self, _ctx: &PortsDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#portId}.
 * @param ctx the parse tree
 */
fn enter_portId(&mut self, _ctx: &PortIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#portId}.
 * @param ctx the parse tree
 */
fn exit_portId(&mut self, _ctx: &PortIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#defaultClockBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_defaultClockBVIStmt(&mut self, _ctx: &DefaultClockBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#defaultClockBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_defaultClockBVIStmt(&mut self, _ctx: &DefaultClockBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#outputClockBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_outputClockBVIStmt(&mut self, _ctx: &OutputClockBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#outputClockBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_outputClockBVIStmt(&mut self, _ctx: &OutputClockBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#inputResetBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_inputResetBVIStmt(&mut self, _ctx: &InputResetBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#inputResetBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_inputResetBVIStmt(&mut self, _ctx: &InputResetBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#clockId}.
 * @param ctx the parse tree
 */
fn enter_clockId(&mut self, _ctx: &ClockIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#clockId}.
 * @param ctx the parse tree
 */
fn exit_clockId(&mut self, _ctx: &ClockIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#defaultResetBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_defaultResetBVIStmt(&mut self, _ctx: &DefaultResetBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#defaultResetBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_defaultResetBVIStmt(&mut self, _ctx: &DefaultResetBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#outputResetBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_outputResetBVIStmt(&mut self, _ctx: &OutputResetBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#outputResetBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_outputResetBVIStmt(&mut self, _ctx: &OutputResetBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#ancestorBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_ancestorBVIStmt(&mut self, _ctx: &AncestorBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#ancestorBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_ancestorBVIStmt(&mut self, _ctx: &AncestorBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#sameFamilyBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_sameFamilyBVIStmt(&mut self, _ctx: &SameFamilyBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#sameFamilyBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_sameFamilyBVIStmt(&mut self, _ctx: &SameFamilyBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#scheduleBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_scheduleBVIStmt(&mut self, _ctx: &ScheduleBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#scheduleBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_scheduleBVIStmt(&mut self, _ctx: &ScheduleBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#operatorId}.
 * @param ctx the parse tree
 */
fn enter_operatorId(&mut self, _ctx: &OperatorIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#operatorId}.
 * @param ctx the parse tree
 */
fn exit_operatorId(&mut self, _ctx: &OperatorIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#pathBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_pathBVIStmt(&mut self, _ctx: &PathBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#pathBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_pathBVIStmt(&mut self, _ctx: &PathBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#interfaceBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_interfaceBVIStmt(&mut self, _ctx: &InterfaceBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#interfaceBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_interfaceBVIStmt(&mut self, _ctx: &InterfaceBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#interfaceBVIMembDecl}.
 * @param ctx the parse tree
 */
fn enter_interfaceBVIMembDecl(&mut self, _ctx: &InterfaceBVIMembDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#interfaceBVIMembDecl}.
 * @param ctx the parse tree
 */
fn exit_interfaceBVIMembDecl(&mut self, _ctx: &InterfaceBVIMembDeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#inoutBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_inoutBVIStmt(&mut self, _ctx: &InoutBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#inoutBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_inoutBVIStmt(&mut self, _ctx: &InoutBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#resetId}.
 * @param ctx the parse tree
 */
fn enter_resetId(&mut self, _ctx: &ResetIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#resetId}.
 * @param ctx the parse tree
 */
fn exit_resetId(&mut self, _ctx: &ResetIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#noResetBVIStmt}.
 * @param ctx the parse tree
 */
fn enter_noResetBVIStmt(&mut self, _ctx: &NoResetBVIStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#noResetBVIStmt}.
 * @param ctx the parse tree
 */
fn exit_noResetBVIStmt(&mut self, _ctx: &NoResetBVIStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#externCImport}.
 * @param ctx the parse tree
 */
fn enter_externCImport(&mut self, _ctx: &ExternCImportContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#externCImport}.
 * @param ctx the parse tree
 */
fn exit_externCImport(&mut self, _ctx: &ExternCImportContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#cFuncArgs}.
 * @param ctx the parse tree
 */
fn enter_cFuncArgs(&mut self, _ctx: &CFuncArgsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#cFuncArgs}.
 * @param ctx the parse tree
 */
fn exit_cFuncArgs(&mut self, _ctx: &CFuncArgsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#cFuncArg}.
 * @param ctx the parse tree
 */
fn enter_cFuncArg(&mut self, _ctx: &CFuncArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#cFuncArg}.
 * @param ctx the parse tree
 */
fn exit_cFuncArg(&mut self, _ctx: &CFuncArgContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#fsmStmt}.
 * @param ctx the parse tree
 */
fn enter_fsmStmt(&mut self, _ctx: &FsmStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#fsmStmt}.
 * @param ctx the parse tree
 */
fn exit_fsmStmt(&mut self, _ctx: &FsmStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#exprFsmStmt}.
 * @param ctx the parse tree
 */
fn enter_exprFsmStmt(&mut self, _ctx: &ExprFsmStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#exprFsmStmt}.
 * @param ctx the parse tree
 */
fn exit_exprFsmStmt(&mut self, _ctx: &ExprFsmStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#seqFsmStmt}.
 * @param ctx the parse tree
 */
fn enter_seqFsmStmt(&mut self, _ctx: &SeqFsmStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#seqFsmStmt}.
 * @param ctx the parse tree
 */
fn exit_seqFsmStmt(&mut self, _ctx: &SeqFsmStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#parFsmStmt}.
 * @param ctx the parse tree
 */
fn enter_parFsmStmt(&mut self, _ctx: &ParFsmStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#parFsmStmt}.
 * @param ctx the parse tree
 */
fn exit_parFsmStmt(&mut self, _ctx: &ParFsmStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#ifFsmStmt}.
 * @param ctx the parse tree
 */
fn enter_ifFsmStmt(&mut self, _ctx: &IfFsmStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#ifFsmStmt}.
 * @param ctx the parse tree
 */
fn exit_ifFsmStmt(&mut self, _ctx: &IfFsmStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#whileFsmStmt}.
 * @param ctx the parse tree
 */
fn enter_whileFsmStmt(&mut self, _ctx: &WhileFsmStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#whileFsmStmt}.
 * @param ctx the parse tree
 */
fn exit_whileFsmStmt(&mut self, _ctx: &WhileFsmStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#forFsmStmt}.
 * @param ctx the parse tree
 */
fn enter_forFsmStmt(&mut self, _ctx: &ForFsmStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#forFsmStmt}.
 * @param ctx the parse tree
 */
fn exit_forFsmStmt(&mut self, _ctx: &ForFsmStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#returnFsmStmt}.
 * @param ctx the parse tree
 */
fn enter_returnFsmStmt(&mut self, _ctx: &ReturnFsmStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#returnFsmStmt}.
 * @param ctx the parse tree
 */
fn exit_returnFsmStmt(&mut self, _ctx: &ReturnFsmStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#repeatFsmStmt}.
 * @param ctx the parse tree
 */
fn enter_repeatFsmStmt(&mut self, _ctx: &RepeatFsmStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#repeatFsmStmt}.
 * @param ctx the parse tree
 */
fn exit_repeatFsmStmt(&mut self, _ctx: &RepeatFsmStmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#loopBodyFsmStmt}.
 * @param ctx the parse tree
 */
fn enter_loopBodyFsmStmt(&mut self, _ctx: &LoopBodyFsmStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#loopBodyFsmStmt}.
 * @param ctx the parse tree
 */
fn exit_loopBodyFsmStmt(&mut self, _ctx: &LoopBodyFsmStmtContext<'input>) { }

}
