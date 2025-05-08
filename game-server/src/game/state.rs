use std::collections::HashMap;

use crossbeam_channel::Sender;

use super::character::Character;
use super::data::gear::LootItem;
use super::data::{npc, ActionType};
use super::engine::Coord;
use super::entity::Entity;
use super::message::{client as c, server as s};
use super::{ClientUpdate, Messages, ServerUpdate};

pub struct State {
    characters: HashMap<u32, Character>,
    entities: Vec<Entity>,
}

impl State {
    pub fn new() -> Self {
        let mut entities = Vec::with_capacity(1000);

        let mosbear = Entity::new(
            2147586830,
            Coord::new(1816., 589., 256.),
            npc::Id::StarvedMosbear,
            npc::Name::StarvedMosbear,
        );
        entities.push(mosbear);

        Self {
            characters: HashMap::with_capacity(1000),
            entities,
        }
    }

    pub fn respond(&mut self, update: &ClientUpdate, messages: &mut Messages) {
        match &update.message() {
            c::Message::MoveNew(move_new) => {
                let character =
                    self.characters.get_mut(&update.character_id).unwrap();

                let s_move_new =
                    s::Message::MoveNew(move_new.calculate(character.id()));
                character.set_location(move_new.location());

                messages
                    .others
                    .entry(update.client_id())
                    .or_default()
                    .push(ServerUpdate::new(s_move_new));
            }
            c::Message::UseSkill(use_skill) => {
                let character =
                    self.characters.get_mut(&update.character_id).unwrap();

                if let Some(entity) = self.entities.iter_mut().find(|entity| {
                    entity.id() == use_skill.target_id && entity.is_alive()
                }) {
                    let damage = 250;
                    entity.hp.update(-damage);
                    messages.broadcast.extend(
                        [
                            s::Message::UseSkill(s::UseSkill::new(
                                character.id(),
                                use_skill.skill_id,
                                use_skill.skill_level,
                                use_skill.target_id,
                            )),
                            s::Message::SkillSucceded(s::SkillSucceded::new(
                                character.id(),
                                use_skill.target_id,
                                use_skill.skill_id,
                                use_skill.skill_level,
                                0,
                                vec![super::engine::damage::Hit::new(
                                    damage,
                                    super::engine::damage::Type::MainHand,
                                )],
                            )),
                            s::Message::HitPointOther(s::HitPointOther::new(
                                entity.id(),
                                entity.hp.percent(),
                            )),
                        ]
                        .map(ServerUpdate::new),
                    );

                    if entity.is_dead() {
                        messages.broadcast.extend(
                            [
                                s::Message::Action(s::Action::new(
                                    entity.id(),
                                    ActionType::Die,
                                    character.stats.move_speed,
                                    character.id(),
                                )),
                                s::Message::Loot(s::Loot::new(entity.id(), 0)),
                            ]
                            .map(ServerUpdate::new),
                        );
                    }
                }
            }
            c::Message::Attack(attack) => {
                if let Some(entity) = self.entities.iter_mut().find(|entity| {
                    entity.id() == attack.target_id && entity.is_alive()
                }) {
                    // Calculate the client attack and broadcast it
                    let character = self
                        .characters
                        .get_mut(&update.character_id())
                        .unwrap();
                    let hits = character.attack(entity);
                    let s_hit_point_other =
                        s::HitPointOther::new(entity.id(), entity.hp.percent());

                    messages.broadcast.extend(
                        [
                            s::Message::Attack(s::Attack::new(
                                entity.id(),
                                character.id(),
                                0,
                                hits,
                            )),
                            s::Message::HitPointOther(s_hit_point_other),
                        ]
                        .map(ServerUpdate::new),
                    );

                    if entity.is_dead() {
                        messages.broadcast.extend(
                            [
                                s::Message::Action(s::Action::new(
                                    entity.id(),
                                    ActionType::Die,
                                    character.stats.move_speed,
                                    character.id(),
                                )),
                                s::Message::Loot(s::Loot::new(entity.id(), 0)),
                            ]
                            .map(ServerUpdate::new),
                        );
                    }

                    // Gain aggro if mob is not fighting
                    if entity.target_id().is_none() {
                        let target = entity.set_taget(character.id());

                        messages.broadcast.extend(
                            [
                                s::Message::NpcChangedTarget(target),
                                s::Message::Action(s::Action::new(
                                    entity.id(),
                                    ActionType::EntityDrawWeapon,
                                    character.stats.move_speed,
                                    character.id(),
                                )),
                                s::Message::Action(s::Action::new(
                                    entity.id(),
                                    ActionType::Attack,
                                    character.stats.move_speed,
                                    character.id(),
                                )),
                            ]
                            .map(ServerUpdate::new),
                        );
                    }
                }
            }
            c::Message::ChangeTarget(change_target) => {
                if let Some(entity) = self
                    .entities
                    .iter()
                    .find(|entity| entity.id() == change_target.target_id)
                {
                    messages.broadcast.push(ServerUpdate::new(
                        s::Message::TargetInfo(s::TargetInfo::new(
                            change_target.target_id,
                            entity.level(),
                            entity.hp.current(),
                            entity.hp.max(),
                        )),
                    ));
                }
            }
            c::Message::Action(action) => {
                let character =
                    self.characters.get_mut(&update.character_id).unwrap();

                messages
                    .broadcast
                    .push(ServerUpdate::new(s::Message::Action(
                        s::Action::new(
                            character.id(),
                            action.ty,
                            character.stats.move_speed,
                            0,
                        ),
                    )));
            }
            c::Message::Loot(msg) => match msg.action {
                c::LootAction::Start => {
                    let character =
                        self.characters.get_mut(&update.character_id).unwrap();

                    messages.broadcast.push(ServerUpdate::new(
                        s::Message::Action(s::Action::new(
                            character.id(),
                            ActionType::Loot,
                            character.stats.move_speed,
                            msg.entity_id,
                        )),
                    ));

                    messages
                        .direct
                        .entry(update.client_id())
                        .or_default()
                        .extend(
                            [
                                s::Message::LootItemlist(s::LootItemlist::new(
                                    msg.entity_id,
                                    vec![LootItem::new(0x098975E6, 1)],
                                )),
                                s::Message::Loot(s::Loot::new(
                                    msg.entity_id,
                                    2,
                                )),
                            ]
                            .map(ServerUpdate::new),
                        );
                }
                c::LootAction::Stop => {}
            },
            c::Message::LootItem(msg) => {
                let character =
                    self.characters.get_mut(&update.character_id).unwrap();

                messages
                    .direct
                    .entry(update.client_id())
                    .or_default()
                    .extend(
                    [
                        s::Message::ChangeItemDesc(s::ChangeItemDesc::new()),
                        s::Message::Action(s::Action::new(
                            character.id(),
                            ActionType::EndLoot,
                            character.stats.move_speed,
                            msg.entity_id,
                        )),
                        s::Message::Loot(s::Loot::new(msg.entity_id, 3)),
                    ]
                    .map(ServerUpdate::new),
                );
            }
            c::Message::CharacterList(_) => {
                let character = Character::new(
                    //704660177,
                    update.client_id() as u32,
                    update.client_id(),
                    Coord::new(1816., 589., 256.),
                );
                let last_logged_in = 1722072570;

                messages
                    .direct
                    .entry(update.client_id())
                    .or_default()
                    .extend(
                        [s::Message::CharacterList(s::CharacterList::new(
                            [0u8; 4],
                            character.name().clone(),
                            character.gear.clone(),
                            last_logged_in,
                            character.id(),
                        ))]
                        .map(ServerUpdate::new),
                    );
                self.characters.insert(character.id(), character);
            }
            c::Message::LevelReady(_) => {
                let new_character =
                    self.characters.get(&update.character_id()).unwrap();

                // Load logging in character for all players
                messages.broadcast.extend(
                    [
                        s::Message::PutUser(s::PutUser::new(&new_character)),
                        s::Message::InvisibleLevel(s::InvisibleLevel::finish(
                            new_character.id(),
                        )),
                    ]
                    .map(ServerUpdate::new),
                );

                // Load all exisiting players for logging in character
                for character in &mut self.characters.values() {
                    // Skip ourselves
                    if character.id() == new_character.id() {
                        continue;
                    }

                    messages
                        .direct
                        .entry(update.client_id())
                        .or_default()
                        .extend(
                            [
                                s::Message::PutUser(s::PutUser::new(
                                    &character,
                                )),
                                s::Message::InvisibleLevel(
                                    s::InvisibleLevel::finish(character.id()),
                                ),
                            ]
                            .map(ServerUpdate::new),
                        );
                }

                // Load all entities for logging in player
                for entity in &self.entities {
                    messages
                        .direct
                        .entry(update.client_id())
                        .or_default()
                        .push(ServerUpdate::new(s::Message::PutNpc(
                            s::PutNpc::new(entity),
                        )));
                }
            }
            c::Message::CurStatus(_) => {
                let character =
                    self.characters.get_mut(&update.character_id()).unwrap();

                let game_time = 148641933;
                messages
                    .direct
                    .entry(update.client_id())
                    .or_default()
                    .extend(
                        [
                            // Most important packet
                            s::Message::World(s::World::new(
                                character.location(),
                            )),
                            // Required
                            s::Message::Status(s::Status::new(
                                &character, game_time,
                            )),
                            s::Message::WorldInfo(s::WorldInfo::new()),
                            s::Message::CurStatus(s::CurStatus::new()),
                            s::Message::AddSkill(s::AddSkill::new(
                                character.skills.clone(),
                            )),
                            s::Message::LoadInventory(s::LoadInventory::start(
                                character.items.values().cloned().collect(),
                            )),
                            s::Message::LoadInventory(s::LoadInventory::end()),
                        ]
                        .map(ServerUpdate::new),
                    );
            }
            _ => (),
        }
    }

    pub fn update(&mut self, messages: &mut Messages) {
        // Run update for all entities
        for entity in &mut self.entities {
            if entity.hp.current() <= 0 {
                continue;
            }

            let target_id = match entity.target_id() {
                Some(id) => id,
                None => continue,
            };

            let target = self.characters.get_mut(&target_id).unwrap();

            match entity.attack(target) {
                Some(hits) => {
                    let attack =
                        s::Attack::new(target.id(), entity.id(), 0, hits);
                    messages.broadcast.extend(
                        [
                            s::Message::Attack(attack),
                            s::Message::HitPointOther(s::HitPointOther::new(
                                target.id(),
                                target.stats.hp.percent(),
                            )),
                        ]
                        .map(ServerUpdate::new),
                    );
                    messages.direct.entry(target.client_id()).or_default().push(
                        ServerUpdate::new(s::Message::HitPoint(
                            s::HitPoint::new(
                                target.stats.hp.current(),
                                target.stats.hp.max(),
                            ),
                        )),
                    )
                }
                None => continue,
            };
        }
    }
}
