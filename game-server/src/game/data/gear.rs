use crate::{copy_bytes, game::Serialise, to_le_bytes};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Slot {
    MainHand = 0,
    OffHand = 1,
    Head = 2,
    Chest = 3,
    Gloves = 4,
    Boots = 5,
    EarRingLeft = 6,
    EarRingRight = 7,
    RingLeft = 8,
    RingRight = 9,
    Necklace = 10,
    Shoulder = 11,
    Legs = 12,
    PowerShardLeft = 13,
    PowerShardRight = 14,
    Wings = 15,
    Waist = 16,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SlotType {
    Invisible = 0,
    Visible,
    VisibleOffHand,
}

#[derive(Debug, Clone)]
pub struct Item {
    id: u32,
    slot_type: SlotType,
}
impl Item {
    pub fn new(id: u32, slot_type: SlotType) -> Item {
        Self { id, slot_type }
    }
}

/// Only the first 16 items are "visible" items
#[derive(Debug, Clone)]
pub struct Gear {
    inner: Vec<Option<Item>>,
}

impl Gear {
    pub fn new() -> Self {
        Self {
            inner: vec![None; 64],
        }
    }

    /// Sets the specified gear slot with an item
    pub fn set(&mut self, slot: Slot, item: Item) {
        self.inner[slot as usize] = Some(item);
    }

    pub fn serialiase_put_user(&self, buf: &mut [u8]) -> usize {
        let mut len = 2;
        let mut mask = 0;

        for (i, item) in self.inner[..16].iter().enumerate() {
            if let Some(item) = item {
                mask |= 1 << i;
                to_le_bytes!(len, buf, item.id);
                len += 14;
            }
        }
        buf[0..2].copy_from_slice(&(mask as u16).to_le_bytes());
        len
    }
    pub fn serialiase_character_list(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;

        for item in &self.inner[..16] {
            if let Some(item) = item {
                to_le_bytes!(len, buf, item.slot_type as u8);
                to_le_bytes!(len, buf, item.id);
                len += 12;
            } else {
                len += 17;
            }
        }

        len
    }
}

#[derive(Debug, Clone)]
pub struct RawItem {
    pub id: u32,
    item_id: u32,
    unknown_id: u32,
    position: u16,
    raw: Vec<u8>,
}
impl RawItem {
    pub fn new(
        id: u32,
        item_id: u32,
        unknown_id: u32,
        position: u16,
        raw: Vec<u8>,
    ) -> Self {
        Self {
            raw,
            id,
            item_id,
            unknown_id,
            position,
        }
    }
}
impl Serialise for RawItem {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        let padding = [0u8; 25];
        to_le_bytes!(len, buf, self.id);
        to_le_bytes!(len, buf, self.item_id);
        to_le_bytes!(len, buf, 0x24 as u16);
        to_le_bytes!(len, buf, self.unknown_id);
        to_le_bytes!(len, buf, 0x0 as u16);
        to_le_bytes!(len, buf, (padding.len() + self.raw.len()) as u16);
        copy_bytes!(len, buf, self.raw);
        copy_bytes!(len, buf, padding);
        to_le_bytes!(len, buf, self.position);
        to_le_bytes!(len, buf, 0x0 as u8);
        len
    }
}

#[derive(Debug, Clone)]
pub struct LootItem {
    id: u32,
    count: u32,
}

impl LootItem {
    pub fn new(id: u32, count: u32) -> Self {
        Self { id, count }
    }
}

impl Serialise for LootItem {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        to_le_bytes!(len, buf, self.id);
        to_le_bytes!(len, buf, self.count);
        len
    }
}
