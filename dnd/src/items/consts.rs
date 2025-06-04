use super::*;

pub const PADDED: Armor = Armor {
    name: "Padded",
    armor_type: ArmorType::Light,
    base_ac: 11,
    dex_mod: true,
    dex_mod_max: u8::MAX,
    req_str: None,
    dis_stealth: true,
    weight: 8.0,
};

pub const LEATHER: Armor = Armor {
    name: "Leather",
    armor_type: ArmorType::Light,
    base_ac: 11,
    dex_mod: true,
    dex_mod_max: u8::MAX,
    req_str: None,
    dis_stealth: false,
    weight: 10.0,
};

pub const STUDDED_LEATHER: Armor = Armor {
    name: "Studded Leather",
    armor_type: ArmorType::Light,
    base_ac: 11,
    dex_mod: true,
    dex_mod_max: u8::MAX,
    req_str: None,
    dis_stealth: false,
    weight: 13.0,
};

//== MEDIUM ==//

pub const HIDE: Armor = Armor {
    name: "Hide",
    armor_type: ArmorType::Medium,
    base_ac: 12,
    dex_mod: true,
    dex_mod_max: 2,
    req_str: None,
    dis_stealth: false,
    weight: 12.0,
};
pub const CHAIN_SHIRT: Armor = Armor {
    name: "Chain Shirt",
    armor_type: ArmorType::Medium,
    base_ac: 13,
    dex_mod: true,
    dex_mod_max: 2,
    req_str: None,
    dis_stealth: false,
    weight: 20.0,
};
pub const SCALE_MAIL: Armor = Armor {
    name: "Scale Mail",
    armor_type: ArmorType::Medium,
    base_ac: 14,
    dex_mod: true,
    dex_mod_max: 2,
    req_str: None,
    dis_stealth: true,
    weight: 45.0,
};
pub const SPIKED_ARMOR: Armor = Armor {
    name: "Spiked Armor",
    armor_type: ArmorType::Medium,
    base_ac: 14,
    dex_mod: true,
    dex_mod_max: 2,
    req_str: None,
    dis_stealth: true,
    weight: 45.0,
};
pub const BREASTPLATE: Armor = Armor {
    name: "Breastplate",
    armor_type: ArmorType::Medium,
    base_ac: 14,
    dex_mod: true,
    dex_mod_max: 2,
    req_str: None,
    dis_stealth: false,
    weight: 20.0,
};
pub const HALFPLATE: Armor = Armor {
    name: "Halfplate",
    armor_type: ArmorType::Medium,
    base_ac: 15,
    dex_mod: true,
    dex_mod_max: 2,
    req_str: None,
    dis_stealth: true,
    weight: 40.0,
};

//== HEAVY ARMOR ==//

pub const RING_MAIL: Armor = Armor {
    name: "Ring Mail",
    armor_type: ArmorType::Heavy,
    base_ac: 14,
    dex_mod: false,
    dex_mod_max: u8::MAX,
    req_str: None,
    dis_stealth: true,
    weight: 40.0,
};
pub const CHAIN_MAIL: Armor = Armor {
    name: "Chain Mail",
    armor_type: ArmorType::Heavy,
    base_ac: 16,
    dex_mod: false,
    dex_mod_max: u8::MAX,
    req_str: Some(13),
    dis_stealth: true,
    weight: 55.0,
};
pub const SPLINT: Armor = Armor {
    name: "Splint",
    armor_type: ArmorType::Heavy,
    base_ac: 17,
    dex_mod: false,
    dex_mod_max: u8::MAX,
    req_str: Some(15),
    dis_stealth: true,
    weight: 60.0,
};
pub const PLATE: Armor = Armor {
    name: "Plate",
    armor_type: ArmorType::Heavy,
    base_ac: 18,
    dex_mod: false,
    dex_mod_max: u8::MAX,
    req_str: Some(15),
    dis_stealth: true,
    weight: 65.0,
};
