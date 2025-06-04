use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::time::Time;

pub mod consts;

#[derive(Debug, Serialize, Deserialize)]
pub struct Armor<'a> {
    name: &'a str,
    armor_type: ArmorType,
    base_ac: u8,
    dex_mod: bool,
    dex_mod_max: u8,
    req_str: Option<u8>,
    dis_stealth: bool,
    weight: f64,
    // TODO: Cost
    // cost: Cost
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
}

impl ArmorType {
    pub fn don_time(&self) -> Time {
        todo!()
    }

    pub fn doff_time(&self) -> Time {
        todo!()
    }
}

#[derive(Debug, Display, Serialize, Deserialize)]
pub enum Tool<'a> {
    AlchemistsSupplies,
    BrewersSupplies,
    CalligraphersSupplies,
    CarpentersTools,
    CartographersTools,
    CobblersTools,
    CooksUtensils,
    GlassblowersTools,
    JewelersTools,
    LeatherworkersTools,
    MasonsTools,
    PaintersSupplies,
    PottersTools,
    SmithsTools,
    TinkersTools,
    WeaversTools,
    WoodcarversTools,
    DiceSet,
    DragonchessSet,
    PlayingCardSet,
    ThreeDragonanteSet,
    Bagpipes,
    Drum,
    Dulcimer,
    Flute,
    Lute,
    Lyre,
    Horn,
    PanFlute,
    Shawm,
    Viol,
    DisguiseKit,
    ForgeryKit,
    HerbalismKit,
    NavigatorsTools,
    PoisonersKit,
    ThievesTools,
    Custom(&'a str)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon<'a> {
    name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WeaponOption {}
