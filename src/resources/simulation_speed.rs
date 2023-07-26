use std::cmp;

pub struct SimulationSpeed {
    speed: i32,
}

// We need a minimum simulation speed, since too low sim speed may result in unresponsiveness
const MIN_SIM_SPEED: i32 = 25;

impl SimulationSpeed {
    pub fn new(initial_speed: i32) -> SimulationSpeed {
        SimulationSpeed {
            speed: initial_speed,
        }
    }

    pub fn get_speed(&self) -> i32 {
        return self.speed;
    }

    pub fn add_speed(&mut self, change: i32) -> () {
        let new_value = cmp::max(MIN_SIM_SPEED, self.speed + change);
        self.speed = new_value;
    }
}

#[cfg(test)]
mod test {
    use crate::resources::simulation_speed::MIN_SIM_SPEED;

    use super::SimulationSpeed;

    #[test]
    fn can_create_simulation_speed_resource() {
        SimulationSpeed::new(500);
    }

    #[test]
    fn has_simulation_speed() {
        let sim_speed = SimulationSpeed::new(500);

        let _speed = sim_speed.get_speed();
    }

    #[test]
    fn speed_is_500_by_default() {
        let sim_speed = SimulationSpeed::new(500);

        let speed = sim_speed.get_speed();

        assert_eq!(500, speed);
    }

    #[test]
    fn can_change_speed() {
        let mut sim_speed = SimulationSpeed::new(500);

        sim_speed.add_speed(-50);

        assert_eq!(450, sim_speed.get_speed());
    }

    #[test]
    fn cannot_change_speed_below_zero() {
        let mut sim_speed = SimulationSpeed::new(500);
        sim_speed.add_speed(-1000);
        assert_eq!(MIN_SIM_SPEED, sim_speed.get_speed());
    }
}
