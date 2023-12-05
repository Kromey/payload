use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

/// Tag component for FPS counter
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct FpsCounter;

/// Tag component for FPS counter's text
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct FpsText;

#[derive(Debug, Deref, DerefMut)]
pub struct FpsUpdateTimer(Timer);

impl Default for FpsUpdateTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.50, TimerMode::Repeating))
    }
}

pub fn setup_fps_counter(mut commands: Commands) {
    commands
        .spawn((
            FpsCounter,
            NodeBundle {
                background_color: Color::NAVY.with_a(0.5).into(),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(5.0),
                    top: Val::Px(5.0),
                    // bottom: Val::Auto,
                    // left: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                FpsText,
                TextBundle {
                    text: Text::from_sections([
                        TextSection {
                            value: "FPS: ".into(),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        },
                        TextSection {
                            value: " N/A".into(),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        },
                    ]),
                    ..Default::default()
                },
            ));
        });
}

pub fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut text_qry: Query<&mut Text, With<FpsText>>,
    mut timer: Local<FpsUpdateTimer>,
    time: Res<Time>,
) {
    timer.tick(time.delta());

    if timer.just_finished() {
        for mut text in text_qry.iter_mut() {
            if let Some(value) = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|fps| fps.smoothed())
            {
                text.sections[1].value = format!("{value:>4.0}");
                info!("FPS: {value:>4.0}");
            } else {
                text.sections[1].value = " N/A".into();
            }
        }
    }
}

pub fn toggle_fps_counter(
    mut counter_qry: Query<&mut Visibility, With<FpsCounter>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F12) {
        for mut vis in counter_qry.iter_mut() {
            *vis = match *vis {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            }
        }
    }
}
