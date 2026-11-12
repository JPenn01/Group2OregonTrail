//! Part 2: Fort, Hunting & Shooting.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.
use std::time::Instant;

use crate::types::{GameState, TurnChoice};
use crate::{io, rng};

const WORDS: [&str; 4] = ["BANG", "BLAM", "POW", "WHAM"];
const MIN_HUNT_BULLETS: f64 = 40.0;
const TOUGH_MSG: &str = "TOUGH---YOU NEED MORE BULLETS TO GO HUNTING";
/// Shows the right turn menu (fort or no fort), settles a valid, affordable choice, flips
/// `fort_available_next_turn`, and returns the choice.
///
/// Skeleton returns `TurnChoice::Continue`.
pub fn turn_menu(state: &mut GameState) -> TurnChoice {
    let choice = loop {
        let picked = if state.fort_available_next_turn {
            let n = io::ask_number(
                "DO YOU WANT TO (1) STOP AT THE NEXT FORT, (2) HUNT, OR (3) CONTINUE",
            );
            // Anything outside 1-2 becomes 3 (continue).
            if n == 1.0 {
                TurnChoice::StopAtFort
            } else if n == 2.0 {
                TurnChoice::Hunt
            } else {
                TurnChoice::Continue
            }
        } else {
            let n = io::ask_number("DO YOU WANT TO (1) HUNT, OR (2) CONTINUE");
            if n == 1.0 { TurnChoice::Hunt } else { TurnChoice::Continue }
        };

        if matches!(picked, TurnChoice::Hunt) && state.bullets < MIN_HUNT_BULLETS {
            println!("{TOUGH_MSG}");
            continue; // same menu again, flag not flipped
        }
        break picked;
    };

    state.fort_available_next_turn = !state.fort_available_next_turn;
    choice
}

/// Fort purchases at the worse 2:3 exchange rate; stopping costs 45 miles.
///
/// Skeleton does nothing.
pub fn fort_purchases(state: &mut GameState) {}

/// Hunting: bullet check, 45 miles, shooting challenge, then `resolve_hunt_outcome`.
///
/// Skeleton does nothing.
pub fn go_hunting(state: &mut GameState) {}

/// The timed "type the word" challenge. Returns the response score: lower is faster, and
/// 9 means the wrong word was typed. Also called from Parts 3 and 4.
///
/// Skeleton returns 2.0 (a middling response) without prompting.
pub fn shooting_challenge(skill_level: f64) -> f64 {
    2.0
}

/// Turns a response score into a hunt result: food gained, bullets spent, or a miss.
///
/// Skeleton does nothing.
pub fn resolve_hunt_outcome(score: f64, state: &mut GameState) {}
