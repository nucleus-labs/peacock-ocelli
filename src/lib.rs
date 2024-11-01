
mod camera;
mod model;

use peacock_pinion::xml::NodeAsync;
use peacock::build::WidgetContext;

pub fn gen_library(_node: NodeAsync) -> Option<WidgetContext> {
    todo!()
}

pub struct Transform {
    position: glam::Vec3,
    rotation: glam::Quat,
    scale: glam::Vec3,
}

pub trait Actor {
    fn get_transform(&self) -> &Transform;

}

impl Transform {
    fn model_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_translation(self.position)
            * glam::Mat4::from_quat(self.rotation)
            * glam::Mat4::from_scale(self.scale)
    }
}
