use krypt::game::encrypt_server_opcode;

use crate::{
    copy_bytes,
    game::{
        character::Character,
        data::{
            gear::{Gear, LootItem, RawItem},
            npc,
            skill::Skill,
            ActionType,
        },
        engine::{damage::Hit, Coord, Direction, MoveType},
        entity::Entity,
    },
    to_le_bytes,
};

use crate::game::Serialise;

const SERVER_BYTE: u8 = 0x40;
const LENGTH_LEN: usize = 2;

const VERSION_CHECK: u16 = 0x0000;
const STATUS: u16 = 0x0001;
const HIT_POINT: u16 = 0x0003;
const HIT_POINT_OTHER: u16 = 0x0005;
const ENTER_WORLD_CHECK: u16 = 0x000D;
const PUT_NPC: u16 = 0x000E;
const WORLD: u16 = 0x000F;
const MESSAGE_CODE: u16 = 0x0019;
const LOAD_INVENTORY: u16 = 0x001A;
const CHANGE_ITEM_DESC: u16 = 0x001D;
const LOAD_CLIENT_SETTINGS: u16 = 0x001E;
const PUT_USER: u16 = 0x0020;
const USE_SKILL: u16 = 0x0021;
const TIME: u16 = 0x0026;
const ACTION: u16 = 0x0025;
const SYNC_TIME: u16 = 0x0027;
const NPC_CHANGED_TARGET: u16 = 0x0028;
const TARGET_INFO: u16 = 0x0029;
const SKILL_SUCCEDED: u16 = 0x002B;
const ADD_SKILL: u16 = 0x002C;
const ABNORMAL_STATUS: u16 = 0x0031;
const ABNORMAL_STATUS_OTHER: u16 = 0x0032;
const ATTACK: u16 = 0x0036;
const MOVE_NEW: u16 = 0x0037;
const WEATHER: u16 = 0x0042;
const INVISIBLE_LEVEL: u16 = 0x0043;
const KEY: u16 = 0x0047;
const ASK_QUIT_RESULT: u16 = 0x0061;
const LOAD_ITEM_COOLTIME: u16 = 0x0066;
const BUDDY_LIST: u16 = 0x0083;
const SA_ACCOUNT_ITEM_NOTI: u16 = 0x0088;
const WORLD_SCENE_STATUS: u16 = 0x008B;
const ALIVE: u16 = 0x008D;
const CUSTOM_ANIM: u16 = 0x0093;
const TITLE: u16 = 0x00AF;
const SECOND_PASSWORD: u16 = 0x00B0;
const L2AUTH_LOGIN_CHECK: u16 = 0x00C6;
const CHARACTER_LIST: u16 = 0x00C7;
const CREATE_CHARACTER: u16 = 0x00C8;
const LOOT_ITEMLIST: u16 = 0x00CD;
const LOOT: u16 = 0xCC;
const RECIPE_LIST: u16 = 0x00CE;
const CUR_STATUS: u16 = 0x00E3;
const CHANGE_CHANNEL: u16 = 0x00E5;
const SIGN_CLIENT: u16 = 0x00E6;
const BUILDER_LEVEL: u16 = 0x00EE;
const WORLD_INFO: u16 = 0x00EC;
const INVINCIBLE_TIME: u16 = 0x00FE;
const RECONNECT_KEY: u16 = 0x00FF;
const _0102: u16 = 0x0102;
const REPLY_NP_LOGIN_GAMESVR: u16 = 0x0106;
const REPLY_NP_AUTH_TOKEN: u16 = 0x0108;
const NPSHOP_GOODS_COUNT: u16 = 0x010A;
const SERVER_ENV: u16 = 0x010E;
const RESULT_PASSPORT: u16 = 0x0113;
const GAMEPASS_INFO: u16 = 0x0119;
const READY_ENTER_WORLD: u16 = 0x011D;
const _0151: u16 = 0x0151;

#[derive(Debug, Clone)]
pub enum Message {
    VersionCheck(VersionCheck),
    HitPoint(HitPoint),
    HitPointOther(HitPointOther),
    Status(Status),
    World(World),
    EnterWorldCheck(EnterWorldCheck),
    ReconnectKey(ReconnectKey),
    LoadClientSettings(LoadClientSettings),
    PutUser(PutUser),
    TargetInfo(TargetInfo),
    SyncTime(SyncTime),
    NpcChangedTarget(NpcChangedTarget),
    MoveNew(MoveNew),
    Key(Key),
    BuddyList(BuddyList),
    Alive(Alive),
    Attack(Attack),
    L2AuthLoginCheck(L2AuthLoginCheck),
    CharacterList(CharacterList),
    RecipeList(RecipeList),
    CurStatus(CurStatus),
    BuilderLevel(BuilderLevel),
    ReplyNpLoginGamesvr(ReplyNpLoginGamesvr),
    ReplyNpAuthToken(ReplyNpAuthToken),
    NpShopGoodsCount(NpShopGoodsCount),
    ReadyEnterWorld(ReadyEnterWorld),
    ServerEnv(ServerEnv),
    SaAccountItemNoti(SaAccountItemNoti),
    SecondPassword(SecondPassword),
    AddSkill(AddSkill),
    LoadInventory(LoadInventory),
    ChangeItemDesc(ChangeItemDesc),
    Action(Action),
    ChangeChannel(ChangeChannel),
    Time(Time),
    WorldInfo(WorldInfo),
    Loot(Loot),
    LootItemlist(LootItemlist),
    WorldSceneStatus(WorldSceneStatus),
    GamepassInfo(GamepassInfo),
    MessageCode(MessageCode),
    _0102(_0102),
    AbnormalStatus(AbnormalStatus),
    AbnormalStatusOther(AbnormalStatusOther),
    InvincibleTime(InvincibleTime),
    InvisibleLevel(InvisibleLevel),
    SignClient(SignClient),
    _0151(_0151),
    Weather(Weather),
    ResultPassport(ResultPassport),
    UseSkill(UseSkill),
    SkillSucceded(SkillSucceded),
    CustomAnim(CustomAnim),
    Title(Title),
    LoadItemCooltime(LoadItemCooltime),
    AskQuitResult(AskQuitResult),
    PutNpc(PutNpc),
}

impl Serialise for Message {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = match self {
            Message::VersionCheck(msg) => msg.serialise(&mut buf[2..]),
            Message::Status(msg) => msg.serialise(&mut buf[2..]),
            Message::World(msg) => msg.serialise(&mut buf[2..]),
            Message::PutNpc(msg) => msg.serialise(&mut buf[2..]),
            Message::EnterWorldCheck(msg) => msg.serialise(&mut buf[2..]),
            Message::LoadClientSettings(msg) => msg.serialise(&mut buf[2..]),
            Message::UseSkill(msg) => msg.serialise(&mut buf[2..]),
            Message::SkillSucceded(msg) => msg.serialise(&mut buf[2..]),
            Message::PutUser(msg) => msg.serialise(&mut buf[2..]),
            Message::CharacterList(msg) => msg.serialise(&mut buf[2..]),
            Message::CurStatus(msg) => msg.serialise(&mut buf[2..]),
            Message::Key(msg) => msg.serialise(&mut buf[2..]),
            Message::ServerEnv(msg) => msg.serialise(&mut buf[2..]),
            Message::SyncTime(msg) => msg.serialise(&mut buf[2..]),
            Message::NpcChangedTarget(msg) => msg.serialise(&mut buf[2..]),
            Message::Action(msg) => msg.serialise(&mut buf[2..]),
            Message::MoveNew(msg) => msg.serialise(&mut buf[2..]),
            Message::NpShopGoodsCount(msg) => msg.serialise(&mut buf[2..]),
            Message::ReplyNpAuthToken(msg) => msg.serialise(&mut buf[2..]),
            Message::ReplyNpLoginGamesvr(msg) => msg.serialise(&mut buf[2..]),
            Message::ReadyEnterWorld(msg) => msg.serialise(&mut buf[2..]),
            Message::L2AuthLoginCheck(msg) => msg.serialise(&mut buf[2..]),
            Message::BuilderLevel(msg) => msg.serialise(&mut buf[2..]),
            Message::Alive(msg) => msg.serialise(&mut buf[2..]),
            Message::TargetInfo(msg) => msg.serialise(&mut buf[2..]),
            Message::SaAccountItemNoti(msg) => msg.serialise(&mut buf[2..]),
            Message::SecondPassword(msg) => msg.serialise(&mut buf[2..]),
            Message::AbnormalStatus(msg) => msg.serialise(&mut buf[2..]),
            Message::_0102(msg) => msg.serialise(&mut buf[2..]),
            Message::RecipeList(msg) => msg.serialise(&mut buf[2..]),
            Message::BuddyList(msg) => msg.serialise(&mut buf[2..]),
            Message::AddSkill(msg) => msg.serialise(&mut buf[2..]),
            Message::Attack(msg) => msg.serialise(&mut buf[2..]),
            Message::LoadInventory(msg) => msg.serialise(&mut buf[2..]),
            Message::ChangeItemDesc(msg) => msg.serialise(&mut buf[2..]),
            Message::ChangeChannel(msg) => msg.serialise(&mut buf[2..]),
            Message::Time(msg) => msg.serialise(&mut buf[2..]),
            Message::Loot(msg) => msg.serialise(&mut buf[2..]),
            Message::LootItemlist(msg) => msg.serialise(&mut buf[2..]),
            Message::WorldInfo(msg) => msg.serialise(&mut buf[2..]),
            Message::GamepassInfo(msg) => msg.serialise(&mut buf[2..]),
            Message::MessageCode(msg) => msg.serialise(&mut buf[2..]),
            Message::_0151(msg) => msg.serialise(&mut buf[2..]),
            Message::AbnormalStatusOther(msg) => msg.serialise(&mut buf[2..]),
            Message::InvincibleTime(msg) => msg.serialise(&mut buf[2..]),
            Message::InvisibleLevel(msg) => msg.serialise(&mut buf[2..]),
            Message::WorldSceneStatus(msg) => msg.serialise(&mut buf[2..]),
            Message::SignClient(msg) => msg.serialise(&mut buf[2..]),
            Message::Weather(msg) => msg.serialise(&mut buf[2..]),
            Message::ResultPassport(msg) => msg.serialise(&mut buf[2..]),
            Message::CustomAnim(msg) => msg.serialise(&mut buf[2..]),
            Message::Title(msg) => msg.serialise(&mut buf[2..]),
            Message::LoadItemCooltime(msg) => msg.serialise(&mut buf[2..]),
            Message::AskQuitResult(msg) => msg.serialise(&mut buf[2..]),
            Message::ReconnectKey(msg) => msg.serialise(&mut buf[2..]),
            Message::HitPointOther(msg) => msg.serialise(&mut buf[2..]),
            Message::HitPoint(msg) => msg.serialise(&mut buf[2..]),
        };

        len += LENGTH_LEN;
        buf[..LENGTH_LEN].copy_from_slice(&(len as u16).to_le_bytes());
        len
    }
}

/// Returns a buffer with the opcode, [SERVER_BYTE] and checksum
#[inline(always)]
fn add_prelude(opcode: u16, buf: &mut [u8]) -> usize {
    let opcode = encrypt_server_opcode(opcode);
    let checksum = !opcode;

    let mut len = 0;
    to_le_bytes!(len, buf, opcode);
    to_le_bytes!(len, buf, SERVER_BYTE);
    to_le_bytes!(len, buf, checksum);
    len
}

#[derive(Debug, Clone)]
pub struct Key {
    key: u32,
}
impl Key {
    pub fn new(key: u32) -> Self {
        Self { key }
    }
}
impl Serialise for Key {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(KEY, buf);

        buf[len..len + 4].copy_from_slice(&self.key.to_le_bytes());
        len += 4;

        len
    }
}

#[link(name = "kernel32")]
#[allow(non_snake_case)]
extern "system" {
    fn GetTickCount() -> u32;
}

#[derive(Debug, Clone)]
pub struct SyncTime {
    server_uptime_ticks: u32,
    session_ticks: u32,
    _unknown: [u8; 4],
    sequence: u32,
}
impl SyncTime {
    pub fn new(session_ticks: u32) -> Self {
        Self {
            server_uptime_ticks: unsafe { GetTickCount() },
            session_ticks,
            _unknown: [0x0, 0x0, 0x0, 0x0],
            sequence: 1,
        }
    }
}
impl Serialise for SyncTime {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(SYNC_TIME, buf);

        let server_uptime_ticks = self.server_uptime_ticks.to_le_bytes();
        buf[len..len + server_uptime_ticks.len()]
            .copy_from_slice(&server_uptime_ticks);
        len += server_uptime_ticks.len();

        let session_ticks = self.session_ticks.to_le_bytes();
        buf[len..len + session_ticks.len()].copy_from_slice(&session_ticks);
        len += server_uptime_ticks.len();

        buf[len..len + self._unknown.len()].copy_from_slice(&self._unknown);
        len += self._unknown.len();

        let sequence = self.sequence.to_le_bytes();
        buf[len..len + sequence.len()].copy_from_slice(&sequence);
        len += sequence.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct VersionCheck {
    opcode: u8,
    server_id: u8,
    server_build_date1: [u8; 4],
    server_build_date2: [u8; 4],
    _unknown_static1: [u8; 4],
    server_build_date3: [u8; 4],
    server_start_epoch: [u8; 4],
    _unknown_static2: [u8; 3],
    _unknown_rand1: [u8; 4],
    _unknown_static3: [u8; 23],
}

impl VersionCheck {
    pub fn new() -> Self {
        Self {
            opcode: 0,
            server_id: 15,
            server_build_date1: [0xE3, 0xAB, 0x03, 0x00],
            server_build_date2: [0xE3, 0xAB, 0x03, 0x00],
            _unknown_static1: [0u8; 4],
            server_build_date3: [0xE3, 0xAB, 0x03, 0x00],
            server_start_epoch: [0x47, 0x04, 0x85, 0x66],
            _unknown_static2: [0x00, 0x02, 0x00],
            _unknown_rand1: [0x90, 0xD2, 0x8C, 0x95],
            _unknown_static3: [
                0x66, 0xF0, 0xF1, 0xFF, 0xFF, 0xF0, 0xF1, 0xFF, 0xFF, 0x01,
                0x00, 0x00, 0x4F, 0x6E, 0x53, 0x9F, 0x01, 0x28, 0x9B, 0x02,
                0x00, 0x00, 0x00,
            ],
        }
    }
}

impl Serialise for VersionCheck {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(VERSION_CHECK, buf);

        buf[len] = self.opcode;
        len += 1;
        buf[len] = self.server_id;
        len += 1;
        buf[len..len + 4].copy_from_slice(&self.server_build_date1);
        len += 4;
        buf[len..len + 4].copy_from_slice(&self.server_build_date2);
        len += 4;
        buf[len..len + 4].copy_from_slice(&self._unknown_static1);
        len += 4;
        buf[len..len + 4].copy_from_slice(&self.server_build_date3);
        len += 4;
        buf[len..len + 4].copy_from_slice(&self.server_start_epoch);
        len += 4;
        buf[len..len + 3].copy_from_slice(&self._unknown_static2);
        len += 3;
        buf[len..len + 4].copy_from_slice(&self._unknown_rand1);
        len += 4;
        buf[len..len + 23].copy_from_slice(&self._unknown_static3);
        len += 23;

        len
    }
}

#[derive(Debug, Clone)]
pub struct NpShopGoodsCount {
    pub raw: [u8; 4],
}
impl NpShopGoodsCount {
    pub fn new() -> Self {
        Self { raw: [1, 0, 0, 0] }
    }
}
impl Serialise for NpShopGoodsCount {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;

        len += add_prelude(NPSHOP_GOODS_COUNT, buf);
        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct ReadyEnterWorld {
    pub unknown_static1: [u8; 2],
    pub unknown_rand1: [u8; 4],
}

impl ReadyEnterWorld {
    pub fn new() -> Self {
        Self {
            unknown_static1: [0u8; 2],
            unknown_rand1: [0xFF, 0x39, 0x60, 0x05],
        }
    }
}
impl Serialise for ReadyEnterWorld {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;

        len += add_prelude(READY_ENTER_WORLD, buf);

        buf[len..len + self.unknown_static1.len()]
            .copy_from_slice(&self.unknown_static1);
        len += self.unknown_static1.len();
        buf[len..len + self.unknown_rand1.len()]
            .copy_from_slice(&self.unknown_rand1);
        len += self.unknown_rand1.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct ReplyNpLoginGamesvr {
    pub raw: [u8; 2],
}
impl ReplyNpLoginGamesvr {
    pub fn new() -> Self {
        Self { raw: [0, 1] }
    }
}
impl Serialise for ReplyNpLoginGamesvr {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;

        len += add_prelude(REPLY_NP_LOGIN_GAMESVR, buf);
        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct ServerEnv {
    _unknown: [u8; 33],
}
impl ServerEnv {
    pub fn new() -> Self {
        Self {
            _unknown: [
                0x5E, 0x01, 0x01, 0x0A, 0x01, 0x0A, 0x0A, 0x0A, 0x02, 0x00,
                0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64, 0x00, 0x00, 0x00,
                0x01, 0x37, 0x00, 0x00, 0x00, 0x37, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00,
            ],
        }
    }
}
impl Serialise for ServerEnv {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;

        len += add_prelude(SERVER_ENV, buf);
        buf[len..len + self._unknown.len()].copy_from_slice(&self._unknown);
        len += self._unknown.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct ReplyNpAuthToken {
    _unknown: [u8; 150],
}
impl ReplyNpAuthToken {
    pub fn new() -> Self {
        Self {
            _unknown: [
                0x01, 0x4F, 0x55, 0x55, 0x77, 0x4E, 0x54, 0x52, 0x46, 0x4F,
                0x44, 0x6B, 0x74, 0x51, 0x7A, 0x42, 0x43, 0x52, 0x43, 0x30,
                0x30, 0x4E, 0x7A, 0x6B, 0x79, 0x4C, 0x54, 0x6C, 0x42, 0x4D,
                0x6A, 0x49, 0x74, 0x4D, 0x7A, 0x4D, 0x78, 0x4D, 0x54, 0x5A,
                0x45, 0x4F, 0x55, 0x56, 0x42, 0x51, 0x6B, 0x49, 0x33, 0x4F,
                0x6A, 0x4E, 0x47, 0x4E, 0x7A, 0x45, 0x34, 0x52, 0x54, 0x46,
                0x42, 0x4C, 0x55, 0x59, 0x77, 0x51, 0x6A, 0x63, 0x74, 0x4E,
                0x44, 0x45, 0x77, 0x51, 0x69, 0x30, 0x34, 0x4D, 0x30, 0x4A,
                0x43, 0x4C, 0x54, 0x56, 0x43, 0x4D, 0x54, 0x55, 0x78, 0x4E,
                0x45, 0x5A, 0x42, 0x4D, 0x54, 0x52, 0x46, 0x4F, 0x41, 0x41,
                0x3D, 0x00, 0x38, 0x42, 0x35, 0x35, 0x37, 0x33, 0x37, 0x34,
                0x2D, 0x31, 0x38, 0x37, 0x32, 0x2D, 0x34, 0x37, 0x42, 0x43,
                0x2D, 0x39, 0x38, 0x33, 0x44, 0x2D, 0x37, 0x35, 0x38, 0x32,
                0x30, 0x37, 0x31, 0x32, 0x31, 0x34, 0x41, 0x36, 0x00, 0x61,
                0x69, 0x6F, 0x6E, 0x67, 0x66, 0x63, 0x00, 0x13, 0x0C, 0x03,
            ],
        }
    }
}
impl Serialise for ReplyNpAuthToken {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;

        len += add_prelude(REPLY_NP_AUTH_TOKEN, buf);
        buf[len..len + self._unknown.len()].copy_from_slice(&self._unknown);
        len += self._unknown.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct CharacterList {
    unknown1: u8,
    auth_server_id: [u8; 4],
    len: u8,
    character_id: u32,
    name: String,
    header: [u8; 238],
    gear: Gear,
    last_logged_in: u32,
    footer: [u8; 146],
}
impl CharacterList {
    pub fn new(
        auth_server_id: [u8; 4],
        name: String,
        gear: Gear,
        last_logged_in: u32,
        character_id: u32,
    ) -> Self {
        let header = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00,
            0x00, 0x00, 0xAB, 0xAB, 0xBC, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xE9,
            0x02, 0x0F, 0x00, 0xCE, 0xB4, 0xCA, 0x00, 0x00, 0x0A, 0x00, 0x00,
            0x00, 0x02, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x09, 0x83, 0x8F, 0x00, 0x03, 0x00, 0x00, 0x00, 0x83, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x04, 0xEF, 0xB3, 0xDA, 0xB2,
            0xF0, 0xDF, 0xEF, 0xBA, 0xF4, 0xCE, 0x00, 0x00, 0xF0, 0xF4, 0xF5,
            0xE4, 0x00, 0x00, 0x00, 0x66, 0x66, 0x26, 0x3F, 0xA3, 0x86, 0x01,
            0x00, 0x30, 0x64, 0x1D, 0x0D, // Coords
            0x92, 0xDA, 0xEB, 0x44, 0xFC, 0x67, 0x2F, 0x44, 0x2D, 0x15, 0x81,
            0x43, // Direction?
            0x11, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x34, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00,
        ];

        let footer = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
            0x00, 0x04, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC5, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
        ];

        Self {
            unknown1: 2,
            auth_server_id,
            len: 1,
            character_id,
            name,
            header,
            last_logged_in,
            gear,
            footer,
        }
    }
}
impl Serialise for CharacterList {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(CHARACTER_LIST, buf);

        buf[len] = self.unknown1;
        len += 1;

        buf[len..len + self.auth_server_id.len()]
            .copy_from_slice(&self.auth_server_id);
        len += self.auth_server_id.len();

        buf[len] = self.len;
        len += 1;

        to_le_bytes!(len, buf, self.character_id);

        self.name.encode_utf16().for_each(|c| {
            buf[len + 0] = c as u8;
            buf[len + 1] = (c >> 8) as u8;
            len += 2;
        });

        buf[len..len + self.header.len()].copy_from_slice(&self.header);
        len += self.header.len();

        buf[len..len + 4].copy_from_slice(&self.last_logged_in.to_le_bytes());
        len += 4;

        len += self.gear.serialiase_character_list(&mut buf[len..]);

        buf[len..len + self.footer.len()].copy_from_slice(&self.footer);
        len += self.footer.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct L2AuthLoginCheck {
    _unknown: [u8; 1185],
}
impl L2AuthLoginCheck {
    pub fn new() -> Self {
        Self {
            _unknown: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x03, 0x03, 0x03, 0x04,
                0x04, 0x04, 0x05, 0x05, 0x05, 0x06, 0x06, 0x06, 0x07, 0x07,
                0x07, 0x08, 0x08, 0x08, 0x09, 0x09, 0x09, 0x0A, 0x0A, 0x0A,
                0x0B, 0x0B, 0x0B, 0x0C, 0x0C, 0x0C, 0x0D, 0x0D, 0x0D, 0x0E,
                0x0E, 0x0E, 0x0F, 0x0F, 0x0F, 0x10, 0x10, 0x10, 0x11, 0x11,
                0x11, 0x12, 0x12, 0x12, 0x13, 0x13, 0x13, 0x14, 0x14, 0x14,
                0x15, 0x15, 0x15, 0x16, 0x16, 0x16, 0x17, 0x17, 0x17, 0x18,
                0x18, 0x18, 0x19, 0x19, 0x19, 0x1A, 0x1A, 0x1A, 0x1B, 0x1B,
                0x1B, 0x1C, 0x1C, 0x1C, 0x1D, 0x1D, 0x1D, 0x1E, 0x1E, 0x1E,
                0x1F, 0x1F, 0x1F, 0x20, 0x20, 0x20, 0x21, 0x21, 0x21, 0x22,
                0x22, 0x22, 0x23, 0x23, 0x23, 0x24, 0x24, 0x24, 0x25, 0x25,
                0x25, 0x26, 0x26, 0x26, 0x27, 0x27, 0x27, 0x28, 0x28, 0x28,
                0x29, 0x29, 0x29, 0x2A, 0x2A, 0x2A, 0x2B, 0x2B, 0x2B, 0x2C,
                0x2C, 0x2C, 0x2D, 0x2D, 0x2D, 0x2E, 0x2E, 0x2E, 0x2F, 0x2F,
                0x2F, 0x30, 0x30, 0x30, 0x31, 0x31, 0x31, 0x32, 0x32, 0x32,
                0x33, 0x33, 0x33, 0x34, 0x34, 0x34, 0x35, 0x35, 0x35, 0x36,
                0x36, 0x36, 0x37, 0x37, 0x37, 0x38, 0x38, 0x38, 0x39, 0x39,
                0x39, 0x3A, 0x3A, 0x3A, 0x3B, 0x3B, 0x3B, 0x3C, 0x3C, 0x3C,
                0x3D, 0x3D, 0x3D, 0x3E, 0x3E, 0x3E, 0x3F, 0x3F, 0x3F, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02,
                0x03, 0x03, 0x03, 0x04, 0x04, 0x04, 0x05, 0x05, 0x05, 0x06,
                0x06, 0x06, 0x07, 0x07, 0x07, 0x08, 0x08, 0x08, 0x09, 0x09,
                0x09, 0x0A, 0x0A, 0x0A, 0x0B, 0x0B, 0x0B, 0x0C, 0x0C, 0x0C,
                0x0D, 0x0D, 0x0D, 0x0E, 0x0E, 0x0E, 0x0F, 0x0F, 0x0F, 0x10,
                0x10, 0x10, 0x11, 0x11, 0x11, 0x12, 0x12, 0x12, 0x13, 0x13,
                0x13, 0x14, 0x14, 0x14, 0x15, 0x15, 0x15, 0x16, 0x16, 0x16,
                0x17, 0x17, 0x17, 0x18, 0x18, 0x18, 0x19, 0x19, 0x19, 0x1A,
                0x1A, 0x1A, 0x1B, 0x1B, 0x1B, 0x1C, 0x1C, 0x1C, 0x1D, 0x1D,
                0x1D, 0x1E, 0x1E, 0x1E, 0x1F, 0x1F, 0x1F, 0x20, 0x20, 0x20,
                0x21, 0x21, 0x21, 0x22, 0x22, 0x22, 0x23, 0x23, 0x23, 0x24,
                0x24, 0x24, 0x25, 0x25, 0x25, 0x26, 0x26, 0x26, 0x27, 0x27,
                0x27, 0x28, 0x28, 0x28, 0x29, 0x29, 0x29, 0x2A, 0x2A, 0x2A,
                0x2B, 0x2B, 0x2B, 0x2C, 0x2C, 0x2C, 0x2D, 0x2D, 0x2D, 0x2E,
                0x2E, 0x2E, 0x2F, 0x2F, 0x2F, 0x30, 0x30, 0x30, 0x31, 0x31,
                0x31, 0x32, 0x32, 0x32, 0x33, 0x33, 0x33, 0x34, 0x34, 0x34,
                0x35, 0x35, 0x35, 0x36, 0x36, 0x36, 0x37, 0x37, 0x37, 0x38,
                0x38, 0x38, 0x39, 0x39, 0x39, 0x3A, 0x3A, 0x3A, 0x3B, 0x3B,
                0x3B, 0x3C, 0x3C, 0x3C, 0x3D, 0x3D, 0x3D, 0x3E, 0x3E, 0x3E,
                0x3F, 0x3F, 0x3F, 0x61, 0x00, 0x10, 0xAB, 0xD7, 0x17, 0x01,
                0x00, 0x80, 0x46, 0x28, 0x07, 0x01, 0x00, 0xF0, 0x88, 0x8F,
                0x06, 0x01, 0x00, 0x10, 0x35, 0x27, 0x07, 0x01, 0x00, 0x20,
                0x5C, 0x27, 0x07, 0x01, 0x00, 0x10, 0x16, 0x1D, 0x0D, 0x0A,
                0x00, 0x30, 0x64, 0x1D, 0x0D, 0x0A, 0x00, 0x20, 0x3D, 0x1D,
                0x0D, 0x01, 0x00, 0x50, 0xB2, 0x1D, 0x0D, 0x03, 0x00, 0x40,
                0x8B, 0x1D, 0x0D, 0x01, 0x00, 0x70, 0x00, 0x1E, 0x0D, 0x01,
                0x00, 0x10, 0xB9, 0xFE, 0x1E, 0x01, 0x00, 0xB0, 0x50, 0xE3,
                0x11, 0x00, 0x00, 0x30, 0x18, 0xE2, 0x11, 0x00, 0x00, 0x30,
                0x45, 0x13, 0x13, 0x00, 0x00, 0x40, 0x6C, 0x13, 0x13, 0x00,
                0x00, 0xB0, 0xAE, 0x7A, 0x12, 0x00, 0x00, 0xC0, 0xD5, 0x7A,
                0x12, 0x00, 0x00, 0x10, 0xCA, 0xE1, 0x11, 0x00, 0x00, 0x10,
                0xF7, 0x12, 0x13, 0x00, 0x00, 0x20, 0x1E, 0x13, 0x13, 0x00,
                0x00, 0xE0, 0xF2, 0x14, 0x13, 0x00, 0x00, 0x90, 0x60, 0x7A,
                0x12, 0x00, 0x00, 0xA0, 0x87, 0x7A, 0x12, 0x00, 0x00, 0x40,
                0x0E, 0x7C, 0x12, 0x00, 0x00, 0x60, 0xFE, 0xE4, 0x11, 0x00,
                0x00, 0x60, 0x8D, 0xE2, 0x11, 0x00, 0x00, 0x90, 0x02, 0xE3,
                0x11, 0x00, 0x00, 0x80, 0xDB, 0xE2, 0x11, 0x00, 0x00, 0xE0,
                0xC5, 0xE3, 0x11, 0x00, 0x00, 0xC0, 0x77, 0xE3, 0x11, 0x00,
                0x00, 0xD0, 0x9E, 0xE3, 0x11, 0x00, 0x00, 0x50, 0x66, 0xE2,
                0x11, 0x00, 0x00, 0x70, 0xB4, 0xE2, 0x11, 0x00, 0x00, 0xE0,
                0x36, 0xE6, 0x11, 0x00, 0x00, 0xF0, 0xCE, 0xE8, 0x11, 0x00,
                0x00, 0x30, 0xFA, 0xE6, 0x11, 0x00, 0x00, 0xA0, 0x0B, 0xE8,
                0x11, 0x00, 0x00, 0x40, 0x21, 0xE7, 0x11, 0x00, 0x00, 0xB0,
                0x32, 0xE8, 0x11, 0x00, 0x00, 0x00, 0x85, 0xE6, 0x11, 0x00,
                0x00, 0x40, 0x92, 0xE9, 0x11, 0x00, 0x00, 0x50, 0xB9, 0xE9,
                0x11, 0x00, 0x00, 0x10, 0x3B, 0xE4, 0x11, 0x00, 0x00, 0x10,
                0x1D, 0xE9, 0x11, 0x00, 0x00, 0x70, 0x25, 0xE5, 0x11, 0x00,
                0x00, 0x90, 0x2F, 0x14, 0x13, 0x00, 0x00, 0xC0, 0xA4, 0x14,
                0x13, 0x00, 0x00, 0x60, 0xBA, 0x13, 0x13, 0x00, 0x00, 0xD0,
                0xCB, 0x14, 0x13, 0x00, 0x00, 0x50, 0x93, 0x13, 0x13, 0x00,
                0x00, 0xA0, 0x56, 0x14, 0x13, 0x00, 0x00, 0x80, 0x08, 0x14,
                0x13, 0x00, 0x00, 0x00, 0xF6, 0xE8, 0x11, 0x00, 0x00, 0xB0,
                0x7D, 0x14, 0x13, 0x00, 0x00, 0xF0, 0x19, 0x15, 0x13, 0x00,
                0x00, 0x50, 0xD7, 0xE4, 0x11, 0x00, 0x00, 0x30, 0x89, 0xE4,
                0x11, 0x00, 0x00, 0x90, 0x73, 0xE5, 0x11, 0x00, 0x00, 0x20,
                0x44, 0xE9, 0x11, 0x00, 0x00, 0x30, 0x6B, 0xE9, 0x11, 0x00,
                0x00, 0x00, 0x72, 0x7B, 0x12, 0x00, 0x00, 0x70, 0x07, 0xEA,
                0x11, 0x00, 0x00, 0x60, 0xE0, 0xE9, 0x11, 0x00, 0x00, 0x80,
                0x2E, 0xEA, 0x11, 0x00, 0x00, 0x40, 0x3F, 0xE2, 0x11, 0x00,
                0x00, 0xE0, 0x23, 0x7B, 0x12, 0x00, 0x00, 0xF0, 0x4A, 0x7B,
                0x12, 0x00, 0x00, 0x30, 0xE7, 0x7B, 0x12, 0x00, 0x00, 0xD0,
                0xFC, 0x7A, 0x12, 0x00, 0x00, 0x10, 0x99, 0x7B, 0x12, 0x00,
                0x00, 0x20, 0xC0, 0x7B, 0x12, 0x00, 0x00, 0x40, 0xB0, 0xE4,
                0x11, 0x00, 0x00, 0x90, 0x8D, 0xAB, 0x13, 0x00, 0x00, 0xA0,
                0x29, 0xE3, 0x11, 0x00, 0x00, 0xD0, 0x80, 0xE8, 0x11, 0x00,
                0x00, 0x70, 0xE1, 0x13, 0x13, 0x00, 0x00, 0x00, 0x14, 0xE4,
                0x11, 0x00, 0x00, 0xE8, 0x17, 0xE4, 0x11, 0x00, 0x00, 0xF0,
                0xEC, 0xE3, 0x11, 0x00, 0x00, 0xD8, 0xF0, 0xE3, 0x11, 0x00,
                0x00, 0xE0, 0xA7, 0xE8, 0x11, 0x00, 0x00, 0xC0, 0x59, 0xE8,
                0x11, 0x00, 0x00, 0x90, 0x9E, 0x8E, 0x06, 0x01, 0x00, 0xA0,
                0xC5, 0x8E, 0x06, 0x01, 0x00, 0xF0, 0x69, 0x85, 0x0C, 0x0A,
                0x00, 0x80, 0x27, 0x1E, 0x0D, 0x0A, 0x00, 0x20, 0x94, 0xC3,
                0x23, 0x01, 0x00, 0x90, 0x7F, 0x84, 0x0C, 0x0A, 0x00, 0xB0,
                0xCD, 0x84, 0x0C, 0x0A, 0x00, 0xA0, 0xA6, 0x84, 0x0C, 0x01,
                0x00, 0xE0, 0x42, 0x85, 0x0C, 0x03, 0x00, 0xC0, 0xF4, 0x84,
                0x0C, 0x01, 0x00, 0xD0, 0x1B, 0x85, 0x0C, 0x01, 0x00, 0x90,
                0x22, 0x66, 0x1E, 0x01, 0x00, 0x60, 0x44, 0xA8, 0x35, 0x01,
                0x00, 0x10, 0x6D, 0xC3, 0x23, 0x01, 0x00, 0x34, 0x00, 0x30,
                0x00, 0x30, 0x00, 0x32, 0x00, 0x38, 0x00, 0x38, 0x00, 0x35,
                0x00, 0x34, 0x00, 0x00, 0x00,
            ],
        }
    }
}
impl Serialise for L2AuthLoginCheck {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(L2AUTH_LOGIN_CHECK, buf);

        buf[len..len + self._unknown.len()].copy_from_slice(&self._unknown);
        len += self._unknown.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct BuilderLevel {
    pub raw: [u8; 29],
}
impl BuilderLevel {
    pub fn new() -> Self {
        Self {
            raw: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        }
    }
}
impl Serialise for BuilderLevel {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(BUILDER_LEVEL, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct SaAccountItemNoti {
    raw: [u8; 4],
}
impl SaAccountItemNoti {
    pub fn new() -> Self {
        Self { raw: [0u8; 4] }
    }
}
impl Serialise for SaAccountItemNoti {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(SA_ACCOUNT_ITEM_NOTI, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum SecondPasswordOpcode {
    Request = 1,
    Response = 2,
}
#[derive(Debug, Clone)]
pub struct SecondPassword {
    opcode: SecondPasswordOpcode,
    unknown: [u8; 11],
}
impl SecondPassword {
    pub fn request() -> Self {
        Self {
            opcode: SecondPasswordOpcode::Request,
            unknown: [0u8; 11],
        }
    }
    /// TODO: Reverse this, this responds okay
    pub fn response() -> Self {
        Self {
            opcode: SecondPasswordOpcode::Response,
            unknown: [
                0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x0, 0x00, 0x00,
            ],
        }
    }
}
impl Serialise for SecondPassword {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(SECOND_PASSWORD, buf);

        buf[len] = self.opcode as u8;
        len += 1;

        match self.opcode {
            SecondPasswordOpcode::Request => len,
            SecondPasswordOpcode::Response => {
                buf[len..len + self.unknown.len()]
                    .copy_from_slice(&self.unknown);
                len += self.unknown.len();
                len
            }
            _ => todo!("Invalid Opcode"),
        }
    }
}

/// Acknowledges a C_ENTER_WORLD
#[derive(Debug, Clone)]
pub struct EnterWorldCheck {
    raw: [u8; 3],
}
impl EnterWorldCheck {
    pub fn new() -> Self {
        Self { raw: [0u8; 3] }
    }
}
impl Serialise for EnterWorldCheck {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(ENTER_WORLD_CHECK, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

/// Place a player character into the game
#[derive(Debug, Clone)]
pub struct PutUser {
    pub character: Character,
    header1: [u8; 35],
    header2: [u8; 24],
    footer1: [u8; 81],
    footer2: [u8; 76],
}
impl PutUser {
    pub fn new(character: &Character) -> Self {
        let header1 = [
            0xA3, 0x86, 0x01, 0x00, 0xA3, 0x86, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x9A, 0x01, 0x40, 0x01, 0x00, 0x00, 0x00, 0x26, 0x01,
            0x04, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x11,
        ];

        let header2 = [
            0x00, 0x00, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // Health Percent
            0x5C, 0x00, 0x00, 0xFF,
        ];

        let footer1 = [
            0xAB, 0xAB, 0xBC, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xE9, 0x02, 0x0F,
            0x00, 0xCE, 0xB4, 0xCA, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x02,
            0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x83,
            0x8F, 0x00, 0x03, 0x00, 0x00, 0x00, 0x83, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x07, 0x04, 0xEF, 0xB3, 0xDA, 0xB2, 0xF0, 0xDF,
            0xEF, 0xBA, 0xF4, 0xCE, 0x00, 0x00, 0xF0, 0xF4, 0xF5, 0xE4, 0x00,
            0x02, 0x66, 0x66, 0x26, 0x3F, 0x00, 0x00, 0x80, 0x3E, 0x00, 0x00,
            0x00, 0x40, 0x00, 0x00,
        ];

        let footer2 = [
            0xD6, 0x06, 0xD6, 0x06, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // Coords Desination?
            0x92, 0xDA, 0xEB, 0x44, 0xFC, 0x67, 0x2F, 0x44, 0x2D, 0x15, 0x81,
            0x43, // ????
            0x00, 0x40, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00,
        ];

        Self {
            character: character.clone(),
            header1,
            footer1,
            footer2,
            header2,
        }
    }
}
impl Serialise for PutUser {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(PUT_USER, buf);

        len += self.character.location().serialise(&mut buf[len..]);

        to_le_bytes!(len, buf, self.character.id());

        buf[len..len + self.header1.len()].copy_from_slice(&self.header1);
        len += self.header1.len();

        len += self.character.serialise_name_utf16(&mut buf[len..]);

        buf[len..len + self.header2.len()].copy_from_slice(&self.header2);
        len += self.header2.len();

        len += self.character.gear.serialiase_put_user(&mut buf[len..]);

        buf[len..len + self.footer1.len()].copy_from_slice(&self.footer1);
        len += self.footer1.len();

        let move_speed =
            (self.character.stats.move_speed.to_bits() >> 16) as u16;
        buf[len..len + 2].copy_from_slice(&move_speed.to_le_bytes());
        len += 2;

        buf[len..len + self.footer2.len()].copy_from_slice(&self.footer2);
        len += self.footer2.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct CurStatus {
    raw: u8,
}
impl CurStatus {
    pub fn new() -> Self {
        Self { raw: 1 }
    }
}
impl Serialise for CurStatus {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(CUR_STATUS, buf);

        buf[len] = self.raw;
        len += 1;

        len
    }
}
#[derive(Debug, Clone)]
pub struct World {
    unknown1: [u8; 13],
    coord: Coord,
    unknown2: [u8; 17],
}
impl World {
    pub fn new(coord: &Coord) -> Self {
        Self {
            unknown1: [
                0x30, 0x64, 0x1D, 0x0D, 0x30, 0x64, 0x1D, 0x0D, 0x00, 0x00,
                0x00, 0x00, 0x00,
            ],
            coord: coord.clone(),
            unknown2: [
                0x11, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        }
    }
}
impl Serialise for World {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(WORLD, buf);

        buf[len..len + self.unknown1.len()].copy_from_slice(&self.unknown1);
        len += self.unknown1.len();

        len += self.coord.serialise(&mut buf[len..]);

        buf[len..len + self.unknown2.len()].copy_from_slice(&self.unknown2);
        len += self.unknown2.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct LoadClientSettings {
    ty: u8,
    raw: Vec<u8>,
}
impl LoadClientSettings {
    pub fn new(ty: u8, raw: Vec<u8>) -> Self {
        Self { raw, ty }
    }
}
impl Serialise for LoadClientSettings {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(LOAD_CLIENT_SETTINGS, buf);

        let body_len = self.raw.len() + size_of::<u32>();
        to_le_bytes!(len, buf, self.ty);
        // Length with padding minus type and itself,
        // this is required, dont know why
        to_le_bytes!(len, buf, 7168 as u16);
        to_le_bytes!(len, buf, body_len as u32);
        copy_bytes!(len, buf, self.raw);

        len
    }
}

#[derive(Debug, Clone)]
pub struct _0102 {
    raw: Vec<u8>,
}
impl _0102 {
    pub fn new() -> Self {
        Self {
            raw: [
                0x24, 0xDD, 0xA1, 0xC5, 0x15, 0xDB, 0xDD, 0x10, 0x0B, 0x0C,
                0xAD, 0x32, 0x9A, 0x31, 0xEC, 0x54, 0xAF, 0x22, 0x47, 0xA5,
                0xC7, 0xC5, 0xA2, 0x92, 0xAA, 0x99, 0x47, 0x1B, 0xB9, 0x92,
                0x73, 0x6B,
            ]
            .to_vec(),
        }
    }
}
impl Serialise for _0102 {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(0102, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

#[derive(Debug, Clone)]
pub enum MoveData {
    Turn { delta: Coord },
    Stop,
    Continue,
    Direct { dest: Coord },
}

// TODO: Prevent malicious Movement
#[derive(Debug, Clone)]
pub struct MoveNew {
    entity_id: u32,
    location: Coord,
    direction: Direction,
    ty: MoveType,
    data: MoveData,
}
impl MoveNew {
    pub fn new(
        entity_id: u32,
        location: Coord,
        direction: Direction,
        ty: MoveType,
        data: MoveData,
    ) -> Self {
        Self {
            entity_id,
            location,
            direction,
            ty,
            data,
        }
    }
}
impl Serialise for MoveNew {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(MOVE_NEW, buf);

        to_le_bytes!(len, buf, self.entity_id);
        len += self.location.serialise(&mut buf[len..]);
        to_le_bytes!(len, buf, self.direction.bits());
        to_le_bytes!(len, buf, self.ty.bits());

        match &self.data {
            MoveData::Turn { delta } => len += delta.serialise(&mut buf[len..]),
            MoveData::Stop => {}
            MoveData::Continue => {}
            MoveData::Direct { dest } => len += dest.serialise(&mut buf[len..]),
        }

        len
    }
}

#[derive(Debug, Clone)]
pub struct RecipeList {
    raw: Vec<u8>,
}
impl RecipeList {
    pub fn new(raw: Vec<u8>) -> Self {
        Self { raw }
    }
}
impl Serialise for RecipeList {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(RECIPE_LIST, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct BuddyList {
    raw: [u8; 3],
}
impl BuddyList {
    pub fn new() -> Self {
        Self { raw: [0, 0, 0] }
    }
}
impl Serialise for BuddyList {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(BUDDY_LIST, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

#[derive(Debug, Clone)]
pub struct AddSkill {
    skills: Vec<Skill>,
}
impl AddSkill {
    pub fn new(skills: Vec<Skill>) -> Self {
        Self { skills }
    }
}
impl Serialise for AddSkill {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(ADD_SKILL, buf);

        buf[len] = self.skills.len() as u8;
        len += 1;
        // Unknown
        buf[len] = 0;
        len += 1;
        for skill in &self.skills {
            len += skill.serialise(&mut buf[len..])
        }

        len
    }
}

#[derive(Debug, Clone)]
pub struct LoadInventory {
    /// 0x01 on first inventory message else 0x00
    first: bool,
    items: Vec<RawItem>,
}
impl LoadInventory {
    pub fn start(items: Vec<RawItem>) -> Self {
        Self { items, first: true }
    }
    /// Must be called after the last LoadInventory message is sent
    pub fn end() -> Self {
        Self {
            items: vec![],
            first: false,
        }
    }
}
impl Serialise for LoadInventory {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(LOAD_INVENTORY, buf);

        to_le_bytes!(len, buf, self.first as u8);
        to_le_bytes!(len, buf, 3_u8);
        to_le_bytes!(len, buf, 0_u16);
        to_le_bytes!(len, buf, self.items.len() as u16);

        for item in &self.items {
            len += item.serialise(&mut buf[len..]);
        }

        len
    }
}

#[derive(Debug, Clone)]
pub struct ChangeChannel {
    raw: Vec<u8>,
}
impl ChangeChannel {
    pub fn new(raw: Vec<u8>) -> Self {
        Self { raw }
    }
}
impl Serialise for ChangeChannel {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(CHANGE_CHANNEL, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}
/// Contains information on player HP/MP/DP etc
#[derive(Debug, Clone)]
pub struct Status {
    character: Character,
    game_time: u32,
}
impl Status {
    pub fn new(character: &Character, game_time: u32) -> Self {
        Self {
            character: character.clone(),
            game_time,
        }
    }
}
impl Serialise for Status {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let stats = &self.character.stats;

        let mut len = 0;
        len += add_prelude(STATUS, buf);

        to_le_bytes!(len, buf, self.character.id());
        to_le_bytes!(len, buf, self.game_time);
        len += stats.primary.serialise(&mut buf[len..]);

        // Resistances??
        let unknown = [
            // Water Resist
            0x00, 0x00, // Wind Resistant
            0x00, 0x00, // Earth
            0x00, 0x00, // Fire
            0x00, 0x00, // ???
            0x00, 0x00, // ???
            0x00, 0x00,
        ];
        copy_bytes!(len, buf, unknown);
        to_le_bytes!(len, buf, stats.level);
        copy_bytes!(len, buf, [0u8; 6]);
        to_le_bytes!(len, buf, stats.exp_to_level);
        copy_bytes!(len, buf, [0u8; 8]);
        to_le_bytes!(len, buf, stats.exp);
        copy_bytes!(len, buf, [0u8; 4]);
        to_le_bytes!(len, buf, stats.hp.max());
        to_le_bytes!(len, buf, stats.hp.current());
        to_le_bytes!(len, buf, stats.mp.max());
        to_le_bytes!(len, buf, stats.mp.current());
        to_le_bytes!(len, buf, stats.dp.max());
        to_le_bytes!(len, buf, stats.dp.current());
        copy_bytes!(len, buf, [0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        to_le_bytes!(len, buf, stats.flight_time_max);
        to_le_bytes!(len, buf, stats.flight_time_current);
        copy_bytes!(len, buf, [0u8; 2]);
        to_le_bytes!(len, buf, stats.secondary.main_hand_attack);
        to_le_bytes!(len, buf, stats.secondary.off_hand_attack);
        to_le_bytes!(len, buf, stats.secondary.physical_defence);
        copy_bytes!(len, buf, [0xDA, 0x00]);
        to_le_bytes!(len, buf, stats.secondary.magic_resist);
        copy_bytes!(len, buf, [0x00, 0x00, 0xC0, 0x3F]);
        to_le_bytes!(len, buf, stats.attack_speed);
        to_le_bytes!(len, buf, stats.secondary.evasion);
        to_le_bytes!(len, buf, stats.secondary.parry);
        to_le_bytes!(len, buf, stats.secondary.block);
        to_le_bytes!(len, buf, stats.secondary.main_hand_crit);
        to_le_bytes!(len, buf, stats.secondary.off_hand_crit);
        to_le_bytes!(len, buf, stats.secondary.main_hand_accuracy);
        to_le_bytes!(len, buf, stats.secondary.off_hand_accuracy);
        copy_bytes!(len, buf, [0x02, 0x02]);
        to_le_bytes!(len, buf, stats.secondary.magic_accuracy);
        to_le_bytes!(len, buf, stats.secondary.crit_spell);
        copy_bytes!(len, buf, [0x32, 0x00]);
        to_le_bytes!(len, buf, stats.cast_speed);
        to_le_bytes!(len, buf, stats.secondary.unknown7);
        to_le_bytes!(len, buf, stats.secondary.magic_boost);
        to_le_bytes!(len, buf, stats.secondary.healing_boost);
        to_le_bytes!(len, buf, stats.secondary.crit_strike_resist);
        to_le_bytes!(len, buf, stats.secondary.spell_resist);
        to_le_bytes!(len, buf, stats.secondary.strike_fortitude);
        to_le_bytes!(len, buf, stats.secondary.spell_fortitude);
        let unknown = [
            0x58, 0x40, 0x36, 0x00, 0x00, 0x00, 0x27, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Combat talent
            0xCB, 0x07, 0x00, 0x00,
        ];
        copy_bytes!(len, buf, unknown);
        // Modified stats?
        len += self.character.stats.primary.serialise(&mut buf[len..]);
        copy_bytes!(len, buf, [0u8; 12]);
        to_le_bytes!(len, buf, stats.hp.base());
        to_le_bytes!(len, buf, stats.mp.base());
        to_le_bytes!(len, buf, stats.dp.current());
        copy_bytes!(len, buf, [0x00, 0x00, 0x64, 0x00, 0x00, 0x00]);
        to_le_bytes!(len, buf, stats.flight_time_max);
        to_le_bytes!(len, buf, stats.secondary.main_hand_attack);
        to_le_bytes!(len, buf, stats.secondary.off_hand_attack);
        to_le_bytes!(len, buf, stats.secondary.unknown1);
        to_le_bytes!(len, buf, stats.secondary.physical_defence);
        to_le_bytes!(len, buf, stats.secondary.magic_resist);
        to_le_bytes!(len, buf, stats.secondary.unknown2);
        to_le_bytes!(len, buf, stats.secondary.unknown3);
        to_le_bytes!(len, buf, stats.secondary.unknown4);
        to_le_bytes!(len, buf, stats.secondary.evasion);
        to_le_bytes!(len, buf, stats.secondary.parry);
        to_le_bytes!(len, buf, stats.secondary.block);
        to_le_bytes!(len, buf, stats.secondary.main_hand_crit);
        to_le_bytes!(len, buf, stats.secondary.off_hand_crit);
        to_le_bytes!(len, buf, stats.secondary.crit_spell);
        to_le_bytes!(len, buf, stats.secondary.unknown5);
        to_le_bytes!(len, buf, stats.secondary.main_hand_accuracy);
        to_le_bytes!(len, buf, stats.secondary.off_hand_accuracy);
        to_le_bytes!(len, buf, stats.secondary.unknown6);
        to_le_bytes!(len, buf, stats.secondary.magic_accuracy);
        to_le_bytes!(len, buf, stats.secondary.unknown7);
        to_le_bytes!(len, buf, stats.secondary.magic_boost);
        to_le_bytes!(len, buf, stats.secondary.healing_boost);
        to_le_bytes!(len, buf, stats.secondary.crit_strike_resist);
        to_le_bytes!(len, buf, stats.secondary.spell_resist);
        to_le_bytes!(len, buf, stats.secondary.strike_fortitude);
        to_le_bytes!(len, buf, stats.secondary.spell_fortitude);

        len
    }
}
#[derive(Debug, Clone)]
pub struct _0151 {
    raw: [u8; 6],
}
impl _0151 {
    pub fn new() -> Self {
        Self { raw: [0u8; 6] }
    }
}
impl Serialise for _0151 {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(0151, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}
#[derive(Debug, Clone)]
pub struct WorldInfo {
    raw: [u8; 11],
}
impl WorldInfo {
    pub fn new() -> Self {
        Self {
            raw: [
                0x09, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
                0x00,
            ],
        }
    }
}
impl Serialise for WorldInfo {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(WORLD_INFO, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

/// Subscription information
#[derive(Debug, Clone)]
pub struct GamepassInfo {
    raw: Vec<u8>,
}
impl GamepassInfo {
    pub fn new(raw: Vec<u8>) -> Self {
        Self { raw }
    }
}
impl Serialise for GamepassInfo {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(GAMEPASS_INFO, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

/// Sends a message to the players chat box
/// TODO: Implement params
#[derive(Debug, Clone)]
pub struct MessageCode {
    message_id: u32,
    params_len: u16,
}
impl MessageCode {
    pub fn new(message_id: u32) -> Self {
        Self {
            message_id,
            params_len: 0,
        }
    }
}
impl Serialise for MessageCode {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(MESSAGE_CODE, buf);

        to_le_bytes!(len, buf, 0x19 as u8);
        copy_bytes!(len, buf, [0u8; 5]);
        to_le_bytes!(len, buf, self.message_id);
        to_le_bytes!(len, buf, self.params_len);

        len
    }
}
#[derive(Debug, Clone)]
pub struct AbnormalStatus {
    raw: [u8; 10],
}
impl AbnormalStatus {
    pub fn new() -> Self {
        Self { raw: [0u8; 10] }
    }
}
impl Serialise for AbnormalStatus {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(ABNORMAL_STATUS, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum EntityType {
    Npc = 1,
    Character = 2,
}

#[derive(Debug, Clone)]
struct Effect {
    skill_id: u16,
    unknown1: u8,
    unknown2: u8,
    unknown3: u32,
}

impl Effect {
    pub fn new() -> Self {
        Self {
            skill_id: 555,
            unknown1: 9,
            unknown2: 1,
            unknown3: 0x07D0,
        }
    }
}

impl Serialise for Effect {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        to_le_bytes!(len, buf, self.skill_id);
        to_le_bytes!(len, buf, self.unknown1);
        to_le_bytes!(len, buf, self.unknown2);
        to_le_bytes!(len, buf, self.unknown3);
        len
    }
}

#[derive(Debug, Clone)]
pub struct AbnormalStatusOther {
    entity_id: u32,
    entity_type: EntityType,
    effects: Vec<Effect>,
}
impl AbnormalStatusOther {
    pub fn new(entity_id: u32) -> Self {
        Self {
            entity_id,
            entity_type: EntityType::Npc,
            effects: vec![Effect::new()],
        }
    }
    pub fn dead(entity_id: u32) -> Self {
        Self {
            entity_id,
            entity_type: EntityType::Npc,
            effects: vec![],
        }
    }
}
impl Serialise for AbnormalStatusOther {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(ABNORMAL_STATUS_OTHER, buf);

        to_le_bytes!(len, buf, self.entity_id);
        to_le_bytes!(len, buf, self.entity_type as u8);
        copy_bytes!(
            len,
            buf,
            [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ]
        );
        to_le_bytes!(len, buf, self.effects.len() as u16);
        for effect in &self.effects {
            len += effect.serialise(&mut buf[len..])
        }

        len
    }
}

#[derive(Debug, Clone)]
pub struct Time {
    game_time: u32,
}
impl Time {
    pub fn new(game_time: u32) -> Self {
        Self { game_time }
    }
}
impl Serialise for Time {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(TIME, buf);

        buf[len..len + 4].copy_from_slice(&self.game_time.to_le_bytes());
        len += 4;

        len
    }
}
#[derive(Debug, Clone)]
pub struct InvincibleTime {
    raw: [u8; 4],
}
impl InvincibleTime {
    pub fn start() -> Self {
        Self {
            raw: [0x60, 0xEA, 0x00, 0x00],
        }
    }
    pub fn finish() -> Self {
        Self {
            raw: [0x00, 0x00, 0x00, 0x00],
        }
    }
}
impl Serialise for InvincibleTime {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(INVINCIBLE_TIME, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}
#[derive(Debug, Clone)]
pub struct InvisibleLevel {
    character_id: u32,
    raw: [u8; 3],
}
impl InvisibleLevel {
    pub fn start(character_id: u32) -> Self {
        Self {
            raw: [0x40, 0x00, 0x01],
            character_id,
        }
    }
    pub fn finish(character_id: u32) -> Self {
        Self {
            raw: [0x00, 0x00, 0x00],
            character_id,
        }
    }
}
impl Serialise for InvisibleLevel {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(INVISIBLE_LEVEL, buf);

        to_le_bytes!(len, buf, self.character_id);
        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}
#[derive(Debug, Clone)]
pub struct WorldSceneStatus {
    raw: [u8; 9],
}
impl WorldSceneStatus {
    pub fn new() -> Self {
        Self {
            raw: [0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        }
    }
}
impl Serialise for WorldSceneStatus {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(WORLD_SCENE_STATUS, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}
#[derive(Debug, Clone)]
pub struct ResultPassport {
    unknown1: [u8; 8],
    character_id: u32,
    unknown2: [u8; 12],
}
impl ResultPassport {
    pub fn new(character_id: u32) -> Self {
        Self {
            unknown1: [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            character_id,
            unknown2: [
                0x15, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
        }
    }
}
impl Serialise for ResultPassport {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(RESULT_PASSPORT, buf);

        buf[len..len + self.unknown1.len()].copy_from_slice(&self.unknown1);
        len += self.unknown1.len();

        to_le_bytes!(len, buf, self.character_id);

        buf[len..len + self.unknown2.len()].copy_from_slice(&self.unknown2);
        len += self.unknown2.len();

        len
    }
}
#[derive(Debug, Clone)]
pub struct Weather {
    raw: [u8; 4],
}
impl Weather {
    pub fn new() -> Self {
        Self {
            raw: [0x00, 0x02, 0x00, 0x00],
        }
    }
}
impl Serialise for Weather {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(WEATHER, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}
#[derive(Debug, Clone)]
pub struct SignClient {
    raw: [u8; 68],
}
impl SignClient {
    pub fn new() -> Self {
        Self {
            raw: [
                0x40, 0x00, 0x00, 0x00, 0x2A, 0xCA, 0x3A, 0x34, 0x7C, 0xA1,
                0xA1, 0xAE, 0x02, 0x67, 0xFE, 0x23, 0x4A, 0x29, 0xB4, 0x32,
                0xCF, 0xF8, 0x4C, 0x44, 0x4D, 0x61, 0x9C, 0xAD, 0x57, 0x31,
                0xC6, 0xA9, 0x78, 0x75, 0x16, 0x55, 0x1B, 0xB5, 0x83, 0x58,
                0x43, 0x26, 0x42, 0x22, 0xA0, 0x48, 0xF3, 0xEF, 0x76, 0xB5,
                0x9D, 0x69, 0x9B, 0x93, 0x6C, 0xC3, 0xD8, 0x46, 0xA1, 0xC2,
                0xB8, 0xA6, 0x72, 0x32, 0x6C, 0xE6, 0xBD, 0x92,
            ],
        }
    }
}
impl Serialise for SignClient {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(SIGN_CLIENT, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}
#[derive(Debug, Clone)]
pub struct Title {
    raw: [u8; 3],
}
impl Title {
    pub fn new() -> Self {
        Self {
            raw: [0x01, 0x34, 0x00],
        }
    }
}
impl Serialise for Title {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(TITLE, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}
#[derive(Debug, Clone)]
pub struct LoadItemCooltime {
    raw: [u8; 2],
}
impl LoadItemCooltime {
    pub fn new() -> Self {
        Self { raw: [0x00, 0x00] }
    }
}
impl Serialise for LoadItemCooltime {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(LOAD_ITEM_COOLTIME, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}
#[derive(Debug, Clone)]
pub struct CustomAnim {
    raw: [u8; 3],
}
impl CustomAnim {
    pub fn new() -> Self {
        Self {
            raw: [0x01, 0x00, 0x00],
        }
    }
}
impl Serialise for CustomAnim {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(CUSTOM_ANIM, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

/// Place an npc into the game
#[derive(Debug, Clone)]
pub struct PutNpc {
    location: Coord,
    /// Unique ID of specific npc
    entity_id: u32,
    status_bar_id: npc::Id,
    appearence_id: npc::Id,
    ty: npc::Type,
    state: npc::State,
    rotation: u8,
    /// Name is an Id that client references to find string of name
    name: npc::Name,
    current_hp_pct: u8,
    current_hp: i32,
    level: u16,
    destination: Coord,
}

impl PutNpc {
    pub fn new(entity: &Entity) -> Self {
        Self {
            location: *entity.location(),
            entity_id: entity.id(),
            status_bar_id: entity.npc_id(),
            appearence_id: entity.npc_id(),
            ty: npc::Type::Aggressive,
            state: npc::State::Alive,
            rotation: 60,
            name: entity.npc_name(),
            current_hp_pct: entity.hp.percent(),
            current_hp: entity.hp.current(),
            level: entity.level(),
            destination: *entity.location(),
        }
    }
}
impl Serialise for PutNpc {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(PUT_NPC, buf);

        len += self.location.serialise(&mut buf[len..]);
        to_le_bytes!(len, buf, self.entity_id);
        to_le_bytes!(len, buf, self.status_bar_id as u32);
        to_le_bytes!(len, buf, self.appearence_id as u32);
        to_le_bytes!(len, buf, self.ty as u8);
        to_le_bytes!(len, buf, self.state as u16);
        to_le_bytes!(len, buf, self.rotation);
        to_le_bytes!(len, buf, self.name as u32);
        copy_bytes!(len, buf, [0u8; 17]);
        to_le_bytes!(len, buf, self.current_hp_pct);
        to_le_bytes!(len, buf, self.current_hp);
        to_le_bytes!(len, buf, self.level as u8);

        let unknown = [
            0x00, 0x00, 0x1F, 0x85, 0xCB, 0x3F, 0x1F, 0x85, 0x2B, 0x40, 0xE7,
            0xFB, 0xA9, 0x3F, 0x5E, 0x08, 0x5E, 0x08, 0x00,
        ];
        copy_bytes!(len, buf, unknown);

        len += self.destination.serialise(&mut buf[len..]);
        let unknown = [
            // MoveType???
            0xE4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        copy_bytes!(len, buf, unknown);

        len
    }
}

#[derive(Debug, Clone)]
pub struct AskQuitResult {
    raw: [u8; 9],
}
impl AskQuitResult {
    pub fn new() -> Self {
        Self {
            raw: [0x01, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF],
        }
    }
}
impl Serialise for AskQuitResult {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(ASK_QUIT_RESULT, buf);

        buf[len..len + self.raw.len()].copy_from_slice(&self.raw);
        len += self.raw.len();

        len
    }
}

/// 4 bytes - ID of entity doing action
/// 1 byte - Action ID
/// 1 byte - 0x01
/// 3 bytes - 0x00, 0x00, 0x00
/// 2 bytes - entity speed (f32 bit float but we only serialise 16 bits)
/// OPTIONAL
/// 4 bytes - target id
/// 1 byte - Emote type
/// 2 bytes - 0x00 0x01
#[derive(Debug, Clone)]
pub struct Action {
    entity_id: u32,
    action: ActionType,
    move_speed: f32,
    target_id: u32,
}
impl Action {
    pub fn new(
        entity_id: u32,
        action: ActionType,
        move_speed: f32,
        target_id: u32,
    ) -> Self {
        Self {
            entity_id,
            action,
            move_speed,
            target_id,
        }
    }
}
impl Serialise for Action {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(ACTION, buf);

        to_le_bytes!(len, buf, self.entity_id);
        to_le_bytes!(len, buf, u8::from(self.action));

        // TODO: Unknown
        copy_bytes!(len, buf, [0x00, 0x00, 0x00, 0x00]);

        let move_speed = (self.move_speed.to_bits() >> 16) as u16;
        to_le_bytes!(len, buf, move_speed);

        to_le_bytes!(len, buf, self.target_id);
        match self.action {
            ActionType::StartEmote(emote) => {
                to_le_bytes!(len, buf, u8::from(emote));
                copy_bytes!(len, buf, [0x00, 0x01]);
            }
            _ => {}
        }

        len
    }
}

#[derive(Debug, Clone)]
pub struct Attack {
    attacker_id: u32,
    sequence: u8,
    target_id: u32,
    target_hp_pct: u8,
    attacker_hp_pct: u8,
    hits: Vec<Hit>,
}

impl Attack {
    pub fn new(
        target_id: u32,
        attacker_id: u32,
        sequence: u8,
        hits: Vec<Hit>,
    ) -> Self {
        Self {
            attacker_id,
            sequence,
            target_id,
            target_hp_pct: 0,
            attacker_hp_pct: 0,
            hits,
        }
    }
}

impl Serialise for Attack {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(ATTACK, buf);

        to_le_bytes!(len, buf, self.attacker_id);
        to_le_bytes!(len, buf, self.sequence);
        copy_bytes!(len, buf, [0u8; 4]);
        to_le_bytes!(len, buf, self.target_id);
        to_le_bytes!(len, buf, self.target_hp_pct);
        to_le_bytes!(len, buf, self.attacker_hp_pct);
        copy_bytes!(len, buf, [0u8; 4]);
        to_le_bytes!(len, buf, self.hits.len() as u8);

        for hit in &self.hits {
            len += hit.serialise(&mut buf[len..]);
        }

        len
    }
}

#[derive(Debug, Clone)]
pub struct Loot {
    /// Who recieved the loot
    entity_id: u32,
    status: u8,
}

impl Loot {
    pub fn new(entity_id: u32, status: u8) -> Self {
        Self { entity_id, status }
    }
}

impl Serialise for Loot {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(LOOT, buf);

        to_le_bytes!(len, buf, self.entity_id);
        to_le_bytes!(len, buf, self.status);

        len
    }
}
#[derive(Debug, Clone)]
pub struct LootItemlist {
    entity_id: u32,
    loot: Vec<LootItem>,
}

impl LootItemlist {
    pub fn new(entity_id: u32, loot: Vec<LootItem>) -> Self {
        Self { entity_id, loot }
    }
}

impl Serialise for LootItemlist {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(LOOT_ITEMLIST, buf);

        to_le_bytes!(len, buf, self.entity_id);
        to_le_bytes!(len, buf, self.loot.len() as u16);
        for (i, loot) in self.loot.iter().enumerate() {
            to_le_bytes!(len, buf, i as u32);
            len += loot.serialise(&mut buf[len..]);
            // TODO: Untradable?
            to_le_bytes!(len, buf, 0u16);
        }
        len
    }
}

#[derive(Debug, Clone)]
pub struct ChangeItemDesc {}

impl ChangeItemDesc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Serialise for ChangeItemDesc {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(CHANGE_ITEM_DESC, buf);

        copy_bytes!(
            len,
            buf,
            [
                0xC5, 0xA3, 0x6B, 0xC7, 0x24, 0x00, //foo
                0x83, 0x80, 0x17, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x3E,
                0x63, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x19, 0x00, 0x00, 0x00, 0x00
            ]
        );

        len
    }
}

/// When you target an entity this updates their status bar
#[derive(Debug, Clone)]
pub struct TargetInfo {
    target_id: u32,
    level: u16,
    max_hp: i32,
    hp: i32,
}
impl TargetInfo {
    pub fn new(target_id: u32, level: u16, hp: i32, max_hp: i32) -> Self {
        Self {
            target_id,
            level,
            max_hp,
            hp,
        }
    }
}
impl Serialise for TargetInfo {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(TARGET_INFO, buf);

        to_le_bytes!(len, buf, self.target_id);
        to_le_bytes!(len, buf, self.level);
        to_le_bytes!(len, buf, self.max_hp);
        to_le_bytes!(len, buf, self.hp);

        len
    }
}

#[derive(Debug, Clone)]
pub struct NpcChangedTarget {
    npc_id: u32,
    target_id: u32,
    _unk: u8,
}
impl NpcChangedTarget {
    pub fn new(target_id: u32, npc_id: u32) -> Self {
        Self {
            target_id,
            npc_id,
            // TODO: What is this?
            _unk: 0x35,
        }
    }
}
impl Serialise for NpcChangedTarget {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(NPC_CHANGED_TARGET, buf);

        to_le_bytes!(len, buf, self.npc_id);
        to_le_bytes!(len, buf, self.target_id);
        to_le_bytes!(len, buf, self._unk);

        len
    }
}

#[derive(Debug, Clone)]
pub struct UseSkill {
    character_id: u32,
    skill_id: u16,
    skill_level: u16,
    target_id: u32,
    unknown: [u8; 8],
}
impl UseSkill {
    pub fn new(
        character_id: u32,
        skill_id: u16,
        skill_level: u16,
        target_id: u32,
    ) -> Self {
        Self {
            character_id,
            skill_id,
            skill_level,
            target_id,
            // 0x3F80 is 1.0???
            unknown: [0, 0, 0, 0, 0, 0x80, 0x3F, 0],
        }
    }
}
impl Serialise for UseSkill {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(USE_SKILL, buf);

        to_le_bytes!(len, buf, self.character_id);
        to_le_bytes!(len, buf, self.skill_id);
        to_le_bytes!(len, buf, self.skill_level);
        to_le_bytes!(len, buf, self.target_id);
        buf[len..len + self.unknown.len()].copy_from_slice(&self.unknown);
        len += self.unknown.len();

        len
    }
}

/// If the attack is a dash we have extra fields
#[derive(Debug, Clone)]
pub struct SkillSucceded {
    character_id: u32,
    target_id: u32,
    skill_id: u16,
    skill_level: u16,
    /// This is deciseconds, 500 = 50 seconds
    skill_cooldown: u32,
    /// Supplied if we dash to a target
    coords: Option<Coord>,
    target_hp_percent: u8,
    character_hp_percent: u8,
    hits: Vec<Hit>,
}
impl SkillSucceded {
    pub fn new(
        character_id: u32,
        target_id: u32,
        skill_id: u16,
        skill_level: u16,
        skill_cooldown: u32,
        hits: Vec<Hit>,
    ) -> Self {
        Self {
            target_id,
            character_id,
            skill_id,
            skill_level,
            skill_cooldown,
            target_hp_percent: 0,
            character_hp_percent: 0,
            hits,
            coords: None,
        }
    }
}
impl Serialise for SkillSucceded {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut ptr = 0;
        ptr += add_prelude(SKILL_SUCCEDED, buf);

        to_le_bytes!(ptr, buf, self.character_id);
        to_le_bytes!(ptr, buf, 0u8);
        to_le_bytes!(ptr, buf, self.target_id);
        to_le_bytes!(ptr, buf, self.skill_id);
        to_le_bytes!(ptr, buf, self.skill_level as u8);
        to_le_bytes!(ptr, buf, self.skill_cooldown);
        copy_bytes!(
            ptr,
            buf,
            [0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00]
        );
        to_le_bytes!(ptr, buf, self.target_id);
        to_le_bytes!(ptr, buf, 0u8);
        to_le_bytes!(ptr, buf, self.target_hp_percent);
        to_le_bytes!(ptr, buf, self.character_hp_percent);
        copy_bytes!(
            ptr,
            buf,
            [
                0x00, // Attack type in the combat log
                0x30, 0x00, 0x00, 0x00,
            ]
        );
        to_le_bytes!(ptr, buf, self.hits.len() as u16);
        for hit in &self.hits {
            ptr += hit.serialise(&mut buf[ptr..]);
        }

        ptr
    }
}

#[derive(Debug, Clone)]
pub struct HitPoint {
    hp_current: i32,
    hp_max: i32,
}
impl HitPoint {
    pub fn new(hp_current: i32, hp_max: i32) -> Self {
        Self { hp_current, hp_max }
    }
}
impl Serialise for HitPoint {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(HIT_POINT, buf);

        to_le_bytes!(len, buf, self.hp_current);
        to_le_bytes!(len, buf, self.hp_max);

        len
    }
}

#[derive(Debug, Clone)]
pub struct HitPointOther {
    entity_id: u32,
    hp_percent: u8,
}
impl HitPointOther {
    pub fn new(entity_id: u32, hp_percent: u8) -> Self {
        Self {
            entity_id,
            hp_percent,
        }
    }
}
impl Serialise for HitPointOther {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(HIT_POINT_OTHER, buf);

        to_le_bytes!(len, buf, self.entity_id);
        to_le_bytes!(len, buf, 0_u32);
        to_le_bytes!(len, buf, 0x05_u8);
        to_le_bytes!(len, buf, self.hp_percent);
        to_le_bytes!(len, buf, 0_u16);
        to_le_bytes!(len, buf, 0xB2_u16);

        len
    }
}

/// TODO: Create real authentication
#[derive(Debug, Clone)]
pub struct ReconnectKey {
    key: [u8; 4],
}
impl ReconnectKey {
    pub fn new() -> Self {
        Self { key: [0u8; 4] }
    }
}
impl Serialise for ReconnectKey {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(RECONNECT_KEY, buf);

        buf[len] = 0;
        len += 1;
        buf[len..len + self.key.len()].copy_from_slice(&self.key);
        len += self.key.len();

        len
    }
}

/// Not sure if this is needed, think it is only called once
#[derive(Debug, Clone)]
pub struct Alive {
    payload: [u8; 2],
}

impl Alive {
    pub fn new() -> Self {
        Self { payload: [0u8; 2] }
    }
}
impl Serialise for Alive {
    fn serialise(&self, buf: &mut [u8]) -> usize {
        let mut len = 0;
        len += add_prelude(ALIVE, buf);
        buf[len..len + self.payload.len()].copy_from_slice(&self.payload);
        len += self.payload.len();
        len
    }
}
