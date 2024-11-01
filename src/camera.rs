
pub const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::mat4(
    glam::vec4(1.0, 0.0, 0.0, 0.0),
    glam::vec4(0.0, 1.0, 0.0, 0.0),
    glam::vec4(0.0, 0.0, 0.5, 0.0),
    glam::vec4(0.0, 0.0, 0.5, 1.0),
);

pub struct Camera {
    transform: crate::Transform,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl Camera {
    /// Rotates the camera to look at a given target point.
    #[inline]
    pub fn look_at(&mut self, target: glam::Vec3, up: glam::Vec3) {
        let direction = (target - self.transform.position).normalize();
        let right = up.cross(direction).normalize();
        let new_up = direction.cross(right).normalize();
    
        // Create a rotation matrix from the right, up, and forward vectors
        let rotation_matrix = glam::Mat3::from_cols(right, new_up, direction);
        self.transform.rotation *= glam::Quat::from_mat3(&rotation_matrix);
    }

    pub fn perspective(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }
}
