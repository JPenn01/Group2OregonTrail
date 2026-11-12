//! Part 3: Travel Decisions.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.

use crate::types::{EatingLevel, GameOver, GameState};

/// What the riders *look* like. The truth can differ (see `resolve_rider_encounter`).
/// This type belongs to Part 3 alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hostility {
    Hostile,
    Friendly,
}

/// Asks how well to eat, re-asking until the choice is valid and affordable. Returns the
/// level settled on. Returns `Err(GameOver::Starved)` when food is below 13.
///
/// Skeleton returns `Ok(EatingLevel::Moderate)` without prompting.
pub fn choose_eating_level(state: &mut GameState) -> Result<EatingLevel, GameOver> {
    if state.food < 13.0 {
        return Err(GameOver::Starved);
    }

    loop {
        println!("DO YOU WANT TO EAT (1) POORLY (2) MODERATELY");
        println!("OR (3) WELL");

        let choice = crate::io::ask_number() as i32; //collects user input

        let level = match choice { //saves eating choice to level
            1 => EatingLevel::Poor,
            2 => EatingLevel::Moderate,
            3 => EatingLevel::Well,
            _ => continue, //silently re-asks the prompt
        };

        if consume_food(state, level).is_ok() {
            state.last_eating_level = level;
            return Ok(level); //finishes the loop, returns validated eating choice
        }
        else {
            println!("YOU CAN'T EAT THAT WELL"); //will repeat the loop
        }
    }
}

/// Deducts the food cost of an eating level if it can be afforded. `Err(())` means it
/// could not be afforded and nothing changed.
///
/// Skeleton returns `Ok(())` without changing anything.
pub fn consume_food(state: &mut GameState, level: EatingLevel) -> Result<(), ()> {
    let cost = match level { //saves the cost of chosen eating level to a variable
        EatingLevel::Poor => 13.0,
        EatingLevel::Moderate => 18.0,
        EatingLevel::Well => 23.0,
    };

    if state.food < cost { //returns an error if the eating level can't be afforded
        Err(())
    }
    else { // deducts the cost of the food from your food amount and returns without an error
        state.food -= cost;
        Ok(())
    }
}

/// Adds this turn's travel distance to `mileage`.
///
/// Skeleton does nothing.
pub fn calculate_daily_mileage(state: &mut GameState) {
    //takes the amount spent on oxen and decreases it by 220, then divides it by 3
    let oxen_bonus = (state.oxen_spend - 220.0) / 3.0;
    //provides a random bonus from 1-10 miles
    let random_bonus = crate::rng::rnd() * 10.0;
    //finalizes the total increase in miles
    state.mileage += 200.0 + oxen_bonus + random_bonus;
}

/// Rolls whether riders appear this turn. `None` means no encounter; otherwise returns
/// how the riders look.
///
/// Skeleton returns `None` (never an encounter).
pub fn rider_encounter_roll(mileage: f64) -> Option<Hostility> {
    //shifts the mileage to make 400 the center point of the probability curve
    let miles = mileage / 100.0 - 4.0;
    //creates the threshold for the encounter roll
    let threshold = (miles * miles + 72.0) / (miles * miles + 12.0) - 1.0;

    //rolls for the encounter, if it fails, no rider encounter
    if crate::rng::rnd() * 10.0 > threshold {
        return None;
    }

    //80% chance for hostile riders, returns hostility type
    if crate::rng::rnd() < 0.8 {
        println!("RIDERS AHEAD. THEY LOOK HOSTILE");
        Some(Hostility::Hostile)
    }
    //20% for non hostile riders, returns hostility type
    else {
        println!("RIDERS AHEAD. THEY DON'T LOOK HOSTILE");
        Some(Hostility::Friendly)
    }
}

/// Plays out the rider encounter: tactic menu, outcome, losses. Returns
/// `Err(GameOver::RiderMassacre)` if the bullets run out against hostile riders.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn resolve_rider_encounter(state: &mut GameState, looks: Hostility) -> Result<(), GameOver> {
    //20% chance for hostility to silenty flip
    let actual_hostility = if crate::rng::rnd() <= 0.2 {
        match looks {
            Hostility::Hostile => Hostility::Friendly,
            Hostility::Friendly => Hostility::Hostile,
        }
    }
    else {
        looks
    };

    //creates a loop for valid input
    let tactic = loop {
        println!("TACTICS");
        println!("(1) RUN (2) ATTACK (3) CONTINUE (4) CIRCLE WAGONS");

        //checks for valid input 1-4
        let choice = crate::io::ask_number() as i32;
        if (1..=4).contains(&choice) {
            break choice; //ends loop if valid input is entered
        }
    };
        //prevents standard closing message
        let mut skip_closing = false;

        //tactic outcomes determined by rider hostility
        match actual_hostility {
            Hostility::Hostile => match tactic { //outcomes for hostile riders
                1 => { //RUN
                    state.mileage += 20.0;
                    state.misc_supplies -= 15.0;
                    state.bullets -= 150.0;
                    state.oxen_spend -= 40.0;
                }
                2 => { //ATTACK
                    let score = shooting_challenge(state); //runs shooting challenge
                    state.bullets -= 80.0 + 40.0 * score;

                    //responses to the shooting challenge
                    if score <= 1.0 {
                        println!("NICE SHOOTING---YOU DROVE THEM OFF");
                    }
                    else if score > 4.0 {
                        println!("LOUSY SHOT---YOU GOT KNIFED");
                        state.injury_pending = true;
                        println!("YOU HAVVE TO SEE OL' DOC BLANCHARD");
                    }
                    else {
                        println!("KINDA SLOW WITH YOUR COLT .45");
                    }
                }
                3 => { //CONTINUE
                    if crate::rng::rnd() <= 0.2 { //20% chance they will not attack
                        println!("THEY DID NOT ATTACK");
                        skip_closing = true; //skips closing message
                    }
                    else {
                        state.bullets -= 150.0;
                        state.misc_supplies -= 15.0;
                    }
                }
                4 => { //CIRCLE WAGONS
                    let score = shooting_challenge(state); //runs shooting challenge
                    state.bullets -= 80.0 + 30.0 * score;
                    state.mileage -= 25.0;

                    //responses to the shooting challenge
                    if score <= 1.0 {
                        println!("NICE SHOOTING---YOU DROVE THEM OFF");
                    }
                    else if score > 4.0 {
                        println!("LOUSY SHOT---YOU GOT KNIFED");
                        state.injury_pending = true;
                        println!("YOU HAVVE TO SEE OL' DOC BLANCHARD");
                    }
                    else {
                        println!("KINDA SLOW WITH YOUR COLT .45");
                    }
                }
                _ => unreachable!(), //doesnt allow options other than 1-4
            },
            Hostility::Friendly => match tactic {
                1 => { //RUN
                    state.oxen_spend -= 10.0;
                    state.mileage += 15.0;
                }
                2 => { //ATTACK
                    state.bullets -= 100.0;
                    state.mileage -= 5.0;
                }
                3 => { //CONTINUE
                //nothing happens
                }
                4 => {
                    //CIRCLE WAGONS
                    state.mileage -= 20.0;
                }
                _ => unreachable!(), //doesnt allow options other than 1-4
            },
        }

        //runs if skip closing is still false
        if !skip_closing {
            match actual_hostility { //prints resolve statement based on hostility
                Hostility::Friendly => println!("RIDERS WERE FRIENDLY, BUT CHECK FOR POSSIBLE LOSSES"),
                Hostility::Hostile => println!("RIDERS WERE HOSTILE--CHECK FOR LOSSES"),
            }
        }
        //runs if riders were hostile and you run out of bullets
        if actual_hostility == Hostility::Hostile && state.bullets < 0.0 {
            println!("YOU RAN OUT OF BULLETS AND GOT MASSACRED BY THE RIDERS");
            return Err(GameOver::RiderMassacre); //game ends by rider massacre
        }
        Ok(())
}
