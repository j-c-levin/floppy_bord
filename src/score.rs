use bevy::prelude::*;

pub struct ScorePlugin;

#[derive(Resource)]
pub struct PlayerScore {
    pub score: usize
}

const SCORE_SIZE: f32 = 20.0;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerScore { score: 0})
            .add_systems(Startup, setup)
            .add_systems(Update, update_scoreboard);
    }
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCORE_SIZE,
                    color: Color::GRAY,
                    ..default()
                }
            ),
            TextSection::from_style(TextStyle {
                font_size: SCORE_SIZE,
                color: Color::GRAY,
                ..default()
            })
        ])
            .with_style( Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            })
    );
}

fn update_scoreboard(
    player_score: Res<PlayerScore>,
    mut query: Query<&mut Text>
) {
    let Ok(mut text) = query.get_single_mut() else {
      println!("update_scorboard: couldn't find score");
        return
    };
    text.sections[1].value = player_score.score.to_string();
}