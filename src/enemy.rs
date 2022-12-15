use crate::{
    GameTextures, TILE_SIZE, Player, player::{self, Explosion}, PLAYER_SIZE, MainState
};

use bevy::{prelude::*, };
use bevy::sprite::collide_aabb::collide;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_enemy_system)
            .add_system(enemy_movement_system)
            .add_system(check_for_explosion_collision);
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
    let enemy_positions = vec![
        (100., 100.),
        (200., 200.),
        (300., 300.),
        (100., -900.)
    ];

    for (x, y) in enemy_positions.iter() {
        commands.spawn(SpriteBundle {
            texture: game_textures.bat.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 64.0)),
                ..Default::default()
            },
            transform: Transform { 
                translation: Vec3::new(*x, *y, 101.), 
                ..Default::default() 
            },
            ..Default::default()
        })
        .insert(Enemy {
            speed: 0.8,
        });
    }
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

fn check_for_explosion_collision(
    mut commands: Commands,
    explosion_query: Query<&Transform, With<Explosion>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut main_state: ResMut<MainState>,
) {
    for explosion_transform in explosion_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let collision = collide(
                explosion_transform.translation,
                Vec2::splat(TILE_SIZE * 2.5),
                enemy_transform.translation,
                Vec2::new(PLAYER_SIZE.0,PLAYER_SIZE.1)
            );

            if collision.is_some() {
                commands.entity(enemy_entity).despawn();

                main_state.score += 100;
            }
        }
    }
}