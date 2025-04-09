use quote::ToTokens;
use syn::{File, Item};

use crate::class_diagram_model::{Class, Field, Function, Visibility};

pub fn init_classes(syntax: &File) -> (Vec<String>, Vec<Class>) {
    // First, collect all struct and enum names for relationship detection
    let mut class_names = vec![];
    let mut classes = vec![];
    for item in &syntax.items {
        match item {
            Item::Struct(item) => {
                let visibility = Visibility::from_vis(&item.vis);

                let mut class = Class::new(visibility, item.ident.to_string());
                for field in &item.fields {
                    class.fields.push(Field::new(
                        Visibility::from_vis(&field.vis),
                        field
                            .ident
                            .clone()
                            .expect("Invalid Field Identity")
                            .to_string(),
                        field.ty.to_token_stream().to_string(),
                    ));
                }

                class_names.push(item.ident.to_string());
                classes.push(class);
            }
            Item::Enum(item) => {
                let visibility = Visibility::from_vis(&item.vis);
                let mut class = Class::new(visibility, item.ident.to_string());
                for field in &item.variants {
                    class.fields.push(Field::new(
                        Visibility::from_vis(&item.vis),
                        field.ident.to_string(),
                        "".to_string(), // field.ty.to_token_stream().to_string(),
                    ));
                }

                class_names.push(item.ident.to_string());
                classes.push(class);
            }
            Item::Trait(item) => {
                let mut class = Class::new(Visibility::from_vis(&item.vis), item.ident.to_string());

                // Add methods from the trait
                for signature in &item.items {
                    if let syn::TraitItem::Fn(method) = signature {
                        let method_name = method.sig.ident.to_string();
                        let return_type = if let syn::ReturnType::Type(_, ty) = &method.sig.output {
                            format!("{}", quote::quote!(#ty))
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
                                    Some(pat_type.to_token_stream().to_string())
                                } else {
                                    None
                                }
                            })
                            .collect();

                        class.functions.push(Function::new(
                            Visibility::from_vis(&item.vis),
                            method_name,
                            params.join(", "),
                            return_type,
                        ))
                    }
                }
                class_names.push(item.ident.to_string());
                classes.push(class);
            }
            Item::Fn(item) => {
                let mut class =
                    Class::new(Visibility::from_vis(&item.vis), item.sig.ident.to_string());
                class.fields.push(Field::new(
                    Visibility::from_vis(&item.vis),
                    item.vis.to_token_stream().to_string(),
                    "".to_string(),
                ));
                class_names.push(item.sig.ident.to_string());
                classes.push(class);
            }
            _ => {}
        }
    }

    return (class_names, classes);
}
