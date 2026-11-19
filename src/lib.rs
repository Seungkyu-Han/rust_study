mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program() {
        println!("The nutritionist is {NUTRITIONIST}");
    }
}
mod cardio;
mod weight_lifting;

use cardio::{CardioTool, Exercise as CardioExercise};
use weight_lifting::Exercise as WeightLiftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weight_lifting: WeightLiftingExercise,
}

impl GymWorkout {
    pub fn new() -> Self {
        diet::ask_about_program();
        cardio::ask_about_program();
        weight_lifting::ask_about_program();

        Self {
            cardio: CardioExercise::new(String::from("tt"), CardioTool::Treadmill, 30),
            weight_lifting: WeightLiftingExercise::new(String::from("ww"), 10),
        }
    }
}
