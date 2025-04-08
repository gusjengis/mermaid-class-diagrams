struct Class {
    visibility: Visibility,
    fields: Vec<Field>,
    functions: Vec<Function>,
    children: Vec<String>,
}

struct Field {
    pub visibilty: Visibility,
    pub name: String,
    pub type_string: String,
}

struct Function {
    pub visibilty: Visibility,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
}

struct Parameter {
    pub name: String,
    pub type_string: String,
}

enum Visibility {
    Public,
    Private,
}

pub trait MermaidDiagram {
    fn to_diagram_syntax(&self) -> String;
}

impl MermaidDiagram for Field {
    fn to_diagram_syntax(&self) -> String {
        format!(
            "{}{}: {}",
            self.visibilty.to_diagram_syntax(),
            self.name,
            self.type_string
        )
    }
}

impl MermaidDiagram for Visibility {
    fn to_diagram_syntax(&self) -> String {
        match self {
            Visibility::Public => "+".to_string(),
            Visibility::Private => "-".to_string(),
        }
    }
}
