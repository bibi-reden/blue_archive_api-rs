//! Contains the [`Student`] structure and its respective structures.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    enums::*,
    serialization,
    types::{Age, Effect, Released, SkillKind, ID},
    IMAGE_DATA_URI,
};

use super::Height;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Student {
    /// The **[`ID`]** of the student.
    pub id: ID,
    /// The **[`Released`]** status of the student.
    #[serde(alias = "IsReleased")]
    pub released: Released,
    pub default_order: u32,
    pub path_name: String,
    pub dev_name: String,
    /// The name of the student as presented in the data, and can have an associated tag alongside it.
    /// An example would be **`Toki (Bunny)`**.
    pub name: String,
    /// The **[`Age`]** of the student.
    #[serde(alias = "CharacterAge")]
    pub age: Age,
    /// The first name of the student. e.g., Ichinose.
    #[serde(alias = "PersonalName")]
    pub first_name: String,
    /// The last name/surname (family name) of the student.
    #[serde(alias = "FamilyName")]
    pub last_name: String,
    /// Also known as the **profile** of the student. Provides a brief explanation of their background.
    #[serde(
        alias = "ProfileIntroduction",
        deserialize_with = "serialization::deserialize_html_encoded_string"
    )]
    pub description: String,
    school: String,
    club: String,
    /// The amount of stars a [`Student`] is rated.
    #[serde(alias = "StarGrade")]
    pub stars: u8,
    squad_type: String,
    tactic_role: String,
    pub summons: Vec<Summon>,
    position: String,
    bullet_type: String,
    armor_type: String,
    pub street_battle_adaptation: u8,  // todo
    pub outdoor_battle_adaptation: u8, // todo
    pub indoor_battle_adaptation: u8,  // todo
    weapon_type: String,
    weapon_img: String,
    pub cover: bool,            // todo
    pub equipment: Vec<String>, // todo
    #[serde(alias = "CollectionBG")]
    collection_bg: String,
    collection_texture: Option<String>,
    family_name_ruby: Option<String>,
    pub school_year: Option<String>,
    /// The birthday of the student represented as (Month, Day).
    pub birthday: String,
    #[serde(alias = "CharacterSSRNew")]
    character_ssr_new: Option<String>,
    hobby: String,
    /// The voice actor of the student.
    #[serde(alias = "CharacterVoice")]
    pub voice_actor: String,
    /// The birthday of the student represented as (MM/DD).
    #[serde(alias = "BirthDay")]
    pub birthday_short: String,
    /// The illustrator of the art of this student.
    pub illustrator: String,
    /// The designer of this student, often related to the [`illustrator`](Student.illustrator) field.
    pub designer: String,
    char_height_metric: String,
    char_height_imperial: Option<String>,
    pub stability_point: u32,
    pub attack_power_1: u32,
    pub attack_power_100: u32,
    #[serde(alias = "MaxHP1")]
    pub max_hp_1: u32,
    #[serde(alias = "MaxHP100")]
    pub max_hp_100: u32,
    pub defense_power_1: u32,
    pub defense_power_100: u32,
    pub heal_power_1: u32,
    pub heal_power_100: u32,
    pub dodge_point: u32,
    pub accuracy_point: u32,
    pub critical_point: u32,
    pub critical_damage_rate: u32,
    pub ammo_count: u16,
    pub ammo_cost: u16,
    pub range: u16,
    pub regen_cost: u16,
    /// Contains a collection of **[`Skills`][`Skill`]**.
    pub skills: Vec<Skill>,
    pub favor_stat_type: Vec<String>,   // todo
    pub favor_stat_value: Vec<Vec<u8>>, // todo
    pub favor_alts: Vec<u32>,           // todo
    pub memory_lobby: Vec<u8>,          // todo
    /// The name of the music in the students' recollection lobby.
    #[serde(alias = "MemoryLobbyBGM")]
    pub memory_lobby_bgm: String,
    pub favor_item_tags: Vec<String>,        // todo
    pub favor_item_unique_tags: Vec<String>, // todo
    pub is_limited: u8,                      // todo: represent this as enum. Limited::(0?, 1?, 2?)
    pub weapon: Weapon,
    gear: GearKind,
    pub skill_ex_material: Vec<Vec<u16>>,       // todo
    pub skill_ex_material_amount: Vec<Vec<u8>>, // todo
    pub skill_material: Vec<Vec<u16>>,          // todo
    pub skill_material_amount: Vec<Vec<u8>>,    // todo
    /// Image data related to the [`Student`].
    #[serde(skip)]
    pub image: StudentImageData,
}

impl std::hash::Hash for Student {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Student {
    /// Gets the full name of a student, with the **surname (`family_name`)** coming first.
    pub fn full_name_last(&self) -> String {
        format!("{} {}", self.last_name, self.first_name)
    }

    /// Gets the full name of a student, with the **first name (`personal_name`)** coming first.
    pub fn full_name_first(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// The quote said when obtaining this student (if an SSR).
    pub fn quote_ssr(&self) -> Option<String> {
        self.character_ssr_new.as_ref().and_then(|quote| {
            (!quote.is_empty()).then_some(html_escape::decode_html_entities(&quote).into())
        })
    }

    /// Gets the [`Height`] of the [`Student`].
    pub fn height(&self) -> Height {
        Height {
            metric: self.char_height_metric.clone(),
            imperial: self
                .char_height_imperial
                .as_ref()
                .map(|height| html_escape::decode_html_entities(&height).to_string()),
        }
    }

    /// The hobby of the student if they have one.
    pub fn hobby(&self) -> Option<String> {
        (self.hobby != "None").then_some(self.hobby.clone())
    }

    /// Tries to get a **[`Gear`]** from data.
    pub fn gear(&self) -> Option<Gear> {
        self.gear.get()
    }

    /// Gets the **[`School`]** of the student.
    pub fn school(&self) -> School {
        School::from_str(&self.school).unwrap_or(School::Unknown(self.school.clone()))
    }

    /// Gets the **[`TacticalRole`]** of the student.
    pub fn tactical_role(&self) -> TacticalRole {
        TacticalRole::from_str(&self.tactic_role)
            .unwrap_or(TacticalRole::Unknown(self.tactic_role.clone()))
    }

    /// Gets the **[`Squad`]** of the student.
    pub fn squad(&self) -> Squad {
        Squad::from_str(&self.squad_type).unwrap_or(Squad::Unknown(self.squad_type.clone()))
    }

    /// Gets the **[`Armor`]** of the student.
    pub fn armor(&self) -> Armor {
        Armor::from_str(&self.armor_type).unwrap_or(Armor::Unknown(self.armor_type.clone()))
    }

    /// Gets the **[`Position`]** of the student.
    pub fn position(&self) -> Position {
        Position::from_str(&self.position).unwrap_or(Position::Unknown(self.position.clone()))
    }

    /// Gets the **[`BulletType`]** of the student.
    pub fn bullet_type(&self) -> BulletType {
        BulletType::from_str(&self.bullet_type)
            .unwrap_or(BulletType::Unknown(self.bullet_type.clone()))
    }

    /// Gets the **[`Club`]** of the student.
    pub fn club(&self) -> Club {
        Club::from_str(&self.club).unwrap_or(Club::Unknown(self.club.clone()))
    }

    /// Gets the **[`WeaponType`]** of the student.
    pub fn weapon_type(&self) -> WeaponType {
        WeaponType::from_str(&self.weapon_type)
            .unwrap_or(WeaponType::Unknown(self.weapon_type.clone()))
    }
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(ID: {}, Name: {}, Age: {}, School: {})",
            self.id,
            self.full_name_last(),
            self.age,
            self.school()
        )
    }
}

/// A [`Student`] specific skill.
///
/// A great portion of it is raw data that has not been fully deserialized and represented.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Skill {
    #[serde(alias = "SkillType")]
    pub kind: SkillKind,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub parameters: Option<Vec<Vec<String>>>,
    pub cost: Option<Vec<u32>>,
    pub icon: Option<String>,
    pub effects: Vec<Effect>,
}

/// A [`Student`] specific summon.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Summon {
    pub id: ID,
    pub source_skill: String,
    pub inherit_caster_stat: Vec<String>,
    pub inherit_caster_amount: Option<Vec<Vec<u32>>>,
}

/// The kind of [`Gear`] that the data may represent.
///
/// There is an issue where Gear in data is represented as `"gear": {}`, therefore this is a mitigation against that.
/// If you have a better implementation of handling this, as in allowing for me to represent the data as an `Option<Gear>`, please send a PR.
/// todo: Could use #[serde(skip_serializing_if = "...")]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum GearKind {
    Present(Gear),
    Empty(Empty),
}
impl GearKind {
    /// Attempts to get a **[`Gear`]**, though if it gets an [`GearKind::Empty`], it will return [`None`].
    pub fn get(&self) -> Option<Gear> {
        match self {
            Self::Present(gear) => Some(gear.clone()),
            Self::Empty(_) => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Gear {
    /// Whether a specific gear was **[`Released`]** or not in a specific region.
    pub released: Released,
    pub stat_type: Vec<String>,
    pub stat_value: Vec<Vec<u16>>,
    pub name: String,
    #[serde(alias = "Desc")]
    pub description: String,
    icon: String,
    pub tier_up_material: Vec<Vec<u16>>,
    pub tier_up_material_amount: Vec<Vec<u8>>,
}
impl Gear {
    /// Returns the url of a gear icon.
    pub fn icon_url(&self) -> String {
        format!("{IMAGE_DATA_URI}/gear/{}", self.icon)
    }
}
/// There is an issue where Gear in data is represented as `"gear": {}`, therefore this is a mitigation against that.
/// If you have a better implementation of handling this, as in allowing for me to represent the data as an `Option<...>`, please send a PR.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Empty {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Weapon {
    /// The name of the weapon.
    pub name: String,
    /// The description of the weapon.
    #[serde(alias = "Desc")]
    pub description: String,
    pub adaptation_type: String,
    pub adaptation_value: u8,
    pub attack_power_1: u32,
    pub attack_power_100: u32,
    #[serde(alias = "MaxHP1")]
    pub max_hp_1: u32,
    #[serde(alias = "MaxHP100")]
    pub max_hp_100: u32,
    pub heal_power_1: u32,
    pub heal_power_100: u32,
    pub stat_level_up_type: LevelUpType,
}

/// The level-up type of a **[`Weapon`]**.
#[derive(Debug, strum_macros::Display, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum LevelUpType {
    Standard,
    Premature,
    LateBloom,
    #[serde(other)]
    Unknown,
}

/// Image data related to a **[`Student`]**.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct StudentImageData {
    /// The portrait associated with this **[`Student`]**.
    pub portrait: Portrait,
    /// The **[`Weapon`]** icon url belonging to the **[`Student`]**.
    pub weapon_icon_url: String,
}

impl StudentImageData {
    /// Creates itself from a given **[`Student`]**.
    pub fn new(student: &Student) -> Self {
        Self {
            portrait: Portrait {
                full_body_url: format!("{IMAGE_DATA_URI}/student/portrait/{}.webp", student.id.0),
                icon_url: format!("{IMAGE_DATA_URI}/student/icon/{}.webp", student.id.0),
                alternative_full_body_url: format!(
                    "{IMAGE_DATA_URI}/student/portrait/{}_2.webp",
                    student.id.0
                ),
                bg_url: format!("{IMAGE_DATA_URI}/background/{}.jpg", student.collection_bg),
            },
            weapon_icon_url: format!("{IMAGE_DATA_URI}/weapon/{}.webp", student.weapon_img),
        }
    }
}

/// Contains portrait data of a **[`Student`]**.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Portrait {
    /// The full body image url associated with this **[`Student`]**.
    pub full_body_url: String,
    /// The icon url associated with this **[`Student`]**.
    pub icon_url: String,
    /// If there is an alternative full-body image url associated with this **[`Student`]**. There is a chance for this
    pub alternative_full_body_url: String,
    /// The background image url associated with this **[`Student`]**.
    pub bg_url: String,
}
