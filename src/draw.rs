use wasm_bindgen::Clamped;
use web_sys::ImageData;

use crate::primitives::RectArea;
use crate::web_init::WebContext2d;

/**
    NB: `rectangle_filled()` was made based on the function of the same name from `graph1` crate.
    @See [Graph1 on github](https://github.com/dipdowel/graph1)
*/

/// Draw a filled rectangle on the canvas.
pub fn rectangle_filled(web_context: &WebContext2d, rect: &RectArea, color: u32) {
    let start_x = rect.top_left.x;
    let start_y = rect.top_left.y;
    let width = rect.dimensions.w;
    let height = rect.dimensions.h;

    // Nothing to draw here
    if width == 0 || height == 0 {
        return;
    }

    let end_x = start_x + width;
    let end_y = start_y + height;

    let mut x = start_x;
    let mut y = start_y;

    let WebContext2d { context, canvas } = web_context;

    let image_data = context
        .get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)
        .expect("Should be able to get image data");

    // Access the pixel data array (RGBA values)
    let mut data = image_data.data();

    let canvas_w = canvas.width();
    let canvas_h = canvas.height();

    // Which pixel in the vector should be filled in next.
    let mut pixel_index: usize;

    loop {
        pixel_index = 4 * (y * canvas_w + x) as usize;
        // Apply the color per Red, Green, Blue, and Alpha channels
        //-------------------------------------------------------------
        data[pixel_index] = ((color >> 24) & 0xff) as u8; // Red
        data[pixel_index + 1] = ((color >> 16) & 0xff) as u8; // Green
        data[pixel_index + 2] = ((color >> 8) & 0xff) as u8; // Blue
        data[pixel_index + 3] = (color & 0xff) as u8;

        x += 1;

        if x == end_x || x == canvas_w {
            y += 1;
            x = start_x;
        };

        if y == end_y || y == canvas_h {
            break;
        }
    }

    // Create a new ImageData object from the modified data
    let new_image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut data),
        canvas.width(),
        canvas.height(),
    )
    .expect("Should be able to create new image data");

    // Put the modified ImageData back onto the canvas
    context
        .put_image_data(&new_image_data, 0.0, 0.0)
        .expect("Should be able to put image data back");
}
