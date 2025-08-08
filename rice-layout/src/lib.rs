mod alignment;
mod arena;
mod div;
mod gap;
mod insets;
mod layout;
mod rect;
mod size;

pub use alignment::{AlignmentH, AlignmentV};
pub use arena::Arena;
pub use div::Div;
pub use gap::Gap;
pub use insets::Insets;
pub use layout::Layout;
pub use rect::Rect;
pub use size::Size;

#[cfg(test)]
mod tests {
    use crate::{AlignmentH, AlignmentV, Gap, Insets, arena::Arena, div::Div, size::Size};

    /// Fixed sizes to test
    static SIZES: &'static [i32] = &[0, 100, 200];

    /// Fixed expandable fractions to test
    static FR: &'static [f32] = &[0.0, 0.5, 1.0];

    #[test]
    fn test_fixed_sizes() {
        for &width in SIZES {
            for &height in SIZES {
                let mut arena = Arena::new();
                let div = Div::new(Size::Fixed(width), Size::Fixed(height));
                let key = arena.compute(&div);
                let root = &arena.nodes[key];
                assert_eq!(root.width, width);
                assert_eq!(root.height, height);
            }
        }
    }

    #[test]
    fn test_fit_children() {
        for &width in SIZES {
            for &height in SIZES {
                let mut arena = Arena::new();
                let div = Div::new(Size::Fit, Size::Fit)
                    .children(vec![Div::new(Size::Fixed(width), Size::Fixed(height))]);
                let key = arena.compute(&div);
                let root = &arena.nodes[key];
                assert_eq!(root.width, width);
                assert_eq!(root.height, height);
            }
        }
    }

    #[test]
    fn test_margins() {
        for &margin_top in SIZES {
            for &margin_right in SIZES {
                for &margin_left in SIZES {
                    for &margin_bottom in SIZES {
                        let mut arena = Arena::new();
                        let div = Div::new(Size::Fit, Size::Fit).children(vec![
                            Div::new(Size::Fixed(100), Size::Fixed(100)).margin(Insets {
                                left: margin_left,
                                top: margin_top,
                                right: margin_right,
                                bottom: margin_bottom,
                            }),
                        ]);
                        let key = arena.compute(&div);
                        let root = &arena.nodes[key];
                        assert_eq!(root.width, 100 + margin_left + margin_right);
                        assert_eq!(root.height, 100 + margin_top + margin_bottom);
                    }
                }
            }
        }
    }

    #[test]
    fn test_padding() {
        for &padding_top in SIZES {
            for &padding_right in SIZES {
                for &padding_left in SIZES {
                    for &padding_bottom in SIZES {
                        let mut arena = Arena::new();
                        let div = Div::new(Size::Fit, Size::Fit)
                            .children(vec![Div::new(Size::Fixed(100), Size::Fixed(100))])
                            .padding(Insets {
                                left: padding_left,
                                top: padding_top,
                                right: padding_right,
                                bottom: padding_bottom,
                            });
                        let key = arena.compute(&div);
                        let root = &arena.nodes[key];
                        assert_eq!(root.width, 100 + padding_left + padding_right);
                        assert_eq!(root.height, 100 + padding_top + padding_bottom);
                    }
                }
            }
        }
    }

    #[test]
    fn test_fit_vertical() {
        for &width1 in SIZES {
            for &width2 in SIZES {
                for &height1 in SIZES {
                    for &height2 in SIZES {
                        let mut arena = Arena::new();
                        let div = Div::new(Size::Fit, Size::Fit)
                            .children(vec![
                                Div::new(Size::Fixed(width1), Size::Fixed(height1)),
                                Div::new(Size::Fixed(width2), Size::Fixed(height2)),
                            ])
                            .vertical(AlignmentV::Left);

                        let key = arena.compute(&div);
                        let root = &arena.nodes[key];
                        assert_eq!(root.width, width1.max(width2)); // Fit width should be max of children
                        assert_eq!(root.height, height1 + height2); // Fit height should be sum of children
                    }
                }
            }
        }
    }

    #[test]
    fn test_fit_horizontal() {
        for &width1 in SIZES {
            for &width2 in SIZES {
                for &height1 in SIZES {
                    for &height2 in SIZES {
                        let mut arena = Arena::new();
                        let div = Div::new(Size::Fit, Size::Fit)
                            .children(vec![
                                Div::new(Size::Fixed(width1), Size::Fixed(height1)),
                                Div::new(Size::Fixed(width2), Size::Fixed(height2)),
                            ])
                            .horizontal(AlignmentH::Top);

                        let key = arena.compute(&div);
                        let root = &arena.nodes[key];
                        assert_eq!(root.width, width1 + width2); // Fit width should be sum of children
                        assert_eq!(root.height, height1.max(height2)); // Fit height should be max of children
                    }
                }
            }
        }
    }

    #[test]
    fn test_positions_vertical() {
        for &width1 in SIZES {
            for &width2 in SIZES {
                for &height1 in SIZES {
                    for &height2 in SIZES {
                        for &margin in SIZES {
                            for &padding in SIZES {
                                for &gap in SIZES {
                                    for align in
                                        [AlignmentV::Left, AlignmentV::Center, AlignmentV::Right]
                                    {
                                        let mut arena = Arena::new();

                                        let div = Div::default()
                                            .padding(Insets::uniform(padding))
                                            .children(vec![
                                                Div::new(Size::Fixed(width1), Size::Fixed(height1))
                                                    .margin(Insets::uniform(margin)),
                                                Div::new(Size::Fixed(width2), Size::Fixed(height2))
                                                    .margin(Insets::uniform(margin)),
                                            ])
                                            .gap(Gap::Fixed(gap))
                                            .vertical(align);

                                        let key = arena.compute(&div);
                                        let root = &arena.nodes[key];
                                        let child1 = &arena.nodes[arena.children[key][0]];
                                        let child2 = &arena.nodes[arena.children[key][1]];

                                        // Assert parent size
                                        assert_eq!(
                                            root.width,
                                            width1.max(width2) + 2 * margin + 2 * padding
                                        );
                                        assert_eq!(
                                            root.height,
                                            height1 + height2 + 4 * margin + 2 * padding + gap
                                        );

                                        // Assert child 1 position
                                        let expected_x1 = match align {
                                            AlignmentV::Left => margin + padding,
                                            AlignmentV::Center => {
                                                (root.width - width1 - 2 * margin - 2 * padding) / 2
                                                    + margin
                                                    + padding
                                            }
                                            AlignmentV::Right => {
                                                root.width - width1 - margin - padding
                                            }
                                        };

                                        assert_eq!(child1.x, expected_x1);
                                        assert_eq!(child1.y, margin + padding);

                                        // Assert child 2 position (below child 1)
                                        let expected_x2 = match align {
                                            AlignmentV::Left => margin + padding,
                                            AlignmentV::Center => {
                                                (root.width - width2 - 2 * margin - 2 * padding) / 2
                                                    + margin
                                                    + padding
                                            }
                                            AlignmentV::Right => {
                                                root.width - width2 - margin - padding
                                            }
                                        };

                                        assert_eq!(child2.x, expected_x2);
                                        assert_eq!(child2.y, child1.y + height1 + 2 * margin + gap);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_positions_horizontal() {
        for &width1 in SIZES {
            for &width2 in SIZES {
                for &height1 in SIZES {
                    for &height2 in SIZES {
                        for &margin in SIZES {
                            for &padding in SIZES {
                                for &gap in SIZES {
                                    for align in
                                        [AlignmentH::Top, AlignmentH::Center, AlignmentH::Bottom]
                                    {
                                        let mut arena = Arena::new();

                                        let div = Div::default()
                                            .padding(Insets::uniform(padding))
                                            .children(vec![
                                                Div::new(Size::Fixed(width1), Size::Fixed(height1))
                                                    .margin(Insets::uniform(margin)),
                                                Div::new(Size::Fixed(width2), Size::Fixed(height2))
                                                    .margin(Insets::uniform(margin)),
                                            ])
                                            .gap(Gap::Fixed(gap))
                                            .horizontal(align);

                                        let key = arena.compute(&div);
                                        let root = &arena.nodes[key];
                                        let child1 = &arena.nodes[arena.children[key][0]];
                                        let child2 = &arena.nodes[arena.children[key][1]];

                                        // Assert parent size
                                        assert_eq!(
                                            root.width,
                                            width1 + width2 + 4 * margin + 2 * padding + gap
                                        );
                                        assert_eq!(
                                            root.height,
                                            height1.max(height2) + 2 * margin + 2 * padding
                                        );

                                        // Assert child 1 position
                                        let expected_y1 = match align {
                                            AlignmentH::Top => margin + padding,
                                            AlignmentH::Center => {
                                                (root.height - height1 - 2 * margin - 2 * padding)
                                                    / 2
                                                    + margin
                                                    + padding
                                            }
                                            AlignmentH::Bottom => {
                                                root.height - height1 - margin - padding
                                            }
                                        };

                                        assert_eq!(child1.x, margin + padding);
                                        assert_eq!(child1.y, expected_y1);

                                        // Assert child 2 position (to the right of child 1)
                                        let expected_y2 = match align {
                                            AlignmentH::Top => margin + padding,
                                            AlignmentH::Center => {
                                                (root.height - height2 - 2 * margin - 2 * padding)
                                                    / 2
                                                    + margin
                                                    + padding
                                            }
                                            AlignmentH::Bottom => {
                                                root.height - height2 - margin - padding
                                            }
                                        };
                                        assert_eq!(child2.x, child1.x + width1 + 2 * margin + gap);
                                        assert_eq!(child2.y, expected_y2);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_auto_gap() {
        // Gap in vertical layout
        for &padding in SIZES {
            for &margin in SIZES {
                for &height1 in SIZES {
                    for &height2 in SIZES {
                        let mut arena = Arena::new();

                        let div = Div::default()
                            .height(Size::Fixed(1_600))
                            .padding(Insets::uniform(padding))
                            .children(vec![
                                Div::new(Size::Fit, Size::Fixed(height1))
                                    .margin(Insets::uniform(margin)),
                                Div::new(Size::Fit, Size::Fixed(height2))
                                    .margin(Insets::uniform(margin)),
                            ])
                            .gap(Gap::Auto)
                            .vertical(AlignmentV::Left);

                        let key = arena.compute(&div);
                        let child2 = &arena.nodes[arena.children[key][1]];

                        assert_eq!(
                            child2.y,
                            padding
                                + margin
                                + height1
                                + margin
                                + (1_600 - 2 * padding - 2 * margin - height1 - height2) / 2
                        );
                    }
                }
            }
        }

        // Gap in horizontal layout
        for &padding in SIZES {
            for &margin in SIZES {
                for &width1 in SIZES {
                    for &width2 in SIZES {
                        let mut arena = Arena::new();

                        let div = Div::default()
                            .width(Size::Fixed(1_600))
                            .padding(Insets::uniform(padding))
                            .children(vec![
                                Div::new(Size::Fixed(width1), Size::Fit)
                                    .margin(Insets::uniform(margin)),
                                Div::new(Size::Fixed(width2), Size::Fit)
                                    .margin(Insets::uniform(margin)),
                            ])
                            .gap(Gap::Auto)
                            .horizontal(AlignmentH::Top);

                        let key = arena.compute(&div);
                        let child2 = &arena.nodes[arena.children[key][1]];

                        assert_eq!(
                            child2.x,
                            padding
                                + margin
                                + width1
                                + margin
                                + (1_600 - 2 * padding - 2 * margin - width1 - width2) / 2
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_expandable() {
        // Test expansion in vertical layout
        for &margin in SIZES {
            for &padding in SIZES {
                for &gap in SIZES {
                    for &fr1 in FR {
                        for &fr2 in FR {
                            let mut arena = Arena::new();
                            let div = Div::new(Size::Fixed(1_600), Size::Fixed(1_600))
                                .children(vec![
                                    Div::new(Size::Expand(fr1), Size::Expand(fr1))
                                        .margin(Insets::uniform(margin)),
                                    Div::new(Size::Expand(fr2), Size::Expand(fr2))
                                        .margin(Insets::uniform(margin)),
                                ])
                                .padding(Insets::uniform(padding))
                                .vertical(AlignmentV::Left)
                                .gap(Gap::Fixed(gap));

                            let key = arena.compute(&div);

                            let available = (1_600 - 4 * margin - 2 * padding - gap) as f32;
                            let total_fr = fr1 + fr2;

                            let child1 = &arena.nodes[arena.children[key][0]];
                            let child2 = &arena.nodes[arena.children[key][1]];

                            assert_eq!(child1.width, 1_600 - 2 * margin - 2 * padding);
                            assert_eq!(child2.width, 1_600 - 2 * margin - 2 * padding);

                            assert_eq!(child1.height, (available * fr1 / total_fr).round() as i32);
                            assert_eq!(child2.height, (available * fr2 / total_fr).round() as i32);
                        }
                    }
                }
            }
        }

        // Test expansion in horizontal layout
        for &margin in SIZES {
            for &padding in SIZES {
                for &gap in SIZES {
                    for &fr1 in FR {
                        for &fr2 in FR {
                            let mut arena = Arena::new();
                            let div = Div::new(Size::Fixed(1_600), Size::Fixed(1_600))
                                .children(vec![
                                    Div::new(Size::Expand(fr1), Size::Expand(fr1))
                                        .margin(Insets::uniform(margin)),
                                    Div::new(Size::Expand(fr2), Size::Expand(fr2))
                                        .margin(Insets::uniform(margin)),
                                ])
                                .padding(Insets::uniform(padding))
                                .horizontal(AlignmentH::Top)
                                .gap(Gap::Fixed(gap));

                            let key = arena.compute(&div);

                            let available = (1_600 - 4 * margin - 2 * padding - gap) as f32;
                            let total_fr = fr1 + fr2;

                            let child1 = &arena.nodes[arena.children[key][0]];
                            let child2 = &arena.nodes[arena.children[key][1]];

                            assert_eq!(child1.height, 1_600 - 2 * margin - 2 * padding);
                            assert_eq!(child2.height, 1_600 - 2 * margin - 2 * padding);

                            assert_eq!(child1.width, (available * fr1 / total_fr).round() as i32);
                            assert_eq!(child2.width, (available * fr2 / total_fr).round() as i32);
                        }
                    }
                }
            }
        }
    }
}
