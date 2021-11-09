// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

#[derive(Debug)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let mana = match self {
            &Player{ health, .. } if health !=  0 => return None,
            &Player{ level, .. }  if level  <  10 => None,
            &Player{ level, .. }  if level  >= 10 => Some(100),
            _ => unimplemented!("No requirements for other cases. Player: {:?}", self)
        };
        Some(Player { health: 100, mana, level: self.level })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self {
            Player{ mana: Some(mana), .. } if *mana >= mana_cost=> {
                let remaining_mana = *mana - mana_cost;
                self.mana = Some(remaining_mana);
                mana_cost * 2
            },
            Player{ mana: Some(_), .. } => 0,
            Player{ mana: None, .. } => {
                self.health = self.health.checked_sub(mana_cost).unwrap_or_default();
                0
            },
        }
    }
}
