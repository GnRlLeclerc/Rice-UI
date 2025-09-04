//! 2D layout engine

mod arena;
mod direction;
mod gap;
mod insets;
mod layout;
mod rect;
mod size;
mod utils;

pub use arena::{Arena, compute_layout};
pub use direction::{Align, Direction};
pub use gap::Gap;
pub use insets::Insets;
pub use layout::Layout;
pub use rect::Rect;
pub use size::Size;

#[cfg(test)]
mod tests {
    use crate::*;

    /// Fixed sizes to test
    static SIZES: &'static [i32] = &[0, 100, 200];

    /// Fixed expandable fractions to test
    static FR: &'static [f32] = &[0.0, 0.5, 1.0];

    #[test]
    fn test_fixed_sizes() {
        for &width in SIZES {
            for &height in SIZES {
                let mut arena = Arena::new();
                let layout = Layout::new(Size::Fixed(width), Size::Fixed(height));
                let key = arena.insert(layout);
                arena.compute_layout(key);
                let root = &arena.rects[key];
                assert_eq!(root.width(), width);
                assert_eq!(root.height(), height);
            }
        }
    }

    #[test]
    fn test_fit_children() {
        for &width in SIZES {
            for &height in SIZES {
                let mut arena = Arena::new();
                let root = arena.insert(Layout::new(Size::Fit, Size::Fit));
                arena.insert_child(Layout::new(Size::Fixed(width), Size::Fixed(height)), root);
                arena.compute_layout(root);
                let root = &arena.rects[root];
                assert_eq!(root.width(), width);
                assert_eq!(root.height(), height);
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
                        let root = arena.insert(Layout::new(Size::Fit, Size::Fit));
                        arena.insert_child(
                            Layout::new(Size::Fixed(100), Size::Fixed(100)).margin(Insets::new(
                                margin_top,
                                margin_bottom,
                                margin_left,
                                margin_right,
                            )),
                            root,
                        );
                        arena.compute_layout(root);

                        let root = &arena.rects[root];
                        assert_eq!(root.width(), 100 + margin_left + margin_right);
                        assert_eq!(root.height(), 100 + margin_top + margin_bottom);
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
                        let root = arena.insert(Layout::new(Size::Fit, Size::Fit).padding(
                            Insets::new(padding_top, padding_bottom, padding_left, padding_right),
                        ));
                        arena.insert_child(Layout::new(Size::Fixed(100), Size::Fixed(100)), root);
                        arena.compute_layout(root);
                        let root = &arena.rects[root];
                        assert_eq!(root.width(), 100 + padding_left + padding_right);
                        assert_eq!(root.height(), 100 + padding_top + padding_bottom);
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
                        let root =
                            arena.insert(Layout::new(Size::Fit, Size::Fit).vertical(Align::Start));
                        arena.insert_child(
                            Layout::new(Size::Fixed(width1), Size::Fixed(height1)),
                            root,
                        );
                        arena.insert_child(
                            Layout::new(Size::Fixed(width2), Size::Fixed(height2)),
                            root,
                        );
                        arena.compute_layout(root);

                        let root = &arena.rects[root];
                        assert_eq!(root.width(), width1.max(width2)); // Fit width should be max of children
                        assert_eq!(root.height(), height1 + height2); // Fit height should be sum of children
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
                        let root = arena
                            .insert(Layout::new(Size::Fit, Size::Fit).horizontal(Align::Start));
                        arena.insert_child(
                            Layout::new(Size::Fixed(width1), Size::Fixed(height1)),
                            root,
                        );
                        arena.insert_child(
                            Layout::new(Size::Fixed(width2), Size::Fixed(height2)),
                            root,
                        );
                        arena.compute_layout(root);
                        let root = &arena.rects[root];
                        assert_eq!(root.width(), width1 + width2); // Fit width should be sum of children
                        assert_eq!(root.height(), height1.max(height2)); // Fit height should be max of children
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
                                    for align in [Align::Start, Align::Center, Align::End] {
                                        let mut arena = Arena::new();
                                        let root = arena.insert(
                                            Layout::default()
                                                .padding(Insets::uniform(padding))
                                                .gap(Gap::Fixed(gap))
                                                .vertical(align),
                                        );

                                        let child1 = arena.insert_child(
                                            Layout::new(Size::Fixed(width1), Size::Fixed(height1))
                                                .margin(Insets::uniform(margin)),
                                            root,
                                        );
                                        let child2 = arena.insert_child(
                                            Layout::new(Size::Fixed(width2), Size::Fixed(height2))
                                                .margin(Insets::uniform(margin)),
                                            root,
                                        );

                                        arena.compute_layout(root);
                                        let root = &arena.rects[root];
                                        let child1 = &arena.rects[child1];
                                        let child2 = &arena.rects[child2];

                                        // Assert parent size
                                        assert_eq!(
                                            root.width(),
                                            width1.max(width2) + 2 * margin + 2 * padding
                                        );
                                        assert_eq!(
                                            root.height(),
                                            height1 + height2 + 4 * margin + 2 * padding + gap
                                        );

                                        // Assert child 1 position
                                        let expected_x1 = match align {
                                            Align::Start => margin + padding,
                                            Align::Center => {
                                                (root.width() - width1 - 2 * margin - 2 * padding)
                                                    / 2
                                                    + margin
                                                    + padding
                                            }
                                            Align::End => root.width() - width1 - margin - padding,
                                        };

                                        assert_eq!(child1.x(), expected_x1);
                                        assert_eq!(child1.y(), margin + padding);

                                        // Assert child 2 position (below child 1)
                                        let expected_x2 = match align {
                                            Align::Start => margin + padding,
                                            Align::Center => {
                                                (root.width() - width2 - 2 * margin - 2 * padding)
                                                    / 2
                                                    + margin
                                                    + padding
                                            }
                                            Align::End => root.width() - width2 - margin - padding,
                                        };

                                        assert_eq!(child2.x(), expected_x2);
                                        assert_eq!(
                                            child2.y(),
                                            child1.y() + height1 + 2 * margin + gap
                                        );
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
                                    for align in [Align::Start, Align::Center, Align::End] {
                                        let mut arena = Arena::new();
                                        let root = arena.insert(
                                            Layout::default()
                                                .padding(Insets::uniform(padding))
                                                .gap(Gap::Fixed(gap))
                                                .horizontal(align),
                                        );
                                        let child1 = arena.insert_child(
                                            Layout::new(Size::Fixed(width1), Size::Fixed(height1))
                                                .margin(Insets::uniform(margin)),
                                            root,
                                        );
                                        let child2 = arena.insert_child(
                                            Layout::new(Size::Fixed(width2), Size::Fixed(height2))
                                                .margin(Insets::uniform(margin)),
                                            root,
                                        );

                                        arena.compute_layout(root);
                                        let root = &arena.rects[root];
                                        let child1 = &arena.rects[child1];
                                        let child2 = &arena.rects[child2];

                                        // Assert parent size
                                        assert_eq!(
                                            root.width(),
                                            width1 + width2 + 4 * margin + 2 * padding + gap
                                        );
                                        assert_eq!(
                                            root.height(),
                                            height1.max(height2) + 2 * margin + 2 * padding
                                        );

                                        // Assert child 1 position
                                        let expected_y1 = match align {
                                            Align::Start => margin + padding,
                                            Align::Center => {
                                                (root.height() - height1 - 2 * margin - 2 * padding)
                                                    / 2
                                                    + margin
                                                    + padding
                                            }
                                            Align::End => {
                                                root.height() - height1 - margin - padding
                                            }
                                        };

                                        assert_eq!(child1.x(), margin + padding);
                                        assert_eq!(child1.y(), expected_y1);

                                        // Assert child 2 position (to the right of child 1)
                                        let expected_y2 = match align {
                                            Align::Start => margin + padding,
                                            Align::Center => {
                                                (root.height() - height2 - 2 * margin - 2 * padding)
                                                    / 2
                                                    + margin
                                                    + padding
                                            }
                                            Align::End => {
                                                root.height() - height2 - margin - padding
                                            }
                                        };
                                        assert_eq!(
                                            child2.x(),
                                            child1.x() + width1 + 2 * margin + gap
                                        );
                                        assert_eq!(child2.y(), expected_y2);
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
                        let root = arena.insert(
                            Layout::new(Size::Fit, Size::Fixed(1_600))
                                .padding(Insets::uniform(padding))
                                .gap(Gap::Auto)
                                .vertical(Align::Start),
                        );
                        arena.insert_child(
                            Layout::new(Size::Fit, Size::Fixed(height1))
                                .margin(Insets::uniform(margin)),
                            root,
                        );
                        let child2 = arena.insert_child(
                            Layout::new(Size::Fit, Size::Fixed(height2))
                                .margin(Insets::uniform(margin)),
                            root,
                        );

                        arena.compute_layout(root);
                        let child2 = &arena.rects[child2];

                        assert_eq!(
                            child2.y(),
                            padding
                                + 2 * margin
                                + height1
                                + margin
                                + (1_600 - 2 * padding - 4 * margin - height1 - height2)
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
                        let root = arena.insert(
                            Layout::new(Size::Fixed(1_600), Size::Fit)
                                .padding(Insets::uniform(padding))
                                .gap(Gap::Auto)
                                .horizontal(Align::Start),
                        );
                        arena.insert_child(
                            Layout::new(Size::Fixed(width1), Size::Fit)
                                .margin(Insets::uniform(margin)),
                            root,
                        );

                        let child2 = arena.insert_child(
                            Layout::new(Size::Fixed(width2), Size::Fit)
                                .margin(Insets::uniform(margin)),
                            root,
                        );

                        arena.compute_layout(root);
                        let child2 = &arena.rects[child2];

                        assert_eq!(
                            child2.x(),
                            padding
                                + 2 * margin
                                + width1
                                + margin
                                + (1_600 - 2 * padding - 4 * margin - width1 - width2)
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
                            let root = arena.insert(
                                Layout::new(Size::Fixed(1_600), Size::Fixed(1_600))
                                    .padding(Insets::uniform(padding))
                                    .gap(Gap::Fixed(gap))
                                    .vertical(Align::Start),
                            );
                            let child1 = arena.insert_child(
                                Layout::new(Size::Expand(fr1), Size::Expand(fr1))
                                    .margin(Insets::uniform(margin)),
                                root,
                            );
                            let child2 = arena.insert_child(
                                Layout::new(Size::Expand(fr2), Size::Expand(fr2))
                                    .margin(Insets::uniform(margin)),
                                root,
                            );

                            arena.compute_layout(root);

                            let available = (1_600 - 4 * margin - 2 * padding - gap) as f32;
                            let total_fr = fr1 + fr2;

                            let child1 = &arena.rects[child1];
                            let child2 = &arena.rects[child2];

                            assert_eq!(child1.width(), 1_600 - 2 * margin - 2 * padding);
                            assert_eq!(child2.width(), 1_600 - 2 * margin - 2 * padding);

                            assert_eq!(
                                child1.height(),
                                (available * fr1 / total_fr).round() as i32
                            );
                            assert_eq!(
                                child2.height(),
                                (available * fr2 / total_fr).round() as i32
                            );
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
                            let root = arena.insert(
                                Layout::new(Size::Fixed(1_600), Size::Fixed(1_600))
                                    .padding(Insets::uniform(padding))
                                    .gap(Gap::Fixed(gap))
                                    .horizontal(Align::Start),
                            );
                            let child1 = arena.insert_child(
                                Layout::new(Size::Expand(fr1), Size::Expand(fr1))
                                    .margin(Insets::uniform(margin)),
                                root,
                            );
                            let child2 = arena.insert_child(
                                Layout::new(Size::Expand(fr2), Size::Expand(fr2))
                                    .margin(Insets::uniform(margin)),
                                root,
                            );

                            arena.compute_layout(root);

                            let available = (1_600 - 4 * margin - 2 * padding - gap) as f32;
                            let total_fr = fr1 + fr2;

                            let child1 = &arena.rects[child1];
                            let child2 = &arena.rects[child2];

                            assert_eq!(child1.height(), 1_600 - 2 * margin - 2 * padding);
                            assert_eq!(child2.height(), 1_600 - 2 * margin - 2 * padding);

                            assert_eq!(child1.width(), (available * fr1 / total_fr).round() as i32);
                            assert_eq!(child2.width(), (available * fr2 / total_fr).round() as i32);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_minmax_widths() {
        // In a vertical layout
        let mut arena = Arena::new();
        let root = arena.insert(Layout::new(Size::Fixed(1000), Size::Fit));
        let child = arena.insert_child(
            Layout::new(Size::Expand(1.0), Size::Fit).max_width(500),
            root,
        );
        arena.compute_layout(root);
        let child = &arena.rects[child];

        assert_eq!(child.width(), 500); // Should respect max_width

        let root = arena.insert(Layout::default());
        let child = arena.insert_child(
            Layout::new(Size::Expand(1.0), Size::Fit).min_width(500),
            root,
        );
        arena.compute_layout(root);

        let child = &arena.rects[child];
        assert_eq!(child.width(), 500); // Should respect min_width

        let root = arena.insert(Layout::new(Size::Fixed(500), Size::Fit));
        let child = arena.insert_child(
            Layout::new(Size::Expand(1.0), Size::Fit)
                .min_width(0)
                .max_width(1000),
            root,
        );
        arena.compute_layout(root);
        let child = &arena.rects[child];
        assert_eq!(child.width(), 500); // Should respect parent's fixed width

        // In a horizontal layout
        let root = arena.insert(Layout::new(Size::Fixed(1000), Size::Fit).horizontal(Align::Start));
        let child1 = arena.insert_child(
            Layout::new(Size::Expand(2.0), Size::Fit).max_width(400),
            root,
        );
        let child2 = arena.insert_child(Layout::new(Size::Expand(1.0), Size::Fit), root);
        arena.compute_layout(root);
        let child1 = &arena.rects[child1];
        let child2 = &arena.rects[child2];

        assert_eq!(child1.width(), 400); // Should respect max_width
        assert_eq!(child2.width(), 600); // Remaining space after child1

        let root = arena.insert(Layout::new(Size::Fixed(1000), Size::Fit).horizontal(Align::Start));
        let child1 = arena.insert_child(Layout::new(Size::Expand(2.0), Size::Fit), root);
        let child2 = arena.insert_child(
            Layout::new(Size::Expand(1.0), Size::Fit).min_width(500),
            root,
        );
        arena.compute_layout(root);
        let child1 = &arena.rects[child1];
        let child2 = &arena.rects[child2];

        assert_eq!(child1.width(), 500); // Should respect min_width
        assert_eq!(child2.width(), 500); // Should take remaining space
    }

    #[test]
    fn test_minmax_heights() {
        // In a horizontal layout
        let mut arena = Arena::new();
        let root = arena.insert(Layout::new(Size::Fit, Size::Fixed(1000)));
        let child = arena.insert_child(
            Layout::new(Size::Fit, Size::Expand(1.0)).max_height(500),
            root,
        );
        arena.compute_layout(root);
        let child = &arena.rects[child];
        assert_eq!(child.height(), 500); // Should respect max_height

        let root = arena.insert(Layout::default().horizontal(Align::Start));
        let child = arena.insert_child(
            Layout::new(Size::Fit, Size::Expand(1.0)).min_height(500),
            root,
        );
        arena.compute_layout(root);
        let child = &arena.rects[child];
        assert_eq!(child.height(), 500); // Should respect min_height

        let root = arena.insert(Layout::new(Size::Fit, Size::Fixed(500)).horizontal(Align::Start));
        let child = arena.insert_child(
            Layout::new(Size::Fit, Size::Expand(1.0))
                .min_height(0)
                .max_height(1000),
            root,
        );
        arena.compute_layout(root);
        let child = &arena.rects[child];
        assert_eq!(child.height(), 500); // Should respect parent's fixed height

        // In a vertical layout
        let root = arena.insert(Layout::new(Size::Fit, Size::Fixed(1000)).vertical(Align::Start));
        let child1 = arena.insert_child(
            Layout::new(Size::Fit, Size::Expand(2.0)).max_height(400),
            root,
        );
        let child2 = arena.insert_child(Layout::new(Size::Fit, Size::Expand(1.0)), root);
        arena.compute_layout(root);

        let child1 = &arena.rects[child1];
        let child2 = &arena.rects[child2];

        assert_eq!(child1.height(), 400); // Should respect max_height
        assert_eq!(child2.height(), 600); // Remaining space after child1

        let root = arena.insert(Layout::new(Size::Fit, Size::Fixed(1000)).vertical(Align::Start));
        let child1 = arena.insert_child(Layout::new(Size::Fit, Size::Expand(2.0)), root);
        let child2 = arena.insert_child(
            Layout::new(Size::Fit, Size::Expand(1.0)).min_height(500),
            root,
        );
        arena.compute_layout(root);

        let child1 = &arena.rects[child1];
        let child2 = &arena.rects[child2];

        assert_eq!(child1.height(), 500); // Should respect min_height
        assert_eq!(child2.height(), 500); // Should take remaining space
    }
}
