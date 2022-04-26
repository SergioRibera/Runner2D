use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_player);
        app.add_system(player_movement);
    }
}

#[derive(Component, Debug)]
pub struct PlayerSettings {
    pub speed: f32,
}

fn get_resource_name(name: &str) -> String {
    format!("player/2BlueWizardIdle/Chara - BlueIdle000{}.png", name)
}

fn startup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(get_resource_name("01").as_str()),
            sprite: Sprite {
                custom_size: Some(Vec2::new(150.0, 150.0)),
                ..default()
            },
            ..default()
        })
        .insert(PlayerSettings { speed: 45.0 });
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut PlayerSettings, &mut Sprite, &mut Transform)>,
) {
    for (player, mut sprite, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x += -1.0 * player.speed * time.delta_seconds();
            sprite.flip_x = true;
            // transform.rotation = Quat::from_rotation_y(PI).into();
            // *atlas = player.run.clone();
        } else if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += player.speed * time.delta_seconds();
            sprite.flip_x = false;
            // transform.rotation = Quat::from_rotation_y(0.0).into();
            // *atlas = player.run.clone();
        } else {
            // *atlas = player.idle.clone();
        }
    }
    // for (mut logo, mut transform) in sprite_position.iter_mut() {
    // match *logo {
    //     Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
    //     Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
    // }
    //
    // if transform.translation.y > 200. {
    //     *logo = Direction::Down;
    // } else if transform.translation.y < -200. {
    //     *logo = Direction::Up;
    // }
    // }
}
