pub const ABILITY_MODIFIER_DEFAULT:i8 = 10;

// Here we have 6 Ability Scores that defines a Character
// Implemented as shown in the Temple of Elemental Evil game manual

/* Keeping the data type as i8(-128 to 127) because AbilityScores can be negative values...
   due to spells/effects etc
*/
#[derive(Copy, Clone, Debug)]
pub struct AbilityScores{
    //Strength
    pub str:i8,
    //Dexterity
    pub dex:i8,
    //Constitution
    pub con:i8,
    //Intelligence
    pub int:i8,
    //Wisdom
    pub wis:i8,
    //Charisma
    pub cha:i8
}


//Enum to represent ability scores
pub enum AbilityScoreType{
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}


//Implement AbilityScores functionalities
impl AbilityScores{

    //Initialize a new AbilityScores with defaults(ABILITY_MODIFIER_DEFAULT)
    pub fn new() -> AbilityScores{
        AbilityScores{
            dex: ABILITY_MODIFIER_DEFAULT,
            str: ABILITY_MODIFIER_DEFAULT,
            con: ABILITY_MODIFIER_DEFAULT,
            int: ABILITY_MODIFIER_DEFAULT,
            wis: ABILITY_MODIFIER_DEFAULT,
            cha: ABILITY_MODIFIER_DEFAULT
        }
    }


    //Get Ability Score value depending on the supplied Ability Score Type
    pub fn ability_score(&self, ability_score_type: AbilityScoreType) -> i8{
        match ability_score_type{
            AbilityScoreType::Strength => self.str,
            AbilityScoreType::Dexterity => self.dex,
            AbilityScoreType::Constitution => self.con,
            AbilityScoreType::Intelligence => self.int,
            AbilityScoreType::Wisdom => self.wis,
            AbilityScoreType::Charisma => self.cha
        }
    }
}


//Ability Scores Modifier Trait (See chart of TOEE manual Page 26)
pub trait AbilityModifier{
    fn ability_bonus(&self, ability_score:i8) -> i8;
}


//IMplement AbilityModifier for AbilityScores
impl AbilityModifier for AbilityScores{
    /*See Ability Modifier chart. As ability scores increase above ABILITY_MODIFIER_DEFAULT(10)
      characters gain a bonus to that specific ability score.

      For ex: if STR is 16, bonus is +3
      
      If an Ability Score is lower than ABILITY_MODIFIER_DEFAULT(10) they receive penalties.
      
      for ex: If STR is 5 penalty is -3
    */
    fn ability_bonus(&self, ability_score:i8) -> i8{

        let lower_bound: i8 = if ability_score % 2 == 0 {ability_score} else {ability_score -1};

        (ABILITY_MODIFIER_DEFAULT - lower_bound) / 2 * -1
    }
}