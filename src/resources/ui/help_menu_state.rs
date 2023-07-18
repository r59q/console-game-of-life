pub struct HelpMenuState {
    is_visible: bool,
}

impl HelpMenuState {
    pub fn new() -> HelpMenuState {
        HelpMenuState { is_visible: true }
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn set_visible(&mut self) -> () {
        self.is_visible = true
    }

    pub fn set_invisible(&mut self) -> () {
        self.is_visible = false
    }

    pub fn toggle_visibility(&mut self) -> () {
        self.is_visible = !self.is_visible
    }
}

#[cfg(test)]
mod test {
    use super::HelpMenuState;

    #[test]
    fn can_create_state() {
        HelpMenuState::new();
    }

    #[test]
    fn can_get_is_visible_and_visible_by_default() {
        let help_menu = HelpMenuState::new();
        assert!(help_menu.is_visible());
    }

    #[test]
    fn can_set_visible() {
        let mut help_menu = HelpMenuState::new();
        help_menu.set_invisible();
        help_menu.set_visible();
        assert!(help_menu.is_visible())
    }

    #[test]
    fn can_set_invisible() {
        let mut help_menu = HelpMenuState::new();
        help_menu.set_invisible();
        assert!(!help_menu.is_visible())
    }

    #[test]
    fn can_toggle_visibility() {
        let mut help_menu = HelpMenuState::new();
        help_menu.toggle_visibility();
        assert!(!help_menu.is_visible())
    }
}
