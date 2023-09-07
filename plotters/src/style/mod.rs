/*!
  The style for shapes and text, font, color, etc.
*/

#[cfg(all(feature = "ab_glyph", feature = "ttf"))]
compile_error!("features \"ab_glyph\" and \"ttf\" cannot be enabled at the same time");

mod color;
pub mod colors;
mod font;
mod palette;
mod shape;
mod size;
mod text;

/// Definitions of palettes of accessibility
pub use self::palette::*;
pub use color::{Color, HSLColor, PaletteColor, RGBAColor, RGBColor};
pub use colors::{BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, TRANSPARENT, WHITE, YELLOW};

#[cfg(feature = "full_palette")]
pub use colors::full_palette;

#[cfg(all(feature = "ab_glyph", not(feature = "ttf")))]
pub use font::register_font;
pub use font::{
    FontDesc, FontError, FontFamily, FontResult, FontStyle, FontTransform, IntoFont, LayoutBox,
};

pub use shape::ShapeStyle;
pub use size::{AsRelative, RelativeSize, SizeDesc};
pub use text::text_anchor;
pub use text::{IntoTextStyle, TextStyle};
