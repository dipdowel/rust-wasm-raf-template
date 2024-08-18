use gloo::utils::{document, window};
use js_sys::Function;
use wasm_bindgen::Clamped;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

use draw::rectangle_filled;

use crate::primitives::{Dimensions2d, Point, RectArea};
use crate::web_init::{get_canvas_and_context, WebContext2d};

mod console;
mod draw;
mod primitives;
mod web_init;

/// id of the canvas element, @see `index.html`
const CANVAS_HTML_ID: &str = "demo_screen";
/// id of the heading element, @see `index.html`
const HEADING_HTML_ID: &str = "demo_heading";
/// How many frames the animation should last
const TOTAL_FRAMES: u32 = 780;

struct ApplicationState {
    hero: RectArea,
}

/// Global state that survives between frames
struct GlobalState {
    /// Various application data that needs to survive between frames
    app: ApplicationState,
    /// Convenience reference to the HTML canvas and its context.
    web_context2d: Option<WebContext2d>,
}

/// Global mutable state (ugly and dangerous 😈)
static mut GLOBAL_STATE: *mut GlobalState = std::ptr::null_mut();

/// Initialize the global state
fn initialize_global_state() {
    unsafe {
        GLOBAL_STATE = Box::into_raw(Box::new(GlobalState {
            app: ApplicationState {
                hero: RectArea {
                    top_left: Point { x: 0, y: 340 },
                    dimensions: Dimensions2d { w: 16, h: 50 },
                },
            },
            web_context2d: None,
        }));
    }
}

/// Access the global state
fn get_global_state() -> &'static mut GlobalState {
    unsafe {
        if GLOBAL_STATE.is_null() {
            initialize_global_state();
        }
        &mut *GLOBAL_STATE
    }
}

/// Sets the web context in the global state
fn set_web_context(web_context2d: WebContext2d) {
    let global_struct = get_global_state();
    global_struct.web_context2d = Some(web_context2d);
}

#[wasm_bindgen]
pub fn main() {
    console::log(&"Rust + WASM + Canvas2D, animation template");
    console::log(&"For the source code visit:");
    console::log(&"");
    console::log(&"--> https://github.com/dipdowel/rust-wasm-raf-template");
    console::log(&"");

    // Set the default heading with a warning
    document()
        .get_element_by_id(HEADING_HTML_ID)
        .unwrap()
        .set_text_content(Some(
            "[WARN] The animation loop is not happening for some reason!",
        ));

    // Save references to the canvas and its 2D context in the global state
    set_web_context(get_canvas_and_context(CANVAS_HTML_ID).unwrap());

    // Kick off the animation loop beginning with frame 0
    animation_loop(0);
}

/// The main animation loop
fn animation_loop(mut frame_count: u32) {
    let heading = document().get_element_by_id(HEADING_HTML_ID).unwrap();

    // Get the canvas and its context
    let web_context_state = get_global_state().web_context2d.as_ref().unwrap();
    let WebContext2d { context, canvas } = web_context_state;

    // Clear the canvas
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    // halt the animation after `TOTAL_FRAMES` number of frames
    if frame_count > TOTAL_FRAMES {
        heading.set_text_content(Some(&format!("{TOTAL_FRAMES} frames rendered.")));
        return;
    }

    // Get the mutable application state
    let app_state = &mut get_global_state().app;

    // Let's move the hero 1px to the right
    app_state.hero.top_left.x += 1;

    // Update the heading with the current frame count
    heading.set_text_content(Some(&format!("Frame: {frame_count} / {TOTAL_FRAMES}")));

    // =========== [ ANIMATE THE HORIZONTAL 1PX PROGRESS BAR ] =====================================

    let image_data = context
        .get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)
        .expect("Should be able to get image data");

    // Access the pixel data array (RGBA values)
    let mut data = image_data.data();

    // Draw the progress bar on the top row of pixels on the canvas
    for i in (0..(frame_count * 4) as usize).step_by(4) {
        data[i + 0] = 0x00; // Red
        data[i + 1] = 0xff; // Green
        data[i + 2] = 0x00; // Blue
        data[i + 3] = 0xff; // Alpha
    }

    // Create a new ImageData object from the modified data
    let new_image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut data),
        canvas.width(),
        canvas.height(),
    )
        .expect("Should be able to create new image data");

    // Render the updated ImageData to the canvas
    context
        .put_image_data(&new_image_data, 0.0, 0.0)
        .expect("Should be able to put image data back");

    // =========== [ ANIMATE THE HERO  ] ===========================================================
    // The wobbly green part of the hero
    let wobbly_y = 250.0 + 50.0 * (2.0 * std::f32::consts::PI * (frame_count as f32 / 60.0)).sin();
    let h_delta = 3.0 * (2.0 * std::f32::consts::PI * (frame_count as f32 / 60.0)).sin();
    app_state.hero.top_left.y = wobbly_y as u32;
    app_state.hero.dimensions.h = (app_state.hero.dimensions.h as f32 + h_delta) as u32;
    rectangle_filled(web_context_state, &app_state.hero, 0x00_77_00_ff);

    // The white part of the hero
    let mut square_2: RectArea = app_state.hero.clone();
    square_2.dimensions.h = 10;
    rectangle_filled(web_context_state, &square_2, 0xff_ff_ff_ff);

    /*
    // High level rectangle rendering, if you like
    context.set_fill_style(&JsValue::from_str("green"));
    context.fill_rect(780.0, 580.0, 20.0, 20.0);
    */

    // =========== [ REQUEST ANIMATION FRAME  ] ====================================================
    frame_count += 1; // Advance the animation one frame at a time

    // Request the next frame (TODO: try using gloo::render::request_animation_frame here )
    window()
        .request_animation_frame(&Function::from(Closure::once_into_js(move || {
            animation_loop(frame_count)
        })))
        .unwrap();
}
