use crate::{game::Serialise, to_le_bytes};

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum Type {
    Unknown = 0x00,
    Evade = 0x01,
    MainHand = 0x0A,
    OffHand = 0x0B,
}

#[derive(Debug, Clone)]
pub struct Hit {
    /// Damage is a signed integer, Negative numbers might be healing
    pub damage: i32,
    /// Describes any special effects, like if there is a dodge, crit etc..
    ty: Type,
}

impl Hit {
    pub fn new(damage: i32, ty: Type) -> Self {
        Self { damage, ty }
    }
}

impl Serialise for Hit {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        to_le_bytes!(len, buf, self.damage);
        to_le_bytes!(len, buf, self.ty as u16);
        len
    }
}
