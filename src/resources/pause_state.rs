pub struct PauseState {
    paused: bool
}
impl PauseState {
    pub(crate) fn new() -> PauseState {
        return PauseState { paused: false  }
    }

    pub(crate) fn is_paused(&self) -> bool {
        return self.paused;
    }

    pub(crate) fn pause(&mut self) {
        self.paused = true;
    }

    pub(crate) fn unpause(&mut self) {
        self.paused = false;
    }
}

#[cfg(test)]
mod test {
    use super::PauseState;


    #[test]
    fn can_create_new_pause_state() {
        let _pause_state = PauseState::new();
    }

    #[test]
    fn pause_state_start_unpaused() {
        let pause_state = PauseState::new();
        let is_paused = pause_state.is_paused();
        assert_eq!(is_paused, false);
    }

    #[test]
    fn can_pause() {
        let mut pause_state = PauseState::new();
        pause_state.pause();
        let is_paused = pause_state.is_paused();
        assert_eq!(is_paused, true);
    }

    #[test]
    fn can_unpause() {
        let mut pause_state = PauseState::new();
        pause_state.unpause();
        let is_paused = pause_state.is_paused();
        assert_eq!(is_paused, false);
    }

    #[test]
    fn can_pause_and_unpause() {
        let mut pause_state = PauseState::new();
        
        pause_state.pause();
        let is_paused = pause_state.is_paused();
        assert_eq!(is_paused, true);
        
        pause_state.unpause();
        let is_paused = pause_state.is_paused();
        assert_eq!(is_paused, false);
    }

    #[test]
    fn can_pause_multiple_times() {
        let mut pause_state = PauseState::new();

        for n in 0..100 {
            pause_state.pause();
            assert_eq!(pause_state.is_paused(), true)
        }
    }

    #[test]
    fn can_unpause_multiple_times() {
        let mut pause_state = PauseState::new();

        for n in 0..100 {
            pause_state.unpause();
            assert_eq!(pause_state.is_paused(), false)
        }
    }
}