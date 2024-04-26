use floem::peniko::Color;
use floem::style::CursorStyle;
use floem::view::{View, Widget};
use floem::views::{container, label, svg, tooltip, Decorators};

pub fn tooltip_label<S: std::fmt::Display + 'static, V: View + 'static>(
    child: V,
    text: impl Fn() -> S + 'static + Clone,
) -> impl View {
    tooltip(child, move || tooltip_tip(label(text.clone())))
}

fn tooltip_tip<V: View + 'static>(child: V) -> impl Widget {
    container(child).style(move |s| {
        s.padding_horiz(10.0)
            .padding_vert(5.0)
            .border(1)
            .border_radius(6)
            .background(Color::rgb8(240, 240, 240))
            .margin_left(0.0)
            .margin_top(4.0)
    })
}

pub fn clickable_icon<S: std::fmt::Display + 'static>(
    icon: impl Fn() -> &'static str + 'static,
    on_click: impl Fn() + 'static,
    tooltip_: impl Fn() -> S + 'static + Clone,
) -> impl View {
    tooltip_label(clickable_icon_base(icon, Some(on_click)), tooltip_)
}

pub fn clickable_icon_base(
    icon: impl Fn() -> &'static str + 'static,
    on_click: Option<impl Fn() + 'static>,
) -> impl View {
    let view = container(svg(move || icon().to_string()).style(move |s| {
        let size = 24;
        s.size(size, size)
            .color(Color::BLACK)
            .disabled(|s| s.color(Color::RED).cursor(CursorStyle::Default))
    }))
    .style(move |s| {
        s.padding(1.0).height(24).hover(|s| {
            s.cursor(CursorStyle::Pointer)
                .border_radius(6.0)
                .background(Color::rgb8(220, 220, 220))
        })
    });

    if let Some(on_click) = on_click {
        view.on_click_stop(move |_| {
            on_click();
        })
    } else {
        view
    }
}
