use std::{
    fs::File,
    io::{BufRead, BufReader, },
};
use bevy::{prelude::*, };

use crate::{GameTextures, TILE_SIZE, SPRITE_SCALE};

pub struct TileMapPlugin;

#[derive(Component)]
struct Map;

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct Breakable;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
       app
        .add_startup_system_to_stage(StartupStage::PostStartup, create_map_system); 
    }
}

fn create_map_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    let file = File::open("assets/map.txt").expect("no map file");

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {

                let (texture, index_z) = match char {
                   '#' => {(game_textures.wall.clone(), 100.0)},
                   '.' => {(game_textures.floor.clone(), 1.0)},
                   '@' => {(game_textures.breakable_wall.clone(), 100.0)},
                    _ => {(game_textures.wall.clone(), 100.0)},
                };

                // spawn a tile
                let mut tile = commands.spawn(SpriteBundle {
                    texture: texture,
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                    transform: Transform { 
                        translation: Vec3::new(x as f32 * TILE_SIZE , -(y as f32) * TILE_SIZE, index_z), 
                        ..Default::default()
                    },
                    ..Default::default()
                    },
                );

                if char == '#' {
                    tile.insert(TileCollider);
                }

                if char == '@' {
                    tile.insert(TileCollider);
                    tile.insert(Breakable);

                    // spawn a floor tile under breakable walls
                    commands.spawn(SpriteBundle {
                        texture: game_textures.floor.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                        transform: Transform { 
                            translation: Vec3::new(x as f32 * TILE_SIZE , -(y as f32) * TILE_SIZE, 1.), 
                            ..Default::default()
                        },
                        ..Default::default()
                        },
                    );

                }
            }
        }
    }
}