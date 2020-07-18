use crate::{
    camera::{Camera, OrthographicProjection, PerspectiveProjection, VisibleEntities},
    pipeline::RenderPipelines,
    render_graph::base,
    Draw, Mesh,
};
use bevy_asset::Handle;
use bevy_ecs::Bundle;
use bevy_transform::components::{Rotation, Scale, Transform, Translation};

#[derive(Bundle, Default)]
pub struct MeshComponents {
    pub mesh: Handle<Mesh>,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

#[derive(Bundle)]
pub struct PerspectiveCameraComponents {
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Default for PerspectiveCameraComponents {
    fn default() -> Self {
        PerspectiveCameraComponents {
            camera: Camera {
                name: Some(base::camera::CAMERA3D.to_string()),
                ..Default::default()
            },
            perspective_projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
        }
    }
}

#[derive(Bundle)]
pub struct OrthographicCameraComponents {
    pub camera: Camera,
    pub orthographic_projection: OrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Default for OrthographicCameraComponents {
    fn default() -> Self {
        OrthographicCameraComponents {
            camera: Camera {
                name: Some(base::camera::CAMERA2D.to_string()),
                ..Default::default()
            },
            orthographic_projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
        }
    }
}