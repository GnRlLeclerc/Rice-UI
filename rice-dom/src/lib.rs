//! DOM management crate

mod colors;
mod dom;
mod mouse;
mod styles;

pub use colors::Color;
pub use dom::DOM;
pub use styles::{ComputedStyle, StyleProp, StyleSheet, StyleValue};
