use Allergen::*;

pub struct Allergies(u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

const ALLERGIES: [Allergen; 8] = [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score % 256)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let &Allergies(score) = self;
        score & (*allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGIES.iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .cloned()
            .collect()
    }
}
