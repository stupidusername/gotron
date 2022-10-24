use rick_and_morty as rm;

struct Field {
    label: String,
    value: String,
}
trait PrettyPrint {
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

pub async fn show_character(id: i64) {
    let c = rm::character::get(id).await;
    match c {
        Ok(res) => res.pretty_print(),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn list_characters() {
    let c = rm::character::get_all().await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn show_location(id: i64) {
    let c = rm::location::get(id).await;
    match c {
        Ok(res) => res.pretty_print(),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn list_locations() {
    let c = rm::location::get_all().await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn show_episode(id: i64) {
    let c = rm::episode::get(id).await;
    match c {
        Ok(res) => res.pretty_print(),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn list_episodes() {
    let c = rm::episode::get_all().await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}
