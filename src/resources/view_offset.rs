pub struct ViewOffset {
    offset:(i64, i64)
}
impl ViewOffset {
    pub(crate) fn new() -> ViewOffset {
        return ViewOffset { offset: (0, 0)  }
    }

    pub fn set_offset(&mut self, x: i64, y: i64) {
        self.offset = (x, y);
    }

    pub(crate) fn get_offset(&self) -> (i64, i64) {
        self.offset
    }
}


#[cfg(test)]
mod test {
    use super::ViewOffset;

    #[test]
    fn can_make_resource() {
        ViewOffset::new();
    }
    
    #[test]
    fn can_set_offset() {
        let mut view_offset: ViewOffset = ViewOffset::new();

        view_offset.set_offset(0, 0)
    }

    #[test]
    fn can_get_offset() {
        let view_offset: ViewOffset = ViewOffset::new();

        let offset: (i64, i64) = view_offset.get_offset();
        assert_eq!(offset, (0,0));
    }

    #[test]
    fn can_get_correct_offset() {
        let mut view_offset: ViewOffset = ViewOffset::new();

        let mut offset: (i64, i64) = view_offset.get_offset();
        assert_eq!(offset, (0,0));
        view_offset.set_offset(2, 2);
        offset = view_offset.get_offset();
        assert_eq!(offset, (2,2));
    }

    #[test]
    fn allow_negative_offset() {
        let mut view_offset: ViewOffset = ViewOffset::new();

        view_offset.set_offset(-2, -2);
        let offset: (i64, i64) = view_offset.get_offset();
        assert_eq!(offset, (-2,-2));
    }
}