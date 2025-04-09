use crate::{class_diagram_model::*, settings::DiagramSettings};

pub trait MermaidDiagram {
    fn to_diagram_syntax(&self, settings: &DiagramSettings) -> String;
}

impl MermaidDiagram for Class {
    fn to_diagram_syntax(&self, settings: &DiagramSettings) -> String {
        let mut res = "".to_string();
        let header = format!("class {} {{\n", self.name);
        res.push_str(header.as_str());

        for field in &self.fields {
            res.push_str(field.to_diagram_syntax(settings).as_str());
            res.push_str("\n");
        }

        for function in &self.functions {
            res.push_str(function.to_diagram_syntax(settings).as_str());
            res.push_str("\n");
        }

        res.push_str("}\n");

        for connection in &self.connections {
            res.push_str(connection.to_diagram_syntax(settings).as_str());
            res.push_str("\n");
        }

        res
    }
}

impl MermaidDiagram for Function {
    fn to_diagram_syntax(&self, settings: &DiagramSettings) -> String {
        let mut parameters = String::new();
        if settings.show_parameters {
            parameters = self.params.clone();
        }
        format!(
            "{}{}({}) {}",
            self.visibility.to_diagram_syntax(settings),
            self.name,
            parameters,
            self.return_type
        )
    }
}

impl MermaidDiagram for Field {
    fn to_diagram_syntax(&self, settings: &DiagramSettings) -> String {
        format!(
            "{}{}: {}",
            self.visibility.to_diagram_syntax(settings),
            self.name,
            self.type_string
        )
    }
}

impl MermaidDiagram for Visibility {
    fn to_diagram_syntax(&self, settings: &DiagramSettings) -> String {
        match self {
            Visibility::Public => "+".to_string(),
            Visibility::Internal => "~".to_string(),
            Visibility::Private => "-".to_string(),
        }
    }
}

impl MermaidDiagram for Connection {
    fn to_diagram_syntax(&self, settings: &DiagramSettings) -> String {
        format!(
            "{} {} {}",
            self.name_a,
            self.relationship.to_diagram_syntax(settings),
            self.name_b,
        )
    }
}

impl MermaidDiagram for Relationship {
    fn to_diagram_syntax(&self, settings: &DiagramSettings) -> String {
        match self {
            Relationship::Inheritance => "<|--".to_string(),
            Relationship::Association => "-->".to_string(),
            Relationship::Dependency => "..>".to_string(),
        }
    }
}
