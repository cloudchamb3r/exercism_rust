pub struct Player {
    pub health: u32,
    pub mana: Option<u32>, // None til lv.10 
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0  {
            let mut mana = None; 
            if self.level >= 10 {
                mana = Some(100);
            }

            return Some(Player {
                health: 100,
                mana,
                level: self.level,
            });
        }
        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = &mut self.mana {
            if *mana >= mana_cost {
                *mana -= mana_cost;
                return 2 * mana_cost;
            }
            0
        } else {
            if let Some(subtracted) = self.health.checked_sub(mana_cost) {
                self.health = subtracted;
            } else {
                self.health = 0;
            }
            0
        }
    }
}
