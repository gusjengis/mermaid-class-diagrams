pub struct Settings {
    pub image_settings: ImageSettings,
    pub diagram_settings: DiagramSettings,
}

pub struct ImageSettings {
    pub _enabled: bool,
    pub scale: u32,
}

pub struct DiagramSettings {
    pub show_parameters: bool,
}

impl Settings {
    pub fn defaults() -> Self {
        Self {
            image_settings: ImageSettings {
                _enabled: true,
                scale: 5,
            },
            diagram_settings: DiagramSettings {
                show_parameters: false,
            },
        }
    }
}
