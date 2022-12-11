use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum WeaponType {
    Knife,
    Pistol,
    Rifle,
    SniperRifle,
    #[serde(rename = "Submachine Gun")]
    SubmachineGun,
    #[serde(rename = "Machine Gun")]
    MachineGun,
    Shotgun,
    Grenade,
    C4,

    // Danger Zone only
    Fists,
    Melee,
    Tablet,
    #[serde(rename = "Breach Charge")]
    BreachCharge,
    #[serde(rename = "Bump Mine")]
    BumpMine,

    Unknown, // Zeus doesn't have any type
}

impl Default for WeaponType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WeaponState {
    Holstered,
    Active,
    Reloading,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Weapon {
    pub name: String,
    #[serde(rename = "paintkit")]
    pub paint_kit: String,
    #[serde(default)]
    pub r#type: WeaponType, // `type` is a reserved keyword
    pub state: WeaponState,
    pub ammo_clip: Option<u16>,
    pub ammo_clip_max: Option<u16>,
    pub ammo_reserve: Option<u16>,
}
