use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct WebContext2d {
    pub context: CanvasRenderingContext2d,
    pub canvas: HtmlCanvasElement,
}

/// Returns the HTML canvas element and its 2D context from the web page.
pub fn get_canvas_and_context(canvas_html_id: &str) -> Result<WebContext2d, &'static str> {
    // Get the canvas element by ID
    let canvas = document()
        .get_element_by_id(canvas_html_id)
        .ok_or("Failed to find the canvas element with ID 'demo_screen'")?;

    let canvas = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| "Failed to convert the element into an HtmlCanvasElement")?;

    let context = canvas
        .get_context("2d")
        .map_err(|_| "Failed to get the 2D context from the canvas")?
        .ok_or("The 2D context is not available")?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| "Failed to convert the context into CanvasRenderingContext2d")?;

    Ok(WebContext2d { context, canvas })
}
