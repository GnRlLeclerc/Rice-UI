pub mod arena;
pub mod div;
pub mod layout;
pub mod rect;
pub mod size;

#[cfg(test)]
mod tests {
    use crate::{arena::Arena, div::Div, size::Size};

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
            .vertical();

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
            .horizontal();

        let key = arena.compute(&div);
        let root = &arena.nodes[key];
        assert_eq!(root.width, 150); // Should be the sum of children widths
        let child1 = &arena.nodes[arena.children[key][0]];
        assert_eq!(child1.width, 50);
        let child2 = &arena.nodes[arena.children[key][1]];
        assert_eq!(child2.width, 100);
    }
}
