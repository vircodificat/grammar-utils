use std::collections::BTreeSet;

use crate::{grammar, table::ParseTable};

use super::*;

#[test]
fn test_virdant() {
    let grammar = grammar! {
        Start -> Package;

        Package -> PackageStmt_star;
        PackageStmt_star -> ;
        PackageStmt_star -> PackageStmt_star PackageStmt;

        PackageStmt -> PackageImport semi;
        PackageStmt -> Item;

        PackageImport -> import Ident;

        Item -> ModDef;
        Item -> StructDef;
        Item -> UnionDef;
        Item -> EnumDef;
        Item -> BuiltinDef;
        Item -> SocketDef;
        Item -> FnDef;

        ModDef -> Ext_opt Export_opt mod_ Ident lcurly ModDefStmtSemi_star rcurly;
        Ext_opt -> ;
        Ext_opt -> ext;
        Export_opt -> ;
        Export_opt -> export;

        ModDefStmtSemi -> ModDefStmt semi;
        ModDefStmtSemi_star -> ;
        ModDefStmtSemi_star -> ModDefStmtSemi_star ModDefStmtSemi;

        StructDef -> struct_ type_ Ident lcurly StructDefStmt_star rcurly;
        StructDefStmt_star -> ;
        StructDefStmt_star -> StructDefStmt_star StructDefStmt;

        StructDefStmt -> Ident colon Type semi;

        UnionDef -> union type_ Ident lcurly UnionDefStmt_star rcurly;
        UnionDefStmt_star -> ;
        UnionDefStmt_star -> UnionDefStmt_star UnionDefStmt;

        UnionDefStmt -> Ident ParamList_opt semi;
        ParamList_opt -> ;
        ParamList_opt -> ParamList;

        ParamList -> lparen Params_opt rparen;
        Params_opt -> ;
        Params_opt -> Params;

        Params -> Param;
        Params -> Params comma Param;

        Param -> Ident colon Type;

        GenericsParams -> lbracket Nat rbracket;
        GenericsParams_opt -> ;
        GenericsParams_opt -> GenericsParams;

        Generics -> lbracket Ident colon Kind rbracket;
        Generics_opt -> ;
        Generics_opt -> Generics;

        EnumDef -> enum_ type_ Ident width Width lcurly EnumDefStmt_star rcurly;
        EnumDefStmt_star -> ;
        EnumDefStmt_star -> EnumDefStmt_star EnumDefStmt;

        EnumDefStmt -> Ident eq Expr semi;

        BuiltinDef -> builtin type_ Ident Generics_opt lcurly rcurly;

        SocketDef -> socket Ident lcurly SocketDefStmt_star rcurly;
        SocketDefStmt_star -> ;
        SocketDefStmt_star -> SocketDefStmt_star SocketDefStmt;

        SocketDefStmt -> mosi Ident colon Type semi;
        SocketDefStmt -> miso Ident colon Type semi;

        FnDef -> fn_ Ident ParamList arrow Type lcurly Expr rcurly;

        ModDefStmt -> ModDefStmtComponent;
        ModDefStmt -> ModDefStmtDriver;
        ModDefStmt -> ModDefStmtInstance;
        ModDefStmt -> ModDefStmtSocket;
        ModDefStmt -> ModDefStmtOn;
        ModDefStmt -> ModDefStmtIf;
        ModDefStmt -> ModDefStmtMatch;

        ModDefStmtComponent -> incoming Ident colon Type OnClause_opt;
        ModDefStmtComponent -> outgoing Ident colon Type OnClause_opt;
        ModDefStmtComponent -> wire Ident colon Type OnClause_opt;
        ModDefStmtComponent -> reg Ident colon Type OnClause_opt;

        OnClause -> on Expr;
        OnClause_opt -> ;
        OnClause_opt -> OnClause;

        ModDefStmtDriver -> Path coloneq Expr;
        ModDefStmtDriver -> Path lteq Expr;
        ModDefStmtDriver -> Path coloneqcolon Path;

        ModDefStmtInstance -> mod_ Ident of Ofness ItBlock_opt;
        ItBlock_opt -> ;
        ItBlock_opt -> ItBlock;

        ItBlock -> ModDefStmtBlock;

        ModDefStmtSocket -> master socket Ident of Ofness;
        ModDefStmtSocket -> slave socket Ident of Ofness;

        ModDefStmtOn -> on Expr lcurly CommandSemi_star rcurly;
        CommandSemi -> Command semi;
        CommandSemi_star -> ;
        CommandSemi_star -> CommandSemi_star CommandSemi;

        Command -> if_ Expr lcurly CommandSemi_star rcurly;
        Command -> assert lparen Expr rparen;
        Command -> display lparen Str comma Expr rparen;
        Command -> finish;
        Command -> fatal;

        ModDefStmtIf -> ModDefStmtIfStart ModDefStmtIfMiddle_star ModDefStmtIfEnd_opt;
        ModDefStmtIfMiddle_star -> ;
        ModDefStmtIfMiddle_star -> ModDefStmtIfMiddle_star ModDefStmtIfMiddle;
        ModDefStmtIfEnd_opt -> ;
        ModDefStmtIfEnd_opt -> ModDefStmtIfEnd;

        ModDefStmtIfStart -> if_ Expr ModDefStmtBlock;
        ModDefStmtIfMiddle -> else_ if_ Expr ModDefStmtBlock;
        ModDefStmtIfEnd -> else_ ModDefStmtBlock;

        ModDefStmtMatch -> match_ Expr lcurly ModDefStmtMatchArm_star rcurly;
        ModDefStmtMatchArm_star -> ;
        ModDefStmtMatchArm_star -> ModDefStmtMatchArm_star ModDefStmtMatchArm;

        ModDefStmtMatchArm -> Pat fatarrow ModDefStmtBlock semi;

        ModDefStmtBlock -> lcurly ModDefStmtSemi_star rcurly;

        Kind -> Ident;

        Type -> Ofness GenericsParams_opt;

        Expr -> ExprIf;
        Expr -> ExprMatch;
        Expr -> ExprStruct;
        Expr -> ExprBinOpLogical;

        ExprIf -> ExprIfStart ExprIfMiddle_opt ExprIfEnd;
        ExprIfMiddle_opt -> ;
        ExprIfMiddle_opt -> ExprIfMiddle;

        ExprIfStart -> if_ Expr lcurly Expr rcurly;
        ExprIfMiddle -> else_ if_ Expr lcurly Expr rcurly;
        ExprIfMiddle -> ExprIfMiddle else_ if_ Expr lcurly Expr rcurly;
        ExprIfEnd -> else_ lcurly Expr rcurly;

        ExprMatch -> match_ Expr lcurly ExprMatchArm_star rcurly;
        ExprMatchArm_star -> ;
        ExprMatchArm_star -> ExprMatchArm_star ExprMatchArm;

        ExprMatchArm -> Pat fatarrow Expr semi;

        Pat -> hash Ident;
        Pat -> at Ident;
        Pat -> at Ident lparen ArgList rparen;
        Pat -> else_;

        ExprStruct -> dollar lcurly AssignList rcurly;

        AssignList -> AssignComma_star Assign;
        AssignList -> AssignComma_star Assign comma;
        AssignList -> Expr;
        AssignList -> Expr comma;
        AssignList -> ;

        AssignComma -> Assign comma;
        AssignComma_star -> ;
        AssignComma_star -> AssignComma_star AssignComma;

        Assign -> Ident eq Expr;

        BinOpLogical -> andand;
        BinOpLogical -> pipepipe;
        BinOpLogical -> hathat;

        ExprBinOpLogical -> ExprBinOpLogical BinOpLogical ExprBinOpCompare;
        ExprBinOpLogical -> ExprBinOpCompare;

        BinOpCompare -> lt;
        BinOpCompare -> lteq;
        BinOpCompare -> gt;
        BinOpCompare -> gteq;
        BinOpCompare -> eqeq;
        BinOpCompare -> neq;

        ExprBinOpCompare -> ExprBinOpCompare BinOpCompare ExprBinOpAdditive;
        ExprBinOpCompare -> ExprBinOpAdditive;

        BinOpAdditive -> plus;
        BinOpAdditive -> minus;
        BinOpAdditive -> and;
        BinOpAdditive -> pipe;
        BinOpAdditive -> hat;

        ExprBinOpAdditive -> ExprBinOpAdditive BinOpAdditive ExprUnOp;
        ExprBinOpAdditive -> ExprUnOp;

        UnOp -> minus;
        UnOp -> tilde;
        UnOp -> bang;

        ExprUnOp -> UnOp ExprUnOp;
        ExprUnOp -> ExprAscription;

        ExprAscription -> ExprPrimary colon Type;
        ExprAscription -> ExprPrimary;

        ExprPrimary -> Ofness lparen ArgList rparen;
        ExprPrimary -> ExprPrimary arrow Ident lparen ArgList rparen;
        ExprPrimary -> ExprPrimary arrow Ident;
        ExprPrimary -> ExprPrimary lbracket Index rbracket;
        ExprPrimary -> ExprPrimary lbracket Index dotdot Index rbracket;
        ExprPrimary -> at Ident lparen ArgList rparen;
        ExprPrimary -> ExprAtom;

        ExprAtom -> Path;
        ExprAtom -> WordLit;
        ExprAtom -> true_;
        ExprAtom -> false_;
        ExprAtom -> string;
        ExprAtom -> hash Ident;
        ExprAtom -> at Ident;
        ExprAtom -> question;
        ExprAtom -> lparen Expr rparen;

        ArgList -> ExprComma_plus Expr;
        ArgList -> ExprComma_plus Expr comma;
        ArgList -> Expr;
        ArgList -> Expr comma;
        ArgList -> ;

        ExprComma -> Expr comma;
        ExprComma_plus -> ExprComma;
        ExprComma_plus -> ExprComma_plus ExprComma;

        Ofness -> Ident;
        Ofness -> Ident coloncolon Ident;

        Path -> Ident;
        Path -> Path dot Ident;

        Index -> nat;
        Width -> nat;

        WordLit -> nat;
        WordLit -> word;

        Ident -> ident;
        Nat -> nat;
        Str -> string;
    };

//    let symbols: BTreeSet<String> = grammar.symbols().into_iter().map(|symbol| symbol.name()).collect();
//    let terminals: BTreeSet<String> = grammar.terminals().into_iter().map(|symbol| symbol.name()).collect();
//    println!("symbols:");
//    for symbol in terminals.into_iter() {
//        println!("  {symbol}");
//    }

    let table = ParseTable::build(&grammar, grammar.rules()[0]);
    println!("Number of states: {}", table.states.len());
    println!();

    println!("Conflicts:");
    for states in table.inadequate_states() {
        eprintln!("    {states:?}");
        //eprintln!("    {:?}", conflict.state());
        eprintln!();
    }
}
