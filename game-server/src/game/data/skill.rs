use crate::{game::Serialise, to_le_bytes};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SkillType {
    Normal = 0,
    Stigma = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct Skill {
    /// Major skill levels are different ID's Ambush I and Ambush II have
    /// different id's for example
    id: u16,
    /// Ether crafting level or skill level
    level: u32,
    /// Set to 1 if it is a crafting ability
    unknown: u32,
    typ: SkillType,
}

impl Skill {
    pub fn new_crafting(id: u16, level: u32) -> Self {
        Self {
            id,
            level,
            unknown: 1,
            typ: SkillType::Normal,
        }
    }
    pub fn morph() -> Self {
        Self {
            id: 4009,
            level: 1,
            unknown: 0,
            typ: SkillType::Normal,
        }
    }
    pub fn new(id: u16, level: u32, typ: SkillType) -> Self {
        Self {
            id,
            level,
            unknown: 0,
            typ,
        }
    }
}

impl Serialise for Skill {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut ptr = 0;

        to_le_bytes!(ptr, buf, self.id);
        to_le_bytes!(ptr, buf, self.level);
        to_le_bytes!(ptr, buf, self.unknown);
        to_le_bytes!(ptr, buf, self.typ as u8);

        ptr
    }
}
