use pest::iterators::{Pair, Pairs};
use std::fs;
use pest::Parser;
use pest_derive::Parser;

use super::ast::{enumm::{Enum, EnumEntry}, expression::{Atom, BinOp, ExprTail, Expression, UnaryOp}, function::{Function, FunctionHeader, Parameter}, identifier::Identifier, program::Program, sstruct::{Struct, StructField, StructMember}, statement::{CaseStatement, ConditionBody, IdentifierExpression, Statement}, toplevel::TopLevel, typedef::{TypeDef, TypeDefInner}, types::{Type, TypeType}};

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct MiniCParser;

pub struct MyMiniCParser {}

impl MyMiniCParser {
    pub fn parse_file(filepath: String) -> Result<Program, String> {
        let unparsed_file = fs::read_to_string(filepath).expect("cannot read file");
        let main_pair = MiniCParser::parse(Rule::program, &unparsed_file)
            .expect("unsuccessful parse")
            .next().unwrap();
        Self::parse_main(main_pair)
    }

    fn parse_main(pair: Pair<Rule>) -> Result<Program, String> {
        let rule = pair.as_rule();
        match rule {
            Rule::program => {
                let mut statements = Vec::<TopLevel>::new();
                for p in pair.into_inner() {
                    let r = p.as_rule();
                    match r {
                        Rule::topLevel => {
                            statements.push(Self::parse_top_level(p.into_inner().next().unwrap())?);
                        }
                        Rule::EOI => (), // Do nothing, reached end of file
                        _ => return Result::Err(String::from("Could not parse top-level statement")),
                    }
                }
                Result::Ok(
                    Program {
                        statements: statements
                    }
                )
            },
            _ => panic!("Error: program type currently not supported"),
        }
    }

    fn parse_top_level(pair: Pair<Rule>) -> Result<TopLevel, String> {
        match pair.as_rule() {
            Rule::varDec => {
                let mut pairs = pair.into_inner();
                let mut modifiers = Vec::<String>::new();
                let mut next = pairs.next().unwrap();
                loop {
                    match next.as_rule() {
                        Rule::varModifier => modifiers.push(String::from(next.as_str())),
                        _ => break,
                    }
                    next = pairs.next().unwrap();
                }

                let typ = Self::parse_type(next)?;
                let name = pairs.next().unwrap().as_str();
                let mut init_val: Option<Expression> = None;
                if let Some(expr_pair) = pairs.next() {
                    let result = Self::parse_expression(expr_pair)?;
                    init_val = Some(result);
                }
                Result::Ok(
                    TopLevel::VarDeclaration { 
                        typ: typ,
                        name: String::from(name),
                        right: init_val,
                        modifier: modifiers,
                    }
                )
            },
            Rule::function => {
                let func = Self::parse_function(pair)?;
                Result::Ok(
                    TopLevel::Function(func),
                )
            },
            Rule::r#struct => {
                let struc = Self::parse_struct(pair)?;
                Result::Ok(
                    TopLevel::Struct(struc),
                )
            },
            Rule::r#enum => {
                let enumm = Self::parse_enum(pair)?;
                Result::Ok(
                    TopLevel::Enum(enumm),
                )
            },
            Rule::libImport => {
                let path = pair.into_inner().next().unwrap().as_str();
                Result::Ok(
                    TopLevel::Import { name: String::from(path), is_lib: true }
                )
            },
            Rule::fileImport => {
                let path = pair.into_inner().next().unwrap().as_str();
                Result::Ok(
                    TopLevel::Import { name: String::from(path), is_lib: false }
                )
            },
            Rule::functionHeader => {
                Result::Ok(
                    TopLevel::FunctionHeader(
                        Self::parse_function_header(pair)?
                    )
                )
            },
            Rule::module => {
                let mut pairs = pair.into_inner();
                let name = String::from(pairs.next().unwrap().as_str());
                let mut body = Vec::<TopLevel>::new();
                for p in pairs {
                    body.push(Self::parse_top_level(p.into_inner().next().unwrap())?);
                }
                Result::Ok(
                    TopLevel::Module { name: name, body: body }
                )
            },
            Rule::r#union => {
                let un = Self::parse_union(pair)?;
                Result::Ok(
                    TopLevel::Struct(un),
                )
            },
            Rule::typedef => {
                let mut pairs = pair.into_inner();
                let first = pairs.next().unwrap();
                let inner = 
                    match first.as_rule() {
                        Rule::r#struct => TypeDefInner::Struct(Self::parse_struct(first)?),
                        Rule::anonStruct => TypeDefInner::Struct(Self::parse_struct(first)?),
                        Rule::r#enum => TypeDefInner::Enum(Self::parse_enum(first)?),
                        Rule::anonEnum => TypeDefInner::Enum(Self::parse_enum(first)?),
                        Rule::r#union => TypeDefInner::Struct(Self::parse_union(first)?),
                        Rule::anonUnion => TypeDefInner::Struct(Self::parse_union(first)?),
                        Rule::typ => TypeDefInner::Type(Self::parse_type(first)?),
                        _ => return Result::Err(String::from("Could not parse typedef inner")),
                    };
                let name = pairs.next().unwrap().as_str();
                Result::Ok(
                    TopLevel::TypeDef(
                        TypeDef {
                            name: String::from(name),
                            typ: inner,
                        }
                    )
                )
            },
            Rule::preprocessorDirective => {
                if pair.as_str().len() < 2 {
                    Result::Err(String::from("Invalid preprocessor directive"))
                } else {
                    let content = String::from(&pair.as_str()[1..]);
                    Result::Ok(TopLevel::PreprocessorDirective(content))
                }
            },
            _ => {
                Result::Err(String::from("Could not parse top-level"))
            },
        }
    }

    pub fn parse_statement(pair: Pair<Rule>) -> Result<Statement, String> {
        match pair.as_rule() {
            Rule::expression => {
                let exp = Self::parse_expression(pair)?;
                Result::Ok(Statement::Expression(exp))
            },
            Rule::varDec => {
                let mut pairs = pair.into_inner();
                let mut modifiers = Vec::<String>::new();
                let mut next = pairs.next().unwrap();
                loop {
                    match next.as_rule() {
                        Rule::varModifier => modifiers.push(String::from(next.as_str())),
                        _ => break,
                    }
                    next = pairs.next().unwrap();
                }

                let typ = Self::parse_type(next)?;
                let name = pairs.next().unwrap().as_str();
                let mut init_val: Option<Expression> = None;
                if let Some(expr_pair) = pairs.next() {
                    let result = Self::parse_expression(expr_pair)?;
                    init_val = Some(result);
                }
                Result::Ok(
                    Statement::VarDec { 
                        typ: typ,
                        name: String::from(name),
                        right: init_val,
                        modifier: modifiers,
                    }
                )
            },
            Rule::r#return => {
                let mut pairs = pair.into_inner();
                let expr = Self::parse_expression(pairs.next().unwrap())?;
                Result::Ok(
                    Statement::Return(Some(expr)),
                )
            },
            Rule::emptyReturn => Result::Ok(Statement::Return(None)),
            Rule::varAssign => {
                let mut pairs = pair.into_inner();
                let ident = Self::parse_identifier_expr(pairs.next().unwrap())?;
                let expr = Self::parse_expression(pairs.next().unwrap())?;
                Result::Ok(
                    Statement::VarAssign { identifier: ident, right: expr }
                )
            },
            Rule::binOpVarAssign => {
                let mut pairs = pair.into_inner();
                let ident = Self::parse_identifier_expr(pairs.next().unwrap())?;
                let binop = Self::parse_binop(String::from(pairs.next().unwrap().as_str()))?;
                let expr = Self::parse_expression(pairs.next().unwrap())?;
                Result::Ok(
                    Statement::BinOpVarAssign { identifier: ident, op: binop, right: expr }
                )
            },
            Rule::incDec => {
                let mut pairs = pair.into_inner();
                let ident = Self::parse_identifier_expr(pairs.next().unwrap())?;
                let op = pairs.next().unwrap().as_str();
                let is_inc: bool;
                if op == "++" {
                    is_inc = true;
                } else if op == "--" {
                    is_inc = false;
                } else {
                    return Result::Err(String::from("Invalid increment/decrement operator (use ++ or --)"));
                }
                Result::Ok(
                    Statement::IncDec { identifier: ident, is_inc: is_inc }
                )
            },
            Rule::r#if => {
                Self::parse_if_statement(pair)
            },
            Rule::r#while => {
                Result::Ok(Statement::While(Self::parse_cond_body(pair.into_inner().next().unwrap())?))
            },
            Rule::r#for => {
                let mut pairs = pair.into_inner();
                let init_stmt = Self::parse_statement(pairs.next().unwrap().into_inner().next().unwrap())?;
                let cond = Self::parse_expression(pairs.next().unwrap())?;
                let step_stmt = Self::parse_statement(pairs.next().unwrap())?;
                let mut body = Vec::<Statement>::new();
                for p in pairs {
                    body.push(Self::parse_statement(p.into_inner().next().unwrap())?);
                }
                Result::Ok(Statement::For { 
                    init: Box::new(init_stmt),
                    condition: cond,
                    increment: Box::new(step_stmt), 
                    body: body
                })
            },
            Rule::doWhile => {
                let mut peekable = pair.into_inner().peekable();
                let mut body = Vec::<Statement>::new();
                let mut last = None;
                let expr: Expression;
                while let Some(item) = peekable.next() {
                    if peekable.peek().is_none() {
                        // on last item, so handle outside of loop
                        last = Some(item);
                        break;
                    }
                    body.push(Self::parse_statement(item.into_inner().next().unwrap())?);
                }
                if let Some(last_item) = last {
                    expr = Self::parse_expression(last_item)?;
                    Result::Ok(
                        Statement::DoWhile { condition: expr, body: body }
                    )
                } else {
                    Result::Err(String::from("Do-while: Missing condition!"))
                }
            },
            Rule::r#continue => {
                Result::Ok(Statement::Continue)
            },
            Rule::r#break => {
                Result::Ok(Statement::Break)
            },
            Rule::switch => {
                let mut pairs = pair.into_inner();
                let switch_atom = Self::parse_atom(pairs.next().unwrap().into_inner().next().unwrap())?;
                let mut cases = Vec::<CaseStatement>::new();
                let mut default: Option<Vec<Statement>> = None;
                for p in pairs {
                    match p.as_rule() {
                        Rule::case => {
                            let mut body = p.into_inner();
                            let case_atom = Self::parse_atom(body.next().unwrap().into_inner().next().unwrap())?;
                            let mut body_lines = Vec::<Statement>::new();
                            for line in body {
                                let statement = Self::parse_statement(line.into_inner().next().unwrap())?;
                                body_lines.push(statement);
                            }
                            cases.push(CaseStatement {
                                atom: case_atom,
                                body: body_lines,
                            })
                        },
                        Rule::default => {
                            let mut body_lines = Vec::<Statement>::new();
                            let body = p.into_inner();
                            for line in body {
                                let statement = Self::parse_statement(line.into_inner().next().unwrap())?;
                                body_lines.push(statement);
                            }
                            default = Some(body_lines);
                        },
                        _ => return Result::Err(String::from("Invalid block in switch body")),
                    }
                }
                Result::Ok(Statement::Switch { atom: switch_atom, cases: cases, default: default })
            },
            _ => {
                println!("\n\n{}\n\n", pair);
                Result::Err(String::from("Could not parse statement"))
            },
        }
    }

    fn parse_expression(pair: Pair<Rule>) -> Result<Expression, String> {
        match pair.as_rule() {
            Rule::expression => {
                let mut pairs = pair.into_inner();
                let atom_pair = pairs.next().unwrap();
                let atom = Self::parse_atom(atom_pair.into_inner().next().unwrap())?;
                let mut expr_tail = ExprTail::None;
                let next = pairs.next();
                if next.is_some() {
                    let expr_tail_pair = next.unwrap();
                    expr_tail = Self::parse_expr_tail(expr_tail_pair)?;
                }
                Result::Ok(Expression {
                    atom: atom,
                    tail: expr_tail,
                })
            },
            _ => Result::Err(String::from("Could not parse expression")),
        }
    }

    fn parse_atom(pair: Pair<Rule>) -> Result<Atom, String> {
        match pair.as_rule() {
            Rule::int => {
                let value: i32 = pair.as_str().parse().unwrap();
                Result::Ok(Atom::Int(value))
            },
            Rule::double => {
                let value: f64 = pair.as_str().parse().unwrap();
                Result::Ok(Atom::Double(value))
            },
            Rule::long => {
                let value: i64 = pair.as_str()[..pair.as_str().len() - 1].parse().unwrap();
                Result::Ok(Atom::TrueLong(value))
            },
            Rule::float => {
                let value: f32 = pair.as_str()[..pair.as_str().len() - 1].parse().unwrap();
                Result::Ok(Atom::Float(value))
            },
            Rule::boolean => {
                let value: bool = pair.as_str().parse().unwrap();
                Result::Ok(Atom::Boolean(value))
            },
            Rule::string => Result::Ok(Atom::String(String::from(pair.into_inner().next().unwrap().as_str()))),
            Rule::hybridIdent => Result::Ok(Atom::Identifier(Self::parse_identifier(pair)?)),
            Rule::typeCast => {
                let mut pairs = pair.into_inner();
                let typ = Self::parse_type(pairs.next().unwrap())?;
                let expr = Self::parse_expression(pairs.next().unwrap())?;
                Result::Ok(
                    Atom::TypeCast { typ: typ, value: Box::new(expr) }
                )
            },
            Rule::unaryOperation => {
                let mut pairs = pair.into_inner();
                let op = Self::parse_unary_op(String::from(pairs.next().unwrap().as_str()))?;
                let expr = Self::parse_expression(pairs.next().unwrap())?;
                Result::Ok(
                    Atom::UnaryOperation { op: op, value: Box::new(expr) }
                )
            },
            Rule::sizeof => {
                let typ = Self::parse_type(pair.into_inner().next().unwrap())?;
                Result::Ok(
                    Atom::SizeOf(typ)
                )
            },
            Rule::expression => {
                let exp = Self::parse_expression(pair)?;
                Result::Ok(Atom::Wrapped(Box::new(exp)))
            },
            Rule::r#char => {
                let c = pair.into_inner().next().unwrap().as_str().as_bytes()[0];
                Result::Ok(Atom::Char(c))
            },
            _ => {
                println!("\n\n{}\n\n", pair);
                Result::Err(String::from("Could not parse atom"))
            },
        }
    }

    fn parse_expr_tail(pair: Pair<Rule>) -> Result<ExprTail, String> {
        match pair.as_rule() {
            Rule::exprTail => {
                if let Some(tail_pair) = pair.into_inner().next() {
                    match tail_pair.as_rule() {
                        Rule::callTail => {
                            let pairs = tail_pair.into_inner();
                            // Silent rule for the args, so last item will be the next tail
                            // Convert to Vec for easier handling
                            let mut items: Vec<Pair<'_, Rule>> = pairs.collect();
                            let next_tail = Self::parse_expr_tail(items.pop().unwrap())?;
                            let mut args: Vec<Expression> = Vec::new();
                            for arg in items {
                                let parsed_arg = Self::parse_expression(arg)?;
                                args.push(parsed_arg);
                            }
                            Result::Ok(
                                ExprTail::Call {
                                    body: args,
                                    next: Box::new(next_tail),
                                }
                            )
                        },
                        Rule::memberAccessTail => {
                            let mut pairs = tail_pair.into_inner();
                            let member = String::from(pairs.next().unwrap().as_str());
                            let next_tail = Self::parse_expr_tail(pairs.next().unwrap())?;

                            Result::Ok(
                                ExprTail::MemberAccess {
                                    member: member,
                                    next: Box::new(next_tail),
                                }
                            )
                        },
                        Rule::pointerAccessTail => {
                            let mut pairs = tail_pair.into_inner();
                            let member = String::from(pairs.next().unwrap().as_str());
                            let next_tail = Self::parse_expr_tail(pairs.next().unwrap())?;

                            Result::Ok(
                                ExprTail::PointerAccess {
                                    member: member,
                                    next: Box::new(next_tail),
                                }
                            )
                        },
                        Rule::binaryOperationTail => {
                            let mut pairs = tail_pair.into_inner();
                            let op = Self::parse_binop(String::from(pairs.next().unwrap().as_str()))?;
                            let expr = Self::parse_expression(pairs.next().unwrap())?;
                            let next_tail = Self::parse_expr_tail(pairs.next().unwrap())?;
                            Result::Ok(
                                ExprTail::BinaryOp { 
                                    op: op,
                                    right: Box::new(expr),
                                    next: Box::new(next_tail),
                                }
                            )
                        },
                        Rule::indexTail => {
                            let mut pairs = tail_pair.into_inner();
                            let expr = Self::parse_expression(pairs.next().unwrap())?;
                            let next_tail = Self::parse_expr_tail(pairs.next().unwrap())?;
                            Result::Ok(
                                ExprTail::Index { 
                                    inner: Box::new(expr),
                                    next: Box::new(next_tail),
                                }
                            )
                        },
                        Rule::ternaryTail => {
                            let mut pairs = tail_pair.into_inner();
                            let second = Self::parse_expression(pairs.next().unwrap())?;
                            let third = Self::parse_expression(pairs.next().unwrap())?;
                            let next_tail = Self::parse_expr_tail(pairs.next().unwrap())?;
                            Result::Ok(
                                ExprTail::TernaryConditional { 
                                    second: Box::new(second), 
                                    third: Box::new(third), 
                                    next: Box::new(next_tail), 
                                }
                            )
                        },
                        _ => Result::Err(String::from("Could not parse expression tail")),
                    }
                } else {
                    Result::Ok(ExprTail::None)
                }
            },
            _ => Result::Err(String::from("Could not parse expression tail")),
        }
    }

    fn parse_type(pair: Pair<Rule>) -> Result<Type, String> {
        match pair.as_rule() {
            Rule::typ => {
                let mut typetype: TypeType = TypeType::Simple;
                if pair.as_str().starts_with("struct") {
                    typetype = TypeType::Struct;
                } else if pair.as_str().starts_with("enum") {
                    typetype = TypeType::Enum;
                } else if pair.as_str().starts_with("union") {
                    typetype = TypeType::Union;
                }
                let pointer_layers = pair.as_str().chars().filter(|c| *c == '*').count();
                let name = Self::parse_identifier(pair.into_inner().next().unwrap())?;
                Result::Ok(
                    Type {
                        typetype: typetype,
                        pointer_layers: pointer_layers,
                        name: name,
                    }
                )
            },
            _ => {
                Result::Err(String::from("Could not parse type"))
            },
        }
    }

    fn parse_identifier(mut pair: Pair<Rule>) -> Result<Identifier, String> {
        pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::identifier => Result::Ok(Identifier::Plain(String::from(pair.as_str()))),
            Rule::moduleIdent => {
                let mut pairs = pair.into_inner();
                let parent = String::from(pairs.next().unwrap().as_str());
                let child = String::from(pairs.next().unwrap().as_str());
                Result::Ok(Identifier::Module(parent, child))
            },
            _ => {
                println!("\n\n{}\n\n", pair);
                Result::Err(String::from("Could not parse identifier"))
            },
        }
    }

    fn parse_function_header(pair: Pair<Rule>) -> Result<FunctionHeader, String> {
        match pair.as_rule() {
            Rule::functionHeader => {
                let is_extern = pair.as_str().starts_with("extern");
                let mut pairs = pair.into_inner();
                let typ = Self::parse_type(pairs.next().unwrap())?;
                let name = String::from(pairs.next().unwrap().as_str());
                let mut params = Vec::<Parameter>::new();
                if let Some(param_list_pairs) = pairs.next() {
                    for p in param_list_pairs.into_inner() {
                        params.push(Self::parse_parameter(p)?);
                    }
                }

                Result::Ok(
                    FunctionHeader {
                        return_type: typ,
                        name: name,
                        params: params,
                        is_extern: is_extern,
                    }
                )
            },
            _ => Result::Err(String::from("Cannot parse function header")),
        }
    }
    fn parse_function(pair: Pair<Rule>) -> Result<Function, String> {
        match pair.as_rule() {
            Rule::function => {
                let mut pairs = pair.into_inner();
                let header = Self::parse_function_header(pairs.next().unwrap())?;

                let mut statements = Vec::<Statement>::new();
                for stmt in pairs {
                    // Should all be statements
                    statements.push(Self::parse_statement(stmt.into_inner().next().unwrap())?);
                }
                
                Result::Ok(
                    Function {
                        header: header,
                        body: statements,
                    }
                )
            },
            _ => Result::Err(String::from("Could not parse function")),
        }
    }

    fn parse_parameter(pair: Pair<Rule>) -> Result<Parameter, String> {
        match pair.as_rule() {
            Rule::parameter => {
                let mut pairs = pair.into_inner();
                let typ = Self::parse_type(pairs.next().unwrap())?;
                let name = pairs.next().unwrap().as_str();
                Result::Ok(
                    Parameter {
                        typ: typ,
                        name: String::from(name),
                    }
                )
            },
            _ => Result::Err(String::from("Could not parse parameter")),
        }
    }

    fn parse_struct(pair: Pair<Rule>) -> Result<Struct, String> {
        match pair.as_rule() {
            Rule::r#struct => {
                let mut pairs = pair.into_inner();
                let name = pairs.next().unwrap().as_str();
                let members = Self::parse_struct_inner(pairs)?;
                Result::Ok(
                    Struct {
                        name: String::from(name),
                        members: members,
                        is_anonymous: false,
                        is_union: false,
                    }
                )
            },
            Rule::anonStruct => {
                let pairs = pair.into_inner();
                let members = Self::parse_struct_inner(pairs)?;
                Result::Ok(
                    Struct {
                        name: String::new(),
                        members: members,
                        is_anonymous: true,
                        is_union: false,
                    }
                )
            },
            _ => Result::Err(String::from("Could not parse struct")),
        }
    }

    fn parse_union(pair: Pair<Rule>) -> Result<Struct, String> {
        match pair.as_rule() {
            Rule::r#union => {
                let mut pairs = pair.into_inner();
                let name = pairs.next().unwrap().as_str();
                let members = Self::parse_struct_inner(pairs)?;
                Result::Ok(
                    Struct {
                        name: String::from(name),
                        members: members,
                        is_anonymous: false,
                        is_union: true,
                    }
                )
            },
            Rule::anonUnion => {
                let pairs = pair.into_inner();
                let members = Self::parse_struct_inner(pairs)?;
                Result::Ok(
                    Struct {
                        name: String::new(),
                        members: members,
                        is_anonymous: true,
                        is_union: true,
                    }
                )
            },
            _ => Result::Err(String::from("Could not parse union")),
        }
    }
    fn parse_struct_inner(pairs: Pairs<'_, Rule>) -> Result<Vec<StructMember>, String> {
        let mut members = Vec::<StructMember>::new();
        for p in pairs {
            match p.as_rule() {
                Rule::structVarDec => {
                    let mut ppairs = p.into_inner();
                    let ftyp = Self::parse_type(ppairs.next().unwrap())?;
                    let fname = ppairs.next().unwrap().as_str();
                    members.push(
                        StructMember::Field(
                            StructField {
                                name: String::from(fname),
                                typ: ftyp,
                            }
                        )
                    );
                },
                Rule::anonStruct => {
                    let struc = Self::parse_struct(p)?;
                    members.push(StructMember::AnonStruct(struc))
                },
                Rule::anonUnion => {
                    let union = Self::parse_union(p)?;
                    members.push(StructMember::AnonStruct(union))
                },
                _ => return Result::Err(String::from("Could not parse inner value of struct/union")),
            }
        }
        Result::Ok(members)
    }

    fn parse_enum(pair: Pair<Rule>) -> Result<Enum, String> {
        match pair.as_rule() {
            Rule::r#enum => {
                let mut pairs = pair.into_inner();
                let name = pairs.next().unwrap().as_str();
                let entries = Self::parse_enum_inner(pairs)?;
                Result::Ok(
                    Enum {
                        name: String::from(name),
                        entries: entries,
                        is_anonymous: false,
                    }
                )
            },
            Rule::anonEnum => {
                let pairs = pair.into_inner();
                let entries = Self::parse_enum_inner(pairs)?;
                Result::Ok(
                    Enum {
                        name: String::new(),
                        entries: entries,
                        is_anonymous: true,
                    }
                )
            },
            _ => Result::Err(String::from("Could not parse enum")),
        }
    }
    fn parse_enum_inner(pairs: Pairs<'_, Rule>) -> Result<Vec<EnumEntry>, String> {
        let mut entries = Vec::<EnumEntry>::new();
        for p in pairs {
            match p.as_rule() {
                Rule::enumEntry => {
                    let mut ppairs = p.into_inner();
                    let vname = ppairs.next().unwrap().as_str();
                    let value: Option<i32>;
                    if let Some(next) = ppairs.next() {
                        value = Some(next.as_str().parse::<i32>().unwrap());
                    } else {
                        value = None;
                    }
                    
                    entries.push(EnumEntry {
                        name: String::from(vname),
                        value: value,
                    });
                },
                _ => return Result::Err(String::from("Could not parse inner value of enum")),
            }
        }
        Result::Ok(entries)
    }

    fn parse_identifier_expr(pair: Pair<Rule>) -> Result<IdentifierExpression, String> {
        match pair.as_rule() {
            Rule::identExpr => {
                if let Some(in_pair) = pair.into_inner().next() {
                    match in_pair.as_rule() {
                        Rule::identifier => 
                            Result::Ok(
                                IdentifierExpression::Standard(String::from(in_pair.as_str()))
                            ),
                        Rule::pointerIdent => {
                            Result::Ok(
                                IdentifierExpression::Pointer(
                                    Self::parse_expression(in_pair.into_inner().next().unwrap())?
                                )
                            )
                        }
                        _ => Result::Err(String::from("Could not parse identifier expression"))
                    }
                } else {
                    Result::Err(String::from("Could not parse identifier expression"))
                }
            },
            _ => Result::Err(String::from("Could not parse identifier expression")),
        }
    }

    fn parse_cond_body(pair: Pair<Rule>) -> Result<ConditionBody, String> {
        let mut pairs = pair.into_inner();
        let cond = Self::parse_expression(pairs.next().unwrap())?;
        let mut body = Vec::<Statement>::new();
        for p in pairs {
            body.push(Self::parse_statement(p.into_inner().next().unwrap())?);
        }
        Result::Ok(
            ConditionBody {
                condition: cond,
                body: body,
            }
        )
    }

    fn parse_if_statement(pair: Pair<Rule>) -> Result<Statement, String> {
        let mut pairs = pair.into_inner();
        let base = Self::parse_cond_body(pairs.next().unwrap())?;
        let mut elseifs = Vec::<ConditionBody>::new();
        let elseifs_pairs = pairs.next().unwrap().into_inner();
        for p in elseifs_pairs {
            elseifs.push(Self::parse_cond_body(p.into_inner().next().unwrap())?);
        }
        let mut els = None;
        if let Some(els_pair) = pairs.next() {
            let mut body = Vec::<Statement>::new();
            for p in els_pair.into_inner() {
                body.push(Self::parse_statement(p.into_inner().next().unwrap())?);
            }
            els = Some(body);
        }

        Result::Ok(Statement::If { base: base, elseifs: elseifs, tail: els })
    }

    fn parse_binop(op: String) -> Result<BinOp, String> {
        if op == "+" {
            Ok(BinOp::Add)
        } else if op == "-" {
            Ok(BinOp::Sub)
        } else if op == "*" {
            Ok(BinOp::Mul)
        } else if op == "/" {
            Ok(BinOp::Div)
        } else if op == "%" {
            Ok(BinOp::Mod)
        } else if op == "&" {
            Ok(BinOp::BitAnd)
        } else if op == "|" {
            Ok(BinOp::BitOr)
        } else if op == "^" {
            Ok(BinOp::BitXor)
        } else if op == "&&" {
            Ok(BinOp::LogicAnd)
        } else if op == "||" {
            Ok(BinOp::LogicOr)
        } else if op == "<<" {
            Ok(BinOp::LeftShift)
        } else if op == ">>" {
            Ok(BinOp::RightShift)
        } else if op == "==" {
            Ok(BinOp::IsEqual)
        } else if op == "!=" {
            Ok(BinOp::IsNotEqual)
        } else if op == ">" {
            Ok(BinOp::IsGT)
        } else if op == ">=" {
            Ok(BinOp::IsGTE)
        } else if op == "<" {
            Ok(BinOp::IsLT)
        } else if op == "<=" {
            Ok(BinOp::IsLTE)
        } else {
            let mut msg = String::from("Invalid binary operator: '");
            msg.push_str(op.as_str());
            msg.push_str("'");
            Err(msg)
        }
    }

    fn parse_unary_op(op: String) -> Result<UnaryOp, String> {
        if op == "+" {
            Ok(UnaryOp::Plus)
        } else if op == "-" {
            Ok(UnaryOp::Minus)
        } else if op == "*" {
            Ok(UnaryOp::Dereference)
        } else if op == "&" {
            Ok(UnaryOp::AddressOf)
        } else if op == "!" {
            Ok(UnaryOp::LogicNot)
        } else if op == "~" {
            Ok(UnaryOp::BitNot)
        } else if op == "++" {
            Ok(UnaryOp::Increment)
        } else if op == "--" {
            Ok(UnaryOp::Decrement)
        } else {
            let mut msg = String::from("Invalid unary operator: '");
            msg.push_str(op.as_str());
            msg.push_str("'");
            Err(msg)
        }
    }
}
