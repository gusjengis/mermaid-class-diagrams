use std::collections::HashMap;

use quote::ToTokens;
type Vis = syn::Visibility;

#[derive(Clone)]
pub struct Class {
    pub(crate) visibility: Visibility,
    pub(crate) name: String,
    pub(crate) fields: Vec<Field>,
    pub(crate) functions: Vec<Function>,
    pub(crate) connections: HashMap<String, Connection>,
}

#[derive(Clone)]
pub struct Field {
    pub(crate) visibility: Visibility,
    pub(crate) name: String,
    pub(crate) type_string: String,
}

#[derive(Clone)]
pub struct Function {
    pub(crate) visibility: Visibility,
    pub(crate) name: String,
    pub(crate) params: String,
    pub(crate) return_type: String,
}

#[derive(Clone)]
pub struct Connection {
    pub(crate) name_a: String,
    pub(crate) name_b: String,
    pub(crate) relationship: Relationship,
}

#[derive(Clone)]
pub enum Relationship {
    Inheritance,
    Association,
    Dependency,
}

#[derive(Clone)]
pub enum Visibility {
    Public,
    Private,
    Internal,
}

impl Class {
    pub fn new(visibility: Visibility, name: String) -> Self {
        Self {
            visibility,
            name,
            fields: vec![],
            functions: vec![],
            connections: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, name: String, connection: Connection) {
        if !self.connections.contains_key(name.as_str()) {
            self.connections.insert(name, connection);
        }
    }
}

impl Field {
    pub fn new(visibility: Visibility, name: String, type_string: String) -> Self {
        Self {
            visibility,
            name,
            type_string,
        }
    }
}

impl Function {
    pub fn new(visibility: Visibility, name: String, params: String, return_type: String) -> Self {
        Self {
            visibility,
            name,
            params,
            return_type,
        }
    }
}

impl Connection {
    pub fn new(name_a: String, name_b: String, relationship: Relationship) -> Self {
        Self {
            name_a,
            name_b,
            relationship,
        }
    }
}

impl Visibility {
    pub fn from_vis(vis: &Vis) -> Self {
        match vis {
            syn::Visibility::Public(_) => Visibility::Public,
            syn::Visibility::Restricted(scope) => {
                match scope.to_token_stream().to_string().as_str() {
                    "pub (crate)" => Visibility::Internal,
                    _ => Visibility::Private,
                }
            }
            _ => Visibility::Private,
        }
    }
}
