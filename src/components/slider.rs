use leptos::prelude::*;
use leptos::web_sys::MouseEvent;

/// A vertical slider with a horizontal draggable bar.
/// 0% at bottom (green) morphing to 100% at top (red).
#[component]
pub fn Slider() -> impl IntoView {
    let (value, set_value) = signal(0i32);
    let (dragging, set_dragging) = signal(false);
    let track_ref = NodeRef::<leptos::html::Div>::new();

    // Color interpolation: green (0%) to red (100%)
    let bar_color = move || {
        let v = value.get() as f32 / 100.0;
        let r = (255.0 * v) as u8;
        let g = (255.0 * (1.0 - v)) as u8;
        format!("rgb({}, {}, 0)", r, g)
    };

    // Bar position: bottom percentage based on value
    let bar_bottom = move || format!("calc({}% - 8px)", value.get());

    let on_mousedown = move |ev: MouseEvent| {
        ev.prevent_default();
        set_dragging.set(true);
    };

    let on_mousemove = move |ev: MouseEvent| {
        if dragging.get() {
            if let Some(track) = track_ref.get() {
                let rect = track.get_bounding_client_rect();
                let track_top = rect.top();
                let track_height = rect.height();
                let mouse_y = ev.client_y() as f64;

                // Calculate value: top = 100%, bottom = 0%
                let relative_y = mouse_y - track_top;
                let ratio = 1.0 - (relative_y / track_height);
                let new_val = (ratio * 100.0).clamp(0.0, 100.0) as i32;
                set_value.set(new_val);
            }
        }
    };

    let on_mouseup = move |_: MouseEvent| {
        set_dragging.set(false);
    };

    view! {
        <div
            class="slider-container"
            style="display: inline-block; padding: 10px;"
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
            on:mouseleave=on_mouseup
        >
            <div
                node_ref=track_ref
                class="slider-track"
                style="width: 60px; height: 300px; background: #ddd; position: relative; border-radius: 4px; cursor: pointer;"
                on:mousedown=on_mousedown
            >
                {/* Horizontal bar (the draggable handle) */}
                <div
                    class="slider-bar"
                    style:position="absolute"
                    style:bottom=bar_bottom
                    style:left="0"
                    style:width="100%"
                    style:height="16px"
                    style:background=bar_color
                    style:border-radius="3px"
                    style:cursor="grab"
                    style:box-shadow="0 2px 4px rgba(0,0,0,0.3)"
                />
            </div>
            <div style="text-align: center; margin-top: 8px; font-size: 14px;">
                {move || format!("{}%", value.get())}
            </div>
        </div>
    }
}
