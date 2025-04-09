pub struct Settings {
    pub image_settings: Image_Settings,
    pub diagram_settings: Diagram_Settings,
}

pub struct Image_Settings {
    pub enabled: bool,
    pub scale: u32,
}

pub struct Diagram_Settings {
    pub show_parameters: bool,
}

impl Settings {
    pub fn defaults() -> Self {
        Self {
            image_settings: Image_Settings {
                enabled: true,
                scale: 5,
            },
            diagram_settings: Diagram_Settings {
                show_parameters: false,
            },
        }
    }
}
