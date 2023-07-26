use std::cmp;

pub struct SimulationSpeed {
    speed: i32,
}

impl SimulationSpeed {
    pub fn new() -> SimulationSpeed {
        SimulationSpeed { speed: 500 }
    }

    pub fn get_speed(&self) -> i32 {
        return self.speed;
    }

    pub fn add_speed(&mut self, change: i32) -> () {
        let new_value = cmp::max(0, self.speed + change);
        self.speed = new_value;
    }
}

#[cfg(test)]
mod test {
    use super::SimulationSpeed;

    #[test]
    fn can_create_simulation_speed_resource() {
        SimulationSpeed::new();
    }

    #[test]
    fn has_simulation_speed() {
        let sim_speed = SimulationSpeed::new();

        let _speed = sim_speed.get_speed();
    }

    #[test]
    fn speed_is_500_by_default() {
        let sim_speed = SimulationSpeed::new();

        let speed = sim_speed.get_speed();

        assert_eq!(500, speed);
    }

    #[test]
    fn can_change_speed() {
        let mut sim_speed = SimulationSpeed::new();

        sim_speed.add_speed(-50);

        assert_eq!(450, sim_speed.get_speed());
    }

    #[test]
    fn cannot_change_speed_below_zero() {
        let mut sim_speed = SimulationSpeed::new();
        sim_speed.add_speed(-1000);
        assert_eq!(0, sim_speed.get_speed());
    }
}
