use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::base::{Experence, Education, OtherSkill, Language};

#[derive(Deserialize, Serialize)]
pub struct CareerCraft {
    pub name: String,
    pub bio: String,
    pub photo: String,
    pub about: String,
    pub experences: HashMap<String, Experence>,
    pub educations: HashMap<String, Education>,
    pub location: String,
    pub phone: String,
    pub mail: Option<String>,
    pub website: Option<String>,
    pub skills: HashMap<String, OtherSkill>,
    pub languages: HashMap<String, Language>,
}
