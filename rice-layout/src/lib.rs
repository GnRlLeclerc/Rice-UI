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
    use crate::{AlignmentH, AlignmentV, Insets, arena::Arena, div::Div, size::Size};

    /// Fixed sizes to test
    static SIZES: &'static [i32] = &[0, 100, 200, 300];

    #[test]
    fn test_fixed_sizes() {
        let mut arena = Arena::new();

        for &width in SIZES {
            for &height in SIZES {
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
        let mut arena = Arena::new();

        for &width in SIZES {
            for &height in SIZES {
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
        let mut arena = Arena::new();
        for &margin_top in SIZES {
            for &margin_right in SIZES {
                for &margin_left in SIZES {
                    for &margin_bottom in SIZES {
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
        let mut arena = Arena::new();
        for &padding_top in SIZES {
            for &padding_right in SIZES {
                for &padding_left in SIZES {
                    for &padding_bottom in SIZES {
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
        let mut arena = Arena::new();

        for &width1 in SIZES {
            for &width2 in SIZES {
                for &height1 in SIZES {
                    for &height2 in SIZES {
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
        let mut arena = Arena::new();

        for &width1 in SIZES {
            for &width2 in SIZES {
                for &height1 in SIZES {
                    for &height2 in SIZES {
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
}
