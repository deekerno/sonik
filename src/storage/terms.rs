pub enum Term {
    Title(String),
    Album(String),
    Artist(String),
}

impl Term {
    fn from_search_query(query: &str) -> Option<Term> {
        let elements = query.split(':').collect::<Vec<&str>>();

        if elements.len() <= 1 {
            None
        } else {
            match elements[0] {
                "title" => Some(Term::Title(elements[1].into())),
                "album" => Some(Term::Album(elements[1].into())),
                "artist" => Some(Term::Artist(elements[1].into())),
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
    pub fn new(input: &str) -> Option<SearchQuery> {
        // Turns user input string into a collection of search terms
        match Term::from_search_query(input) {
            Some(terms) => Some(SearchQuery { terms }),
            _ => None,
        }
    }
}
