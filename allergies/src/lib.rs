pub struct Allergies {
    score : u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score: score & 255 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.score & (*allergen as u32)) > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut ret = vec![];
        if (1 & self.score) > 0 { ret.push(Allergen::Eggs); }
        if (2 & self.score) > 0 { ret.push(Allergen::Peanuts); }
        if (4 & self.score) > 0 { ret.push(Allergen::Shellfish); }
        if (8 & self.score) > 0 { ret.push(Allergen::Strawberries); }
        if (16 & self.score) > 0 { ret.push(Allergen::Tomatoes); }
        if (32 & self.score) > 0 { ret.push(Allergen::Chocolate); }
        if (64 & self.score) > 0 { ret.push(Allergen::Pollen); }
        if (128 & self.score) > 0 { ret.push(Allergen::Cats); }
        ret
    }
}
