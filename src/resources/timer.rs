use std::time::{Duration, SystemTime};

pub struct Timer {
    pub start_time: SystemTime,
    pub now: SystemTime,
    pub delta_time: Duration,
    cell_update_time: SystemTime,
}

impl Timer {
    pub fn new() -> Self {
        let sys_time = std::time::SystemTime::now();
        return Timer {
            start_time: sys_time,
            now: sys_time,
            delta_time: Duration::from_micros(0),
            cell_update_time: sys_time,
        };
    }

    pub fn update(&mut self) {
        let duration_since_now = SystemTime::duration_since(&SystemTime::now(), self.now);

        self.now = SystemTime::now();

        match duration_since_now {
            Ok(duration) => {
                self.delta_time = duration;
            }
            Err(_) => {
                panic!("This really should not happen!")
            }
        }
    }

    pub fn get_time_since_cell_updates(&self) -> Duration {
        SystemTime::duration_since(&SystemTime::now(), self.cell_update_time)
            .expect("Failed to compute time since last cell update")
    }

    pub fn reset_cell_update_timer(&mut self) -> () {
        self.cell_update_time = SystemTime::now();
    }
}

#[cfg(test)]
mod test {
    use std::time::{Duration, SystemTime};

    use crate::resources::timer::Timer;

    #[test]
    fn has_now_time() {
        let timer = Timer::new();
        let duration = SystemTime::duration_since(&timer.start_time, timer.now);
        assert_eq!(duration.unwrap().as_micros(), 0)
    }

    #[test]
    fn can_update_timer() {
        let mut timer = Timer::new();

        assert_eq!(timer.delta_time.as_micros(), 0);

        std::thread::sleep(Duration::from_micros(100));

        timer.update();

        let second_test = timer.delta_time.as_micros();
        assert_ne!(second_test, 0);

        timer.update();

        assert_ne!(timer.delta_time.as_micros(), second_test);
    }

    #[test]
    fn can_get_time_since_last_cell_update() {
        let timer = Timer::new();
        let duration = timer.get_time_since_cell_updates();

        assert_eq!(0, duration.as_millis())
    }

    #[test]
    fn cell_update_timer_changes_with_time() {
        let timer = Timer::new();
        std::thread::sleep(Duration::from_millis(10));

        assert_ne!(0, timer.get_time_since_cell_updates().as_millis())
    }

    #[test]
    fn can_reset_cell_update_timer() {
        let mut timer = Timer::new();
        std::thread::sleep(Duration::from_millis(10));
        timer.reset_cell_update_timer();
        assert_eq!(0, timer.get_time_since_cell_updates().as_millis());
    }
}
