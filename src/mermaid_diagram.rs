use crate::class_diagram_model::*;

pub trait MermaidDiagram {
    fn to_diagram_syntax(&self) -> String;
}

impl MermaidDiagram for Class {
    fn to_diagram_syntax(&self) -> String {
        let mut res = "".to_string();
        let header = format!("class {} {{\n", self.name);
        res.push_str(header.as_str());

        for field in &self.fields {
            res.push_str(field.to_diagram_syntax().as_str());
            res.push_str("\n");
        }

        for function in &self.functions {
            res.push_str(function.to_diagram_syntax().as_str());
            res.push_str("\n");
        }

        res.push_str("}\n");

        for connection in &self.connections {
            res.push_str(connection.to_diagram_syntax().as_str());
            res.push_str("\n");
        }

        res
    }
}

impl MermaidDiagram for Function {
    fn to_diagram_syntax(&self) -> String {
        format!(
            "{}{}() {}",
            self.visibility.to_diagram_syntax(),
            self.name,
            self.return_type
        )
    }
}

impl MermaidDiagram for Field {
    fn to_diagram_syntax(&self) -> String {
        format!(
            "{}{}: {}",
            self.visibility.to_diagram_syntax(),
            self.name,
            self.type_string
        )
    }
}

impl MermaidDiagram for Visibility {
    fn to_diagram_syntax(&self) -> String {
        match self {
            Visibility::Public => "+".to_string(),
            Visibility::Internal => "~".to_string(),
            Visibility::Private => "-".to_string(),
        }
    }
}

impl MermaidDiagram for Connection {
    fn to_diagram_syntax(&self) -> String {
        format!(
            "{} {} {}",
            self.name_a,
            self.relationship.to_diagram_syntax(),
            self.name_b,
        )
    }
}

impl MermaidDiagram for Relationship {
    fn to_diagram_syntax(&self) -> String {
        match self {
            Relationship::Inheritance => "<|--".to_string(),
            Relationship::Association => "-->".to_string(),
        }
    }
}
