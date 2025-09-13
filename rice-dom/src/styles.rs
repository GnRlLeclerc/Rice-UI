//! Style rules for rendering components

use crate::dense::DenseMap;

use crate::Color;

/// Style rules for a component
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StyleSheet {
    pub default: DenseMap<StyleValue>,
    pub hovered: DenseMap<StyleValue>,
    pub clicked: DenseMap<StyleValue>,
}

/// Style properties enum
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleProp {
    BackgroundColor = 0,
}

impl TryFrom<u8> for StyleProp {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StyleProp::BackgroundColor),
            _ => Err(()),
        }
    }
}

/// Value for a style property
#[derive(Debug, Clone, PartialEq)]
pub enum StyleValue {
    Color(Color),
}

impl Default for StyleValue {
    fn default() -> Self {
        StyleValue::Color(Color::default())
    }
}

/// Computed style ready for rendering
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ComputedStyle {
    /// Background color
    pub background_color: Color,
}

impl StyleProp {
    /// Apply a style value to a computed style
    pub fn apply(&self, value: &StyleValue, style: &mut ComputedStyle) {
        match (self, value) {
            (StyleProp::BackgroundColor, StyleValue::Color(color)) => {
                style.background_color = color.clone();
            }
            _ => unreachable!("Mismatched style property and value"),
        }
    }

    /// Reset the corresponding computed style property to its default value
    pub fn reset(&self, style: &mut ComputedStyle) {
        match self {
            StyleProp::BackgroundColor => {
                style.background_color = Color::default();
            }
        }
    }
}

impl StyleSheet {
    /// Apply default styles to a computed style
    pub fn apply_default(&self, style: &mut ComputedStyle) {
        Self::apply_styles(&self.default, style);
    }
    /// Apply hover styles to a computed style
    pub fn apply_hovered(&self, style: &mut ComputedStyle) {
        Self::apply_styles(&self.hovered, style);
    }
    /// Apply clicked styles to a computed style
    pub fn apply_clicked(&self, style: &mut ComputedStyle) {
        Self::apply_styles(&self.clicked, style);
    }
    /// Reset hover styles to default
    pub fn reset_hover(&self, style: &mut ComputedStyle) {
        self.reset_styles(&self.hovered, style);
    }
    /// Reset clicked styles to default
    pub fn reset_clicked(&self, style: &mut ComputedStyle) {
        self.reset_styles(&self.clicked, style);
    }

    /// Apply all styles from a map to a computed style
    fn apply_styles(map: &DenseMap<StyleValue>, style: &mut ComputedStyle) {
        for (prop, value) in map.iter() {
            let prop = StyleProp::try_from(prop).expect("Invalid style property");
            prop.apply(value, style);
        }
    }

    /// Reset all style properties from a map to their default values, granularly.
    /// If a property does not exist in the default map, it is reset to the hardcoded default.
    fn reset_styles(&self, map: &DenseMap<StyleValue>, style: &mut ComputedStyle) {
        for (prop, _) in map.iter() {
            let prop = StyleProp::try_from(prop).expect("Invalid style property");
            if self.default.get(prop as u8).is_some() {
                // If the property exists in the default map, reset to that value
                if let Some(default_value) = self.default.get(prop as u8) {
                    prop.apply(default_value, style);
                }
            } else {
                // Otherwise, reset to hardcoded default
                prop.reset(style);
            }
        }
    }
}
