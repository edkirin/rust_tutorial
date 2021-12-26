mod guessing_game;
use crate::guessing_game::guessing_game_main;
mod strings;
use crate::strings::strings_demo;
mod movie_traits;
use crate::movie_traits::movie_traits_demo;

#[allow(dead_code)]
enum TutorialSection {
    GuessingGame,
    StringsDemo,
    MoviesTraitsDemo,
}


fn main() {
    println!("-------------------------------------------------------------");

    let tutorial: TutorialSection = TutorialSection::MoviesTraitsDemo;

    match tutorial {
        TutorialSection::GuessingGame => {
            guessing_game_main();
        }
        TutorialSection::StringsDemo => {
            strings_demo();
        }
        TutorialSection::MoviesTraitsDemo => {
            movie_traits_demo();
        }
    }
}
