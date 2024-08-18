/**
     NB: There primitive types were copied from the `graph1` crate.
    @See [Graph1 on github](https://github.com/dipdowel/graph1)
*/

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct RectArea {
    pub top_left: Point,
    pub dimensions: Dimensions2d,
}

#[derive(Clone, Copy, Debug)]
pub struct Dimensions2d {
    pub w: u32,
    pub h: u32,
}

