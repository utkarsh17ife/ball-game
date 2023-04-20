use bevy::prelude::*;

use crate::game::score::resources::*;
use crate::events::GameOver;

pub fn update_high_score(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScore>,
) {
    for event in game_over_event_reader.iter() {
        high_score.scores.push(("Player".to_string(), event.score));
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string())
    }
}

pub fn high_score_updated(high_score: ResMut<HighScore>) {
    if high_score.is_changed() {
        println!("High score: {:?}", high_score.scores);
    }
}
