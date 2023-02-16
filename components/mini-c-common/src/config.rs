#[derive(Debug)]
pub struct SushiConfig {
    pub is_color_enabled: bool,
}

impl SushiConfig {
    pub fn with_colors() -> Self {
        Self {
            is_color_enabled: true,
        }
    }
    pub fn without_colors() -> Self {
        Self {
            is_color_enabled: false,
        }
    }
}
