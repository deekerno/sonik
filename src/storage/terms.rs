pub enum Term {
    Any(String),
    Title(String),
    Album(String),
    Artist(String),
    //YearBefore(i32),
    //YearAfter(i32),
}

impl Term {
    fn from_search_query(query: &str) -> Option<Term> {
        let elements = query.split(':').collect::<Vec<&str>>();

        if elements.len() == 1 {
            Some(Term::Any(elements[0].into()))
        } else {
            match elements[0] {
                "title" => Some(Term::Title(elements[1].into())),
                "album" => Some(Term::Album(elements[1].into())),
                "artist" => Some(Term::Artist(elements[1].into())),
                //"year_before" => Some(Term::YearBefore(elements[1].parse::<i32>().unwrap())),
                //"year_after" => Some(Term::YearAfter(elements[1].parse::<i32>().unwrap())),
                _ => None,
            }
        }
    }
}

// A search query will end up being just a collection of search terms
pub struct SearchQuery {
    pub terms: Term,
}

impl SearchQuery {
    pub fn new(input: &str) -> SearchQuery {
        // Turns user input string into a collection of search terms
        let terms = Term::from_search_query(input).unwrap();

        SearchQuery { terms }
    }
}
