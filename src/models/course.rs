#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Course {
    Welpenkurs,
    Junghundekurs,
    Agility,
    Obedience,
    Mantrailing,
}

impl Course {
    pub fn display_name(&self) -> &'static str {
        match self {
            Course::Welpenkurs => "Welpenkurs",
            Course::Junghundekurs => "Junghundekurs",
            Course::Agility => "Agility",
            Course::Obedience => "Obedience",
            Course::Mantrailing => "Mantrailing",
        }
    }
}
