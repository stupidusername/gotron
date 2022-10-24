use std::fmt::Display;
use thiserror::Error;

use rick_and_morty as rm;
use serde::Serialize;

pub struct Field<'a> {
    label: &'a str,
    value: &'a dyn Display,
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
                label: "id",
                value: &self.id,
            },
            Field {
                label: "name",
                value: &self.name,
            },
            Field {
                label: "status",
                value: &self.status,
            },
            Field {
                label: "species",
                value: &self.species,
            },
            Field {
                label: "character_type",
                value: &self.character_type,
            },
            Field {
                label: "origin",
                value: &self.origin.name,
            },
            Field {
                label: "location",
                value: &self.location.name,
            },
            Field {
                label: "gender",
                value: &self.gender,
            },
            Field {
                label: "created",
                value: &self.created,
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
                label: "id",
                value: &self.id,
            },
            Field {
                label: "name",
                value: &self.name,
            },
            Field {
                label: "location_type",
                value: &self.location_type,
            },
            Field {
                label: "dimension",
                value: &self.dimension,
            },
            Field {
                label: "created",
                value: &self.created,
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
                label: "id",
                value: &self.id,
            },
            Field {
                label: "name",
                value: &self.name,
            },
            Field {
                label: "air_date",
                value: &self.air_date,
            },
            Field {
                label: "episode",
                value: &self.episode,
            },
            Field {
                label: "created",
                value: &self.created,
            },
        ]
    }
}

pub fn print_entity(entity: &(impl PrettyPrint + Serialize), output: &super::Output) {
    match output {
        super::Output::Pretty => {
            entity.pretty_print();
        }
        super::Output::Json => {
            println!("{}", serde_json::to_string(entity).unwrap());
        }
    };
}

pub fn print_entities(entities: &Vec<impl PrettyPrint + Serialize>, output: &super::Output) {
    match output {
        super::Output::Pretty => {
            for (i, entity) in entities.iter().enumerate() {
                if i > 0 {
                    println!();
                }
                entity.pretty_print();
            }
        }
        super::Output::Json => {
            println!("{}", serde_json::to_string(entities).unwrap());
        }
    };
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("could not retrive information from Rick and Morty API")]
    ApiClientError(#[from] rm::Error),
}

pub async fn get_character(id: i64) -> Result<rm::character::Character, CliError> {
    rm::character::get(id)
        .await
        .map_err(|e| CliError::ApiClientError(e))
}

pub async fn get_all_characters() -> Result<Vec<rm::character::Character>, CliError> {
    rm::character::get_all()
        .await
        .map_err(|e| CliError::ApiClientError(e))
}

pub async fn get_location(id: i64) -> Result<rm::location::Location, CliError> {
    rm::location::get(id)
        .await
        .map_err(|e| CliError::ApiClientError(e))
}

pub async fn get_all_locations() -> Result<Vec<rm::location::Location>, CliError> {
    rm::location::get_all()
        .await
        .map_err(|e| CliError::ApiClientError(e))
}

pub async fn get_episode(id: i64) -> Result<rm::episode::Episode, CliError> {
    rm::episode::get(id)
        .await
        .map_err(|e| CliError::ApiClientError(e))
}

pub async fn get_all_episodes() -> Result<Vec<rm::episode::Episode>, CliError> {
    rm::episode::get_all()
        .await
        .map_err(|e| CliError::ApiClientError(e))
}
