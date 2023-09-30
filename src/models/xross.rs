use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::base::{Education, Language, ComputerSkill, Experence};


#[derive(Deserialize, Serialize)]
pub struct Xross {
    pub name: String,
    pub photo: String,
    pub location: String,
    pub phone: String,
    pub mail: Option<String>,
    pub website: Option<String>,
    pub bio: Option<String>,
    pub educations: HashMap<String, Education>,
    pub profile_content: String,
    pub languages: HashMap<String, Language>,
    pub computer_skills: HashMap<String, ComputerSkill>,
    pub experences: HashMap<String, Experence>,
}
