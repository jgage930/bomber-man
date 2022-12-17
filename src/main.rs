use bevy::{prelude::*, sprite::collide_aabb::collide};

use player::PlayerPlugin;
use player::Player;
use tilemap::{TileMapPlugin, TileCollider};
use enemy::{EnemyPlugin, };
use hud::HudPlugin;

mod player;
mod components;
mod tilemap;
mod enemy;
mod hud;

// Asset Constants
const PLAYER_SPRITE: &str = "player.png";
const PLAYER_SIZE: (f32, f32) = (64.0, 128.); 

const WALL_SPRITE: &str = "wall_tile.png";
const FLOOR_SPRITE: &str = "floor_tile.png";
const BOMB_SPRITE: &str = "bomb.png";
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const BREAKABLE_WALL_SPRITE: &str = "breakable_wall.png";
const BAT_SPRITE: &str = "bat.png";
const FONT: &str = "font.ttf";

const BACKGROUND_MUSIC: &str = "background-beat.ogg";

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
    bat: Handle<Image>,
    font: Handle<Font>,
}

#[derive(Resource)]
pub struct MainState {
    // a container to hold all of the variables needed by the full game
    pub score: usize,
}
// End Resources

#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug)]
pub enum GameState {
    StartMenu,
    Game,
    GameOver,
}

fn main() {
    App::new()
    .add_state(GameState::StartMenu)
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
    .add_plugin(HudPlugin)
    .add_startup_system(setup_system)
    .add_system_set(SystemSet::on_enter(GameState::StartMenu).with_system(spawn_main_menu))
    .add_system_set(
        SystemSet::on_update(GameState::StartMenu)
            .with_system(button_system)
    )
    .run();
}

fn setup_system(
    mut commands: Commands,
    assest_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
    audio: Res<Audio>,
) {
    // create a camera
    commands.spawn(Camera2dBundle::default());

    // Define our window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

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
        font: assest_server.load(FONT),
    };

    commands.insert_resource(game_textures);
    commands.insert_resource(MainState {
        score: 0,
    });

    let music = assest_server.load(BACKGROUND_MUSIC);
    audio.play_with_settings(
        music,
        PlaybackSettings {
            repeat: true,
            ..Default::default()
        }
    );
}


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);


fn spawn_main_menu (
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Press To Start",
                        TextStyle {
                            font: game_textures.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press to Start".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Press To Start".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Press To Start".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}