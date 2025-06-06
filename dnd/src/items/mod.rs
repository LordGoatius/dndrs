use derive_more::{Debug, Display};
use serde::{Deserialize, Serialize};

use crate::{attack::DamageType, dice::Dice, time::Time};

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
    #[display("Alchemist's Supplies")]
    AlchemistsSupplies,
    #[display("Brewer's Supplies")]
    BrewersSupplies,
    #[display("Calligrapher's Supplies")]
    CalligraphersSupplies,
    #[display("Carpenter's Tools")]
    CarpentersTools,
    #[display("Cartographer's Tools")]
    CartographersTools,
    #[display("Cobbler's Tools")]
    CobblersTools,
    #[display("Cook's Utensils")]
    CooksUtensils,
    #[display("Glassblower's Tools")]
    GlassblowersTools,
    #[display("Jeweler's Tools")]
    JewelersTools,
    #[display("Leatherworker's Tools")]
    LeatherworkersTools,
    #[display("Mason's Tools")]
    MasonsTools,
    #[display("Painter's Supplies")]
    PaintersSupplies,
    #[display("Potter's Tools")]
    PottersTools,
    #[display("Smith's Tools")]
    SmithsTools,
    #[display("Tinker's Tools")]
    TinkersTools,
    #[display("Weaver's Tools")]
    WeaversTools,
    #[display("Woodcarver's Tools")]
    WoodcarversTools,
    #[display("Dice Set")]
    DiceSet,
    #[display("Dragonchess Set")]
    DragonchessSet,
    #[display("Playing Card Set")]
    PlayingCardSet,
    #[display("Three Dragon Ante Set")]
    ThreeDragonanteSet,
    #[display("Bagpipes")]
    Bagpipes,
    #[display("Drum")]
    Drum,
    #[display("Dulcimer")]
    Dulcimer,
    #[display("Flute")]
    Flute,
    #[display("Lute")]
    Lute,
    #[display("Lyre")]
    Lyre,
    #[display("Horn")]
    Horn,
    #[display("Pan Flute")]
    PanFlute,
    #[display("Shawm")]
    Shawm,
    #[display("Viol")]
    Viol,
    #[display("Disguise Kit")]
    DisguiseKit,
    #[display("Forgery Kit")]
    ForgeryKit,
    #[display("Herbalism Kit")]
    HerbalismKit,
    #[display("Navigator's Tools")]
    NavigatorsTools,
    #[display("Poisoner's Kit")]
    PoisonersKit,
    #[display("Thieves' Tools")]
    ThievesTools,
    Custom(&'a str),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon<'a> {
    name: &'a str,
    properties: WeaponProperties,
    damage: Dice,
    damage_type: DamageType,
    weight: f64,
    // cost: Cost
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponProperties {
    ammunition: Option<Ammunition>,
    finesse: Option<Finesse>,
    heavy: Option<Heavy>,
    light: Option<Light>,
    loading: Option<Loading>,
    range: Option<Range>,
    reach: Option<Reach>,
    special: Option<Special>,
    thrown: Option<Thrown>,
    two_handed: Option<TwoHanded>,
    versatile: Option<Versatile>,
}

//== Type State Properties ==//

#[derive(Debug, Display, Serialize, Deserialize)]
#[display("{}/{}", self.0, self.1)]
pub struct RangeVal(usize, usize);

macro_rules! define_property {
    ($type_name:ident, $name:literal, $description:literal) => {
        #[derive(Debug, Display, serde::Serialize, serde::Deserialize)]
        #[debug($description)]
        #[display($name)]
        pub struct $type_name {}
    };
    ($type_name:ident, $name:literal, $description:literal, R) => {
        #[derive(Debug, Display, serde::Serialize, serde::Deserialize)]
        #[debug($description)]
        #[display($name)]
        pub struct $type_name {
            range: RangeVal
        }
    }
}

define_property!(Ammunition, "Ammunition", "You can use a weapon that has the ammunition property to make a ranged attack only if you have ammunition to fire from the weapon. Each time you attack with the weapon, you expend one piece of ammunition. Drawing the ammunition from a quiver, case, or other container is part of the attack. Loading a one-handed weapon requires a free hand. At the end of the battle, you can recover half your expended ammunition by taking a minute to search the battlefield. If you use a weapon that has the ammunition property to make a melee attack, you treat the weapon as an improvised weapon. A sling must be loaded to deal any damage when used in this way");
define_property!(Finesse, "Finesse", "When making an attack with a finesse weapon, you use your choice of your Strength or Dexterity modifier for the attack and damage rolls. You must use the same modifier for both rolls");
define_property!(Heavy, "Heavy", "Creatures that are Small or Tiny have disadvantage on attack rolls with heavy weapons. A heavy weapon's size and bulk make it too large for a Small or Tiny creature to use effectively");
define_property!(Light, "Light", "A light weapon is small and easy to handle, making it ideal for use when fighting with two weapons");
define_property!(Loading, "Loading", "Because of the time required to load this weapon, you can fire only one piece of ammunition from it when you use an action, bonus action, or reaction to fire it, regardless of the number of attacks you can normally make");
define_property!(Range, "Range", "A weapon that can be used to make a ranged attack has a range shown in parentheses after the ammunition or thrown property. The range lists two numbers. The first is the weapon’s normal range in feet, and the second indicates the weapon’s maximum range. When attacking a target beyond normal range, you have disadvantage on the attack roll. You can’t attack a target beyond the weapon’s long range", R);
define_property!(Reach, "Reach", "This weapon adds 5 feet to your reach when you attack with it. This property also determines your reach for opportunity attacks with a reach weapon");
define_property!(Special, "Special", "A weapon with the special property has unusual rules governing its use, explained in the weapon's description (See below");
define_property!(Thrown, "Thrown", "If a weapon has the thrown property, you can throw the weapon to make a ranged attack. If the weapon is a melee weapon, you use the same ability modifier for that attack roll and damage roll that you would use for a melee attack with the weapon. For example, if you throw a handaxe, you use your Strength, but if you throw a dagger, you can use either your Strength or your Dexterity, since the dagger has the finesse property", R);
define_property!(TwoHanded, "Two-Handed", "This weapon requires two hands to use. This property is relevant only when you attack with the weapon, not when you simply hold it");

// Versatile needs a field
#[derive(Debug,Display,Serialize,Deserialize)]
#[debug("This weapon can be used with one or two hands. A damage value in parentheses appears with the property—the damage when the weapon is used with two hands to make a melee attack")]
#[display("Versatile")]
pub struct Versatile {
    alt_die: Dice
}
