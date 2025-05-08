#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Id {
    SpringOutlaw = 210484,
    LakeSpirit = 210660,
    TahabataPyrelord = 217166,
    StarvedMosbear = 210564,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum State {
    Alive = 0x41,
    DeadNoLoot = 0x07,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Name {
    SprigOutlaw = 300623,
    StarvedMosbear = 300703,
}

// TODO: Verify this
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Type {
    Attackable = 0,
    Peace = 2,
    Aggressive = 8,
    Invulernable = 10,
    Friend = 38,
    Support = 54,
}
