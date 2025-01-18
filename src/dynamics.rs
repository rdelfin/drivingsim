use nalgebra::Vector2;
use std::time::Duration;

pub struct VehicleState {
    pub position: Vector2<f32>,
    pub speed: f32,
    pub angle: f32,
    pub base_length: f32,
    pub max_speed: f32,
}

impl Default for VehicleState {
    fn default() -> VehicleState {
        VehicleState {
            position: Vector2::default(),
            speed: 0.,
            angle: 0.,
            base_length: 10.,
            max_speed: 10.,
        }
    }
}

// We will use a simple bicycle model for this, with the following dynamics equations:Vector2
// dx/dt     = v*cos(theta)
// dy/dt     = v*sin(theta)
// dtheta/dt = v*tan(deltoid)/L
// dv/dt     = a
//
// where:
// - x and y are the position of the vehicle in some reference frame
// - deltoid is the wheel steer angle
// - theta is the vehicle's actual angle
// - v is the linear velocity of the vehicle
// - a is the acceleration
// - L is the wheel base length, or distance between both wheels

impl VehicleState {
    pub fn step(&mut self, acceleration: f32, wheel_steer_angle: f32, delta_t: Duration) {
        let delta_t_s = delta_t.as_secs_f32();
        // The order of modification is important to make sure things propagate correctly
        self.position += self.speed * Vector2::new(self.angle.cos(), self.angle.sin()) * delta_t_s;
        self.angle += self.speed * wheel_angle.tan() / self.base_length * delta_t_s;
        self.speed += acceleration * delta_t_s;
        self.speed = self.speed.clamp(-self.max_speed, self.max_speed);
    }
}
