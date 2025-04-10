use proc_macro2::TokenStream;
use quote::ToTokens;
use std::clone;
use std::collections::HashMap;
use std::f32::NAN;
use std::path::Path;
use std::{any::Any, collections::HashSet};
use syn::{File, Item, Token, TraitItem, Type, UseName, Visibility, token};

use crate::class_diagram_model::{Class, Connection, Function, Relationship};

pub fn find_relationships(
    syntax: &File,
    class_map: &HashMap<&str, usize>,
    classes: &mut Vec<Class>,
) {
    for item in &syntax.items {
        let mut item_name = "".to_string();
        // let mut children = vec![];
        match item {
            Item::Struct(item) => {
                item_name = item.ident.to_string();
                if class_map.contains_key(item_name.as_str()) {
                    for field in &item.fields {
                        for token in field.to_token_stream() {
                            if class_map.contains_key(token.to_string().as_str()) {
                                let class = &mut classes
                                    [*class_map.get(item_name.to_string().as_str()).unwrap()];
                                let connection = Connection::new(
                                    item.ident.to_string(),
                                    token.to_string(),
                                    Relationship::Association,
                                );
                                class.add_connection(token.to_string(), connection);
                            }
                        }
                    }
                }
            }
            Item::Enum(item) => {}
            Item::Trait(trait_) => {
                for item in &trait_.items {
                    match item {
                        TraitItem::Fn(method) => {
                            for param in method.sig.inputs.iter() {
                                if let syn::FnArg::Typed(p) = param {
                                    for token in p.to_token_stream() {
                                        if class_map.contains_key(token.to_string().as_str()) {
                                            let mut class = &mut classes[*class_map
                                                .get(token.to_string().as_str())
                                                .unwrap()];
                                            let connection = Connection::new(
                                                trait_.ident.to_string(),
                                                class.name.clone(),
                                                Relationship::Dependency,
                                            );
                                            class.add_connection(class.name.clone(), connection);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Item::Impl(item) => match &item.trait_ {
                Some(trait__) => {
                    let trait_name = trait__.1.get_ident().unwrap();
                    if class_map.contains_key(trait_name.to_string().as_str()) {
                        let class =
                            &mut classes[*class_map.get(trait_name.to_string().as_str()).unwrap()];
                        let impler_name = item.self_ty.to_token_stream().to_string();
                        let connection = Connection::new(
                            impler_name.clone(),
                            trait_name.to_string(),
                            Relationship::Inheritance,
                        );
                        class.connections.insert(impler_name, connection);
                    }
                }
                None => {
                    let class_name = item.self_ty.to_token_stream().to_string();
                    if class_map.contains_key(class_name.as_str()) {
                        let class = &mut classes[*class_map.get(class_name.as_str()).unwrap()];
                        for item in &item.items {
                            match item {
                                syn::ImplItem::Fn(method) => {
                                    let visibility =
                                        crate::class_diagram_model::Visibility::from_vis(
                                            &method.vis,
                                        );
                                    let name = method.sig.ident.to_string();
                                    let return_type =
                                        if let syn::ReturnType::Type(_, ty) = &method.sig.output {
                                            format!("{}", quote::quote!(#ty))
                                        } else {
                                            String::new()
                                        };
                                    let params: Vec<String> = method
                                        .sig
                                        .inputs
                                        .iter()
                                        .filter_map(|param| {
                                            if let syn::FnArg::Typed(pat_type) = param {
                                                Some(pat_type.to_token_stream().to_string())
                                            } else {
                                                None
                                            }
                                        })
                                        .collect();
                                    class.functions.push(Function::new(
                                        visibility,
                                        name,
                                        params.join(", "),
                                        return_type,
                                    ));

                                    for param in method.sig.inputs.iter() {
                                        if let syn::FnArg::Typed(p) = param {
                                            for token in p.to_token_stream() {
                                                if class_map
                                                    .contains_key(token.to_string().as_str())
                                                {
                                                    let connection = Connection::new(
                                                        class_name.clone(),
                                                        token.to_string(),
                                                        Relationship::Dependency,
                                                    );
                                                    class.add_connection(
                                                        class.name.clone(),
                                                        connection,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            },
            Item::Fn(method) => {
                for param in method.sig.inputs.iter() {
                    if let syn::FnArg::Typed(p) = param {
                        for token in p.to_token_stream() {
                            if class_map.contains_key(token.to_string().as_str()) {
                                let mut class = &mut classes
                                    [*class_map.get(token.to_string().as_str()).unwrap()];
                                let connection = Connection::new(
                                    method.sig.ident.to_string(),
                                    class.name.clone(),
                                    Relationship::Dependency,
                                );
                                class.add_connection(class.name.clone(), connection);
                            }
                        }
                    }
                }

                println!("FUNCTION: {}\n\n\n", method.sig.ident.to_string());
                eval_statements(
                    &method.block.stmts,
                    class_map,
                    classes,
                    method.sig.ident.to_string(),
                );
            }
            _ => {}
        }
    }
}

fn eval_statements(
    statements: &Vec<syn::Stmt>,
    class_map: &HashMap<&str, usize>,
    classes: &mut Vec<Class>,
    ident: String,
) {
    for statement in statements {
        match statement {
            syn::Stmt::Local(local) => {
                println!("Stmt:Local");
                if let Some(init) = &local.init {
                    eval_expr(&init.expr, class_map, classes, ident.clone());
                }
                check_connection(local.to_token_stream(), class_map, classes, ident.clone());
            }
            syn::Stmt::Item(item) => {
                println!("Stmt:Item");
                check_connection(item.to_token_stream(), class_map, classes, ident.clone());
            }
            syn::Stmt::Expr(expr, semi) => {
                eval_expr(expr, class_map, classes, ident.clone());
            }
            syn::Stmt::Macro(stmt_macro) => {
                println!("Stmt:Macro");
                check_connection(
                    stmt_macro.to_token_stream(),
                    class_map,
                    classes,
                    ident.clone(),
                );
            }
        }
    }
}

fn eval_arms(
    arms: &Vec<syn::Arm>,
    class_map: &HashMap<&str, usize>,
    classes: &mut Vec<Class>,
    ident: String,
) {
    for arm in arms {
        eval_expr(&arm.body, class_map, classes, ident.clone());
    }
}

fn eval_exprs(
    exprs: Vec<&syn::Expr>,
    class_map: &HashMap<&str, usize>,
    classes: &mut Vec<Class>,
    ident: String,
) {
    for expr in exprs {
        eval_expr(expr, class_map, classes, ident.clone());
    }
}

fn eval_expr(
    expr: &syn::Expr,
    class_map: &HashMap<&str, usize>,
    classes: &mut Vec<Class>,
    ident: String,
) {
    match expr {
        syn::Expr::Array(expr) => {
            println!("Expr:Array");
            let exprs = expr.elems.iter().take(expr.elems.len()).collect();
            eval_exprs(exprs, class_map, classes, ident.clone());
        }
        syn::Expr::Assign(expr) => {
            println!("Expr:Assign");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Async(expr) => {
            println!("Expr:Async");
            eval_statements(&expr.block.stmts, class_map, classes, ident.clone());
        }
        syn::Expr::Await(expr) => {
            println!("Expr:Await");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Binary(expr) => {
            println!("Expr:Binary");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Block(expr) => {
            println!("Expr:Block");
            eval_statements(&expr.block.stmts, class_map, classes, ident.clone());
        }
        syn::Expr::Break(expr) => {
            println!("Expr:Break");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Call(expr) => {
            println!("Expr:Call");
            eval_expr(&expr.func, class_map, classes, ident.clone());
            let exprs = expr.args.iter().take(expr.args.len()).collect();
            eval_exprs(exprs, class_map, classes, ident.clone());
        }
        syn::Expr::Cast(expr) => {
            println!("Expr:Cast");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Closure(expr) => {
            println!("Expr:Closure");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Const(expr) => {
            println!("Expr:Const");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Continue(expr) => {
            println!("Expr:Continue");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Field(expr) => {
            println!("Expr:Field");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::ForLoop(expr) => {
            println!("Expr:ForLoop");
            eval_statements(&expr.body.stmts, class_map, classes, ident.clone());
        }
        syn::Expr::Group(expr) => {
            println!("Expr:Group");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::If(expr) => {
            println!("Expr:If");
            eval_expr(&expr.cond, class_map, classes, ident.clone());
            eval_statements(&expr.then_branch.stmts, class_map, classes, ident.clone());
            if let Some(else_) = &expr.else_branch {
                eval_expr(&else_.1, class_map, classes, ident.clone());
            }
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Index(expr) => {
            println!("Expr:Index");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Infer(expr) => {
            println!("Expr:Infer");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Let(expr) => {
            println!("Expr:Let");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Lit(expr) => {
            println!("Expr:Lit");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Loop(expr) => {
            println!("Expr:Loop");
            eval_statements(&expr.body.stmts, class_map, classes, ident.clone());
        }
        syn::Expr::Macro(expr) => {
            println!("Expr:Macro");
            check_connection(
                expr.mac.to_token_stream().clone(),
                class_map,
                classes,
                ident.clone(),
            );
        }
        syn::Expr::Match(expr) => {
            println!("Expr:Match");
            eval_expr(&expr.expr, class_map, classes, ident.clone());
            eval_arms(&expr.arms, class_map, classes, ident);
        }
        syn::Expr::MethodCall(expr) => {
            println!("Expr:MethodCall");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Paren(expr) => {
            println!("Expr:Paren");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Path(expr) => {
            println!("Expr:Path");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Range(expr) => {
            println!("Expr:Range");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::RawAddr(expr) => {
            println!("Expr:RawAddr");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Reference(expr) => {
            println!("Expr:Reference");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Repeat(expr) => {
            println!("Expr:Repeat");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Return(expr) => {
            println!("Expr:Return");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Struct(expr) => {
            println!("Expr:Struct");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Try(expr) => {
            println!("Expr:Try");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::TryBlock(expr) => {
            println!("Expr:TryBlock");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Tuple(expr) => {
            println!("Expr:Tuple");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Unary(expr) => {
            println!("Expr:Unary");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Unsafe(expr) => {
            println!("Expr:Unsafe");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Verbatim(token_stream) => {
            println!("Expr:Verbatim");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::While(expr) => {
            println!("Expr:While");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        syn::Expr::Yield(expr) => {
            println!("Expr:Yield");
            check_connection(expr.to_token_stream(), class_map, classes, ident.clone());
        }
        _ => check_connection(expr.to_token_stream(), class_map, classes, ident.clone()),
    }
}

fn check_connection(
    stream: TokenStream,
    class_map: &HashMap<&str, usize>,
    classes: &mut Vec<Class>,
    ident: String,
) {
    for token in stream {
        println!("{}", token);
        if class_map.contains_key(token.to_string().as_str()) {
            println!("#");
            let mut class = &mut classes[*class_map.get(token.to_string().as_str()).unwrap()];
            let connection =
                Connection::new(ident.clone(), class.name.clone(), Relationship::Dependency);
            class.add_connection(ident.clone(), connection);
        }
    }
}
