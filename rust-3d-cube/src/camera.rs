use glm::{Vector3, Mat4, radians};
use glm::ext::{look_at, perspective};

#[derive(Clone)]
#[derive(Copy)]
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
          right: Vector3::new(1.0, 0.0, 0.0),
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

    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn process_keyboard(
        &mut self,
        direction: i32 // just use int because i'm too lazy to figure out how enums work for now
                       // lmao
        ) {
        let velocity = 2.0;
        match direction {
            0 => self.position = (self.position + self.front) * velocity, // W
            1 => self.position = (self.position - self.front) * velocity, // S
            2 => self.position = (self.position - self.right) * velocity, // A
            3 => self.position = (self.position + self.right) * velocity, // D
            _ => {}
        }
    }

    //void ProcessKeyboard(MoveDirection direction, float deltaTime){
    //  float velocity = Speed * deltaTime;
    //  if(direction == FORWARD)
    //    Position += Front * velocity;
    //
    //  if(direction == BACKWARD)
    //    Position -= Front * velocity;
    //
    //  if(direction == RIGHT)
    //    Position += Right * velocity;
    //
    //  if(direction == LEFT)
    //    Position -= Right * velocity;
    //}
    //
    //void ProcessMouseMovement(double xPosIn, double yPosIn){
    //  float xpos = static_cast<float>(xPosIn);
    //  float ypos = static_cast<float>(yPosIn);
    //
    //  if(FirstMouse){
    //    LastX = xpos;
    //    LastY = ypos;
    //    FirstMouse = false;
    //  }
    //
    //  float xoffset = xpos - LastX;
    //  float yoffset = LastY - ypos;
    //  LastX = xpos;
    //  LastY = ypos;
    //
    //  xoffset *= Sensitivity;
    //  yoffset *= Sensitivity;
    //
    //  Yaw += xoffset;
    //  Pitch += yoffset;
    //
    //  /*std::cout << "-----    \n";*/
    //  /*std::cout << xoffset << " " << yoffset << "\n";*/
    //  /*std::cout << Yaw << " " << Pitch << "\n";*/
    //
    //  if(Pitch > 89.0f){
    //    Pitch = 89.0f;
    //  }
    //
    //  if(Pitch < -89.0f){
    //    Pitch = -89.0f;
    //  }
    //
    //  UpdateCameraVector();
    //}
    //
    //void UpdateCameraVector(){
    //  glm::vec3 direction;
    //  direction.x = cos(glm::radians(Yaw)) * cos(glm::radians(Pitch));
    //  direction.y = sin(glm::radians(Pitch));
    //  direction.z = sin(glm::radians(Yaw)) * cos(glm::radians(Pitch));
    //  Front = glm::normalize(direction);
    //  Right = glm::normalize(glm::cross(Front, glm::vec3(0.0f, 1.0f, 0.0f))); // world up
    //  Up = glm::normalize(glm::cross(Right, Front));
    //}
}
