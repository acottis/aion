use crate::consume_le_bytes;

pub mod damage;

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    x: f32,
    y: f32,
    z: f32,
}

impl Coord {
    pub const LEN: usize = 12;

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn serialise(&self, buf: &mut [u8]) -> usize {
        assert!(buf.len() >= Self::LEN);
        buf[..4].copy_from_slice(&self.x.to_le_bytes());
        buf[4..8].copy_from_slice(&self.y.to_le_bytes());
        buf[8..12].copy_from_slice(&self.z.to_le_bytes());

        Self::LEN
    }

    pub fn deserialise(buf: &[u8; Self::LEN]) -> Self {
        let mut _len = 0;
        Self {
            x: consume_le_bytes!(_len, buf, f32),
            y: consume_le_bytes!(_len, buf, f32),
            z: consume_le_bytes!(_len, buf, f32),
        }
    }

    pub fn distance(&self, location: &Coord) -> f32 {
        let dx = self.x - location.x;
        let dy = self.y - location.y;
        let dz = self.z - location.z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        distance
    }
}

impl std::ops::Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// When an entity moves these bits are set to tell the game about
/// the type of movement
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct MoveType(u8);

impl MoveType {
    pub const STOP: MoveType = MoveType(0x00);
    pub const FALL: MoveType = MoveType(0x08);
    pub const GLIDE: MoveType = MoveType(0x04);
    pub const CONTINUE: MoveType = MoveType(0x80);
    pub const TURN: MoveType = MoveType(0x40);
    /// Mouse click to move mostly
    pub const DIRECT: MoveType = MoveType(0x20);

    pub const NPC_DIRECT: MoveType = MoveType(0xE0);
    pub const NPC_RUN_FAST: MoveType = MoveType(0xE2);
    pub const NPC_WALK_SLOW: MoveType = MoveType(0xE4);

    pub const fn from_bits(bits: u8) -> Self {
        MoveType(bits)
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    #[inline(always)]
    pub fn contains(self, other: MoveType) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl From<u8> for MoveType {
    fn from(value: u8) -> Self {
        MoveType::from_bits(value)
    }
}

/// The direct a character is moving 0x00 represents south
/// The range is 0x00 - 0x77 (0 - 119) (mod 120)
#[derive(Debug, Clone)]
pub struct Direction(u8);

impl Direction {
    const MODULUS: u8 = 120;

    pub fn new(direction: u8) -> Self {
        Self(direction % Self::MODULUS)
    }

    pub fn bits(&self) -> u8 {
        self.0
    }
}
