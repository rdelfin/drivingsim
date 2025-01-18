use crate::dynamics::VehicleState;
use nalgebra::Vector2;
use std::time::Duration;

mod dynamics;
pub mod render;

#[derive(Default)]
pub struct SimState {
    pub vehicle_state: VehicleState,
    pub reward_points: Vec<(Vector2<f32>, f32)>,
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
    pub fn new(initial_vehicle_state: VehicleState) -> Simulator {
        Simulator {
            state: SimState {
                vehicle_state: initial_vehicle_state,
                reward_points: vec![
                    // (Vector2::new(10., 10.), 10.),
                    (Vector2::new(1000., 700.), 100.),
                ],
            },
            max_accelerator: 100.,
            max_angle: std::f32::consts::FRAC_PI_4,
        }
    }

    // This returns a reward
    pub fn advance(&mut self, action: Action, delta_t: Duration) -> f32 {
        let acceleration = action
            .acceleration
            .clamp(-self.max_accelerator, self.max_accelerator);
        let wheel_steer_angle = action
            .wheel_steer_angle
            .clamp(-self.max_angle, self.max_angle);

        self.state
            .vehicle_state
            .step(acceleration, wheel_steer_angle, delta_t);

        let got_reward_fn =
            |val: &(Vector2<f32>, f32)| (self.state.vehicle_state.position - val.0).norm() < 50.;

        let mut reward = 0.;
        self.state.reward_points.retain(|val| {
            if got_reward_fn(val) {
                reward += val.1;
                false
            } else {
                true
            }
        });

        reward
    }

    pub fn observe<O: Observer>(&self, observer: &O) -> O::Observation {
        observer.observe(&self.state)
    }

    pub fn get_state(&self) -> &SimState {
        &self.state
    }
}
