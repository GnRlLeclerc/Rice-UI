mod alignment;
mod arena;
mod div;
mod insets;
mod layout;
mod rect;
mod size;

pub use alignment::{AlignmentH, AlignmentV};
pub use arena::Arena;
pub use div::Div;
pub use insets::Insets;
pub use layout::Layout;
pub use rect::Rect;
pub use size::Size;

#[cfg(test)]
mod tests {
    use crate::{AlignmentH, AlignmentV, Insets, arena::Arena, div::Div, size::Size};

    #[test]
    fn test_div() {
        let div = Div::default();
        assert_eq!(div.height, Size::Fit);
        assert_eq!(div.width, Size::Fit);
    }

    #[test]
    fn test_fixed_widths() {
        let mut arena = Arena::new();
        let div = Div::new(Size::Fixed(100), Size::Fit).with_children(vec![
            Div::new(Size::Fixed(50), Size::Fixed(100)),
            Div::new(Size::Fixed(100), Size::Fixed(200)),
        ]);

        let key = arena.compute(&div);
        let root = &arena.nodes[key];
        assert_eq!(root.width, 100);
        let child1 = &arena.nodes[arena.children[key][0]];
        assert_eq!(child1.width, 50);
        let child2 = &arena.nodes[arena.children[key][1]];
        assert_eq!(child2.width, 100);
    }

    #[test]
    fn test_fit_widths_vertical() {
        let mut arena = Arena::new();
        let div = Div::new(Size::Fit, Size::Fit)
            .with_children(vec![
                Div::new(Size::Fixed(50), Size::Fixed(100)),
                Div::new(Size::Fixed(100), Size::Fixed(200)),
            ])
            .vertical(AlignmentV::Left);

        let key = arena.compute(&div);
        let root = &arena.nodes[key];
        assert_eq!(root.width, 100); // Should be the max of children widths
        let child1 = &arena.nodes[arena.children[key][0]];
        assert_eq!(child1.width, 50);
        let child2 = &arena.nodes[arena.children[key][1]];
        assert_eq!(child2.width, 100);
    }

    #[test]
    fn test_fit_widths_horizontal() {
        let mut arena = Arena::new();
        let div = Div::new(Size::Fit, Size::Fit)
            .with_children(vec![
                Div::new(Size::Fixed(50), Size::Fixed(100)),
                Div::new(Size::Fixed(100), Size::Fixed(200)),
            ])
            .horizontal(AlignmentH::Top);

        let key = arena.compute(&div);
        let root = &arena.nodes[key];
        assert_eq!(root.width, 150); // Should be the sum of children widths
        let child1 = &arena.nodes[arena.children[key][0]];
        assert_eq!(child1.width, 50);
        let child2 = &arena.nodes[arena.children[key][1]];
        assert_eq!(child2.width, 100);
    }

    #[test]
    fn test_fit_heights_vertical() {
        let mut arena = Arena::new();
        let div = Div::new(Size::Fit, Size::Fit)
            .with_children(vec![
                Div::new(Size::Fixed(50), Size::Fixed(100)),
                Div::new(Size::Fixed(100), Size::Fixed(200)),
            ])
            .vertical(AlignmentV::Left);

        let key = arena.compute(&div);
        let root = &arena.nodes[key];
        assert_eq!(root.height, 300); // Should be the sum of children heights
        let child1 = &arena.nodes[arena.children[key][0]];
        assert_eq!(child1.height, 100);
        let child2 = &arena.nodes[arena.children[key][1]];
        assert_eq!(child2.height, 200);
    }

    #[test]
    fn test_fit_heights_horizontal() {
        let mut arena = Arena::new();
        let div = Div::new(Size::Fit, Size::Fit)
            .with_children(vec![
                Div::new(Size::Fixed(50), Size::Fixed(100)),
                Div::new(Size::Fixed(100), Size::Fixed(200)),
            ])
            .horizontal(AlignmentH::Top);

        let key = arena.compute(&div);
        let root = &arena.nodes[key];
        assert_eq!(root.height, 200); // Should be the max of children heights
        let child1 = &arena.nodes[arena.children[key][0]];
        assert_eq!(child1.height, 100);
        let child2 = &arena.nodes[arena.children[key][1]];
        assert_eq!(child2.height, 200);
    }

    #[test]
    fn test_margins() {
        let mut arena = Arena::new();
        let div = Div::new(Size::Fit, Size::Fit).with_children(vec![
            Div::new(Size::Fixed(50), Size::Fixed(50)).with_margin(Insets {
                left: 1,
                top: 2,
                right: 3,
                bottom: 4,
            }),
        ]);

        let key = arena.compute(&div);
        let root = &arena.nodes[key];

        assert_eq!(root.width, 54); // 50 + 1 + 3 = 54
        assert_eq!(root.height, 56); // 50 + 2 + 4 = 56
    }

    #[test]
    fn test_padding() {
        let mut arena = Arena::new();
        let div = Div::new(Size::Fit, Size::Fit)
            .with_children(vec![Div::new(Size::Fixed(50), Size::Fixed(50))])
            .with_padding(Insets {
                left: 1,
                top: 2,
                right: 3,
                bottom: 4,
            });

        let key = arena.compute(&div);
        let root = &arena.nodes[key];

        assert_eq!(root.width, 54); // 50 + 1 + 3 = 54
        assert_eq!(root.height, 56); // 50 + 2 + 4 = 56
    }

    #[test]
    fn test_borders() {
        let mut arena = Arena::new();
        let div = Div::new(Size::Fit, Size::Fit).with_children(vec![
            Div::new(Size::Fixed(50), Size::Fixed(50)).with_border(Insets {
                left: 1,
                top: 2,
                right: 3,
                bottom: 4,
            }),
        ]);

        let key = arena.compute(&div);
        let root = &arena.nodes[key];

        assert_eq!(root.width, 54); // 50 + 1 + 3 = 54
        assert_eq!(root.height, 56); // 50 + 2 + 4 = 56
    }

    #[test]
    fn test_align_vertical() {
        let mut arena = Arena::new();
        // Test centering without padding or margins
        let div = Div::new(Size::Fixed(100), Size::Fixed(100))
            .with_children(vec![Div::new(Size::Fixed(50), Size::Fixed(50))])
            .vertical(AlignmentV::Center);

        let key = arena.compute(&div);
        let root = &arena.nodes[key];
        assert_eq!(root.x, 0); // Parent position

        let child = &arena.nodes[arena.children[key][0]];
        assert_eq!(child.x, 25);
    }

    #[test]
    fn test_align_horizontal() {
        let mut arena = Arena::new();
        // Test centering without padding or margins
        let div = Div::new(Size::Fixed(100), Size::Fixed(100))
            .with_children(vec![Div::new(Size::Fixed(50), Size::Fixed(50))])
            .horizontal(AlignmentH::Center);

        let key = arena.compute(&div);
        let root = &arena.nodes[key];
        assert_eq!(root.y, 0); // Parent position

        let child = &arena.nodes[arena.children[key][0]];
        assert_eq!(child.y, 25);
    }
}
