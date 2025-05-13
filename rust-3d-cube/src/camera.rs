use glm::{Vector3, Mat4, radians};
use glm::ext::{look_at, perspective};

pub struct Camera {
  pub position: Vector3<f32>,
  pub front: Vector3<f32>,
  pub right: Vector3<f32>,
  pub up: Vector3<f32>,
  pub yaw: f32,
  pub pitch: f32,
  pub sensitivity: f32,
  pub fov: f32,
  pub width: f32,
  pub height: f32,
}

impl Camera {
    pub fn new(
        new_position: Vector3<f32>,
        new_width: f32,
        new_height: f32,
    ) -> Self {
        Camera {
          position: new_position,
          front: Vector3::new(0.0, 0.0, -1.0),
          right: Vector3::new(1.0, 1.0, 0.0),
          up: Vector3::new(0.0, 1.0, 0.0),
          yaw: -90.0,
          pitch: 0.0,
          sensitivity: 0.1,
          fov: 45.0,
          width: new_width,
          height: new_height
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        look_at(self.position, self.front, self.up)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        // TODO:
        // handle resolution/viewport or whatever its called
        perspective(radians(self.fov), self.width / self.height, 0.001, 1000.0)
    }
}
