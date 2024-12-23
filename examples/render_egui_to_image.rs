use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiRenderToImage};
use wgpu_types::{Extent3d, TextureUsages};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(EguiPlugin);
    app.add_systems(Startup, setup_worldspace);
    app.add_systems(Update, (update_screenspace, update_worldspace));
    app.run();
}

fn update_screenspace(mut contexts: EguiContexts) {
    egui::Window::new("Screenspace UI").show(contexts.ctx_mut(), |ui| {
        ui.label("I'm rendering to screenspace!");
    });
}

fn update_worldspace(mut contexts: Query<&mut bevy_egui::EguiContext, With<EguiRenderToImage>>) {
    for mut ctx in contexts.iter_mut() {
        egui::Window::new("Worldspace UI").show(ctx.get_mut(), |ui| {
            ui.label("I'm rendering to an image in worldspace!");
        });
    }
}

fn setup_worldspace(
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let image = images.add({
        let size = Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
        };
        let mut image = Image {
            // You should use `0` so that the pixels are transparent.
            data: vec![0; (size.width * size.height * 4) as usize],
            ..default()
        };
        image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
        image.texture_descriptor.size = size;
        image
    });

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(Handle::clone(&image)),
            alpha_mode: AlphaMode::Blend,
            // Remove this if you want it to use the world's lighting.
            unlit: true,
            ..default()
        })),
    ));
    commands.spawn(EguiRenderToImage::new(image));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.5, 1.5, 1.5).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
}
