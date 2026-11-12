//! Part 4: Events, Mountains & Endings.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.
//!
//! On a 3-person team, nobody owns this part. Leave this file as it is: it is the
//! placeholder module, it is ungraded, and the rest of the game builds and runs against it.

use crate::types::{EventKind, GameOver, GameState};
<<<<<<< HEAD

/// Rolls a random percentage and returns the matching event from the event table.
///
/// Skeleton returns `EventKind::HelpfulIndians`.
pub fn roll_random_event() -> EventKind {
    EventKind::HelpfulIndians
}

/// Applies one event's message and losses. Returns `Err` if the event ends the game.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn apply_event(state: &mut GameState, event: EventKind) -> Result<(), GameOver> {
    Ok(())
}

=======
use crate::rng::rnd;
/// Rolls a random percentage and returns the matching event from the event table.
///
/// Skeleton returns `EventKind::HelpfulIndians`.

pub fn roll_random_event() -> EventKind {
    let roll = (rnd() * 100.0) as i32;

    if roll < 6 {
        EventKind::WagonBreakdown
    } else if roll < 11 {
        EventKind::OxInjuredLeg
    } else if roll < 13 {
        EventKind::DaughterBrokeArm
    } else if roll < 15 {
        EventKind::OxWandersOff
    } else if roll < 17 {
        EventKind::SonLost
    } else if roll < 22 {
        EventKind::UnsafeWater
    } else if roll < 32 {
        EventKind::RainOrCold
    } else if roll < 35 {
        EventKind::Bandits
    } else if roll < 37 {
        EventKind::WagonFire
    } else if roll < 42 {
        EventKind::HeavyFog
    } else if roll < 44 {
        EventKind::Snakebite
    } else if roll < 54 {
        EventKind::RiverSwamped
    } else if roll < 64 {
        EventKind::WildAnimals
    } else if roll < 69 {
        EventKind::HailStorm
    } else if roll < 95 {
        EventKind::EatingIllnessRoll
    } else {
        EventKind::HelpfulIndians
    }
}
/// Applies one event's message and losses. Returns `Err` if the event ends the game.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn apply_event(
    state: &mut GameState,
    event: EventKind,
) -> Result<(), GameOver> {
    match event {
        EventKind::WagonBreakdown => {
            println!("WAGON BREAKS DOWN--LOSE TIME AND SUPPLIES FIXING IT");
            state.mileage -= 15.0 + (rnd() * 6.0);
            state.misc_supplies -= 8.0;
        }

        EventKind::OxInjuredLeg => {
            println!("OX INJURES LEG---SLOWS YOU DOWN REST OF TRIP");
            state.mileage -= 25.0;
            state.oxen_spend -= 20.0;
        }

        EventKind::DaughterBrokeArm => {
            println!("BAD LUCK---YOUR DAUGHTER BROKE HER ARM");
            println!("YOU HAD TO STOP AND USE SUPPLIES TO MAKE A SLING");
            state.mileage -= 5.0 + (rnd() * 5.0);
            state.misc_supplies -= 2.0 + (rnd() * 4.0);
        }

        EventKind::OxWandersOff => {
            println!("OX WANDERS OFF---SPEND TIME LOOKING FOR IT");
            state.mileage -= 17.0;
        }

        EventKind::SonLost => {
            println!("YOUR SON GETS LOST---SPEND HALF THE DAY LOOKING FOR HIM");
            state.mileage -= 10.0;
        }

        EventKind::UnsafeWater => {
            println!("UNSAFE WATER--LOSE TIME LOOKING FOR CLEAN SPRING");
            state.mileage -= 2.0 + (rnd() * 11.0);
        }

        EventKind::WagonFire => {
            println!("THERE WAS A FIRE IN YOUR WAGON--FOOD AND SUPPLIES DAMAGE!");
            state.food -= 40.0;
            state.bullets -= 400.0;
            state.misc_supplies -= 3.0 + (rnd() * 9.0);
            state.mileage -= 15.0;
        }

        EventKind::HeavyFog => {
            println!("LOSE YOUR WAY IN HEAVY FOG---TIME IS LOST");
            state.mileage -= 10.0 + (rnd() * 6.0);
        }

        _ => {
            // Remaining events for a later checkpoint
        }
    }

    Ok(())
}
>>>>>>> 06de3b1d2a1127313c5f4c7c04c41ed73672bc09
/// Mountain crossing past mile 950: rugged mountains, South Pass, Blue Mountains.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn cross_mountains(state: &mut GameState) -> Result<(), GameOver> {
    Ok(())
}

/// Rolls illness severity. Returns `Err(GameOver::RanOutOfMedicalSupplies)` when the
/// medicine runs out.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn illness_check(state: &mut GameState) -> Result<(), GameOver> {
    Ok(())
}

/// Prints the ending for a cause of death, then the formalities sequence.
///
/// Skeleton prints one generic line so a finished game is visible.
pub fn resolve_ending(state: &GameState, cause: GameOver) {
    println!("THE TRIP HAS ENDED: {:?}", cause);
}

/// Prints the arrival sequence: partial-turn food and date, final resource table.
///
/// Skeleton prints one generic line so a finished game is visible.
pub fn arrive_at_oregon_city(state: &mut GameState) {
    println!("YOU ARRIVED AT OREGON CITY");
}
