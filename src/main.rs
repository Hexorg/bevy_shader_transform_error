//! A shader that uses the GLSL shading language.

use bevy::{
    input::mouse::MouseMotion, pbr::{MaterialPipeline, MaterialPipelineKey}, prelude::*, reflect::TypePath, render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    }
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<FootprintMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera_with_mouse)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FootprintMaterial>>,
    mut smat:ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        material: materials.add(FootprintMaterial {
            color: Color::BLUE,
            color_texture: Some(asset_server.load("pattern.png")),
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });
    commands.spawn(PbrBundle{
        mesh:meshes.add(Mesh::from(shape::Plane{size:10.0, subdivisions:0})),
        transform:Transform::IDENTITY,
        material:smat.add(StandardMaterial{base_color:Color::DARK_GREEN,..default()}),
        ..default()
    });
    // commands.spawn(DirectionalLightBundle{
    //     directional_light:DirectionalLight{illuminance:50000.0,..default()},
    //     transform:Transform::from_translation(Vec3::new(10.0, 100.0, 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_camera_with_mouse(
    mut rotation:Local<(f32, f32)>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query:Query<&mut Transform, With<Camera>>
) {
    let mut t = query.single_mut();
    if rotation.0 == 0.0 && rotation.1 == 0.0 {
        (rotation.0, rotation.1, _) = t.rotation.to_euler(EulerRot::YXZ);
    }
    const SPEED:f32 = -0.001;
    for ev in motion_evr.read() {
        rotation.0 += SPEED*ev.delta.x;
        rotation.1 += SPEED*ev.delta.y;
        t.rotation = Quat::from_euler(EulerRot::YXZ, rotation.0, rotation.1, 0.0)
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct FootprintMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
/// When using the GLSL shading language for your shader, the specialize method must be overridden.
impl Material for FootprintMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.frag".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}