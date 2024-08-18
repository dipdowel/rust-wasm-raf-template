
/// A wrapper around the browser's `console.log()`
pub fn log(s: &str) {
    web_sys::console::log_1(&s.into());
}