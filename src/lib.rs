#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use web_sys::Document;
use winit::platform::web::WindowExtWebSys;
use winit::{
    dpi::LogicalSize, event::*, event_loop::EventLoop, keyboard::KeyCode, window::WindowBuilder,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    // init canvas
    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.

        let w = window.request_inner_size(LogicalSize::new(500, 500));

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let canvas = web_sys::Element::from(window.canvas().unwrap());
                canvas.set_id("can");
                doc.body().unwrap().append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    event_loop
        .run(move |event, target| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::Resized(size) => {
                    web_sys::window()
                        .and_then(|win| win.document())
                        .and_then(|doc| doc.get_element_by_id("can"))
                        .and_then(|el| {
                            el.set_attribute("width", &size.width.to_string()).unwrap();
                            el.set_attribute("height", &size.height.to_string())
                                .unwrap();

                            let args = js_sys::Array::new();
                            args.push(&JsValue::from_str("aaa"));
                            web_sys::console::log(&args);
                            Some(())
                        })
                        .unwrap();
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: winit::keyboard::PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => target.exit(),
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}
