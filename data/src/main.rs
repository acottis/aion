use quick_xml::de::from_str;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

#[derive(Debug, Deserialize)]
#[serde(rename = "client_items")]
struct ClientItems {
    #[serde(rename = "client_item")]
    items: Vec<ClientItem>,
}

#[derive(Debug, Deserialize)]
struct ClientItem {
    id: u32,
    name: String,
    min_damage: Option<u16>,
    max_damage: Option<u16>,
    str: Option<u8>,
    agi: Option<u8>,
    kno: Option<u8>,
    hit_accuracy: Option<u16>,
    critical: Option<u16>,
    parry: Option<u16>,
    magical_skill_boost: Option<u16>,
    magical_hit_accuracy: Option<u16>,
    attack_type: Option<String>,
    attack_delay: Option<u16>,
    hit_count: u8,
    attack_gap: Option<f32>,
    attack_range: Option<f32>,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(s) => match s.as_str() {
            "TRUE" => Ok(true),
            "FALSE" => Ok(false),
            other => Err(Error::invalid_type(
                Unexpected::Str(other),
                &"TRUE or FALSE",
            )),
        },
        Err(_) => todo!(),
    }
}

fn main() {

    let xml_data = include_str!("../../bxml/items.xml");
    let parsed: ClientItems = from_str(xml_data).unwrap();

}
