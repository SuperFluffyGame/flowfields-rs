mod state;
use state::State;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use winit::dpi::PhysicalSize;
use winit::event_loop::ControlFlow;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;
use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};

static STAT: i32 = 19923;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // init canvas
    #[cfg(target_arch = "wasm32")]
    {
        window.set_inner_size(PhysicalSize::new(500, 400));
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let canvas = window.canvas();
                doc.body()
                    .unwrap()
                    .append_child(&web_sys::Element::from(canvas))
                    .ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut state = State::new(window).await;
    event_loop.run(move |event, _target, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window().id() => {
            if !state.input(&event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            state.render().unwrap();
        }
        Event::MainEventsCleared => {
            state.window().request_redraw();
        }
        _ => {}
    })
}
