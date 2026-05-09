use std::collections::HashMap;
use std::convert::TryFrom;

use crate::models::course::Course;

#[derive(Debug)]
pub struct FormData {
    pub first_name: String,
    pub last_name: String,
    pub dog_name: String,
    pub breed: String,
    pub courses: Vec<Course>,
}

impl FormData {
    pub fn is_valid(&self) -> bool {
        !self.first_name.is_empty()
            && !self.last_name.is_empty()
            && !self.dog_name.is_empty()
            && !self.breed.is_empty()
            && !self.courses.is_empty()
    }
}

fn is_checked(data: &HashMap<String, String>, key: &str) -> bool {
    data.get(key).map(|v: &String| v == "Yes").unwrap_or(false)
}

impl TryFrom<&HashMap<String, String>> for FormData {
    type Error = anyhow::Error;

    fn try_from(data: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let first_name: String = data.get("owner_name").cloned().unwrap_or_default();
        let last_name: String = data.get("owner_surname").cloned().unwrap_or_default();
        let dog_name: String = data.get("dog_name").cloned().unwrap_or_default();
        let breed: String = data.get("dog_breed").cloned().unwrap_or_default();

        let mut courses: Vec<Course> = Vec::new();

        if is_checked(data, "course_welpen") {
            courses.push(Course::Welpenkurs);
        }
        if is_checked(data, "course_junghundenkurs") {
            courses.push(Course::Junghundekurs);
        }
        if is_checked(data, "course_agility") {
            courses.push(Course::Agility);
        }
        if is_checked(data, "course_obedience") {
            courses.push(Course::Obedience);
        }
        if is_checked(data, "course_mantrailing") {
            courses.push(Course::Mantrailing);
        }

        let form: FormData = FormData {
            first_name,
            last_name,
            dog_name,
            breed,
            courses,
        };

        if !form.is_valid() {
            return Err(anyhow::anyhow!("Formular unvollständig"));
        }

        Ok(form)
    }
}