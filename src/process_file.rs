use quote::ToTokens;
use std::path::Path;
use std::{any::Any, collections::HashSet};
use syn::{File, Item, TraitItem, Type};

pub fn process_file(syntax: &File, diagram: &mut String, defined_types: &HashSet<&String>) {
    for item in &syntax.items {
        let mut item_name = "".to_string();
        let mut children = vec![];
        match item {
            Item::Struct(s) => {
                item_name = s.ident.to_string();
                // Start a Mermaid class definition:
                diagram.push_str(&format!("class {} {{\n", item_name));

                // Optionally, list fields (if the struct has named fields)
                if let syn::Fields::Named(fields) = &s.fields {
                    for field in fields.named.iter() {
                        if let Some(ident) = &field.ident {
                            for token in field.ty.to_token_stream() {
                                if defined_types.contains(&token.to_string()) {
                                    children.push(token.to_string());
                                }
                            }
                            let mut field = quote::quote!(#field.ty).to_string(); // Get field string
                            field.truncate(field.len() - 4); // Remove weird stuff from the end
                            diagram.push_str(&format!("  +{}\n", field));
                        }
                    }
                }
                diagram.push_str("}\n\n");
            }
            Item::Enum(e) => {
                item_name = e.ident.to_string();
                diagram.push_str(&format!("class {} {{\n", item_name));
                // Enumerators can be treated as values or attributes:
                for variant in &e.variants {
                    diagram.push_str(&format!("  +{}\n", variant.ident));
                }
                diagram.push_str("}\n\n");
            }
            Item::Trait(t) => {
                item_name = t.ident.to_string();
                diagram.push_str(&format!("class {} {{\n", item_name));
                // Add methods from the trait
                for item in &t.items {
                    if let syn::TraitItem::Fn(method) = item {
                        let method_name = method.sig.ident.to_string();
                        let return_type = if let syn::ReturnType::Type(_, ty) = &method.sig.output {
                            format!(" -> {}", quote::quote!(#ty))
                        } else {
                            String::new()
                        };

                        // Format parameters
                        let params: Vec<String> = method
                            .sig
                            .inputs
                            .iter()
                            .filter_map(|param| {
                                if let syn::FnArg::Typed(pat_type) = param {
                                    Some(quote::quote!(#pat_type).to_string())
                                } else {
                                    None
                                }
                            })
                            .collect();

                        diagram.push_str(&format!(
                            "  +{}({}){}\n",
                            method_name,
                            params.join(", "),
                            return_type
                        ));
                    }
                }
                diagram.push_str("}\n\n");
            }
            // You can extend to include impl blocks for relationships if desired.
            _ => {}
        }

        for child in &children {
            diagram.push_str(format!("{} <|-- {}\n", item_name, child).as_str());
        }
        if !children.is_empty() {
            diagram.push_str("\n");
        }
    }
}
