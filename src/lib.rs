use crate::dynamics::VehicleState;
use std::time::Duration;

mod dynamics;
mod render;

#[derive(Default)]
pub struct SimState {
    pub vehicle_state: VehicleState,
}

pub trait Observer {
    type Observation;
    fn observe(&self, state: &SimState) -> Self::Observation;
}

pub struct Action {
    acceleration: f32,
    wheel_steer_angle: f32,
}

pub struct Simulator {
    state: SimState,
    max_accelerator: f32,
    max_angle: f32,
}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator {
            state: SimState::default(),
            max_accelerator: 0.5,
            max_angle: 45.,
        }
    }

    pub fn advance(&mut self, action: Action, delta_t: Duration) {
        let acceleration = action
            .acceleration
            .clamp(-self.max_accelerator, self.max_accelerator);
        let wheel_steer_angle = action
            .wheel_steer_angle
            .clamp(-self.max_angle, self.max_angle);

        self.state
            .vehicle_state
            .step(acceleration, wheel_steer_angle, delta_t);
    }

    pub fn observe<O: Observer>(&self, observer: &O) -> O::Observation {
        observer.observe(&self.state)
    }

    pub fn get_state(&self) -> &SimState {
        &self.state
    }
}
