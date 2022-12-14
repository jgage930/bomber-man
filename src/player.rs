use std::time::Duration;

use crate::enemy::Enemy;
use crate::tilemap::{TileCollider, Breakable};
use crate::{
    GameTextures,
    TILE_SIZE,
    BOMB_TIME,
    PLAYER_SIZE, MainState
};

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub const STARTING_BOMB_COUNT: usize = 5;
pub const STARTING_HEALTH: f32 = 100.;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    speed: f32,
    num_bombs: usize,
    pub position: Vec2,
    health: f32
}

#[derive(Component)]
pub struct Bomb {
    timer: Timer,
}

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Repeating))
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        .add_system(player_movement_system)
        .add_system(place_bomb_system)
        .add_system(explode_bomb_system)
        .add_system(camera_follow_system)
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .add_system(check_for_explosion_collision_system)
        .add_system(enemy_collision_check);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    commands.spawn(SpriteBundle {
        texture: game_textures.player.clone(),
        sprite: Sprite { 
            custom_size: Some(Vec2::new(32.0, 64.0)), 

            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(400., 100., 10.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player {
        speed: 6.0, 
        num_bombs: STARTING_BOMB_COUNT,
        position: Vec2::new(400., 100.),
        health: STARTING_HEALTH,
    });
}

fn player_movement_system(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::W) {
        y_delta += player.speed * TILE_SIZE * time.delta_seconds();
    } 

    if keyboard.pressed(KeyCode::S) {
        y_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) {
        x_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        x_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }

    player.position = Vec2::new(transform.translation.x, transform.translation.y);
}


// camera script
fn camera_follow_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn place_bomb_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    keyboard: Res<Input<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    mut player_component_query: Query<&mut Player>,
) {
    let player_transform = player_query.single();
    
    let mut player_component = player_component_query.single_mut();

    let (bomb_x, bomb_y) = (player_transform.translation.x, player_transform.translation.y);

    if keyboard.just_pressed(KeyCode::Return) && player_component.num_bombs > 0 {
        commands.spawn(SpriteBundle{
            texture: game_textures.bomb.clone(),    
            sprite: Sprite{
                custom_size: Some(Vec2::new(TILE_SIZE / 2., TILE_SIZE / 2.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(bomb_x, bomb_y, 50.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bomb {
            timer: Timer::new(Duration::from_secs(BOMB_TIME), TimerMode::Once)
        });

        player_component.num_bombs -= 1;
    }
}

fn explode_bomb_system(
    mut commands: Commands,
    time: Res<Time>,
    mut bomb_query: Query<(Entity, &Transform, &mut Bomb)>,
) {
    for (entity, transform, mut bomb) in bomb_query.iter_mut() {
        bomb.timer.tick(time.delta());

        if bomb.timer.finished() {
            // spawn the explosion to spawn at bomb position:w
            commands.spawn(ExplosionToSpawn(transform.translation.clone()));
    
            commands.entity(entity).despawn();

        }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        commands.spawn(SpriteSheetBundle {
            texture_atlas: game_textures.explosion.clone(),
            transform: Transform {
                translation: explosion_to_spawn.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Explosion)
        .insert(ExplosionTimer(Timer::from_seconds(0.05, TimerMode::Repeating)));

        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            sprite.index += 1;

            if sprite.index >= 16 {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn check_for_explosion_collision_system(
    mut commands: Commands,
    wall_query: Query<(Entity, &Transform), With<Breakable>>,
    explosion_query: Query<&Transform, With<Explosion>>,
    mut main_state: ResMut<MainState>,
) {
    for (entity, wall_transform) in wall_query.iter() {
        for explosion_transform in explosion_query.iter() {
            let explosion_translation = explosion_transform.translation;
            let wall_translation = wall_transform.translation;

            let collision = collide(
                explosion_translation, 
                Vec2::splat(TILE_SIZE * 2.5), 
                wall_translation, 
                Vec2::splat(TILE_SIZE) 
            );

            if collision.is_some() {
                commands.entity(entity).despawn();
                
                main_state.score += 10;
            }
        }
    }
    
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::new(PLAYER_SIZE.0 * 0.7, PLAYER_SIZE.1 * 0.7),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true 
}

fn enemy_collision_check(
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    let (player_transform, mut player) = player_query.single_mut();

    for enemy_transform in enemy_query.iter() {
        let collision = collide(
            player_transform.translation,
            Vec2::new(PLAYER_SIZE.0 * 0.7, PLAYER_SIZE.1 * 0.7),
            enemy_transform.translation,
            Vec2::new(PLAYER_SIZE.0 * 0.7, PLAYER_SIZE.1 * 0.7)
        );

        if collision.is_some() {
            player.health -= 5.;
            println!("health: {}", player.health);
        }
    }
}