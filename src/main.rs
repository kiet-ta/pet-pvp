//ECS: Entity Component System paradigm
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Thêm plugin vật lý
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // Debug render: Vẽ viền xanh lá để check hitbox (Tắt đi nếu chỉ muốn nhìn Sprite)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_brawl_arena)
        .run();
}

fn setup_brawl_arena(mut commands: Commands) {
    // 1. QUAN TRỌNG NHẤT: Phải spawn Camera thì mới thấy đường
    commands.spawn(Camera2d);

    // 2. Tạo Sàn đấu (Platform) - Màu Xám
    commands.spawn((
        // Phần Hình ảnh (Visual)
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),         // Màu xám (R, G, B)
            custom_size: Some(Vec2::new(500.0, 20.0)), // Kích thước hiển thị
            ..default()
        },
        // Phần Vật lý (Physics)
        // LƯU Ý: Rapier dùng "Half-size" (nửa kích thước).
        // Muốn dài 500 thì điền 250, cao 20 thì điền 10.
        Collider::cuboid(250.0, 10.0),
        // Vị trí
        Transform::from_xyz(0.0, -200.0, 0.0),
        GlobalTransform::default(),
    ));

    // 3. Tạo Player - Màu Đỏ (Red)
    commands.spawn((
        RigidBody::Dynamic, // Vật thể động (rơi tự do)
        // Hình ảnh Player
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),        // Màu Đỏ rực
            custom_size: Some(Vec2::new(50.0, 50.0)), // To lên chút cho dễ nhìn
            ..default()
        },
        // Hitbox vật lý
        Collider::cuboid(25.0, 25.0), // Một nửa của 50
        // Một chút độ nảy (Restitution) cho vui
        Restitution::coefficient(0.7),
        // Vị trí xuất phát trên cao
        Transform::from_xyz(0.0, 200.0, 0.0),
        GlobalTransform::default(),
        Velocity::default(),
    ));
}
