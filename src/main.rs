mod guessing_game;
use crate::guessing_game::guessing_game_main;
mod strings;
use crate::strings::strings_demo;
mod movie_traits;
use crate::movie_traits::movie_traits_demo;
mod structs;
use crate::structs::structs_demo;
mod handle_json;

use crate::handle_json::handle_json_demo;
mod error_handling;
use crate::error_handling::error_handling_demo;


#[allow(dead_code)]
enum TutorialSection {
    GuessingGame,
    StringsDemo,
    MoviesTraitsDemo,
    StructsDemo,
    HandleJsonDemo,
    ErrorHandlingDemo,
}


fn main() {
    println!("-------------------------------------------------------------");

    let tutorial: TutorialSection = TutorialSection::HandleJsonDemo;

    match tutorial {
        TutorialSection::GuessingGame => guessing_game_main(),
        TutorialSection::StringsDemo => strings_demo(),
        TutorialSection::MoviesTraitsDemo => movie_traits_demo(),
        TutorialSection::StructsDemo => structs_demo(),
        TutorialSection::HandleJsonDemo => handle_json_demo(),
        TutorialSection::ErrorHandlingDemo => error_handling_demo(),
    }
}
