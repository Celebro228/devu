use raylib::prelude::*;


pub struct Conf {
    /// Default: "DeVu Project"
    pub title: String,
    /// Screen size
    /// Default: (1280, 720)
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
            title: "DeVu".to_string(),
            size: (1280, 720),
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

impl Conf {
    pub(crate) fn build(self) -> (RaylibHandle, RaylibThread) {
        let mut builder = init();
        builder.title(&self.title);
        builder.size(self.size.0, self.size.1);
        if self.resizable {
            builder.resizable();
        }
        if self.msaa_4x {
            builder.msaa_4x();
        }
        if self.vsync {
            builder.vsync();
        }
        if !self.logging {
            builder.log_level(TraceLogLevel::LOG_NONE);
        }
        builder.build()
    }
}