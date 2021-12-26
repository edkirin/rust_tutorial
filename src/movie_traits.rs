use chrono::Datelike;

struct Movie {
    title: String,
    director: String,
    release_year: u32,
    genre: String,
}

// Defining a Details trait by defining the functionality it should include
trait Details {
    fn description(&self) -> String;
    fn years_since_release(&self) -> u32;
}

// Implementing the Details trait on Movie struct
impl Details for Movie {
    // Method returns an overview of the movie
    fn description(&self) -> String {
        return format!(
            "{}, released in {}, is a {} movie directed by {} (Years since: {}).",
            self.title,
            self.release_year,
            self.genre,
            self.director,
            self.years_since_release(),
        );
    }

    // Method returns the number of years between the writing year of this shot i.e.
    // 2020 and the release year of the movie
    fn years_since_release(&self) -> u32 {
        return chrono::Local::now().year() as u32 - self.release_year;
    }
}

pub fn movie_traits_demo() {
    let mut movies = Vec::<Movie>::with_capacity(20);

    movies.push(Movie {
        title: "Titanic".to_string(),
        director: "James Cameron".to_string(),
        release_year: 1997,
        genre: "historical".to_string(),
    });

    movies.push(Movie {
        title: "The Dark Knight".to_string(),
        director: "Christopher Nolan".to_string(),
        release_year: 2008,
        genre: "action".to_string(),
    });

    for movie in movies {
        println!("\n{}", movie.description());
        println!(
            "The movie was released {} years ago.",
            movie.years_since_release()
        );
    }
}
