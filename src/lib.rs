use serde::{Deserialize, Serialize};

mod colour;
pub use colour::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Render {
    glyph: char,
    fg: String,
    bg: Option<String>,
    x: i32,
    y: i32,
}

pub type RenderVec = Vec<Render>;

pub trait Renderable {
    fn fg(&self) -> Colour;

    fn bg(&self) -> Option<Colour>;

    fn char(&self) -> char;

    fn make_render(&self, x: i32, y: i32) -> Render {
        Render {
            x,
            y,
            fg: self.fg().to_string(),
            bg: self.bg().map(|bg| bg.to_string()),
            glyph: self.char(),
        }
    }
}
