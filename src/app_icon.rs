pub struct AppIcon;

impl AppIcon {
    pub const CLOSE: &'static str = include_str!("../assets/wnd-close.svg");
    pub const WND_MINIMIZE: &'static str = include_str!("../assets/wnd-minimize.svg");
    pub const LOGO: &'static str = include_str!("../assets/logo.svg");
    pub const WND_RESTORE: &'static str = include_str!("../assets/wnd-restore.svg");
    pub const WND_MAXIMIZE: &'static str = include_str!("../assets/wnd-maximize.svg");
}
