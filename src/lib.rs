#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let canvas = window.canvas().unwrap();
                doc.body()
                    .unwrap()
                    .append_child(&web_sys::Element::from(canvas))
                    .ok()?;
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
