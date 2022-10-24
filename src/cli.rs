use rick_and_morty as rm;
use serde::Serialize;

pub struct Field {
    label: String,
    value: String,
}
pub trait PrettyPrint {
    fn get_title(&self) -> &str;

    fn get_fields(&self) -> Vec<Field>;

    fn pretty_print(&self) {
        let title = self.get_title();
        println!("{title}");
        println!("{}", "-".repeat(title.len()));
        for field in self.get_fields() {
            println!("{}: {}", field.label, field.value);
        }
    }
}

impl PrettyPrint for rm::character::Character {
    fn get_title(&self) -> &str {
        &self.name
    }

    fn get_fields(&self) -> Vec<Field> {
        vec![
            Field {
                label: String::from("id"),
                value: self.id.to_string(),
            },
            Field {
                label: String::from("name"),
                value: self.name.clone(),
            },
            Field {
                label: String::from("status"),
                value: self.status.clone(),
            },
            Field {
                label: String::from("species"),
                value: self.species.clone(),
            },
            Field {
                label: String::from("character_type"),
                value: self.character_type.clone(),
            },
            Field {
                label: String::from("origin"),
                value: self.origin.name.clone(),
            },
            Field {
                label: String::from("location"),
                value: self.location.name.clone(),
            },
            Field {
                label: String::from("gender"),
                value: self.gender.clone(),
            },
            Field {
                label: String::from("created"),
                value: self.created.clone(),
            },
        ]
    }
}

impl PrettyPrint for rm::location::Location {
    fn get_title(&self) -> &str {
        &self.name
    }

    fn get_fields(&self) -> Vec<Field> {
        vec![
            Field {
                label: String::from("id"),
                value: self.id.to_string(),
            },
            Field {
                label: String::from("name"),
                value: self.name.clone(),
            },
            Field {
                label: String::from("location_type"),
                value: self.location_type.clone(),
            },
            Field {
                label: String::from("dimension"),
                value: self.dimension.clone(),
            },
            Field {
                label: String::from("created"),
                value: self.created.clone(),
            },
        ]
    }
}

impl PrettyPrint for rm::episode::Episode {
    fn get_title(&self) -> &str {
        &self.name
    }

    fn get_fields(&self) -> Vec<Field> {
        vec![
            Field {
                label: String::from("id"),
                value: self.id.to_string(),
            },
            Field {
                label: String::from("name"),
                value: self.name.clone(),
            },
            Field {
                label: String::from("air_date"),
                value: self.air_date.clone(),
            },
            Field {
                label: String::from("episode"),
                value: self.episode.clone(),
            },
            Field {
                label: String::from("created"),
                value: self.created.clone(),
            },
        ]
    }
}

pub fn print_entity(entity: &(impl PrettyPrint + Serialize), pretty: bool) {
    if pretty {
        entity.pretty_print();
    } else {
        println!("{}", serde_json::to_string(entity).unwrap());
    }
}

pub fn print_entities(entities: &Vec<impl PrettyPrint + Serialize>, pretty: bool) {
    if pretty {
        for (i, entity) in entities.iter().enumerate() {
            if i > 0 {
                println!();
            }
            entity.pretty_print();
        }
    } else {
        println!("{}", serde_json::to_string(entities).unwrap());
    }
}

pub async fn get_character(id: i64) -> Result<rm::character::Character, rm::Error> {
    rm::character::get(id).await
}

pub async fn get_all_characters() -> Result<Vec<rm::character::Character>, rm::Error> {
    rm::character::get_all().await
}

pub async fn get_location(id: i64) -> Result<rm::location::Location, rm::Error> {
    rm::location::get(id).await
}

pub async fn get_all_locations() -> Result<Vec<rm::location::Location>, rm::Error> {
    rm::location::get_all().await
}

pub async fn get_episode(id: i64) -> Result<rm::episode::Episode, rm::Error> {
    rm::episode::get(id).await
}

pub async fn get_all_episodes() -> Result<Vec<rm::episode::Episode>, rm::Error> {
    rm::episode::get_all().await
}
