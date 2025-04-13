use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use std::collections::HashMap;
use syn::{File, Item, TraitItem};

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
                                            let class = &mut classes[*class_map
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
                                let class = &mut classes
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

                check_tokens(
                    method.block.to_token_stream(),
                    class_map,
                    classes,
                    method.sig.ident.to_string(),
                );
            }
            _ => {}
        }
    }
}

fn check_tokens(
    tokens: TokenStream,
    class_map: &HashMap<&str, usize>,
    classes: &mut Vec<Class>,
    sig: String,
) {
    for token in tokens {
        match token {
            TokenTree::Group(group) => {
                check_tokens(group.stream(), class_map, classes, sig.clone());
            }
            _ => {
                check_token(&token, class_map, classes, sig.clone());
            }
        }
    }
}

fn check_token(
    token: &TokenTree,
    class_map: &HashMap<&str, usize>,
    classes: &mut Vec<Class>,
    ident: String,
) {
    if class_map.contains_key(token.to_string().as_str()) {
        create_connection(token, class_map, classes, ident);
    }
}

fn create_connection(
    token: &TokenTree,
    class_map: &HashMap<&str, usize>,
    classes: &mut Vec<Class>,
    ident: String,
) {
    let class = &mut classes[*class_map.get(token.to_string().as_str()).unwrap()];
    let connection = Connection::new(ident.clone(), class.name.clone(), Relationship::Dependency);
    class.add_connection(ident.clone(), connection);
}
