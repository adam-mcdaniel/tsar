use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

use super::tokens::*;
use super::lower::*;

use core::fmt::Debug;

#[derive(Parser)]
// The pest grammar file for Tsar
#[grammar = "assembler/tsar.pest"]
pub struct Tsar;


/// This takes an iterator, and returns the next value from the iterator as a result
fn next<A: Debug + Iterator>(pairs: &mut A) -> Result<<A as core::iter::Iterator>::Item, String> {
    match pairs.next() {
        Some(val) => Ok(val),
        None => Err(format!("Could not get value from {:#?}", pairs)),
    }
}

/// This tokenizes an identifier.
/// An identifer can be:
/// `a`, `ab`, `_a`, `a1`, `_a_1`, etc
fn identifier(pair: Pair<Rule>) -> Result<String, String> {
    // Get the tokens actual string representation from the parser,
    // meaning, get exactly the text that the parser parsed.
    // Then, trim whitespace (there isnt any), and then convert to string
    Ok(pair.as_span().as_str().trim().to_string())
}

/// This tokenizes a scoped name.
/// A scoped name is a series of identifiers separated by `::` tokens.
/// For example, `std::io::println` is a valid scoped name
fn scoped_name(pair: Pair<Rule>) -> Result<ScopedName, String> {
    let mut result = vec![];
    // Get the tokens inside the token
    for ident in pair.into_inner() {
        // Add the identifiers to the resulting scoped name
        result.push(identifier(ident)?);
    }
    // Return the scoped name
    Ok(ScopedName(result))
}

/// This tokenizes dot name suffixes.
/// For example, the `.b` portion of `a.b` is a dot name suffix
fn dot_name(pair: Pair<Rule>) -> Result<SecondValue, String> {
    // Get the tokens to tokenize
    let mut tokens = pair.clone().into_inner();
    // Get the first token of the tokens
    // In a dot value, there is only one token,
    // the identifier following the dot
    let val = next(&mut tokens)?;
    // Convert it to a dotname and return
    Ok(SecondValue::DotName(DotName(identifier(val)?)))
}

/// This tokenizes indexed name suffixes.
/// For example, the `[b]` portion of `a[b]` is a indexed name suffix
fn indexed_name(pair: Pair<Rule>) -> Result<SecondValue, String> {
    // Get the tokens to tokenize
    let mut tokens = pair.clone().into_inner();
    // Get the first token of the tokens
    // In an indexed value, there is only one token,
    // the value inside the square brackets
    let val = next(&mut tokens)?;
    // Get the inside of the brackets as a Value
    let inside = value(val)?.lower();
    // Convert to an indexed name and return
    Ok(SecondValue::IndexedName(IndexedName(inside)))
}

/// This tokenizes names being assigned to.
/// x
/// x.a.b.c
/// x["a"]["b"]["c"]["d"]
fn name(pair: Pair<Rule>) -> Result<Value, String> {
    // Get the tokens for this statement
    let mut tokens = pair.clone().into_inner();
    // Get the first token to match
    let head = next(&mut tokens)?;
    match head.as_rule() {
        // If the name is a scoped name, return a scoped name
        Rule::scoped_name => Ok(Value(FirstValue::ScopedName(scoped_name(head)?), vec![])),
        // If the name begins with an identifier, we could have either
        // a dot name, an indexed name, or just an identifier.
        Rule::identifier => {
            match next(&mut tokens) {
                Ok(val) => {
                    // If the name is more than one token long and not a scoped name
                    match val.as_rule() {
                        // If the name is a dot name, return a dot name
                        Rule::dot_name => {
                            let mut dot_names = vec![dot_name(val)?];
                            for dot in tokens {
                                dot_names.push(dot_name(dot)?);
                            }
                            Ok(Value(FirstValue::Identifier(Identifier(identifier(head)?)), dot_names))
                        },
                        // If the name is an indexed name, return an indexed name
                        Rule::indexed_name => {
                            let mut index_names = vec![indexed_name(val)?];
                            for index in tokens {
                                index_names.push(indexed_name(index)?);
                            }
                            Ok(Value(FirstValue::Identifier(Identifier(identifier(head)?)), index_names))
                        },
                        // This block should never be reached, the syntax of the
                        // pest file prevents any other value from being consumed.
                        _ => unreachable!(),
                    }
                },
                Err(_) => {
                    // Then the name is just an identifier
                    Ok(Value(FirstValue::Identifier(Identifier(identifier(head)?)), vec![]))
                }
            }
        },
        _ => unreachable!(),
    }
}

/// This is for `use` statements.
/// use core::string;
/// use core::string::strlen;
/// use core::string::{strlen};
/// use core::string::{strlen, concat};
/// use core::list::{len, concat, zip, map, reduce};
fn import(pair: Pair<Rule>) -> Result<Import, String> {
    let mut tokens = pair.clone().into_inner();
    let head = next(&mut tokens)?;
    match head.as_rule() {
        // The import is a singular scoped name
        Rule::scoped_name => {
            // Get the vector of identifiers
            let name = scoped_name(head)?.0;
            let mut vec = vec![];
            // Exclude the last one
            for ident in &name[0..name.len() - 1] {
                vec.push(ident.clone());
            }
            // Return the scoped name without the last identifier,
            // and put that identifier into a vector by itself
            // `use core::string::strlen;` becomes
            // Import { module: "core::string", imports: ["strlen"] }
            Ok(Import {
                module: ScopedName(vec),
                imports: vec![name[name.len() - 1].clone()],
            })
        }
        // The import is a list of names from a module, like
        // `use core::string::{strlen, concat};`
        Rule::identifier => {
            // Create a vector of identifiers for the import scope
            let mut module = vec![identifier(head)?];
            let mut imports = vec![];
            // Collect all the identifiers in the scope, and leave behind the import body
            let collected = tokens.into_iter().collect::<Vec<Pair<Rule>>>();
            for ident in &collected[..collected.len() - 1] {
                module.push(identifier(ident.clone())?);
            }
            // Put everything thats in the import body (between brackets), into a list
            for import in collected[collected.len() - 1].clone().into_inner() {
                imports.push(identifier(import)?);
            }
            // Return the scope and the list of imports between the brackets
            // `use core::string::{strlen, concat};` becomes
            // Import { module: "core::string", imports: ["strlen", "concat"] }
            Ok(Import {
                module: ScopedName(module),
                imports: imports,
            })
        }
        _ => unreachable!(),
    }
}

/// This function tokenizes a module declaration
fn module(pair: Pair<Rule>) -> Result<Module, String> {
    let mut tokens = pair.clone().into_inner();
    let name = next(&mut tokens)?;

    match name.as_rule() {
        Rule::identifier => Ok(Module(ScopedName(vec![identifier(name)?]))),
        Rule::scoped_name => Ok(Module(scoped_name(name)?)),
        _ => unreachable!(),
    }
}

pub fn group(pair: Pair<Rule>) -> Result<Value, String> {
    value(next(&mut pair.into_inner())?)
}

pub fn tail(pair: Pair<Rule>) -> Result<SecondValue, String> {
    let mut tokens = pair.clone().into_inner();
    let val = next(&mut tokens)?;

    match val.as_rule() {
        Rule::tail => tail(val),
        Rule::call => {
            let mut args = vec![];
            for item in val.into_inner() {
                args.push(value(item)?);
            }
            Ok(SecondValue::Call(Call(args)))
        }
        Rule::dot_name => dot_name(val),
        Rule::indexed_name => indexed_name(val),
        _ => unreachable!()
    }
}

pub fn math(lhs: Value, pair: Pair<Rule>) -> Result<Value, String> {
    let mut tokens = pair.clone().into_inner();
    let operator = next(&mut next(&mut tokens)?.into_inner())?;
    let rhs = next(&mut tokens)?;
    Ok(Value(
    FirstValue::Math(match operator.as_rule() {
        Rule::modulus => Math::Remainder(Box::new(lhs), Box::new(value(rhs)?)),
        Rule::add => Math::Add(Box::new(lhs), Box::new(value(rhs)?)),
        Rule::subtract => Math::Subtract(Box::new(lhs), Box::new(value(rhs)?)),
        Rule::multiply => Math::Multiply(Box::new(lhs), Box::new(value(rhs)?)),
        Rule::divide => Math::Divide(Box::new(lhs), Box::new(value(rhs)?)),
        Rule::equal => Math::Is(Box::new(lhs), Box::new(value(rhs)?)),
        Rule::not_equal => Math::Isnt(Box::new(lhs), Box::new(value(rhs)?)),
        other => panic!("{:#?}", other)
    }), vec![]))
}

pub fn value(pair: Pair<Rule>) -> Result<Value, String> {
    let mut tokens = pair.clone().into_inner();
    let val = next(&mut tokens)?;
    match val.as_rule() {
        Rule::value => value(val),
        Rule::group => {
            let inner = FirstValue::Group(Box::new(group(val)?));

            let mut second_values = vec![];
            while let Ok(val) = next(&mut tokens) {
                match val.as_rule() {
                    Rule::tail => second_values.push(tail(val)?),
                    Rule::math => {
                        return Ok(math(Value(inner, second_values), val)?);
                    },
                    _ => unreachable!()
                }
            }

            Ok(Value(inner, second_values))
        },
        Rule::literal | Rule::scoped_name | Rule::identifier | Rule::head => {
            let mut second_values = vec![];
            while let Ok(suffix) = next(&mut tokens) {
                // second_values.push(tail(val)?);
                match suffix.as_rule() {
                    Rule::tail => second_values.push(tail(suffix)?),
                    Rule::math => {
                        return Ok(math(Value(head(val)?, second_values), suffix)?);
                    },
                    _ => unreachable!()
                }
            }
            Ok(Value(head(val)?, second_values))
        },
        _ => unreachable!(),
    }
}

pub fn function(pair: Pair<Rule>) -> Result<Function, String> {
    let mut tokens = pair.clone().into_inner();
    let args = next(&mut tokens)?;

    match args.as_rule() {
        Rule::identifier => {
            let result = next(&mut tokens)?;
            match result.as_rule() {
                Rule::value => Ok(Function(vec![Identifier(identifier(args)?)], vec![expr(result)?])),
                Rule::suite => Ok(Function(vec![Identifier(identifier(args)?)], suite(result)?)),
                _ => unreachable!()
            }
        },
        Rule::args => {
            let mut args_result = vec![];
            for arg in args.into_inner() {
                args_result.push(Identifier(identifier(arg)?));
            }

            let result = next(&mut tokens)?;
            match result.as_rule() {
                Rule::value => Ok(Function(args_result, vec![expr(result)?])),
                Rule::suite => Ok(Function(args_result, suite(result)?)),
                _ => unreachable!()
            }
        }
        _ => unreachable!()
    }
}

pub fn function_def(pair: Pair<Rule>) -> Result<FunctionDef, String> {
    let mut tokens = pair.clone().into_inner();
    let name = head(pair)?;
    next(&mut tokens)?;

    let args = {
        let _args = next(&mut tokens)?;
        let mut args_result = vec![];
        for arg in _args.into_inner() {
            args_result.push(Identifier(identifier(arg)?));
        }

        args_result
    };

    
    let result = next(&mut tokens)?;
    match result.as_rule() {
        Rule::value => Ok(FunctionDef(name, Function(args, vec![expr(result)?]))),
        Rule::suite => Ok(FunctionDef(name, Function(args, suite(result)?))),
        _ => unreachable!()
    }
}

pub fn suite(pair: Pair<Rule>) -> Result<Vec<Expr>, String> {
    let tokens = pair.clone().into_inner();
    let mut result = vec![];

    for item in tokens {
        match item.as_rule() {
            Rule::expr => result.push(expr(item)?),
            Rule::value => result.push(Expr::Value(value(item)?)),
            _ => unreachable!()
        };
    }
    Ok(result)
}


pub fn expr(pair: Pair<Rule>) -> Result<Expr, String> {
    let mut tokens = pair.clone().into_inner();
    let val = next(&mut tokens)?;

    match val.as_rule() {
        Rule::assignment => {
            let mut assign = val.into_inner();
            Ok(Expr::Assignment(Assignment(
                name(next(&mut assign)?)?,
                value(next(&mut assign)?)?
            )))
        },
        Rule::class_def => {
            let mut methods = vec![];
            let mut class = val.clone().into_inner();
            let name = head(val)?;
            next(&mut class)?;
            while let Ok(method) = next(&mut class) {
                methods.push(function_def(method)?);
            }

            Ok(Expr::ClassDef(ClassDef(
                name, methods
            )))
        },
        Rule::function_def => Ok(Expr::FunctionDef(function_def(val)?)),
        Rule::while_loop => Err(String::from("unimplemented")),
        Rule::if_then_else => Err(String::from("unimplemented")),
        Rule::value => Ok(Expr::Value(value(val)?)),
        Rule::head => Ok(Expr::Value(value(pair)?)),
        Rule::expr => expr(val),
        _ => unreachable!()
    }
}

pub fn literal(pair: Pair<Rule>) -> Result<Literal, String> {
    let mut tokens = pair.clone().into_inner();
    let val = next(&mut tokens)?;

    match val.as_rule() {
        Rule::literal => literal(val),
        Rule::foreign_function_literal => {
            Ok(Literal::ForeignFunction(Identifier(identifier(next(&mut val.into_inner())?)?)))
        },
        Rule::string_literal => {
            let s = val.as_span().as_str();
            Ok(Literal::String(s[1..s.len()-1].to_string()))
        },
        Rule::number_literal => {
            Ok(Literal::Number(Number(val.as_span().as_str().to_string())))
        },
        Rule::function => {
            Ok(Literal::Function(function(val)?))
        },
        _ => unreachable!(),
    }
}

pub fn head(pair: Pair<Rule>) -> Result<FirstValue, String> {
    let mut tokens = pair.clone().into_inner();
    let value = next(&mut tokens)?;
    match value.as_rule() {
        Rule::head => head(value),
        Rule::literal => Ok(FirstValue::Literal(literal(value)?)),
        Rule::scoped_name => Ok(FirstValue::ScopedName(scoped_name(value)?)),
        Rule::identifier => Ok(FirstValue::Identifier(Identifier(identifier(value)?))),
        Rule::group => Ok(FirstValue::Group(Box::new(group(value)?))),
        // other => {
        //     println!("{:#?}", other);
        //     Err(String::from(""))
        // }
        _ => unreachable!()
    }
}


pub fn parse(input: &str) -> Result<Program, String> {
    let mut pairs = match Tsar::parse(Rule::program, input) {
        Ok(parsed) => Ok(parsed),
        Err(e) => Err(format!("{}", e)),
    }?;

    let mut code = next(&mut pairs)?.into_inner().collect::<Vec<Pair<Rule>>>();
    // modules
    if let Ok(section) = next(&mut pairs) {
        code.extend(section.into_inner());
    }
    // code
    if let Ok(section) = next(&mut pairs) {
        code.extend(section.into_inner());
    }

    let mut result_imports = vec![];
    let mut result_modules = vec![];
    let mut result_code = vec![];

    for i in code {
        match i.as_rule() {
            Rule::import => result_imports.push(import(i)?),
            Rule::module => result_modules.push(module(i)?),
            Rule::expr => result_code.push(expr(i)?),
            Rule::EOI => {}
            _ => unreachable!(),
        };
    }

    Ok(Program(result_imports, result_modules, result_code))
}
