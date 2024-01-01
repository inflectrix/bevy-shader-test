use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[repr(C)]
#[derive(Asset, TypePath, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "26888a44-9bd9-47d6-8cfd-bbfe064b96cb"]
pub struct CustomMaterial {
    #[uniform(0)]
    pub time: f32,

    pub alpha_mode: AlphaMode, // unnecessary but nice to have
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_mat.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
