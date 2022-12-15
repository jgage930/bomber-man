use bevy::prelude::*;

use crate::{GameTextures, player::Player, MainState};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_hud)
            .add_system(update_hud);
    }
}

#[derive(Component)]
pub struct Hud;

fn spawn_hud(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    let font = game_textures.font.clone();

    let text_style = TextStyle {
        font, font_size: 60., color: Color::WHITE
    };

    let hud_text =  Text::from_sections([
        TextSection::new("Health: 100", text_style.clone()),
        TextSection::new("Score: 0", text_style.clone()),
        TextSection::new("Bombs: 5", text_style.clone())
    ]);

    commands.spawn(Text2dBundle{
        text: hud_text 
            .with_alignment(TextAlignment::TOP_LEFT),
        transform: Transform {
            translation: Vec3::new(0., 0., 101.),
            ..Default::default() 
        },
        ..Default::default()
    })
    .insert(Hud);
}

fn update_hud(
    player_query: Query<(&Transform, &Player), With<Player>>,
    mut hud_query: Query<&mut Text, With<Hud>>,
    main_state: Res<MainState>
) {
    let (player_transform, player) = player_query.single();
    let mut text = hud_query.single_mut(); 

    text.sections[0].value = format!("Health: {} \n", player.health);
    text.sections[1].value = format!("Score: {} \n", main_state.score);
    text.sections[2].value = format!("Bombs: {} \n", player.num_bombs);
}