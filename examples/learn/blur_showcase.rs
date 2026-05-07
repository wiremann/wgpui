//! Element Blur Showcase
//!
//! Demonstrates element-level `blur(...)` used alongside `opacity(...)`.
//! Run with:
//!   cargo run --example blur_showcase

#[path = "../prelude.rs"]
mod example_prelude;

use example_prelude::init_example;
use gpui::{
    App, Application, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

struct BlurShowcase;

fn color_strip() -> impl IntoElement {
    div()
        .flex()
        .h_20()
        .rounded_lg()
        .overflow_hidden()
        .child(div().flex_1().bg(rgb(0xff5d73)))
        .child(div().flex_1().bg(rgb(0xffc857)))
        .child(div().flex_1().bg(rgb(0x56cfe1)))
        .child(div().flex_1().bg(rgb(0x80ed99)))
        .child(div().flex_1().bg(rgb(0x6d77ff)))
}

fn blur_card(
    id: &'static str,
    title: &'static str,
    subtitle: &'static str,
    blur_radius: f32,
    opacity: f32,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .flex_col()
        .gap_2()
        .p_4()
        .rounded_lg()
        .bg(rgb(0x2a2a2a))
        .opacity(opacity)
        .blur(blur_radius)
        .border_1()
        .border_color(rgb(0x444444))
        .child(div().text_sm().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(div().text_xs().text_color(rgb(0x999999)).child(subtitle))
}

fn demo_row(
    id: &'static str,
    heading: &'static str,
    blur_radius: f32,
    opacity: f32,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .flex_col()
        .gap_2()
        .child(div().text_xs().text_color(rgb(0x999999)).child(heading))
        .child(
            div()
                .relative()
                .child(color_strip())
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .bottom_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(blur_card(
                            "blur-card",
                            "Frosted Element",
                            "Element-level blur + opacity",
                            blur_radius,
                            opacity,
                        )),
                ),
        )
}

impl Render for BlurShowcase {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {

        div()
            .size_full()
            .p_6()
            .flex()
            .flex_col()
            .gap_5()
            .bg(rgb(0x1e1e1e))
            .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Element Blur Showcase"))
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x999999))
                    .child("Compare rows: blur radius increases while opacity stays semi-transparent."),
            )
            .child(demo_row(
                "row-none",
                "blur(0.0), opacity(0.82)",
                0.0,
                0.82,
            ))
            .child(demo_row(
                "row-soft",
                "blur(6.0), opacity(0.82)",
                6.0,
                0.82,
            ))
            .child(demo_row(
                "row-medium",
                "blur(12.0), opacity(0.82)",
                12.0,
                0.82,
            ))
            .child(demo_row(
                "row-strong",
                "blur(18.0), opacity(0.82)",
                18.0,
                0.82,
            ))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        init_example(cx, "Blur Showcase");

        let bounds = Bounds::centered(None, size(px(900.), px(760.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| BlurShowcase),
        )
        .expect("open window");
    });
}
