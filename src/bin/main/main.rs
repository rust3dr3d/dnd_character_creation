use dnd_cc_lib::{Character, BasicInfo, Gender};

fn main(){
    println!("Running...");
    dnd_cc_lib::about();

    let amund:Character = Character{
        name: "Amund",
        age: 25,
        gender: Gender::Male
    };

    let inga:Character = Character{
        name: "Inga",
        age: 20,
        gender: Gender::Female
    };

    amund.print_basic_info();
    inga.print_basic_info();
}