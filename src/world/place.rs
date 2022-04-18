#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Place {
    Dominatory,
    Home,
    HSE,
    Road(Box<Place>, Box<Place>),
}

impl Default for Place {
    fn default() -> Self {
        Self::Dominatory
    }
}