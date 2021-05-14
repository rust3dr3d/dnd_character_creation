use dnd_cc_lib::basic::*;

fn main(){
    println!("Running...");
    dnd_cc_lib::about();

    let amund:Character = Character::new(
        "Amund",
        25_u8,
        Gender::Male,
    );

    let inga:Character = Character::new(
        "Inga",
        20_u8,
        Gender::Female
    );

    amund.print_basic_info();
    inga.print_basic_info();

    let ability_score_str: i8 = amund.ability_scores.ability_score(AbilityScoreType::Strength);
    let ability_bonus_str: i8 = amund.ability_scores.ability_bonus(ability_score_str);

    println!("{} has an Ability Score:STR: {}", amund.name(), ability_score_str);
    println!("His Ability Score:STR Bonus is: {}\n", ability_bonus_str);

}