pub enum Term {
    Any(String),
    Title(String),
    Album(String),
    Artist(String),
    YearBefore(i32),
    YearAfter(i32)
}

impl Term {
    pub fn from_search_query(query: &str) -> Option<Term> {
        let elements = query.split(':').collect::<Vec<&str>>();

        if elements.len() == 1 {
            Some(Term::Any(elements[0].into()))
        } else {
            match elements[0] {
                "title" => Some(Term::Title(elements[1].into())),
                "album" => Some(Term::Album(elements[1].into())),
                "artist" => Some(Term::Artist(elements[1].into())),
                "year_before" => Some(Term::YearBefore(elements[1].parse::<i32>().unwrap())),
                "year_after" => Some(Term::YearAfter(elements[1].parse::<i32>().unwrap())),
                _ => return None
            }
        }
    }

    pub fn to_sql_query(self) -> String {
        match self {
            Term::Any(x) => format!("title like '%{}%' or album like '%{}%' or artist like '%{}%' or albumartists like '%{}%'", x, x, x, x),
            Term::Title(x) => format!("title like '%{}%'", x),
            Term::Album(x) => format!("album like '%{}%'", x),
            Term::Artist(x) => format!("artist like '%{}%' or albumartists like '%{}%'", x, x),
            Term::YearBefore(x) => format!("year <= {}", x),
            Term::YearAfter(x) => format!("year >= {}", x),
        }
    }
}

// A search query will end up being just a collection of search terms
pub struct SearchQuery {
    terms: Vec<Term>,
}

impl SearchQuery {
    pub fn new(input: &str) -> SearchQuery {
        // Turns user input string into a collection of search terms
        let terms = input.split(',').filter_map(Term::from_search_query).collect();
        SearchQuery{terms: terms}
    }

    pub fn to_sql_query(self) -> String {
        let mut sql_query: String = "select * from tracks".into();

        // If there's anything at all, start chaining terms together in SQL
        if !self.terms.is_empty() {
            sql_query.push_str(" where ");
            sql_query.push_str(&self.terms.into_iter()
                               .map(|x| x.to_sql_query())
                               .collect::<Vec<String>>()
                               .join(" AND  "));
        }

        // Return the final SQL query string
        sql_query
    }
}
