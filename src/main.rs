mod app_icon;
mod app_view;
mod utils;
mod widgets;

use app_view::App;
use floem::kurbo::Size;
use floem::peniko::Color;
use floem::taffy::AlignContent;
use floem::view::View;
use floem::views::{h_stack, label, text_editor, v_stack, Decorators};
use floem::window::WindowConfig;
use floem::Application;

fn card_view() -> impl View {
    h_stack((
        label(|| "a").style(|s| s.flex_grow(0.5)),
        label(|| "b").style(|s| s.flex_grow(1.)),
    ))
    .style(|s| {
        s.background(Color::rgb8(240, 240, 240))
            .flex_row()
            .justify_content(AlignContent::Center)
            .padding(8)
            .border_radius(8.)
    })
}

impl App {
    fn main_view(&self) -> impl View {
        v_stack((
            label(|| "Title")
                .style(|s| s.font_size(24.).justify_content(AlignContent::SpaceAround)),
            card_view(),
            card_view(),
            text_editor("")
                .style(|s| s.height(200))
                .editor_style(|s| s.hide_gutter(true)),
        ))
        .style(|s| {
            s.width_full()
                .padding(4)
                .gap(0, 10)
                .background(Color::WHITE)
                .font_size(13.)
        })
    }
}

fn main() {
    let app_cfg = WindowConfig::default()
        .size(Size {
            width: 400.0,
            height: 550.0,
        })
        .title("App Title")
        .show_titlebar(false)
        .resizable(true);

    let mut app = App::new();
    Application::new()
        .window(move |wnd_id| app.app_view(wnd_id), Some(app_cfg))
        .run();
}
