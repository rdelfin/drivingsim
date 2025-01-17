use nalgebra::{Rotation2, Vector2};
use std::time::Duration;

#[derive(Default)]
pub struct SimState {
    pub vehicle_position: Vector2<f32>,
    pub vehicle_speed: f32,
    pub angle: f32,
}

pub trait Observer {
    type Observation;
    fn observe(&self, state: &SimState) -> Self::Observation;
}

pub struct Action {
    accelerator: f32,
    steering_curvature: f32,
}

pub struct Simulator {
    state: SimState,
    max_speed: f32,
    max_accelerator: f32,
    max_angle: f32,
}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator {
            state: SimState::default(),
            max_speed: 10.,
            max_accelerator: 0.5,
            max_angle: 45.,
        }
    }

    pub fn advance(&mut self, action: Action, delta_t: Duration) {
        let delta_t_s = delta_t.as_secs_f32();
        let capped_accelerator = action
            .accelerator
            .clamp(-self.max_accelerator, self.max_accelerator);
        self.state.vehicle_speed += capped_accelerator * delta_t_s;
        self.state.vehicle_speed = self
            .state
            .vehicle_speed
            .clamp(-self.max_speed, self.max_speed);
        self.state.vehicle_position += Vector2::new(self.state.vehicle_speed, 0.);
    }

    pub fn observe<O: Observer>(&self, observer: &O) -> O::Observation {
        observer.observe(&self.state)
    }
}
