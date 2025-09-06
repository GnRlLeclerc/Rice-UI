//! Style rules for rendering components

use crate::Color;

/// Computed style ready for rendering
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Style {
    /// Background color
    pub background_color: Color,
}

/// Individual style rules
#[derive(Debug, Clone, PartialEq)]
pub enum StyleRule {
    BackgroundColor(Color),
}

/// Style rules that are applied to a style struct based on state
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StyleRules {
    pub default: Vec<StyleRule>,
    pub hovered: Vec<StyleRule>,
    pub clicked: Vec<StyleRule>,
}

impl StyleRule {
    /// Apply a style rule to a style struct
    pub fn apply(&self, style: &mut Style) {
        match self {
            StyleRule::BackgroundColor(color) => {
                style.background_color = color.clone();
            }
        }
    }
}

impl StyleRules {
    /// Apply default rules to a style struct
    pub fn apply_default(&self, style: &mut Style) {
        apply_rules(&self.default, style);
    }

    /// Apply hovered rules to a style struct
    pub fn apply_hovered(&self, style: &mut Style) {
        apply_rules(&self.hovered, style);
    }

    /// Apply clicked rules to a style struct
    pub fn apply_clicked(&self, style: &mut Style) {
        apply_rules(&self.clicked, style);
    }
}

// TODO: handle case where a hover or clicked rule is not present as a default,
// and must be "manually" reset back to default

fn apply_rules(rules: &[StyleRule], style: &mut Style) {
    for rule in rules {
        rule.apply(style);
    }
}
