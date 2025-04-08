use std::path::Path;
use syn::{File, Item, TraitItem};

pub fn process_file(syntax: &File, diagram: &mut String) {
    for item in &syntax.items {
        match item {
            Item::Struct(s) => {
                let struct_name = s.ident.to_string();
                // Start a Mermaid class definition:
                diagram.push_str(&format!("class {} {{\n", struct_name));

                // Optionally, list fields (if the struct has named fields)
                if let syn::Fields::Named(fields) = &s.fields {
                    for field in fields.named.iter() {
                        if let Some(ident) = &field.ident {
                            // You might want to show the field type as well:
                            let ty = quote::quote!(#field.ty).to_string();
                            diagram.push_str(&format!("  +{}: {}\n", ident, ty));
                        }
                    }
                }
                diagram.push_str("}\n\n");
            }
            Item::Enum(e) => {
                let enum_name = e.ident.to_string();
                diagram.push_str(&format!("class {} {{\n", enum_name));
                // Enumerators can be treated as values or attributes:
                for variant in &e.variants {
                    diagram.push_str(&format!("  +{}\n", variant.ident));
                }
                diagram.push_str("}\n\n");
            }
            Item::Trait(t) => {
                let trait_name = t.ident.to_string();
                diagram.push_str(&format!("class {} {{\n", trait_name));
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
    }
}
