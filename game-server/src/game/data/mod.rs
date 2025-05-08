use super::Deserialise;

pub mod gear;
pub mod npc;
pub mod skill;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Emote {
    Dance = 19,
    Unknown(u8),
}
impl From<Emote> for u8 {
    fn from(value: Emote) -> Self {
        match value {
            Emote::Dance => 19,
            Emote::Unknown(emote) => emote,
        }
    }
}
impl Deserialise for Emote {
    fn deserialise(buf: &[u8]) -> Self
    where
        Self: Sized,
    {
        match buf.first().unwrap() {
            19 => Self::Dance,
            emote => Self::Unknown(*emote),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ActionType {
    Target,
    Jump,
    Rest,
    EndRest,
    Fly,
    Land,
    Die,
    /// Emotes have a subaction for each emote type
    StartEmote(Emote),
    EndEmote,
    Attack,
    SheathWeapon,
    ToggleWalk,
    EntityDrawWeapon,
    TogglePowerShard,
    CharacterDrawWeapon,
    CharacterSheathWeapon,
    Loot,
    EndLoot,
    EndGlideSprint,
    Unknown(u8),
}
impl From<ActionType> for u8 {
    fn from(value: ActionType) -> Self {
        match value {
            ActionType::Target => 0,
            ActionType::Jump => 1,
            ActionType::Rest => 2,
            ActionType::EndRest => 3,
            ActionType::Fly => 13,
            ActionType::Land => 14,
            ActionType::Die => 18,
            ActionType::StartEmote(..) => 21,
            ActionType::EndEmote => 22,
            ActionType::Attack => 24,
            ActionType::SheathWeapon => 25,
            ActionType::ToggleWalk => 26,
            ActionType::EntityDrawWeapon => 35,
            ActionType::TogglePowerShard => 36,
            ActionType::CharacterDrawWeapon => 38,
            ActionType::CharacterSheathWeapon => 39,
            ActionType::Loot => 40,
            ActionType::EndLoot => 41,
            ActionType::EndGlideSprint => 56,
            ActionType::Unknown(action) => action,
        }
    }
}
impl Deserialise for ActionType {
    fn deserialise(buf: &[u8]) -> Self
    where
        Self: Sized,
    {
        match buf.first().unwrap() {
            0 => Self::Target,
            1 => Self::Jump,
            2 => Self::Rest,
            3 => Self::EndRest,
            13 => Self::Fly,
            14 => Self::Land,
            18 => Self::Die,
            21 => Self::StartEmote(Emote::deserialise(&buf[1..])),
            22 => Self::EndEmote,
            24 => Self::Attack,
            25 => Self::SheathWeapon,
            26 => Self::ToggleWalk,
            35 => Self::EntityDrawWeapon,
            36 => Self::TogglePowerShard,
            38 => Self::CharacterDrawWeapon,
            39 => Self::CharacterSheathWeapon,
            40 => Self::Loot,
            41 => Self::EndLoot,
            56 => Self::EndGlideSprint,
            action => Self::Unknown(*action),
        }
    }
}
