//! Part 1: Foundation & Setup.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.
use crate::io;
use crate::types::{GameOver, GameState};

/// Prints the skill menu, reads the choice with `io::ask_number`, and returns it.
/// A value above 5 silently becomes 0 (see the Part 1 page).
pub fn get_shooting_skill_level() -> f64 {
    println!("HOW GOOD A SHOT ARE YOU WITH YOUR RIFLE?");
    println!(" (1) ACE MARKSMAN, (2) GOOD SHOT, (3) FAIR TO MIDDLIN'");
    println!(" (4) NEED MORE PRACTICE, (5) SHAKY KNEES");
    println!("ENTER ONE OF THE ABOVE __ THE BETTER YOU CLAIM YOU ARE THE");
    println!("FASTER YOU'LL HAVE TO BE WITH YOUR GUN TO BE SUCCESSFUL.");
// Shot selection
    let n = io::ask_number("");
    if n > 5.0 {
	0.0
    } else {
    	n
    }
}
// Range check for shot level, if over 5 default to 0


/// Runs the whole initial-purchase sequence and returns the starting `GameState`.
pub fn purchase_supplies(skill_level: f64) -> GameState {
    loop {
// Sets up the loop for purshasing supplies
	let oxen = loop {
	    let amount = io::ask_number("HOW MUCH DO YOU WANT TO SPEND ON YOUR OXEN TEAM");
	    if amount < 200.0 {
		println!("NOT ENOUGH");
	    } else if amount > 300.0 {
		println!("TOO MUCH");
	    } else {
		break amount;
	    }
	};
// Loop set up for the Oxen team and runs a check to make sure you aren't spending too much
	let food = loop {
	    let amount = io::ask_number("HOW MUCH DO YOU WANT TO SPEND ON FOOD");
	    if amount < 0.0 {
	        println!("IMPOSSIBLE");
	    } else {
	        break amount;
	    }
        };
// Loop set up for asking the user how much they'd like to spend on food
        let ammo = loop {
            let amount = io::ask_number("HOW MUCH DO YOU WANT TO SPEND ON AMMUNITION");
            if amount < 0.0 {
                println!("IMPOSSIBLE");
            } else {
                break amount;
            }
        };
// Loop set up for ammo and runs a check to make sure you dont write a negative number

    	let clothing = loop {
            let amount = io::ask_number("HOW MUCH DO YOU WANT TO SPEND ON CLOTHING");
            if amount < 0.0 {
                println!("IMPOSSIBLE");
            } else {
                break amount;
            }
        };
// Same as above, but for Clothing

	let misc = loop {
	    let amount = io::ask_number("HOW MUCH DO YOU WANT TO SPEND ON MISCELLANEOUS SUPPLIES");
	    if amount < 0.0 {
                println!("IMPOSSIBLE");
	    } else {
                break amount;
	    }
	};
// Same as above, but for Miscellaneous Supplies
	let total = oxen + food + ammo + clothing + misc;
	if total > 700.0 {
	    println!("YOU OVERSPENT--YOU ONLY HAD $700 TO SPEND. BUY AGAIN");
	    continue; // sends the user back to the beginning of the loop
	}

	let cash = 700.0 - total;
	let bullets = ammo * 50.0; // You get 50 bullets per dollar


	println!(
	    "AFTER ALL YOUR PURCHASES, YOU NOW HAVE {} DOLLARS LEFT",
	    io::fmt_num(cash) // Drops the decimal
	);
	println!("MONDAY MARCH 29 1847");

	return GameState {
	    food,
	    bullets,
	    clothing,
	    misc_supplies: misc,
	    cash,
	    oxen_spend: oxen,
	    skill_level,
	    ..GameState::default() // This means I used the deafault value for every field I didnt list
	};
    }
}

const DATES: [&str; 19] = [
    "APRIL 12", "APRIL 26", "MAY 10", "MAY 24", "JUNE 7", "JUNE 21",
    "JULY 5", "JULY 19", "AUGUST 2", "AUGUST 16", "AUGUST 30",
    "SEPTEMBER 13", "SEPTEMBER 27", "OCTOBER 11", "OCTOBER 25",
    "NOVEMBER 8", "NOVEMBER 22", "DECEMBER 6", "DECEMBER 20",
]; // Array created for each of the 19 2 week dates, pulls one of the 19 dates, &str because its a string slice
 
/// Advances to the next turn and prints its date. Returns `Err(GameOver::TrailTooLong)`
/// when the trip runs past the last date in the table.
pub fn advance_turn_date(state: &mut GameState) -> Result<(), GameOver> {
    state.turn_number += 1; // Tracks GameState and += 1 moves everything forward one turn

    if state.turn_number >= 20 {
	println!("YOU HAVE BEEN ON THE TRAIL TOO LONG ------");
        println!("YOUR FAMILY DIES IN THE FIRST BLIZZARD OF WINTER");
	return Err(GameOver::TrailTooLong); // Fail to reach end and past 20 turns you die
    }

    let date = DATES[(state.turn_number - 1) as usize]; // Turn 1 now is April 12th, arry set to 0
    println!();
    println!("MONDAY {} 1847", date);
    println!();
    Ok(()) // Succsessful result
}


/// Start-of-turn cleanup and the status table: clamp negatives, low-food warning, floor
/// values, record `turn_start_mileage`, print mileage and the resource table.
pub fn print_status_table(state: &mut GameState) {
    // Fixing anything that got turned negative from the previous turn
    if state.food < 0.0 {
	state.food = 0.0;
    }
    if state.bullets < 0.0 {
	state.bullets = 0.0;
    }
    if state.clothing < 0.0 {
	state.clothing = 0.0;
    }
    if state.misc_supplies < 0.0 {
	state.misc_supplies = 0.0;
    }

    // Warning if you have low food
    if state.food < 13.0 {
	println!("YOU'D BETTER DO SOME HUNTING OR BUY FOOD SOON!!!!");
    }

    // Using floor to round all numbers down to whole numbers instead of keeping them as decimals
    state.food = state.food.floor();
    state.bullets = state.bullets.floor();
    state.clothing = state.clothing.floor();
    state.misc_supplies = state.misc_supplies.floor();
    state.cash = state.cash.floor();
    state.mileage = state.mileage.floor();

    // Making the turn start mileage the current mileage
    state.turn_start_mileage = state.mileage;

    // Flag set for the mountain crossing logic that I was tasked to make a flag for in checkpoint 1 
    if state.force_display_950 {
	println!("TOTAL MILEAGE IS 950");
	state.force_display_950 = false;
    } else {
	println!("TOTAL MILEAGE IS {}", io::fmt_num(state.mileage));
    }

    // Making all headers for resources and fixing them so that they all align left (was done in demo we were given)
    println!(
	"{:<14}{:<14}{:<14}{:<14}{}",
        "FOOD", "BULLETS", "CLOTHING", "MISC. SUPP.", "CASH"
    );
    println!(
	"{:<14}{:<14}{:<14}{:<14}{}",
        io::fmt_num(state.food), // Turns each number into a string, drops the decimal, and lets me display them aligned left like above
        io::fmt_num(state.bullets),
        io::fmt_num(state.clothing),
        io::fmt_num(state.misc_supplies),
        io::fmt_num(state.cash)
    );
}
/// Charges the $20 doctor's bill when an illness or injury flag is set. Returns
/// `Err(GameOver::CantAffordDoctor)` when cash cannot cover it.
pub fn apply_doctor_bill(state: &mut GameState) -> Result<(), GameOver> {
    // If you have an illness or injury it will now make needs_doctor true
    let mut needs_doctor = false;
    if state.illness_pending {
        needs_doctor = true;
    }
    if state.injury_pending {
        needs_doctor = true;
    }

    // If you don't need a doctor then it will pass, otherwise it will charge you 20 dollars for a doctor
    if needs_doctor == false {
        return Ok(());
    }

    state.cash -= 20.0;
    // If you have a negative number after trying to get a doctor, the game will end
    if state.cash < 0.0 {
	return Err(GameOver::CantAffordDoctor);
    }

    // Setting both of the flags back to false because its the end of the doctors visit
    println!("DOCTOR'S BILL IS $20");
    state.illness_pending = false;
    state.injury_pending = false;
    Ok(())
}


<<<<<<< HEAD
#[cfg(test)]
mod tests {
    use super::*;

    //I am testing the advances of the dates and turns
    // Im making sure the game advances through the dates correctly and ends the game
    // when a player goes past the 19 dates
    #[test]
    fn turn_dates_and_blizzard() {
        let mut s = GameState::default();
        // This starts a new game and makes sure the first turn actually advances right, the turn number should increase by 1
        assert!(advance_turn_date(&mut s).is_ok());
        assert_eq!(s.turn_number, 1);
        // Here im setting the turn number to 18, which would be the last starting point I can have because one more turn would make you >
        s.turn_number = 18;
        assert!(advance_turn_date(&mut s).is_ok());
        // Testing to see if going past turn 19 will cause the TrailTooLong ending
        s.turn_number = 19;
        assert_eq!(advance_turn_date(&mut s), Err(GameOver::TrailTooLong));
        // Here im testing to see what will happen if im well over 19 turns
	s.turn_number = 24;
        assert_eq!(advance_turn_date(&mut s), Err(GameOver::TrailTooLong));
    }


    //Im testing the doctor bills and the player status table
    //I want to see if the medical expenses are calculated right
    //and that the current stats (cash food etc.) are all correct
    #[test]
    fn doctor_bill_and_status_table() {
	//This gives the player 100 dollars and marks them as sick
	//Doctor will charge 20 dollars like normal and then clear the illness flag I set
	let mut s = GameState::default();
	s.cash = 100.0;
	s.illness_pending = true;
	assert!(apply_doctor_bill(&mut s).is_ok());
        assert_eq!(s.cash, 80.0);
        assert_eq!(s.illness_pending, false);
	//This tests that if the player isnt sick or injured, then they wont get any doctor bills
	let mut s = GameState::default();
        s.cash = 100.0;
        assert!(apply_doctor_bill(&mut s).is_ok());
        assert_eq!(s.cash, 100.0);
	//This test makes sure that you can have exactly 20 dollars and have it be able to cover the cost of the treatment
	//Should leave you with $0
	let mut s = GameState::default();
        s.cash = 20.0;
        s.injury_pending = true;
        assert!(apply_doctor_bill(&mut s).is_ok());
        assert_eq!(s.cash, 0.0);
	//This tests gives the player 19 dollars and makes sure that I get a gameover for not being able to afford the doctor
	let mut s = GameState::default();
        s.cash = 19.0;
        s.injury_pending = true;
        assert_eq!(apply_doctor_bill(&mut s), Err(GameOver::CantAffordDoctor));
	//This test forces the 950 mile flag and makes sure that the flag goes away after activating
	let mut s = GameState::default();
        s.force_display_950 = true;
        print_status_table(&mut s);
        assert_eq!(s.force_display_950, false);
	//This test sets the value of each of the resources to negative, so it should make them go to 0, like I said in my flowchart
	s.food = -5.0;
        s.bullets = -1.0;
        s.clothing = -3.0;
        s.misc_supplies = -2.0;
        print_status_table(&mut s);
        assert_eq!(s.food, 0.0);
        assert_eq!(s.bullets, 0.0);
        assert_eq!(s.clothing, 0.0);
        assert_eq!(s.misc_supplies, 0.0);
    }
}
=======
>>>>>>> 06de3b1d2a1127313c5f4c7c04c41ed73672bc09
