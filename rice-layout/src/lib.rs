pub mod div;
pub mod layout;
pub mod rect;
pub mod size;

#[cfg(test)]
mod tests {
    use crate::{div::Div, size::Size};

    #[test]
    fn test_div() {
        let div = Div::default();
        assert_eq!(div.height, Size::Fit);
        assert_eq!(div.width, Size::Fit);
    }
}
