use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use iyes_loopless::prelude::*;

use crate::{
    ALLOW_EXIT, AppState,
    assets::Assets,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin)
            .add_enter_system(AppState::MainMenu, setup_main_menu)
            .add_exit_system(AppState::MainMenu, despawn_main_menu)
            .add_system(main_menu_ui.run_in_state(AppState::MainMenu));
    }
}

pub fn setup_main_menu(
    mut commands: Commands,
    assets: Res<Assets>,
) {
    // 2D camera to view Title Text
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let style = TextStyle {
        font: assets.font.clone(),
        font_size: 120.0,
        color: Color::WHITE,
    };
    let alignment = TextAlignment {
        horizontal: HorizontalAlign::Center,
        ..default()
    };
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("PONG", style.clone(), alignment),
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..default()
        });
}

pub fn despawn_main_menu(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Camera>, With<Text>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn main_menu_ui(
    mut commands: Commands,
    mut ctx: ResMut<EguiContext>,
    mut exit: EventWriter<AppExit>,
) {
    let window = egui::Window::new("Main Menu")
        .title_bar(false)
        .auto_sized()
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -50.0])
        .frame(egui::Frame::none());
    window.show(ctx.ctx_mut(), |ui| {
        ui.set_width(250.0);
        ui.vertical_centered_justified(|ui| {
            let play = egui::RichText::new("Play").size(60.0);
            if ui.button(play).clicked() {
                commands.insert_resource(NextState(AppState::InGame));
            }

            if ALLOW_EXIT {
                let quit = egui::RichText::new("Quit").size(60.0);
                if ui.button(quit).clicked() {
                    exit.send(AppExit);
                }
            }
        });
    });
}
