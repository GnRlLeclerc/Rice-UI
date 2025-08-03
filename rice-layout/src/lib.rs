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
    fn test_layout() {
        let mut arena = Arena::new();
        let div = Div::new(Size::Fixed(100), Size::Fit).with_children(vec![
            Div::new(Size::Fixed(50), Size::Fixed(100)),
            Div::new(Size::Fixed(100), Size::Fixed(200)),
        ]);

        let key = arena.compute(&div);
        let root = &arena.nodes[key];
        assert_eq!(root.width, 100);
        let child1 = &arena.nodes[root.children[0]];
        assert_eq!(child1.width, 50);
        let child2 = &arena.nodes[root.children[1]];
        assert_eq!(child2.width, 100);
    }
}
