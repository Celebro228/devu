pub struct Conf {
    /// Default: "DeVu Project"
    pub title: String,
    /// Screen size
    /// Default: (800, 400)
    pub size: (i32, i32),
    /// Default: true
    pub resizable: bool,
    /// Default: true
    pub msaa_4x: bool,
    /// Default: true
    pub vsync: bool,
    /// Default for debug mod: true
    /// Default for release mod: false
    pub logging: bool,
}

impl Default for Conf {
    fn default() -> Self {
        Self {
            title: "DeVu Project".to_string(),
            size: (800, 400),
            resizable: true,
            msaa_4x: true,
            vsync: true,
            #[cfg(debug_assertions)]
            logging: true,
            #[cfg(not(debug_assertions))]
            logging: false,
        }
    }
}