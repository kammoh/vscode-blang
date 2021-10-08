#![allow(nonstandard_style)]
// Generated from BSV.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::bsvparser::*;

pub trait BSVListener<'input> : ParseTreeListener<'input,BSVParserContextType>{

/**
 * Enter a parse tree produced by {@link BSVParser#packagedef}.
 * @param ctx the parse tree
 */
fn enter_packagedef(&mut self, _ctx: &PackagedefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#packagedef}.
 * @param ctx the parse tree
 */
fn exit_packagedef(&mut self, _ctx: &PackagedefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#packagedecl}.
 * @param ctx the parse tree
 */
fn enter_packagedecl(&mut self, _ctx: &PackagedeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#packagedecl}.
 * @param ctx the parse tree
 */
fn exit_packagedecl(&mut self, _ctx: &PackagedeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#endpackage}.
 * @param ctx the parse tree
 */
fn enter_endpackage(&mut self, _ctx: &EndpackageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#endpackage}.
 * @param ctx the parse tree
 */
fn exit_endpackage(&mut self, _ctx: &EndpackageContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#lowerCaseIdentifier}.
 * @param ctx the parse tree
 */
fn enter_lowerCaseIdentifier(&mut self, _ctx: &LowerCaseIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#lowerCaseIdentifier}.
 * @param ctx the parse tree
 */
fn exit_lowerCaseIdentifier(&mut self, _ctx: &LowerCaseIdentifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#upperCaseIdentifier}.
 * @param ctx the parse tree
 */
fn enter_upperCaseIdentifier(&mut self, _ctx: &UpperCaseIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#upperCaseIdentifier}.
 * @param ctx the parse tree
 */
fn exit_upperCaseIdentifier(&mut self, _ctx: &UpperCaseIdentifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#anyidentifier}.
 * @param ctx the parse tree
 */
fn enter_anyidentifier(&mut self, _ctx: &AnyidentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#anyidentifier}.
 * @param ctx the parse tree
 */
fn exit_anyidentifier(&mut self, _ctx: &AnyidentifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#exportdecl}.
 * @param ctx the parse tree
 */
fn enter_exportdecl(&mut self, _ctx: &ExportdeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#exportdecl}.
 * @param ctx the parse tree
 */
fn exit_exportdecl(&mut self, _ctx: &ExportdeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#exportitem}.
 * @param ctx the parse tree
 */
fn enter_exportitem(&mut self, _ctx: &ExportitemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#exportitem}.
 * @param ctx the parse tree
 */
fn exit_exportitem(&mut self, _ctx: &ExportitemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#importdecl}.
 * @param ctx the parse tree
 */
fn enter_importdecl(&mut self, _ctx: &ImportdeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#importdecl}.
 * @param ctx the parse tree
 */
fn exit_importdecl(&mut self, _ctx: &ImportdeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#packagestmt}.
 * @param ctx the parse tree
 */
fn enter_packagestmt(&mut self, _ctx: &PackagestmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#packagestmt}.
 * @param ctx the parse tree
 */
fn exit_packagestmt(&mut self, _ctx: &PackagestmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#packageide}.
 * @param ctx the parse tree
 */
fn enter_packageide(&mut self, _ctx: &PackageideContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#packageide}.
 * @param ctx the parse tree
 */
fn exit_packageide(&mut self, _ctx: &PackageideContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#interfacedecl}.
 * @param ctx the parse tree
 */
fn enter_interfacedecl(&mut self, _ctx: &InterfacedeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#interfacedecl}.
 * @param ctx the parse tree
 */
fn exit_interfacedecl(&mut self, _ctx: &InterfacedeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#interfacememberdecl}.
 * @param ctx the parse tree
 */
fn enter_interfacememberdecl(&mut self, _ctx: &InterfacememberdeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#interfacememberdecl}.
 * @param ctx the parse tree
 */
fn exit_interfacememberdecl(&mut self, _ctx: &InterfacememberdeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodproto}.
 * @param ctx the parse tree
 */
fn enter_methodproto(&mut self, _ctx: &MethodprotoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodproto}.
 * @param ctx the parse tree
 */
fn exit_methodproto(&mut self, _ctx: &MethodprotoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodprotoformals}.
 * @param ctx the parse tree
 */
fn enter_methodprotoformals(&mut self, _ctx: &MethodprotoformalsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodprotoformals}.
 * @param ctx the parse tree
 */
fn exit_methodprotoformals(&mut self, _ctx: &MethodprotoformalsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodprotoformal}.
 * @param ctx the parse tree
 */
fn enter_methodprotoformal(&mut self, _ctx: &MethodprotoformalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodprotoformal}.
 * @param ctx the parse tree
 */
fn exit_methodprotoformal(&mut self, _ctx: &MethodprotoformalContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#subinterfacedecl}.
 * @param ctx the parse tree
 */
fn enter_subinterfacedecl(&mut self, _ctx: &SubinterfacedeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#subinterfacedecl}.
 * @param ctx the parse tree
 */
fn exit_subinterfacedecl(&mut self, _ctx: &SubinterfacedeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedeftype}.
 * @param ctx the parse tree
 */
fn enter_typedeftype(&mut self, _ctx: &TypedeftypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedeftype}.
 * @param ctx the parse tree
 */
fn exit_typedeftype(&mut self, _ctx: &TypedeftypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeformals}.
 * @param ctx the parse tree
 */
fn enter_typeformals(&mut self, _ctx: &TypeformalsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeformals}.
 * @param ctx the parse tree
 */
fn exit_typeformals(&mut self, _ctx: &TypeformalsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeformal}.
 * @param ctx the parse tree
 */
fn enter_typeformal(&mut self, _ctx: &TypeformalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeformal}.
 * @param ctx the parse tree
 */
fn exit_typeformal(&mut self, _ctx: &TypeformalContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedefsynonym}.
 * @param ctx the parse tree
 */
fn enter_typedefsynonym(&mut self, _ctx: &TypedefsynonymContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedefsynonym}.
 * @param ctx the parse tree
 */
fn exit_typedefsynonym(&mut self, _ctx: &TypedefsynonymContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedefenum}.
 * @param ctx the parse tree
 */
fn enter_typedefenum(&mut self, _ctx: &TypedefenumContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedefenum}.
 * @param ctx the parse tree
 */
fn exit_typedefenum(&mut self, _ctx: &TypedefenumContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedefenumelement}.
 * @param ctx the parse tree
 */
fn enter_typedefenumelement(&mut self, _ctx: &TypedefenumelementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedefenumelement}.
 * @param ctx the parse tree
 */
fn exit_typedefenumelement(&mut self, _ctx: &TypedefenumelementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedefstruct}.
 * @param ctx the parse tree
 */
fn enter_typedefstruct(&mut self, _ctx: &TypedefstructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedefstruct}.
 * @param ctx the parse tree
 */
fn exit_typedefstruct(&mut self, _ctx: &TypedefstructContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typedeftaggedunion}.
 * @param ctx the parse tree
 */
fn enter_typedeftaggedunion(&mut self, _ctx: &TypedeftaggedunionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typedeftaggedunion}.
 * @param ctx the parse tree
 */
fn exit_typedeftaggedunion(&mut self, _ctx: &TypedeftaggedunionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#structmember}.
 * @param ctx the parse tree
 */
fn enter_structmember(&mut self, _ctx: &StructmemberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#structmember}.
 * @param ctx the parse tree
 */
fn exit_structmember(&mut self, _ctx: &StructmemberContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#unionmember}.
 * @param ctx the parse tree
 */
fn enter_unionmember(&mut self, _ctx: &UnionmemberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#unionmember}.
 * @param ctx the parse tree
 */
fn exit_unionmember(&mut self, _ctx: &UnionmemberContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#substruct}.
 * @param ctx the parse tree
 */
fn enter_substruct(&mut self, _ctx: &SubstructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#substruct}.
 * @param ctx the parse tree
 */
fn exit_substruct(&mut self, _ctx: &SubstructContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#subunion}.
 * @param ctx the parse tree
 */
fn enter_subunion(&mut self, _ctx: &SubunionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#subunion}.
 * @param ctx the parse tree
 */
fn exit_subunion(&mut self, _ctx: &SubunionContext<'input>) { }

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
 * Enter a parse tree produced by {@link BSVParser#moduleinst}.
 * @param ctx the parse tree
 */
fn enter_moduleinst(&mut self, _ctx: &ModuleinstContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#moduleinst}.
 * @param ctx the parse tree
 */
fn exit_moduleinst(&mut self, _ctx: &ModuleinstContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#tuplebind}.
 * @param ctx the parse tree
 */
fn enter_tuplebind(&mut self, _ctx: &TuplebindContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#tuplebind}.
 * @param ctx the parse tree
 */
fn exit_tuplebind(&mut self, _ctx: &TuplebindContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#varinit}.
 * @param ctx the parse tree
 */
fn enter_varinit(&mut self, _ctx: &VarinitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#varinit}.
 * @param ctx the parse tree
 */
fn exit_varinit(&mut self, _ctx: &VarinitContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#varbinding}.
 * @param ctx the parse tree
 */
fn enter_varbinding(&mut self, _ctx: &VarbindingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#varbinding}.
 * @param ctx the parse tree
 */
fn exit_varbinding(&mut self, _ctx: &VarbindingContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#actionbinding}.
 * @param ctx the parse tree
 */
fn enter_actionbinding(&mut self, _ctx: &ActionbindingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#actionbinding}.
 * @param ctx the parse tree
 */
fn exit_actionbinding(&mut self, _ctx: &ActionbindingContext<'input>) { }

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
 * Enter a parse tree produced by {@link BSVParser#typeclassdecl}.
 * @param ctx the parse tree
 */
fn enter_typeclassdecl(&mut self, _ctx: &TypeclassdeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeclassdecl}.
 * @param ctx the parse tree
 */
fn exit_typeclassdecl(&mut self, _ctx: &TypeclassdeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeclasside}.
 * @param ctx the parse tree
 */
fn enter_typeclasside(&mut self, _ctx: &TypeclassideContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeclasside}.
 * @param ctx the parse tree
 */
fn exit_typeclasside(&mut self, _ctx: &TypeclassideContext<'input>) { }

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
 * Enter a parse tree produced by {@link BSVParser#overloadeddecl}.
 * @param ctx the parse tree
 */
fn enter_overloadeddecl(&mut self, _ctx: &OverloadeddeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#overloadeddecl}.
 * @param ctx the parse tree
 */
fn exit_overloadeddecl(&mut self, _ctx: &OverloadeddeclContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#tctype}.
 * @param ctx the parse tree
 */
fn enter_tctype(&mut self, _ctx: &TctypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#tctype}.
 * @param ctx the parse tree
 */
fn exit_tctype(&mut self, _ctx: &TctypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeclassinstance}.
 * @param ctx the parse tree
 */
fn enter_typeclassinstance(&mut self, _ctx: &TypeclassinstanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeclassinstance}.
 * @param ctx the parse tree
 */
fn exit_typeclassinstance(&mut self, _ctx: &TypeclassinstanceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#overloadeddef}.
 * @param ctx the parse tree
 */
fn enter_overloadeddef(&mut self, _ctx: &OverloadeddefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#overloadeddef}.
 * @param ctx the parse tree
 */
fn exit_overloadeddef(&mut self, _ctx: &OverloadeddefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#moduledef}.
 * @param ctx the parse tree
 */
fn enter_moduledef(&mut self, _ctx: &ModuledefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#moduledef}.
 * @param ctx the parse tree
 */
fn exit_moduledef(&mut self, _ctx: &ModuledefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#moduleproto}.
 * @param ctx the parse tree
 */
fn enter_moduleproto(&mut self, _ctx: &ModuleprotoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#moduleproto}.
 * @param ctx the parse tree
 */
fn exit_moduleproto(&mut self, _ctx: &ModuleprotoContext<'input>) { }

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
 * Enter a parse tree produced by {@link BSVParser#modulestmt}.
 * @param ctx the parse tree
 */
fn enter_modulestmt(&mut self, _ctx: &ModulestmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#modulestmt}.
 * @param ctx the parse tree
 */
fn exit_modulestmt(&mut self, _ctx: &ModulestmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methoddef}.
 * @param ctx the parse tree
 */
fn enter_methoddef(&mut self, _ctx: &MethoddefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methoddef}.
 * @param ctx the parse tree
 */
fn exit_methoddef(&mut self, _ctx: &MethoddefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodformals}.
 * @param ctx the parse tree
 */
fn enter_methodformals(&mut self, _ctx: &MethodformalsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodformals}.
 * @param ctx the parse tree
 */
fn exit_methodformals(&mut self, _ctx: &MethodformalsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodformal}.
 * @param ctx the parse tree
 */
fn enter_methodformal(&mut self, _ctx: &MethodformalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodformal}.
 * @param ctx the parse tree
 */
fn exit_methodformal(&mut self, _ctx: &MethodformalContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#methodcond}.
 * @param ctx the parse tree
 */
fn enter_methodcond(&mut self, _ctx: &MethodcondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#methodcond}.
 * @param ctx the parse tree
 */
fn exit_methodcond(&mut self, _ctx: &MethodcondContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#subinterfacedef}.
 * @param ctx the parse tree
 */
fn enter_subinterfacedef(&mut self, _ctx: &SubinterfacedefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#subinterfacedef}.
 * @param ctx the parse tree
 */
fn exit_subinterfacedef(&mut self, _ctx: &SubinterfacedefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#ruledef}.
 * @param ctx the parse tree
 */
fn enter_ruledef(&mut self, _ctx: &RuledefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#ruledef}.
 * @param ctx the parse tree
 */
fn exit_ruledef(&mut self, _ctx: &RuledefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#rulecond}.
 * @param ctx the parse tree
 */
fn enter_rulecond(&mut self, _ctx: &RulecondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#rulecond}.
 * @param ctx the parse tree
 */
fn exit_rulecond(&mut self, _ctx: &RulecondContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#functiondef}.
 * @param ctx the parse tree
 */
fn enter_functiondef(&mut self, _ctx: &FunctiondefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#functiondef}.
 * @param ctx the parse tree
 */
fn exit_functiondef(&mut self, _ctx: &FunctiondefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#functionproto}.
 * @param ctx the parse tree
 */
fn enter_functionproto(&mut self, _ctx: &FunctionprotoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#functionproto}.
 * @param ctx the parse tree
 */
fn exit_functionproto(&mut self, _ctx: &FunctionprotoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#externcimport}.
 * @param ctx the parse tree
 */
fn enter_externcimport(&mut self, _ctx: &ExterncimportContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#externcimport}.
 * @param ctx the parse tree
 */
fn exit_externcimport(&mut self, _ctx: &ExterncimportContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#externcfuncargs}.
 * @param ctx the parse tree
 */
fn enter_externcfuncargs(&mut self, _ctx: &ExterncfuncargsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#externcfuncargs}.
 * @param ctx the parse tree
 */
fn exit_externcfuncargs(&mut self, _ctx: &ExterncfuncargsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#externcfuncarg}.
 * @param ctx the parse tree
 */
fn enter_externcfuncarg(&mut self, _ctx: &ExterncfuncargContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#externcfuncarg}.
 * @param ctx the parse tree
 */
fn exit_externcfuncarg(&mut self, _ctx: &ExterncfuncargContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#varassign}.
 * @param ctx the parse tree
 */
fn enter_varassign(&mut self, _ctx: &VarassignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#varassign}.
 * @param ctx the parse tree
 */
fn exit_varassign(&mut self, _ctx: &VarassignContext<'input>) { }

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
 * Enter a parse tree produced by {@link BSVParser#bsvtype}.
 * @param ctx the parse tree
 */
fn enter_bsvtype(&mut self, _ctx: &BsvtypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#bsvtype}.
 * @param ctx the parse tree
 */
fn exit_bsvtype(&mut self, _ctx: &BsvtypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typeide}.
 * @param ctx the parse tree
 */
fn enter_typeide(&mut self, _ctx: &TypeideContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typeide}.
 * @param ctx the parse tree
 */
fn exit_typeide(&mut self, _ctx: &TypeideContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#typenat}.
 * @param ctx the parse tree
 */
fn enter_typenat(&mut self, _ctx: &TypenatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#typenat}.
 * @param ctx the parse tree
 */
fn exit_typenat(&mut self, _ctx: &TypenatContext<'input>) { }

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
 * Enter a parse tree produced by the {@code condexpr}
 * labeled alternative in {@link BSVParser#expression}.
 * @param ctx the parse tree
 */
fn enter_condexpr(&mut self, _ctx: &CondexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code condexpr}
 * labeled alternative in {@link BSVParser#expression}.
 * @param ctx the parse tree
 */
fn exit_condexpr(&mut self, _ctx: &CondexprContext<'input>) { }

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
 * Enter a parse tree produced by the {@code bitconcat}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_bitconcat(&mut self, _ctx: &BitconcatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code bitconcat}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_bitconcat(&mut self, _ctx: &BitconcatContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code whenexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_whenexpr(&mut self, _ctx: &WhenexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code whenexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_whenexpr(&mut self, _ctx: &WhenexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code varexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_varexpr(&mut self, _ctx: &VarexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code varexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_varexpr(&mut self, _ctx: &VarexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code interfaceexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_interfaceexpr(&mut self, _ctx: &InterfaceexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code interfaceexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_interfaceexpr(&mut self, _ctx: &InterfaceexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code blockexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_blockexpr(&mut self, _ctx: &BlockexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code blockexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_blockexpr(&mut self, _ctx: &BlockexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code stringliteral}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_stringliteral(&mut self, _ctx: &StringliteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringliteral}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_stringliteral(&mut self, _ctx: &StringliteralContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code syscallexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_syscallexpr(&mut self, _ctx: &SyscallexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code syscallexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_syscallexpr(&mut self, _ctx: &SyscallexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code callexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_callexpr(&mut self, _ctx: &CallexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code callexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_callexpr(&mut self, _ctx: &CallexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code intliteral}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_intliteral(&mut self, _ctx: &IntliteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code intliteral}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_intliteral(&mut self, _ctx: &IntliteralContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code realliteral}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_realliteral(&mut self, _ctx: &RealliteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code realliteral}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_realliteral(&mut self, _ctx: &RealliteralContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code valueofexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_valueofexpr(&mut self, _ctx: &ValueofexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueofexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_valueofexpr(&mut self, _ctx: &ValueofexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code castexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_castexpr(&mut self, _ctx: &CastexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code castexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_castexpr(&mut self, _ctx: &CastexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code typeassertionexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_typeassertionexpr(&mut self, _ctx: &TypeassertionexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code typeassertionexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_typeassertionexpr(&mut self, _ctx: &TypeassertionexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code resetbyexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_resetbyexpr(&mut self, _ctx: &ResetbyexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code resetbyexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_resetbyexpr(&mut self, _ctx: &ResetbyexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code taggedunionexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_taggedunionexpr(&mut self, _ctx: &TaggedunionexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taggedunionexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_taggedunionexpr(&mut self, _ctx: &TaggedunionexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code arraysub}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_arraysub(&mut self, _ctx: &ArraysubContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code arraysub}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_arraysub(&mut self, _ctx: &ArraysubContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code undefinedexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_undefinedexpr(&mut self, _ctx: &UndefinedexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code undefinedexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_undefinedexpr(&mut self, _ctx: &UndefinedexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code clockedbyexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_clockedbyexpr(&mut self, _ctx: &ClockedbyexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code clockedbyexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_clockedbyexpr(&mut self, _ctx: &ClockedbyexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code actionvalueblockexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_actionvalueblockexpr(&mut self, _ctx: &ActionvalueblockexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code actionvalueblockexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_actionvalueblockexpr(&mut self, _ctx: &ActionvalueblockexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code fieldexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_fieldexpr(&mut self, _ctx: &FieldexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fieldexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn exit_fieldexpr(&mut self, _ctx: &FieldexprContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code parenexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
 * @param ctx the parse tree
 */
fn enter_parenexpr(&mut self, _ctx: &ParenexprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parenexpr}
 * labeled alternative in {@link BSVParser#exprprimary}.
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
 * Enter a parse tree produced by {@link BSVParser#beginendblock}.
 * @param ctx the parse tree
 */
fn enter_beginendblock(&mut self, _ctx: &BeginendblockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#beginendblock}.
 * @param ctx the parse tree
 */
fn exit_beginendblock(&mut self, _ctx: &BeginendblockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#actionblock}.
 * @param ctx the parse tree
 */
fn enter_actionblock(&mut self, _ctx: &ActionblockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#actionblock}.
 * @param ctx the parse tree
 */
fn exit_actionblock(&mut self, _ctx: &ActionblockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#actionvalueblock}.
 * @param ctx the parse tree
 */
fn enter_actionvalueblock(&mut self, _ctx: &ActionvalueblockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#actionvalueblock}.
 * @param ctx the parse tree
 */
fn exit_actionvalueblock(&mut self, _ctx: &ActionvalueblockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#regwrite}.
 * @param ctx the parse tree
 */
fn enter_regwrite(&mut self, _ctx: &RegwriteContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#regwrite}.
 * @param ctx the parse tree
 */
fn exit_regwrite(&mut self, _ctx: &RegwriteContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_stmt(&mut self, _ctx: &StmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_stmt(&mut self, _ctx: &StmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#ifstmt}.
 * @param ctx the parse tree
 */
fn enter_ifstmt(&mut self, _ctx: &IfstmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#ifstmt}.
 * @param ctx the parse tree
 */
fn exit_ifstmt(&mut self, _ctx: &IfstmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#casestmt}.
 * @param ctx the parse tree
 */
fn enter_casestmt(&mut self, _ctx: &CasestmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#casestmt}.
 * @param ctx the parse tree
 */
fn exit_casestmt(&mut self, _ctx: &CasestmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#casestmtitem}.
 * @param ctx the parse tree
 */
fn enter_casestmtitem(&mut self, _ctx: &CasestmtitemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#casestmtitem}.
 * @param ctx the parse tree
 */
fn exit_casestmtitem(&mut self, _ctx: &CasestmtitemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#casestmtpatitem}.
 * @param ctx the parse tree
 */
fn enter_casestmtpatitem(&mut self, _ctx: &CasestmtpatitemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#casestmtpatitem}.
 * @param ctx the parse tree
 */
fn exit_casestmtpatitem(&mut self, _ctx: &CasestmtpatitemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#casestmtdefaultitem}.
 * @param ctx the parse tree
 */
fn enter_casestmtdefaultitem(&mut self, _ctx: &CasestmtdefaultitemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#casestmtdefaultitem}.
 * @param ctx the parse tree
 */
fn exit_casestmtdefaultitem(&mut self, _ctx: &CasestmtdefaultitemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#whilestmt}.
 * @param ctx the parse tree
 */
fn enter_whilestmt(&mut self, _ctx: &WhilestmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#whilestmt}.
 * @param ctx the parse tree
 */
fn exit_whilestmt(&mut self, _ctx: &WhilestmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#forstmt}.
 * @param ctx the parse tree
 */
fn enter_forstmt(&mut self, _ctx: &ForstmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#forstmt}.
 * @param ctx the parse tree
 */
fn exit_forstmt(&mut self, _ctx: &ForstmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#forinit}.
 * @param ctx the parse tree
 */
fn enter_forinit(&mut self, _ctx: &ForinitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#forinit}.
 * @param ctx the parse tree
 */
fn exit_forinit(&mut self, _ctx: &ForinitContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#simplevardeclassign}.
 * @param ctx the parse tree
 */
fn enter_simplevardeclassign(&mut self, _ctx: &SimplevardeclassignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#simplevardeclassign}.
 * @param ctx the parse tree
 */
fn exit_simplevardeclassign(&mut self, _ctx: &SimplevardeclassignContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#fortest}.
 * @param ctx the parse tree
 */
fn enter_fortest(&mut self, _ctx: &FortestContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#fortest}.
 * @param ctx the parse tree
 */
fn exit_fortest(&mut self, _ctx: &FortestContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#forincr}.
 * @param ctx the parse tree
 */
fn enter_forincr(&mut self, _ctx: &ForincrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#forincr}.
 * @param ctx the parse tree
 */
fn exit_forincr(&mut self, _ctx: &ForincrContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#varincr}.
 * @param ctx the parse tree
 */
fn enter_varincr(&mut self, _ctx: &VarincrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#varincr}.
 * @param ctx the parse tree
 */
fn exit_varincr(&mut self, _ctx: &VarincrContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#returnstmt}.
 * @param ctx the parse tree
 */
fn enter_returnstmt(&mut self, _ctx: &ReturnstmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#returnstmt}.
 * @param ctx the parse tree
 */
fn exit_returnstmt(&mut self, _ctx: &ReturnstmtContext<'input>) { }

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
 * Enter a parse tree produced by {@link BSVParser#constantpattern}.
 * @param ctx the parse tree
 */
fn enter_constantpattern(&mut self, _ctx: &ConstantpatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#constantpattern}.
 * @param ctx the parse tree
 */
fn exit_constantpattern(&mut self, _ctx: &ConstantpatternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#taggedunionpattern}.
 * @param ctx the parse tree
 */
fn enter_taggedunionpattern(&mut self, _ctx: &TaggedunionpatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#taggedunionpattern}.
 * @param ctx the parse tree
 */
fn exit_taggedunionpattern(&mut self, _ctx: &TaggedunionpatternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#tuplepattern}.
 * @param ctx the parse tree
 */
fn enter_tuplepattern(&mut self, _ctx: &TuplepatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#tuplepattern}.
 * @param ctx the parse tree
 */
fn exit_tuplepattern(&mut self, _ctx: &TuplepatternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#attributeinstance}.
 * @param ctx the parse tree
 */
fn enter_attributeinstance(&mut self, _ctx: &AttributeinstanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#attributeinstance}.
 * @param ctx the parse tree
 */
fn exit_attributeinstance(&mut self, _ctx: &AttributeinstanceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link BSVParser#attrspec}.
 * @param ctx the parse tree
 */
fn enter_attrspec(&mut self, _ctx: &AttrspecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link BSVParser#attrspec}.
 * @param ctx the parse tree
 */
fn exit_attrspec(&mut self, _ctx: &AttrspecContext<'input>) { }

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

}
