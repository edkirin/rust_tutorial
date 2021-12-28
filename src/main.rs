mod guessing_game;
mod strings;
mod movie_traits;
mod structs;
mod handle_json;
mod error_handling;
mod generics;

use crate::guessing_game::guessing_game_main;
use crate::strings::strings_demo;
use crate::movie_traits::movie_traits_demo;
use crate::structs::structs_demo;
use crate::handle_json::handle_json_demo;
use crate::error_handling::error_handling_demo;
use crate::generics::generics_demo;


#[allow(dead_code)]
enum TutorialSection {
    GuessingGame,
    StringsDemo,
    MoviesTraitsDemo,
    StructsDemo,
    HandleJsonDemo,
    ErrorHandlingDemo,
    GenericsDemo,
}


fn main() {
    println!("-------------------------------------------------------------");

    let tutorial: TutorialSection = TutorialSection::GenericsDemo;

    match tutorial {
        TutorialSection::GuessingGame => guessing_game_main(),
        TutorialSection::StringsDemo => strings_demo(),
        TutorialSection::MoviesTraitsDemo => movie_traits_demo(),
        TutorialSection::StructsDemo => structs_demo(),
        TutorialSection::HandleJsonDemo => handle_json_demo(),
        TutorialSection::ErrorHandlingDemo => error_handling_demo(),
        TutorialSection::GenericsDemo => generics_demo(),
    }
}
