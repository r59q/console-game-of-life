use std::time::{Duration, SystemTime};

pub struct Timer {
    pub start_time: SystemTime,
    pub now: SystemTime,
    pub delta_time: Duration,
}

impl Timer {
    pub fn new() -> Self {
        let sys_time = std::time::SystemTime::now();
        return Timer { start_time: sys_time, now: sys_time, delta_time: Duration::from_micros(0) };
    }

    pub fn update(&mut self) {
        let duration_since_now
            = SystemTime::duration_since(&SystemTime::now(), self.now);

        self.now = SystemTime::now();

        match duration_since_now {
            Ok(duration) => {
                self.delta_time = duration;
            }
            Err(_) => { panic!("This really should not happen!") }
        }
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
}