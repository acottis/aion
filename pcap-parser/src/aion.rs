pub fn client_message_from_opcode(opcode: u16) -> String {
    match opcode {
        0 => "C_VERSION".into(),
        2 => "C_LOGOUT".into(),
        3 => "C_ASK_QUIT".into(),
        4 => "C_READY_TO_QUIT".into(),
        5 => "C_DEAD_RESTART".into(),
        6 => "C_CHECK_LEVEL_DATA_VERSION".into(),
        7 => "C_EDIT_CHARACTER".into(),
        8 => "C_ENTER_WORLD".into(),
        9 => "C_LEVEL_READY".into(),
        10 => "C_SAVE_CLIENT_SETTINGS".into(),
        0xb => "C_FIND_NPC_POS".into(),
        0xc => "C_CHANGE_OPTION_FLAGS".into(),
        0xd => "C_CHANGE_DIRECTION".into(),
        0xe => "C_CAPTCHA".into(),
        0xf => "C_ACCEPT_TELEPORT".into(),
        0x10 => "C_REQUEST_GUILD_NAME".into(),
        0x11 => "C_BLINK".into(),
        0x12 => "C_SYNC_TIME".into(),
        0x13 => "C_GATHER".into(),
        0x14 => "C_MINIGAME".into(),
        0x15 => "C_FUNCTIONAL_PET_MOVE".into(),
        0x16 => "C_FUNCTIONAL_PET".into(),
        0x17 => "C_TOGGLE_DOOR".into(),
        0x18 => "C_TOGGLE_CHEST".into(),
        0x19 => "C_GIVE_ITEM".into(),
        0x1a => "C_PETITION".into(),
        0x1b => "C_SAY".into(),
        0x1c => "C_WHISPER".into(),
        0x1f => "C_CHANGE_TARGET".into(),
        0x20 => "C_ATTACK".into(),
        0x21 => "C_USE_SKILL".into(),
        0x22 => "C_TURN_OFF_TOGGLE_SKILL".into(),
        0x23 => "C_TURN_OFF_ABNORMAL_STATUS".into(),
        0x24 => "C_TURN_OFF_MAINTAIN_SKILL".into(),
        0x25 => "C_USE_ITEM".into(),
        0x26 => "C_USE_EQUIPMENT_ITEM".into(),
        0x27 => "C_ASK_PC_INFO".into(),
        0x28 => "C_SAVE".into(),
        0x29 => "C_BUILDER_COMMAND".into(),
        0x2a => "C_BUILDER_CONTROL".into(),
        0x2b => "C_ACTION".into(),
        0x2c => "C_ALIVE".into(),
        0x2d => "C_GUILD".into(),
        0x2e => "C_LEAVE_INSTANTDUNGEON".into(),
        0x2f => "C_REQUEST_GUILD_EMBLEM_IMG".into(),
        0x30 => "C_MOVE_NEW".into(),
        0x31 => "C_PATH_FLY".into(),
        0x32 => "C_ANSWER".into(),
        0x33 => "C_BUY_SELL".into(),
        0x34 => "C_START_DIALOG".into(),
        0x35 => "C_END_DIALOG".into(),
        0x36 => "C_HACTION".into(),
        0x37 => "C_REQUEST_GUILD_HISTORY".into(),
        0x38 => "C_BOOKMARK".into(),
        0x39 => "C_DELETE_BOOKMARK".into(),
        0x3a => "C_TODAY_WORDS".into(),
        0x3b => "C_CHANGE_EMBLEM_VER".into(),
        0x3d => "C_ASK_PARTY_INFO".into(),
        0x3e => "C_ASK_LOG".into(),
        0x3f => "C_ASK_XCHG".into(),
        0x40 => "C_ADD_XCHG".into(),
        0x41 => "C_REMOVE_XCHG".into(),
        0x42 => "C_XCHG_GOLD".into(),
        0x43 => "C_CHECK_XCHG".into(),
        0x44 => "C_ACCEPT_XCHG".into(),
        0x45 => "C_CANCEL_XCHG".into(),
        0x46 => "C_WIND_PATH".into(),
        0x47 => "C_CUSTOM_ANIM".into(),
        0x4a => "C_ENCHANT_ITEM".into(),
        0x4c => "C_GUILD_FUND".into(),
        0x4d => "C_PARTY_MATCH".into(),
        0x4e => "C_CHARGE_ITEM".into(),
        0x50 => "C_GIVE_UP_QUEST".into(),
        0x51 => "C_QUIT_CUTSCENE".into(),
        0x54 => "C_ACCOUNT_INSTANTDUNGEON".into(),
        0x55 => "C_UNUSED_NEW_5".into(),
        0x56 => "C_QUERY_NUMBER_RESULT".into(),
        0x57 => "C_FATIGUE_KOREA".into(),
        0x58 => "C_TRADE_IN".into(),
        0x5a => "C_CHANGE_ITEM_SKIN".into(),
        0x5b => "C_GIVE_ITEM_PROC".into(),
        0x5d => "C_GET_ON_VEHICLE".into(),
        0x5e => "C_GET_OFF_VEHICLE".into(),
        0x60 => "C_PARTY".into(),
        0x61 => "C_PARTY_BY_NAME".into(),
        0x62 => "C_ALLI_CHANGE_GROUP".into(),
        99 => "C_UNUSED_19".into(),
        100 => "C_VIEW_OTHER_INVENTORY".into(),
        0x67 => "C_PING".into(),
        0x68 => "C_NCGUARD".into(),
        0x69 => "C_UNUSED_21".into(),
        0x6a => "C_PLATE".into(),
        0x6b => "C_SIMPLE_DICE".into(),
        0x6c => "C_SPLIT_GOLD".into(),
        0x6d => "C_GET_PK_COUNT".into(),
        0x6e => "C_QUERY_BUDDY".into(),
        0x6f => "C_ADD_BUDDY".into(),
        0x70 => "C_REMOVE_BUDDY".into(),
        0x71 => "C_SMS".into(),
        0x72 => "C_DUEL".into(),
        0x74 => "C_DESTROY_ITEM".into(),
        0x76 => "C_REQUEST_ABYSS_GUILD_INFO".into(),
        0x77 => "C_PERSONAL_SHOP".into(),
        0x78 => "C_SHOP_MSG".into(),
        0x79 => "C_PET_ORDER".into(),
        0x7a => "C_GIVE_EXP_TO_PET".into(),
        0x7b => "C_VENDOR_ITEMLIST_CATEGORY".into(),
        0x7c => "C_VENDOR_ITEMLIST_NAME".into(),
        0x7d => "C_VENDOR_MYLIST".into(),
        0x7e => "C_VENDOR_BUY".into(),
        0x7f => "C_VENDOR_COMMIT".into(),
        0x80 => "C_VENDOR_CANCEL".into(),
        0x81 => "C_VENDOR_MYLOG".into(),
        0x82 => "C_VENDOR_COLLECT".into(),
        0x83 => "C_COMMPACKET".into(),
        0x84 => "C_MAIL_WRITE".into(),
        0x85 => "C_MAIL_LIST".into(),
        0x86 => "C_MAIL_READ".into(),
        0x87 => "C_MAIL_SETREAD".into(),
        0x88 => "C_MAIL_GETITEM".into(),
        0x89 => "C_MAIL_DELETE".into(),
        0x8a => "C_DICE".into(),
        0x8b => "C_CHANGE_TITLE".into(),
        0x8c => "C_REMOVE_TITLE".into(),
        0x8d => "C_COMBINE".into(),
        0x8e => "C_LOCATION".into(),
        0x8f => "C_MOVEBACK".into(),
        0x90 => "C_RECONNECT".into(),
        0x91 => "C_POLL_ANSWER".into(),
        0x92 => "C_REJECT_RESURRECT_BY_OTHER".into(),
        0x93 => "C_SPIN".into(),
        0x94 => "C_DESTINATION_AIRPORT".into(),
        0x95 => "C_L2AUTH_LOGIN".into(),
        0x96 => "C_CHARACTER_LIST".into(),
        0x97 => "C_CREATE_CHARACTER".into(),
        0x98 => "C_DELETE_CHARACTER".into(),
        0x99 => "C_RESTORE_CHARACTER".into(),
        0x9a => "C_LOOT".into(),
        0x9b => "C_LOOT_ITEM".into(),
        0x9c => "C_MOVE_ITEM_TO_ANOTHER_SLOT".into(),
        0x9d => "C_MOVE_STACKABLE_ITEM".into(),
        0x9e => "C_RECIPE_LIST".into(),
        0x9f => "C_SEARCH_USERS".into(),
        0xa0 => "C_UPLOAD_GUILD_EMBLEM_IMG_BEGIN".into(),
        0xa1 => "C_UPLOAD_GUILD_EMBLEM_IMG_DATA".into(),
        0xa2 => "C_MAIL_POSTMAN".into(),
        0xa3 => "C_ALL_FOG_CLEARED".into(),
        0xa4 => "C_SHARE_QUEST".into(),
        0xa5 => "C_ADD_BUDDY_ANS".into(),
        0xa6 => "C_ADD_BLOCK".into(),
        0xa7 => "C_REMOVE_BLOCK".into(),
        0xa8 => "C_QUERY_BLOCK".into(),
        0xa9 => "C_CHANGE_BLOCK_NAME".into(),
        0xaa => "C_CUR_STATUS".into(),
        0xab => "C_VIRTUAL_AUTH".into(),
        0xac => "C_CHANGE_CHANNEL".into(),
        0xad => "C_FOLLOW_CHANNEL".into(),
        0xae => "C_SIGN_CLIENT".into(),
        0xaf => "C_SAVE_MACRO".into(),
        0xb0 => "C_DELETE_MACRO".into(),
        0xb1 => "C_CHECK_EXIST".into(),
        0xb2 => "C_SWAP_ITEM_SLOT".into(),
        0xb3 => "C_CHANGE_BLOCK_MEMO".into(),
        0xb4 => "C_DEBUG_COMMAND".into(),
        0xb5 => "C_TACTICS_SIGN".into(),
        0xb8 => "C_RECONNECT_AUTH".into(),
        0xb9 => "C_GROUP_ITEM_DIST".into(),
        0xba => "C_GROUP_CHANGE_LOOTDIST".into(),
        0xbb => "C_SA_ACCOUNT_ITEM_QUERY".into(),
        0xbc => "C_SA_ACCOUNT_ITEM_ACK".into(),
        0xbd => "C_REQUEST_ABYSS_RANKER_INFO".into(),
        0xbe => "C_ROUTE_INFO".into(),
        0xbf => "C_CHECK_MESSAGE".into(),
        0xc0 => "C_ACCUSE_CHARACTER".into(),
        0xc1 => "C_INSTANCE_DUNGEON_COOLTIMES".into(),
        0xc2 => "C_SHOP_REQUEST".into(),
        0xc3 => "C_ASK_BOT_POINT".into(),
        0xc4 => "C_RECALLED_BY_OTHER_ANSWER".into(),
        0xc5 => "C_REQUEST_SERIAL_KILLER_LIST".into(),
        0xc6 => "C_ADDED_SERVICE_REQUEST".into(),
        199 => "C_SNDC_CHECK_MESSAGE".into(),
        200 => "C_GGAUTH_CHECK_ANSWER".into(),
        0xc9 => "C_MATCHMAKER_REQ".into(),
        0xca => "C_CLIENTSIDE_NPC_MOVE".into(),
        0xcb => "C_CLIENTSIDE_NPC_ACTION".into(),
        0xcc => "C_CLIENTSIDE_NPC_ATTACK".into(),
        0xcd => "C_CLIENTSIDE_NPC_BLINK".into(),
        0xce => "C_CLIENTSIDE_NPC_USE_SKILL".into(),
        0xcf => "C_COMPOUND_2H_WEAPON".into(),
        0xd0 => "C_REMOVE_COMPOUND".into(),
        0xd1 => "C_ASK_GLOBAL_PLAYTIME_FATIGUE_IN".into(),
        0xd2 => "C_2ND_PASSWORD".into(),
        0xd3 => "C_UNUSED_2ND_PASSWORD1".into(),
        0xd4 => "C_UNUSED_2ND_PASSWORD2".into(),
        0xd5 => "C_SA_GOODSLIST".into(),
        0xd6 => "C_SA_CONFIRMGOODS".into(),
        0xe4 => "C_GPK_CHECK_HEARTBEAT".into(),
        0xda => "C_REQUEST_NPSHOP_GOODS_COUNT".into(),
        0xfc => "C_GLOBAL_TRADE_LIST".into(),
        0xfd => "C_GLOBAL_TRADE_MYLIST".into(),
        0xfe => "C_GLOBAL_TRADE_BUY".into(),
        0xff => "C_GLOBAL_TRADE_COMMIT".into(),
        0x100 => "C_GLOBAL_TRADE_CANCEL".into(),
        0x101 => "C_GLOBAL_TRADE_SALESLOG".into(),
        0x102 => "C_GLOBAL_TRADE_COLLECT".into(),
        0x103 => "C_GLOBAL_TRADE_AVG_PRICE".into(),
        0x104 => "C_GLOBAL_TRADE_STATE".into(),
        0x105 => "C_GLOBAL_TRADE_HISTORY".into(),
        0xf3 => "C_REQUEST_RANK_LIST".into(),
        0xf4 => "C_REQUEST_RANK_INFO".into(),
        0xf8 => "C_REQUEST_CHANGE_RANKING_BADGE".into(),
        0xf9 => "C_REQUEST_RANKING_BADGE_LIST".into(),
        0xf0 => "C_HQ_LOGIN".into(),
        0xe2 => "C_READY_ENTER_WORLD_ACK".into(),
        0x10c => "C_USER_STORY_BOOK_REGISTER_ITEM".into(),
        0x114 => "C_PROTOCOL_MAX".into(),
        _ => "UNKNOWN".into(),
    }
}
pub fn server_message_from_opcode(opcode: u16) -> String {
    match opcode {
        0 => "S_VERSION_CHECK".into(),
        1 => "S_STATUS".into(),
        2 => "S_STATUS_OTHER".into(),
        3 => "S_HIT_POINT".into(),
        4 => "S_MANA_POINT".into(),
        5 => "S_HIT_POINT_OTHER".into(),
        6 => "S_DP".into(),
        7 => "S_DP_USER".into(),
        8 => "S_EXP".into(),
        9 => "S_LOGIN_CHECK".into(),
        10 => "S_CUTSCENE_NPC_INFO".into(),
        0xb => "S_CHANGE_GUILD_MEMBER_NICKNAME".into(),
        0xc => "S_GUILD_HISTORY".into(),
        0xd => "S_ENTER_WORLD_CHECK".into(),
        0xe => "S_PUT_NPC".into(),
        0xf => "S_WORLD".into(),
        0x10 => "S_DUMMY_PACKET".into(),
        0x11 => "S_PUT_OBJECT".into(),
        0x12 => "S_PUT_VEHICLE".into(),
        0x13 => "S_BUILDER_RESULT".into(),
        0x14 => "S_REQUEST_TELEPORT".into(),
        0x15 => "S_BLINK".into(),
        0x16 => "S_REMOVE_OBJECT".into(),
        0x17 => "S_WAIT_LIST".into(),
        0x18 => "S_MESSAGE".into(),
        0x19 => "S_MESSAGE_CODE".into(),
        0x1a => "S_LOAD_INVENTORY".into(),
        0x1b => "S_ADD_INVENTORY".into(),
        0x1c => "S_REMOVE_INVENTORY".into(),
        0x1d => "S_CHANGE_ITEM_DESC".into(),
        0x1e => "S_LOAD_CLIENT_SETTINGS".into(),
        0x1f => "S_CHANGE_STANCE".into(),
        0x20 => "S_PUT_USER".into(),
        0x21 => "S_USE_SKILL".into(),
        0x22 => "S_GATHER_OTHER".into(),
        0x23 => "S_GATHER".into(),
        0x24 => "S_WIELD".into(),
        0x25 => "S_ACTION".into(),
        0x26 => "S_TIME".into(),
        0x27 => "S_SYNC_TIME".into(),
        0x28 => "S_NPC_CHANGED_TARGET".into(),
        0x29 => "S_TARGET_INFO".into(),
        0x2a => "S_SKILL_CANCELED".into(),
        0x2b => "S_SKILL_SUCCEDED".into(),
        0x2c => "S_ADD_SKILL".into(),
        0x2d => "S_DELETE_SKILL".into(),
        0x2e => "S_TOGGLE_SKILL_ON_OFF".into(),
        0x2f => "S_ADD_MAINTAIN_SKILL".into(),
        0x30 => "S_DELETE_MAINTAIN_SKILL".into(),
        0x31 => "S_ABNORMAL_STATUS".into(),
        0x32 => "S_ABNORMAL_STATUS_OTHER".into(),
        0x33 => "S_LOAD_SKILL_COOLTIME".into(),
        0x34 => "S_ASK".into(),
        0x35 => "S_CANCEL_ASK".into(),
        0x36 => "S_ATTACK".into(),
        0x37 => "S_MOVE_NEW".into(),
        0x38 => "S_MOVE_OBJECT".into(),
        0x39 => "S_CHANGE_DIRECTION".into(),
        0x3a => "S_POLYMORPH".into(),
        0x3b => "S_SKILL_OTHER".into(),
        0x3c => "S_NPC_HTML_MESSAGE".into(),
        0x3e => "S_GUILD_OTHER_INFO".into(),
        0x3f => "S_ADD_BOOKMARK".into(),
        0x40 => "S_ITEM_LIST".into(),
        0x41 => "S_GUILD_OTHER_MEMBER_INFO".into(),
        0x42 => "S_WEATHER".into(),
        0x43 => "S_INVISIBLE_LEVEL".into(),
        0x44 => "S_RECALLED_BY_OTHER".into(),
        0x45 => "S_EFFECT".into(),
        0x46 => "S_LOAD_WORKINGQUEST".into(),
        0x47 => "S_KEY".into(),
        0x48 => "S_RESET_SKILL_COOLING_TIME".into(),
        0x49 => "S_XCHG_START".into(),
        0x4a => "S_ADD_XCHG".into(),
        0x4b => "S_REMOVE_XCHG".into(),
        0x4c => "S_XCHG_GOLD".into(),
        0x4d => "S_XCHG_RESULT".into(),
        0x4e => "S_ADDREMOVE_SOCIAL".into(),
        0x4f => "S_CHECK_MESSAGE".into(),
        0x50 => "S_USER_CHANGED_TARGET".into(),
        0x52 => "S_EDIT_CHARACTER".into(),
        0x53 => "S_SERIAL_KILLER_LIST".into(),
        0x54 => "S_ABYSS_NEXT_PVP_CHANGE_TIME".into(),
        0x55 => "S_ABYSS_CHANGE_NEXT_PVP_STATUS".into(),
        0x56 => "S_CAPTCHA".into(),
        0x57 => "S_ADDED_SERVICE_CHANGE".into(),
        0x58 => "S_FIND_NPC_POS_RESULT".into(),
        0x59 => "S_PARTY_INFO".into(),
        0x5a => "S_PARTY_MEMBER_INFO".into(),
        0x60 => "S_GGAUTH_CHECK_QUERY".into(),
        0x61 => "S_ASK_QUIT_RESULT".into(),
        0x62 => "S_ASK_INFO_RESULT".into(),
        99 => "S_FATIGUE_INFO".into(),
        100 => "S_FUNCTIONAL_PET".into(),
        0x65 => "S_QUERY_NUMBER".into(),
        0x66 => "S_LOAD_ITEM_COOLTIME".into(),
        0x67 => "S_TODAY_WORDS".into(),
        0x68 => "S_PLAY_CUTSCENE".into(),
        0x69 => "S_GET_ON_VEHICLE".into(),
        0x6a => "S_GET_OFF_VEHICLE".into(),
        0x6c => "S_KICK".into(),
        0x6d => "S_GUILD_INFO".into(),
        0x6e => "S_ADD_GUILD_MEMBER".into(),
        0x6f => "S_DELETE_GUILD_MEMBER".into(),
        0x70 => "S_CHANGE_GUILD_MEMBER_INFO".into(),
        0x71 => "S_CHANGE_GUILD_OTHER".into(),
        0x72 => "S_ATTACK_RESULT".into(),
        0x74 => "S_DYNCODE_DATA".into(),
        0x75 => "S_SNDC_CHECK_MESSAGE".into(),
        0x76 => "S_CHANGE_GUILD_MEMBER_INTRO".into(),
        0x77 => "S_WANTED_LOGIN".into(),
        0x78 => "S_INSTANT_DUNGEON_INFO".into(),
        0x79 => "S_MATCHMAKER_INFO".into(),
        0x7a => "S_LOAD_FINISHEDQUEST".into(),
        0x7b => "S_QUEST".into(),
        0x7c => "S_NCGUARD".into(),
        0x7e => "S_UPDATE_ZONE_QUEST".into(),
        0x7f => "S_PING".into(),
        0x80 => "S_SHOP_RESULT".into(),
        0x81 => "S_EVENT".into(),
        0x83 => "S_BUDDY_LIST".into(),
        0x84 => "S_BOOK_LIST".into(),
        0x85 => "S_SHOP_SELL_LIST".into(),
        0x86 => "S_GROUP_ITEM_DIST".into(),
        0x87 => "S_ETC_STATUS".into(),
        0x88 => "S_SA_ACCOUNT_ITEM_NOTI".into(),
        0x89 => "S_ABYSS_RANKER_INFOS".into(),
        0x8a => "S_ABYSS_GUILD_INFOS".into(),
        0x8b => "S_WORLD_SCENE_STATUS".into(),
        0x8c => "S_INSTANCE_DUNGEON_COOLTIMES".into(),
        0x8d => "S_ALIVE".into(),
        0x8e => "S_DEBUG_PUT_BEACON".into(),
        0x8f => "S_PLACEABLE_BINDSTONE_INFO".into(),
        0x90 => "S_PERSONAL_SHOP".into(),
        0x91 => "S_VENDOR".into(),
        0x92 => "S_ENTER_WORLD_NOTIFY".into(),
        0x93 => "S_CUSTOM_ANIM".into(),
        0x94 => "S_SHOPAGENT2".into(),
        0x96 => "S_TRADE_IN".into(),
        0x98 => "S_ADD_PET".into(),
        0x99 => "S_REMOVE_PET".into(),
        0x9a => "S_CHANGE_PET_STATUS".into(),
        0x9b => "S_CHANGE_MASTER".into(),
        0x9c => "S_GUILD_MEMBER_INFO".into(),
        0x9d => "S_CHANGE_GUILD_INFO".into(),
        0x9e => "S_SHOP_POINT_INFO".into(),
        0x9f => "S_CHANGE_NPC_STATUS".into(),
        0xa0 => "S_MAIL".into(),
        0xa1 => "S_ALLOW_PET_USE_SKILL".into(),
        0xa2 => "S_WIND_PATH_RESULT".into(),
        0xa3 => "S_WIND_STATE_INFO".into(),
        0xa4 => "S_LOAD_GATHERCOMBINE_COOLTIME".into(),
        0xa5 => "S_PARTY_MATCH".into(),
        0xa6 => "S_USER_SELL_HISTORY_LIST".into(),
        0xa7 => "S_LOAD_WAREHOUSE".into(),
        0xa8 => "S_ADD_WAREHOUSE".into(),
        0xa9 => "S_REMOVE_WAREHOUSE".into(),
        0xaa => "S_CHANGE_WAREHOUSE_ITEM_DESC".into(),
        0xab => "S_SHOP_CATEGORY_INFO".into(),
        0xac => "S_SHOP_GOODS_LIST".into(),
        0xad => "S_SHOP_GOODS_INFO".into(),
        0xaf => "S_TITLE".into(),
        0xb0 => "S_2ND_PASSWORD".into(),
        0xb2 => "S_FATIGUE_KOREA".into(),
        0xb3 => "S_COMBINE_OTHER".into(),
        0xb4 => "S_COMBINE".into(),
        0xb5 => "S_PLAY_MODE".into(),
        0xb6 => "S_USE_ITEM".into(),
        0xb7 => "S_CHANGE_FLAG".into(),
        0xb8 => "S_DUEL".into(),
        0xb9 => "S_CLIENTSIDE_NPC_BLINK".into(),
        0xba => "S_FUNCTIONAL_PET_MOVE".into(),
        0xbb => "S_RECONNECT_OTHER_SERVER".into(),
        0xbc => "S_LOAD_PVP_ENV".into(),
        0xbd => "S_CHANGE_PVP_ENV".into(),
        0xbe => "S_POLL_CONTENTS".into(),
        0xbf => "S_GM_COMMENT".into(),
        0xc0 => "S_RESURRECT_INFO".into(),
        0xc1 => "S_RESURRECT_BY_OTHER".into(),
        0xc2 => "S_MOVEBACK".into(),
        0xc3 => "S_ROUTEMAP_INFO".into(),
        0xc4 => "S_GAUGE".into(),
        0xc5 => "S_SHOW_NPC_MOTION".into(),
        0xc6 => "S_L2AUTH_LOGIN_CHECK".into(),
        199 => "S_CHARACTER_LIST".into(),
        200 => "S_CREATE_CHARACTER".into(),
        0xc9 => "S_DELETE_CHARACTER".into(),
        0xca => "S_RESTORE_CHARACTER".into(),
        0xcb => "S_FORCE_BLINK".into(),
        0xcc => "S_LOOT".into(),
        0xcd => "S_LOOT_ITEMLIST".into(),
        0xce => "S_RECIPE_LIST".into(),
        0xcf => "S_SKILL_ACTIVATED".into(),
        0xd0 => "S_ABYSS_INFO".into(),
        0xd1 => "S_CHANGE_ABYSS_PVP_STATUS".into(),
        0xd2 => "S_SEARCH_USER_RESULT".into(),
        0xd3 => "S_GUILD_EMBLEM_UPLOAD_RESULT".into(),
        0xd4 => "S_GUILD_EMBLEM_IMG_BEGIN".into(),
        0xd5 => "S_GUILD_EMBLEM_IMG_DATA".into(),
        0xd6 => "S_GUILD_EMBLEM_UPDATED".into(),
        0xd7 => "S_SKILL_PENALTY_STATUS".into(),
        0xd8 => "S_SKILL_PENALTY_STATUS_OTHER".into(),
        0xd9 => "S_ABYSS_SHIELD_INFO".into(),
        0xdc => "S_ARTIFACT_INFO".into(),
        0xde => "S_BUDDY_RESULT".into(),
        0xdf => "S_BLOCK_RESULT".into(),
        0xe0 => "S_BLOCK_LIST".into(),
        0xe1 => "S_NOTIFY_BUDDY".into(),
        0xe3 => "S_CUR_STATUS".into(),
        0xe4 => "S_VIRTUAL_AUTH".into(),
        0xe5 => "S_CHANGE_CHANNEL".into(),
        0xe6 => "S_SIGN_CLIENT".into(),
        0xe7 => "S_LOAD_MACRO".into(),
        0xe8 => "S_MACRO_RESULT".into(),
        0xe9 => "S_EXIST_RESULT".into(),
        0xea => "S_EXTRA_ITEM_CHANGE_CONTEXT".into(),
        0xeb => "S_RESURRECT_LOC_INFO".into(),
        0xec => "S_WORLD_INFO".into(),
        0xed => "S_ABYSS_POINT".into(),
        0xee => "S_BUILDER_LEVEL".into(),
        0xef => "S_PETITION_STATUS".into(),
        0xf0 => "S_BUDDY_DATA".into(),
        0xf1 => "S_ADD_RECIPE".into(),
        0xf2 => "S_REMOVE_RECIPE".into(),
        0xf3 => "S_CHANGE_ABYSS_TELEPORTER_STATUS".into(),
        0xf4 => "S_FLIGHT_POINT".into(),
        0xf5 => "S_ALLIANCE_INFO".into(),
        0xf6 => "S_ALLIANCE_MEMBER_INFO".into(),
        0xf7 => "S_GROUP_INFO".into(),
        0xf8 => "S_GROUP_MEMBER_INFO".into(),
        0xf9 => "S_TACTICS_SIGN".into(),
        0xfa => "S_GROUP_READY".into(),
        0xfc => "S_TAX_INFO".into(),
        0xfd => "S_STORE_SALE_INFO".into(),
        0xfe => "S_INVINCIBLE_TIME".into(),
        0xff => "S_RECONNECT_KEY".into(),
        0x100 => "S_WEB_NOTI".into(),
        0x101 => "S_BM_PACK_LIST".into(),
        0x106 => "S_REPLY_NP_LOGIN_GAMESVR".into(),
        0x107 => "S_REPLY_NP_CONSUME_TOKEN_RESULT".into(),
        0x108 => "S_REPLY_NP_AUTH_TOKEN".into(),
        0x11f => "S_GPK_AUTH".into(),
        0x120 => "S_GPK_HEARTBEAT".into(),
        0x10a => "S_NPSHOP_GOODS_COUNT".into(),
        0x10b => "S_NPSHOP_GOODS_CHANGE".into(),
        0x10c => "S_RESPONSE_NPSHOP_GOODS_LIST".into(),
        0x10d => "S_RESPONSE_NPSHOP_GOODS_RECV".into(),
        0x119 => "S_GAMEPASS_INFO".into(),
        0x11a => "S_GAMEPASS_OTHER_UPDATED".into(),
        0x130 => "S_RANK_LIST".into(),
        0x131 => "S_RANK_INFO".into(),
        0x138 => "S_RANKING_BADGE".into(),
        0x139 => "S_RANKING_BADGE_OTHER".into(),
        0x13a => "S_RANKING_BADGE_LIST".into(),
        0x13c => "S_GLOBAL_TRADE_LIST".into(),
        0x13d => "S_GLOBAL_TRADE_MYLIST".into(),
        0x13e => "S_GLOBAL_TRADE_BUY".into(),
        0x13f => "S_GLOBAL_TRADE_COMMIT".into(),
        0x140 => "S_GLOBAL_TRADE_CANCEL".into(),
        0x141 => "S_GLOBAL_TRADE_SALESLOG".into(),
        0x142 => "S_GLOBAL_TRADE_COLLECT".into(),
        0x143 => "S_GLOBAL_TRADE_AVG_PRICE".into(),
        0x144 => "S_GLOBAL_TRADE_STATE".into(),
        0x145 => "S_GLOBAL_TRADE_HISTORY".into(),
        0x10e => "S_SERVER_ENV".into(),
        0x11d => "S_READY_ENTER_WORLD".into(),
        0x132 => "S_RESULT_PASSPORT_FIRST".into(),
        0x133 => "S_RESULT_PASSPORT".into(),
        0x10f => "S_LOAD_ACHIEVEMENT".into(),
        0x110 => "S_PROGRESS_ACHIEVEMENT".into(),
        0x111 => "S_REWARD_ACHIEVEMENT_RESULT".into(),
        0x112 => "S_CREATE_ACHIEVEMENT_EVENT".into(),
        0x113 => "S_DELETE_ACHIEVEMENT_EVENT".into(),
        0x114 => "S_UPDATE_ACHIEVEMENT_EVENT".into(),
        0x115 => "S_REWARD_ACHIEVEMENT_EVENT_RESUL".into(),
        0x116 => "S_CLEAR_ACHIEVEMENT_EVENT".into(),
        0x117 => "S_BATTLEPASS_LIST".into(),
        0x118 => "S_BATTLEPASS_UPDATED".into(),
        0x122 => "S_USER_CLASSIC_WARDROBE_LOAD".into(),
        0x123 => "S_USER_CLASSIC_WARDROBE_INFO_UPD".into(),
        0x124 => "S_USER_CLASSIC_WARDROBE_DATA_UPD".into(),
        0x109 => "S_LOAD_PROMOTION".into(),
        0x12f => "S_LOAD_EQUIPMENT_CHANGE".into(),
        0x146 => "S_USER_BIND_STONE_INFO".into(),
        0x121 => "S_CHAT_ACCUSE".into(),
        0x126 => "S_SPAM_FILTER_FLAG".into(),
        0x127 => "S_GLOBAL_EVENT_BOOST_LIST".into(),
        0x128 => "S_CHANNEL_CHATTING_PERMISSION".into(),
        0x129 => "S_LOAD_CHANNEL_CHATTING_BLACKLIS".into(),
        0x12a => "S_RESPONSE_CHANNEL_CHATTING_TELL".into(),
        299 => "S_ADD_CHANNEL_CHATTING_BLACKLIST".into(),
        300 => "S_CHANNEL_CHATTING_BLACKLIST_SET".into(),
        0x12d => "S_REMOVE_CHANNEL_CHATTING_BLACKL".into(),
        0x13b => "S_GOTCHA_NOTIFY".into(),
        0x14c => "S_USER_STORY_BOOK_FINISHED_LIST_".into(),
        0x14d => "S_USER_STORY_BOOK_REGISTERED_ITE".into(),
        0x14e => "S_USER_STORY_BOOK_UPDATE_RESULT".into(),
        0x14f => "S_USER_STORY_BOOK_RESET".into(),
        0x16e => "S_PROTOCOL_MAX".into(),

        _ => "UNKNOWN".into(),
    }
}

/// When an entity moves these bits are set to tell the game about
/// the type of movement
#[repr(u8)]
#[derive(Clone, Copy)]
enum MoveState {
    Stop = 0x00,
    Glide = 0x04,
    Fall = 0x08,
    Change = 0x40,
    Move = 0x80,

    NpcDirect = 0xE0,
    NpcWalkSlow = 0xE4,
    NpcRunFast = 0xE2,
}

#[derive(Debug, Clone, Copy)]
pub struct MoveType(u8);

impl MoveType {
    #[inline(always)]
    pub fn new(ty: u8) -> Self {
        Self(ty)
    }
    pub fn inner(&self) -> u8 {
        self.0
    }
    #[inline(always)]
    pub fn is_gliding(&self) -> bool {
        (self.0 & MoveState::Glide as u8) == MoveState::Glide as u8
    }
    #[inline(always)]
    pub fn is_moving(&self) -> bool {
        (self.0 & MoveState::Move as u8) == MoveState::Move as u8
    }
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        (self.0 & MoveState::Fall as u8) == MoveState::Fall as u8
    }
    #[inline(always)]
    pub fn is_changing(&self) -> bool {
        (self.0 & MoveState::Change as u8) == MoveState::Change as u8
    }
    #[inline(always)]
    pub fn is_stopping(&self) -> bool {
        (self.0 & MoveState::Stop as u8) == MoveState::Stop as u8
    }

    pub fn is(&self, state: MoveState) -> bool {
        (self.0 & state as u8) == state as u8
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

    pub fn inner(&self) -> u8 {
        self.0
    }
}

#[derive(Debug)]
enum MoveData {
    Continue { time: u32 },
    Change { x: f32, y: f32, z: f32, time: f32 },
    GlideChange { raw: [u8; 17] },
    GlideContinue { unknown: u8, time: u32 },
}
#[derive(Debug)]
pub struct CMoveNew {
    x: f32,
    y: f32,
    z: f32,
    direction: Direction,
    ty: MoveType,
    data: MoveData,
}
impl CMoveNew {
    pub fn deserialise(mut buf: &[u8]) -> Self {
        let x = f32::from_le_bytes(buf[..4].try_into().unwrap());
        buf = &buf[4..];

        let y = f32::from_le_bytes(buf[..4].try_into().unwrap());
        buf = &buf[4..];

        let z = f32::from_le_bytes(buf[..4].try_into().unwrap());
        buf = &buf[4..];

        let direction = Direction::new(buf[0]);
        buf = &buf[1..];

        let ty = MoveType::new(buf[0]);
        buf = &buf[1..];

        let data = if ty.is_gliding() {
            if ty.is_changing() {
                MoveData::GlideChange {
                    raw: buf.try_into().unwrap(),
                }
            } else {
                MoveData::GlideContinue {
                    unknown: buf[0],
                    time: u32::from_le_bytes(buf[1..].try_into().unwrap()),
                }
            }
        } else {
            if ty.is_changing() && !ty.is_falling() {
                MoveData::Change {
                    x: f32::from_le_bytes(buf[..4].try_into().unwrap()),
                    y: f32::from_le_bytes(buf[4..8].try_into().unwrap()),
                    z: f32::from_le_bytes(buf[8..12].try_into().unwrap()),
                    time: f32::from_le_bytes(buf[12..16].try_into().unwrap()),
                }
            } else {
                MoveData::Continue {
                    time: u32::from_le_bytes(buf.try_into().unwrap()),
                }
            }
        };

        Self {
            x,
            y,
            z,
            direction,
            ty,
            data,
        }
    }
}

#[derive(Debug)]
enum SMoveData {
    Continue { time: u32 },
    NpcWalkSlow { x: f32, y: f32, unknown: [u8; 5] },
    GlideChange { raw: [u8; 17] },
    GlideContinue { unknown: u8, time: u32 },
    Stop,
    NpcWalkFast { x: f32, y: f32, z: f32 },
    //NpcDirect { x: f32, y: f32, unknown: [u8; 4] },
    NpcDirect { x: f32, y: f32, z: f32 },
}

#[derive(Debug)]
pub struct SMoveNew {
    id: [u8; 4],
    x: f32,
    y: f32,
    z: f32,
    direction: Direction,
    ty: MoveType,
    data: SMoveData,
}

impl SMoveNew {
    pub fn deserialise(mut buf: &[u8]) -> Self {
        let id = buf[..4].try_into().unwrap();
        buf = &buf[4..];

        let x = f32::from_le_bytes(buf[..4].try_into().unwrap());
        buf = &buf[4..];

        let y = f32::from_le_bytes(buf[..4].try_into().unwrap());
        buf = &buf[4..];

        let z = f32::from_le_bytes(buf[..4].try_into().unwrap());
        buf = &buf[4..];

        let direction = Direction::new(buf[0]);
        buf = &buf[1..];

        let ty = MoveType::new(buf[0]);
        buf = &buf[1..];

        println!("{direction:02X?}, {ty:02X?} {buf:02X?}");
        let data = if ty.is(MoveState::NpcWalkSlow) {
            SMoveData::NpcWalkSlow {
                x: f32::from_le_bytes(buf[..4].try_into().unwrap()),
                y: f32::from_le_bytes(buf[4..8].try_into().unwrap()),
                unknown: buf[8..13].try_into().unwrap(),
            }
        } else if ty.is(MoveState::NpcRunFast) {
            SMoveData::NpcWalkFast {
                x: f32::from_le_bytes(buf[..4].try_into().unwrap()),
                y: f32::from_le_bytes(buf[4..8].try_into().unwrap()),
                z: f32::from_le_bytes(buf[8..12].try_into().unwrap()),
            }
        } else if ty.is(MoveState::NpcDirect) {
            SMoveData::NpcDirect {
                x: f32::from_le_bytes(buf[..4].try_into().unwrap()),
                y: f32::from_le_bytes(buf[4..8].try_into().unwrap()),
                z: f32::from_le_bytes(buf[8..12].try_into().unwrap()),
                //    unknown: buf[8..].try_into().unwrap(),
            }
        } else {
            SMoveData::Stop
        };

        Self {
            id,
            x,
            y,
            z,
            direction,
            ty,
            data,
        }
    }
}
