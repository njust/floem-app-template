use crate::app_icon::AppIcon;
use crate::utils;
use crate::widgets::clickable_icon;
use floem::event::{Event, EventListener};
use floem::kurbo::Size;
use floem::peniko::Color;
use floem::reactive::RwSignal;
use floem::view::View;
use floem::views::{
    drag_resize_window_area, drag_window_area, empty, h_stack, svg, v_stack, Decorators,
};
use floem::window::{ResizeDirection, WindowId};

pub struct App {
    wnd_size: RwSignal<Size>,
    window_maximized: RwSignal<bool>,
    wnd_id: Option<WindowId>,
}

impl App {
    pub fn new() -> Self {
        Self {
            wnd_id: None,
            window_maximized: RwSignal::new(false),
            wnd_size: RwSignal::new(Size::new(0., 0.)),
        }
    }

    pub fn app_view(&mut self, wnd_id: WindowId) -> impl View {
        self.wnd_id.replace(wnd_id);
        let window_size = self.wnd_size.clone();
        let window_maximized = self.window_maximized.clone();
        utils::load_icon();
        v_stack((
            drag_resize_window_area(ResizeDirection::West, empty())
                .style(|s| s.absolute().width(4.0).height_full()),
            drag_resize_window_area(ResizeDirection::North, empty())
                .style(|s| s.absolute().width_full().height(4.0)),
            drag_resize_window_area(ResizeDirection::South, empty()).style(move |s| {
                s.absolute()
                    .margin_top(window_size.get().height as f32 - 4.0)
                    .width_full()
                    .height(4.0)
            }),
            self.title_bar(),
            drag_resize_window_area(ResizeDirection::East, empty()).style(move |s| {
                s.absolute()
                    .margin_left(window_size.get().width as f32 - 4.0)
                    .width(4.0)
                    .height_full()
            }),
            self.main_view(),
        ))
        .style(|s| {
            s.width_full()
                .background(Color::WHITE)
                .font_size(13.)
                .border(1)
                .border_color(Color::GRAY)
        })
        .on_event_cont(EventListener::WindowMaximizeChanged, move |event| {
            if let Event::WindowMaximizeChanged(maximized) = event {
                window_maximized.set(*maximized);
            }
        })
        .on_resize(move |rect| window_size.set(Size::new(rect.x1, rect.y1)))
    }

    fn title_bar(&self) -> impl View {
        let wnd_id = self.wnd_id.unwrap().clone();
        let is_maximized = self.window_maximized;
        h_stack((
            svg(|| AppIcon::LOGO.to_string()).style(|s| s.size(24, 24).margin(8)),
            drag_window_area(empty())
                .style(|s| s.height_full().flex_basis(0.0).flex_grow(1.0).margin_top(4)),
            h_stack((
                clickable_icon(
                    || AppIcon::WND_MINIMIZE,
                    move || floem::action::minimize_window(),
                    || "Minimize",
                ),
                clickable_icon(
                    move || {
                        let is_max = is_maximized.get();
                        if is_max {
                            AppIcon::WND_RESTORE
                        } else {
                            AppIcon::WND_MAXIMIZE
                        }
                    },
                    move || {
                        let is_max = is_maximized.get();
                        floem::action::set_window_maximized(!is_max);
                    },
                    move || {
                        let is_max = is_maximized.get();
                        if is_max {
                            "Restore"
                        } else {
                            "Maximize"
                        }
                    },
                ),
                clickable_icon(
                    || AppIcon::CLOSE,
                    move || floem::close_window(wnd_id),
                    || "Close",
                ),
            ))
            .style(|s| s.margin_top(4).margin_right(4).gap(4, 0)),
        ))
    }
}
