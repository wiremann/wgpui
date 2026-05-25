use gpui::*;

struct TextGradientDemo {
    progress: f32,
}

impl Render for TextGradientDemo {
    fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(white())
            .size_full()
            .child(
                div()
                    .text_gradient_horizontal(
                        linear_color_stop(rgb(0xFF0000), 0.0),
                        linear_color_stop(rgb(0x0000FF), 1.0),
                    )
                    .text_size(px(32.0))
                    .child("Horizontal Gradient (Red to Blue)"),
            )
            .child(
                div()
                    .text_gradient_vertical(
                        linear_color_stop(rgb(0x00FF00), 0.0),
                        linear_color_stop(rgb(0xFF00FF), 1.0),
                    )
                    .text_size(px(32.0))
                    .child("Vertical Gradient (Green to Magenta)"),
            )
            .child(
                div()
                    .text_gradient(
                        45.0,
                        linear_color_stop(rgb(0xFFFF00), 0.0),
                        linear_color_stop(rgb(0x00FFFF), 1.0),
                    )
                    .text_size(px(32.0))
                    .child("45° Diagonal Gradient (Yellow to Cyan)"),
            )
            .child(
                div()
                    .text_gradient(
                        135.0,
                        linear_color_stop(rgb(0xFF8800), 0.0),
                        linear_color_stop(rgb(0x8800FF), 1.0),
                    )
                    .text_size(px(32.0))
                    .child("135° Diagonal Gradient (Orange to Purple)"),
            )
            .child(
                div()
                    .text_gradient_horizontal(
                        linear_color_stop(rgb(0x0088FF), 0.0),
                        linear_color_stop(rgb(0xCCCCCC), self.progress),
                    )
                    .text_size(px(32.0))
                    .child("Progress Bar Text Effect"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgb(0x666666))
                    .child(format!("Progress: {:.0}%", self.progress * 100.0)),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(100.0), px(100.0)),
                    size: size(px(800.0), px(600.0)),
                })),
                ..Default::default()
            },
            |cx| {
                cx.spawn(|view, mut cx| async move {
                    loop {
                        for i in 0..=100 {
                            let progress = i as f32 / 100.0;
                            view.update(&mut cx, |this, cx| {
                                this.progress = progress;
                                cx.notify();
                            })
                            .ok();
                            async_timer::Timer::after(std::time::Duration::from_millis(20))
                                .await;
                        }
                        async_timer::Timer::after(std::time::Duration::from_secs(1)).await;
                    }
                })
                .detach();

                TextGradientDemo { progress: 0.0 }
            },
        )
        .unwrap();
    });
}
