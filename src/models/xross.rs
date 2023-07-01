use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Education {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Language {
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComputerSkill {
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Experence {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Xross {
    pub name: String,
    pub photo: String,
    pub phone: String,
    pub website: Option<String>,
    pub mail: Option<String>,
    pub bio: Option<String>,
    pub educations: HashMap<String, Education>,
    pub profile_content: String,
    pub location: String,
    pub languages: HashMap<String, Language>,
    pub computer_skills: HashMap<String, ComputerSkill>,
    pub experences: HashMap<String, Experence>,
}
