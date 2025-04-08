use syn::{File, Item};

pub fn discover_types(syntax: &File) -> Vec<String> {
    // First, collect all struct and enum names for relationship detection
    let mut defined_types = vec![];

    for item in &syntax.items {
        match item {
            Item::Struct(s) => {
                defined_types.push(s.ident.to_string());
            }
            Item::Enum(e) => {
                defined_types.push(e.ident.to_string());
            }
            Item::Trait(e) => {
                defined_types.push(e.ident.to_string());
            }
            _ => {}
        }
    }

    return defined_types;
}
