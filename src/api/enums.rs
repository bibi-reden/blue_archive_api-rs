use crate::enums::School;

pub enum CharacterNameOrQuery {
    Name(String),
    Query(StudentQuery),
}

pub enum EquipmentIDOrString {
    ID(u32),
    String(String),
}

pub enum Endpoints {
    Status,
    /**
        Takes a [`Option<CharNameOrQuery>`], which is used to pass in a
        `Name`: ([`String`]), or a `Query`: ([`StudentQuery`]) for advanced queries.
    */
    Character(Option<CharacterNameOrQuery>),
    /**
        Takes a `ID`: ([`u32`]) or a `String`: [`String`], to use for querying data about equipment.
    */
    Equipment(EquipmentIDOrString),
    Stage,
    Raid,
    Banner,
}

pub enum StudentQuery {
    Role(String),
    Type(String),
    School(School),
    Position(String),
    Weapon(String),
    Damage(String),
    Armor(String),
    Released(bool),
    ID(u32),
}

impl std::fmt::Display for StudentQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StudentQuery::Role(q) => write!(f, "query?role={}", q),
            StudentQuery::Type(q) => write!(f, "query?type={}", q),
            StudentQuery::School(q) => write!(f, "query?school={}", q),
            StudentQuery::Position(q) => write!(f, "query?position={}", q),
            StudentQuery::Weapon(q) => write!(f, "query?weapon={}", q),
            StudentQuery::Damage(q) => write!(f, "query?damage={}", q),
            StudentQuery::Armor(q) => write!(f, "query?heavy%20armor={}", q),
            StudentQuery::Released(b) => write!(f, "?released={}", b),
            StudentQuery::ID(i) => write!(f, "{}?id=true", i),
        }
    }
}

#[derive(Debug)]
pub enum BlueArchiveError {
    /// Something goes wrong with requesting data from the API.
    RequestError(reqwest::Error),
    /// In the case that deserialization failed.
    DeserializationError(reqwest::Error),
    /// When randomiation fails.
    RandomError,
}

impl std::fmt::Display for BlueArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlueArchiveError::RequestError(err) => write!(f, "{}", err),
            BlueArchiveError::DeserializationError(err) => write!(f, "{}", err),
            BlueArchiveError::RandomError => write!(
                f,
                "Randomizing students failed due to the slice being empty."
            ),
        }
    }
}

impl std::error::Error for BlueArchiveError {}