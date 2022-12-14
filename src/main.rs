use bevy::{prelude::*, sprite::collide_aabb::collide};

use player::PlayerPlugin;
use player::Player;
use tilemap::{TileMapPlugin, TileCollider};
use enemy::{EnemyPlugin, };

mod player;
mod components;
mod tilemap;
mod enemy;

// Asset Constants
const PLAYER_SPRITE: &str = "player.png";
const PLAYER_SIZE: (f32, f32) = (64.0, 128.); 

const WALL_SPRITE: &str = "wall_tile.png";
const FLOOR_SPRITE: &str = "floor_tile.png";
const BOMB_SPRITE: &str = "bomb.png";
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const BREAKABLE_WALL_SPRITE: &str = "breakable_wall.png";
const BAT_SPRITE: &str = "bat.png";

const SPRITE_SCALE: f32 = 0.5;

const TILE_SIZE: f32 = 64.0;
// End Asset Constants

// Game Constants
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

const BOMB_TIME: u64 = 2; // how long before a bomb explodes in seconds
// End Game Constants

// Resources
#[derive(Resource)]
pub struct WinSize {
    pub w: f32, pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    player: Handle<Image>,
    floor: Handle<Image>,
    wall: Handle<Image>,
    bomb: Handle<Image>,
    explosion: Handle<TextureAtlas>,
    breakable_wall: Handle<Image>,
    bat: Handle<Image>
}

#[derive(Resource)]
pub struct MainState {
    // a container to hold all of the variables needed by the full game
    score: usize,
}

// End Resources

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins
        .set(WindowPlugin {
            window: WindowDescriptor {
            title: "Bomber Man!".to_string(),
            width: 640.,
            height: 640.,
            ..Default::default()
        },
        ..Default::default() })
        .set(ImagePlugin::default_nearest()))
    .add_plugin(PlayerPlugin)
    .add_plugin(TileMapPlugin)
    .add_plugin(EnemyPlugin)
    .add_startup_system(setup_system)
    .run();
}

fn setup_system(
    mut commands: Commands,
    assest_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,

) {
    // create a camera
    commands.spawn(Camera2dBundle::default());

    // Define our window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    commands.insert_resource(WinSize {w: win_w, h: win_h});

    let explosion_handle = assest_server.load(EXPLOSION_SHEET); 
    let texture_atlast = TextureAtlas::from_grid(explosion_handle, Vec2::new(64., 64.), 4, 4, None, None);
    let explosion = texture_atlases.add(texture_atlast);

    // Define our game textures
    let game_textures = GameTextures {
        player: assest_server.load(PLAYER_SPRITE), 
        floor: assest_server.load(FLOOR_SPRITE),
        wall: assest_server.load(WALL_SPRITE),
        bomb: assest_server.load(BOMB_SPRITE),
        explosion,
        breakable_wall: assest_server.load(BREAKABLE_WALL_SPRITE),
        bat: assest_server.load(BAT_SPRITE),
    };

    commands.insert_resource(game_textures);
    commands.insert_resource(MainState {
        score: 0,
    });
}