use crate::{
    GameTextures, TILE_SIZE, Player, player
};

use bevy::{prelude::*, transform};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_enemy_system)
            .add_system(enemy_movement_system);
    }
}

#[derive(Component)]
pub struct Enemy {
    speed: f32,
}

fn spawn_enemy_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>
) {
    commands.spawn(SpriteBundle {
        texture: game_textures.orc.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(32.0, 64.0)),
            ..Default::default()
        },
        transform: Transform { 
            translation: Vec3::new(100., 200., 10.), 
            ..Default::default() 
        },
        ..Default::default()
    })
    .insert(Enemy {
        speed: 0.5,
    });
}

fn enemy_movement_system(
    mut query: Query<(&Enemy, &mut Transform)>,
    player_query: Query<&Player>,
    time: Res<Time>,
) {
    let player = player_query.single();
    let player_position = player.position;
    
    for (enemy, mut transform) in query.iter_mut() {
        let enemy_position = transform.translation;

        let (mut dx, mut dy) = (player_position.x - enemy_position.x, player_position.y - enemy_position.y);

        let distance = dx.hypot(dy);

        (dx, dy) = (dx / distance, dy / distance);

        transform.translation.x += dx * enemy.speed * TILE_SIZE * time.delta_seconds();
        transform.translation.y += dy * enemy.speed * TILE_SIZE * time.delta_seconds();
    }
}