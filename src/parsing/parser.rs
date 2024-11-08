use pest::iterators::Pair;
use std::fs;
use pest::Parser;
use pest_derive::Parser;

use super::ast::{expression::{Atom, ExprTail, Expression}, statement::Statement, toplevel::TopLevel, types::Type};

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct MiniCParser;

pub struct MyMiniCParser {}

impl MyMiniCParser {
    pub fn parse_file(filepath: String) {

    }

    pub fn parse_top_level(pair: Pair<Rule>) -> Result<TopLevel, String> {
        match pair.as_rule() {
            Rule::varDec => {
                let mut pairs = pair.into_inner();
                let typ = Self::parse_type(pairs.next().unwrap());
                if typ.is_err() {
                    return Result::Err(typ.err().unwrap());
                }
                let name = pairs.next().unwrap().as_str();
                let mut init_val: Option<Expression> = None;
                if let Some(expr_pair) = pairs.next() {
                    let result = Self::parse_expression(expr_pair);
                    if result.is_err() {
                        return Result::Err(result.err().unwrap());
                    }
                    init_val = Some(result.unwrap());
                }
                Result::Ok(
                    TopLevel::VarDeclaration { 
                        typ: typ.unwrap(),
                        name: String::from(name),
                        right: init_val,
                    }
                )
            },
            _ => Result::Err(String::from("Could not parse top-level")),
        }
    }

    pub fn parse_statement(pair: Pair<Rule>) -> Result<Statement, String> {
        match pair.as_rule() {
            Rule::expression => {
                let exp = Self::parse_expression(pair);
                if exp.is_err() {
                    Result::Err(exp.err().unwrap())
                } else {
                    Result::Ok(Statement::Expression(exp.unwrap()))
                }
            },
            _ => Result::Err(String::from("Could not parse statement")),
        }
    }

    fn parse_expression(pair: Pair<Rule>) -> Result<Expression, String> {
        match pair.as_rule() {
            Rule::expression => {
                let mut pairs = pair.into_inner();
                let atom_pair = pairs.next().unwrap();
                let atom = Self::parse_atom(atom_pair.into_inner().next().unwrap());
                let mut expr_tail = Result::Ok(ExprTail::None);
                if pairs.next().is_some() {
                    let expr_tail_pair = pairs.next().unwrap();
                    expr_tail = Self::parse_expr_tail(expr_tail_pair);
                }
                if atom.is_err() {
                    Result::Err(atom.err().unwrap())
                } else if expr_tail.is_err() {
                    Result::Err(expr_tail.err().unwrap())
                } else {
                    Result::Ok(Expression {
                        atom: atom.unwrap(),
                        tail: expr_tail.unwrap(),
                    })
                }
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
            Rule::reference => Result::Ok(Atom::Reference(String::from(pair.into_inner().next().unwrap().as_str()))),
            Rule::identifier => Result::Ok(Atom::Identifier(String::from(pair.as_str()))),
            _ => Result::Err(String::from("Could not parse atom")),
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
                            let next_tail = Self::parse_expr_tail(items.pop().unwrap());
                            if next_tail.is_err() {
                                return Result::Err(next_tail.err().unwrap());
                            }
                            let mut args: Vec<Expression> = Vec::new();
                            for arg in items {
                                let parsed_arg = Self::parse_expression(arg);
                                if parsed_arg.is_err() {
                                    return Result::Err(parsed_arg.err().unwrap());
                                }
                                args.push(parsed_arg.ok().unwrap());
                            }
                            Result::Ok(
                                ExprTail::Call {
                                    body: args,
                                    next: Box::new(next_tail.unwrap()),
                                }
                            )
                        },
                        Rule::memberAccessTail => {
                            let mut pairs = tail_pair.into_inner();
                            let member = String::from(pairs.next().unwrap().as_str());
                            let next_tail = Self::parse_expr_tail(pairs.next().unwrap());
                            if next_tail.is_err() {
                                return Result::Err(next_tail.err().unwrap());
                            }

                            Result::Ok(
                                ExprTail::MemberAccess {
                                    member: member,
                                    next: Box::new(next_tail.unwrap()),
                                }
                            )
                        },
                        Rule::pointerAccessTail => {
                            let mut pairs = tail_pair.into_inner();
                            let member = String::from(pairs.next().unwrap().as_str());
                            let next_tail = Self::parse_expr_tail(pairs.next().unwrap());
                            if next_tail.is_err() {
                                return Result::Err(next_tail.err().unwrap());
                            }

                            Result::Ok(
                                ExprTail::PointerAccess {
                                    member: member,
                                    next: Box::new(next_tail.unwrap()),
                                }
                            )
                        },
                        // TODO: Index, binop
                        _ => Result::Err(String::from("Could not parse expression tail")),
                    }
                } else {
                    Result::Err(String::from("Coult not read inner of expression tail"))
                }
            },
            _ => Result::Err(String::from("Could not parse expression tail")),
        }
    }

    fn parse_type(pair: Pair<Rule>) -> Result<Type, String> {
        match pair.as_rule() {
            Rule::typ => {
                let is_struct = pair.as_str().starts_with("struct");
                let is_ptr = pair.as_str().ends_with("*");
                let name = pair.into_inner().next().unwrap().as_str();
                Result::Ok(
                    Type {
                        is_struct: is_struct,
                        is_pointer: is_ptr,
                        name: String::from(name),
                    }
                )
            },
            _ => Result::Err(String::from("Could not parse type")),
        }
    }
}
