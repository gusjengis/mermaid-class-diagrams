use std::path::Path;
use syn::{File, Item};

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
                // You might add methods from the trait here
                diagram.push_str("}\n\n");
            }
            // You can extend to include impl blocks for relationships if desired.
            _ => {}
        }
    }
}
