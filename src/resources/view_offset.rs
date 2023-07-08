pub struct ViewOffset {
    offset: (i64, i64),
}
impl ViewOffset {
    pub(crate) fn new() -> ViewOffset {
        return ViewOffset { offset: (0, 0) };
    }

    pub fn set_offset(&mut self, x: i64, y: i64) {
        self.offset = (x, y);
    }

    pub(crate) fn get_offset(&self) -> (i64, i64) {
        self.offset
    }

    pub fn get_mouse_to_world_offset(&self, mouse_pos: (u32, u32)) -> (i64, i64) {
        (
            self.offset.0 + mouse_pos.0 as i64,
            self.offset.1 + mouse_pos.1 as i64,
        )
    }

    pub fn get_mouse_to_world_offset_i32(&self, mouse_pos: (u32, u32)) -> (i32, i32) {
        (
            i32::try_from(self.offset.0 + mouse_pos.0 as i64).unwrap_or(i32::MAX),
            i32::try_from(self.offset.1 + mouse_pos.1 as i64).unwrap_or(i32::MAX),
        )
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
        assert_eq!(offset, (0, 0));
    }

    #[test]
    fn can_get_correct_offset() {
        let mut view_offset: ViewOffset = ViewOffset::new();

        let mut offset: (i64, i64) = view_offset.get_offset();
        assert_eq!(offset, (0, 0));
        view_offset.set_offset(2, 2);
        offset = view_offset.get_offset();
        assert_eq!(offset, (2, 2));
    }

    #[test]
    fn allow_negative_offset() {
        let mut view_offset: ViewOffset = ViewOffset::new();

        view_offset.set_offset(-2, -2);
        let offset: (i64, i64) = view_offset.get_offset();
        assert_eq!(offset, (-2, -2));
    }

    #[test]
    fn can_get_mouse_to_world_offset() {
        let mut view_offset: ViewOffset = ViewOffset::new();

        view_offset.set_offset(-2, -2);
        let world_pos: (i64, i64) = view_offset.get_mouse_to_world_offset((2 as u32, 2 as u32));
        assert_eq!(world_pos, (0, 0));
    }
}
