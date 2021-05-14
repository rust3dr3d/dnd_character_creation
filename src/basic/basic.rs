#[path = "../ability_scores/ability_scores.rs"]
mod ability_scores;

use ability_scores::AbilityScores;
pub use ability_scores::AbilityScoreType;
pub use ability_scores::AbilityModifier;


/* Character Struct for Player, NPCs and Enemies */
#[derive(Debug)]
pub struct Character{
    pub name: &'static str,
    pub age: u8,
    pub gender: Gender,

    //Plugging in Ability Scores from ability_scores module
    pub ability_scores:AbilityScores
}


/* Gender Enum */
#[derive(Copy, Clone, Debug)]
pub enum Gender{
    Male,
    Female,
    Other
}


/*Basic information Trait for Characters */
pub trait BasicInfo{
    //Creates a new Character with the supplied basic information
    fn new(name: &'static str, age:u8, gender:Gender) -> Self;
    
    fn name(&self) -> &'static str;
    fn age(&self) -> u8;
    fn gender(&self) -> Gender;
    
    fn print_basic_info(&self){
        println!("Name: {}, Age: {}, Gender: {:#?}\n", self.name(), self.age(), self.gender());
    }
}


//Implement BasicInfo for Character struct
impl BasicInfo for Character{

    //Create a new Character and return it
    fn new(name:&'static str, age:u8, gender:Gender) -> Character{
        Character{
            name,
            age,
            gender,
            ability_scores: AbilityScores::new()
        }
    }


    //Get Character's name
    fn name(&self) -> &'static str{
        self.name
    }


    //Get Character's age
    fn age(&self) -> u8{
        self.age
    }

    
    //Get Character's gender
    fn gender(&self) -> Gender{
        self.gender
    }
}
