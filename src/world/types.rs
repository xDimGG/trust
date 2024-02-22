use std::{error::Error, fmt, io, str::{self, Utf8Error}};

use crate::binary::types::Vector2;
use crate::world::tile::Tile;
use crate::world::entity::Entity;

use super::binary::FileReader;

#[derive(PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum FileType {
	None,
	Map,
	World,
	Player,
}

impl From<u8> for FileType {
	fn from(value: u8) -> Self {
		match value {
			1 => Self::Map,
			2 => Self::World,
			3 => Self::Player,
			_ => Self::None,
		}
	}
}

#[derive(PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum GameMode {
	Normal,
	Expert,
	Master,
	Creative,
}

impl From<u8> for GameMode {
	fn from(value: u8) -> Self {
		match value {
			1 => Self::Expert,
			2 => Self::Master,
			3 => Self::Creative,
			_ => Self::Normal,
		}
	}
}

pub const MAGIC_STRING: &[u8] = "relogic".as_bytes();

#[derive(Debug)]
pub enum WorldDecodeError {
	UnexpectedEOI,
	InvalidNumber,
	BadFileSignature,
	ExpectedWorldType,
	InvalidFooter,
	InvalidEntityKind,
	InvalidCreativePower,
	InvalidString(Utf8Error),
	PositionCheckFailed(String),
	UnsupportedVersion(i32),
	FSError(io::Error),
}

impl fmt::Display for WorldDecodeError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::UnexpectedEOI => write!(f, "Expected more data but reached end of input"),
			Self::InvalidNumber => write!(f, "Could not parse number"),
			Self::InvalidString(err) => write!(f, "Could not parse string, got {}", err),
			Self::BadFileSignature => write!(f, "Invalid file signature (expecting \"{}\")", str::from_utf8(MAGIC_STRING).unwrap()),
			Self::ExpectedWorldType => write!(f, "Expected file type to be world file"),
			Self::InvalidFooter => write!(f, "Footer of file does not match header"),
			Self::InvalidEntityKind => write!(f, "Entity kind is not recognized"),
			Self::InvalidCreativePower => write!(f, "Creative power is not recognized"),
			Self::PositionCheckFailed(s) => write!(f, "Position of buffer cursor does not match metadata position for field {}", s),
			Self::UnsupportedVersion(v) => write!(f, "Unsupported file version: {}", v),
			Self::FSError(err) => write!(f, "Got FS error: {}", err),
		}
	}
}

impl Error for WorldDecodeError {}

#[derive(Debug, Clone)]
pub struct World {
	pub metadata: Metadata,
	pub format: Format,
	pub header: Header,
	pub tiles: Vec<Vec<Tile>>,
	pub chests: Vec<Chest>,
	pub signs: Vec<Sign>,
	pub npcs: Vec<NPC>,
	pub entities: Vec<Entity>,
	pub weighted_pressure_plates: Vec<WeightedPressurePlate>,
	pub room_locations: Vec<RoomLocation>,
	pub bestiary: Bestiary,
	pub creative_powers: Vec<CreativePower>,
}

#[derive(Debug, Clone)]
pub struct Metadata {
	pub version: i32,
	pub file_type: FileType,
	pub revision: u32,
	pub favorite: bool,
}

pub const BG_COUNT: usize = 13;

#[derive(Debug, Clone)]
pub struct Header {
	pub name: String,
	pub seed_text: String,
	pub worldgen_version: u64,
	pub uuid: Option<[u8; 16]>,
	pub id: i32,
	pub left: i32,
	pub right: i32,
	pub top: i32,
	pub bottom: i32,
	pub width: i32,
	pub height: i32,
	pub game_mode: GameMode,
	pub world_drunk: bool,
	pub world_for_the_worthy: bool,
	pub world_anniversary: bool,
	pub world_dont_starve: bool,
	pub world_not_the_bees: bool,
	pub world_remix: bool,
	pub world_no_traps: bool,
	pub world_zenith: bool,
	pub creation_time: i64,
	pub has_crimson: bool,
	pub hard_mode: bool,
	pub moon_type: i32,
	pub tree_x: [i32; 3],
	pub tree_style: [i32; 4],
	pub cave_back_x: [i32; 3],
	pub cave_back_style: [i32; 4],
	pub ice_back_style: i32,
	pub jungle_back_style: i32,
	pub hell_back_style: i32,
	pub spawn_x: i32,
	pub spawn_y: i32,
	pub world_surface: f64,
	pub rock_layer: f64,
	pub temp_time: f64,
	pub temp_day_time: bool,
	pub temp_moon_phase: i32,
	pub temp_blood_moon: bool,
	pub temp_eclipse: bool,
	pub dungeon_x: i32,
	pub dungeon_y: i32,
	pub downed_boss_1: bool,
	pub downed_boss_2: bool,
	pub downed_boss_3: bool,
	pub downed_queen_bee: bool,
	pub downed_mech_boss_1: bool,
	pub downed_mech_boss_2: bool,
	pub downed_mech_boss_3: bool,
	pub downed_mech_boss_any: bool,
	pub downed_plant_boss: bool,
	pub downed_golem_boss: bool,
	pub downed_slime_king: bool,
	pub saved_goblin: bool,
	pub saved_wizard: bool,
	pub saved_mechanic: bool,
	pub downed_goblins: bool,
	pub downed_clown: bool,
	pub downed_frost: bool,
	pub downed_pirates: bool,
	pub smashed_shadow_orb: bool,
	pub spawn_meteor: bool,
	pub shadow_orb_count: i32,
	pub altar_count: i32,
	pub after_party_of_doom: bool,
	pub invasion_delay: i32,
	pub invasion_size: i32,
	pub invasion_type: i32,
	pub invasion_x: f64,
	pub slime_rain_time: f64,
	pub sundial_cooldown: i32,
	pub temp_raining: bool,
	pub temp_rain_time: i32,
	pub temp_max_rain: f32,
	pub ore_tier_cobalt: i32,
	pub ore_tier_mythril: i32,
	pub ore_tier_adamantite: i32,
	pub bg: [u8; BG_COUNT],
	pub cloud_bg_active: f32,
	pub cloud_bg_alpha: f64,
	pub num_clouds: i16,
	pub wind_speed_target: f32,
	pub angler_who_finished_today: Vec<String>,
	pub saved_angler: bool,
	pub angler_quest: i32,
	pub saved_stylist: bool,
	pub saved_tax_collector: bool,
	pub saved_golfer: bool,
	pub invasion_size_start: i32,
	pub temp_cultist_delay: i32,
	pub npc_kill_counts: Vec<i32>,
	pub fast_forward_time_to_dawn: bool,
	pub downed_fishron: bool,
	pub downed_martians: bool,
	pub downed_ancient_cultist: bool,
	pub downed_moonlord: bool,
	pub downed_halloween_king: bool,
	pub downed_halloween_tree: bool,
	pub downed_christmas_ice_queen: bool,
	pub downed_christmas_santank: bool,
	pub downed_christmas_tree: bool,
	pub downed_tower_solar: bool,
	pub downed_tower_vortex: bool,
	pub downed_tower_nebula: bool,
	pub downed_tower_stardust: bool,
	pub tower_active_solar: bool,
	pub tower_active_vortex: bool,
	pub tower_active_nebula: bool,
	pub tower_active_stardust: bool,
	pub lunar_apocalypse_is_up: bool,
	pub temp_party_manual: bool,
	pub temp_party_genuine: bool,
	pub temp_party_cooldown: i32,
	pub temp_party_celebrating_npcs: Vec<i32>,
	pub temp_sandstorm_happening: bool,
	pub temp_sandstorm_time_left: i32,
	pub temp_sandstorm_severity: f32,
	pub temp_sandstorm_intended_severity: f32,
	pub saved_bartender: bool,
	pub downed_dd2_invasion_t1: bool,
	pub downed_dd2_invasion_t2: bool,
	pub downed_dd2_invasion_t3: bool,
	pub combat_book_was_used: bool,
	pub temp_lantern_night_cooldown: i32,
	pub temp_lantern_night_genuine: bool,
	pub temp_lantern_night_manual: bool,
	pub temp_lantern_night_next_night_is_genuine: bool,
	pub tree_top_variations: Vec<i32>,
	pub force_halloween_for_today: bool,
	pub force_xmas_for_today: bool,
	pub ore_tier_copper: i32,
	pub ore_tier_iron: i32,
	pub ore_tier_silver: i32,
	pub ore_tier_gold: i32,
	pub bought_cat: bool,
	pub bought_dog: bool,
	pub bought_bunny: bool,
	pub downed_empress_of_light: bool,
	pub downed_queen_slime: bool,
	pub downed_deerclops: bool,
	pub unlocked_slime_blue_spawn: bool,
	pub unlocked_merchant_spawn: bool,
	pub unlocked_demolition_spawn: bool,
	pub unlocked_party_girl_spawn: bool,
	pub unlocked_dye_trader_spawn: bool,
	pub unlocked_truffle_spawn: bool,
	pub unlocked_arms_dealer_spawn: bool,
	pub unlocked_nurse_spawn: bool,
	pub unlocked_princess_spawn: bool,
	pub combat_book_volume_two_was_used: bool,
	pub peddlers_satchel_was_used: bool,
	pub unlocked_slime_green_spawn: bool,
	pub unlocked_slime_old_spawn: bool,
	pub unlocked_slime_purple_spawn: bool,
	pub unlocked_slime_rainbow_spawn: bool,
	pub unlocked_slime_red_spawn: bool,
	pub unlocked_slime_yellow_spawn: bool,
	pub unlocked_slime_copper_spawn: bool,
	pub fast_forward_time_to_dusk: bool,
	pub moondial_cooldown: u8,
}

#[derive(Debug, Clone)]
pub struct Format {
	pub importance: Vec<bool>,
	pub positions: Vec<i32>,
}

pub const WALL_COUNT: u16 = 347; // WallID.Count

#[derive(Debug, Clone)]
pub struct Chest {
	pub x: i32,
	pub y: i32,
	pub name: String,
	pub items: Vec<ChestItem>,
}

#[derive(Debug, Clone, Default)]
pub struct ChestItem {
	pub id: i32,
	pub stack: i16,
	pub prefix: u8,
}

#[derive(Debug, Clone)]
pub struct Sign {
	pub x: i32,
	pub y: i32,
	pub text: String,
}

#[derive(Debug, Clone)]
pub struct NPC {
	pub id: i32,
	pub name: String,
	pub position: Vector2,
	pub homeless: bool,
	pub shimmer: bool,
	pub home_x: i32,
	pub home_y: i32,
	pub variation: i32,
}

#[derive(Debug, Clone)]
pub struct WeightedPressurePlate {
	pub x: i32,
	pub y: i32,
}

#[derive(Debug, Clone)]
pub struct RoomLocation {
	pub id: i32,
	pub x: i32,
	pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Bestiary {
	pub kills: Vec<(String, i32)>, // (npc id, kill count)
	pub sights: Vec<String>, // npc IDs
	pub chats: Vec<String>, // npc IDs
}

#[derive(Debug, Clone)]
pub enum CreativePower {
	FreezeTime(bool),
	StartDayImmediately,
	StartNoonImmediately,
	StartNightImmediately,
	StartMidnightImmediately,
	GodmodePower,
	ModifyWindDirectionAndStrength,
	ModifyRainPower,
	ModifyTimeRate(f32),
	FreezeRainPower(bool),
	FreezeWindDirectionAndStrength(bool),
	FarPlacementRangePower,
	DifficultySliderPower(f32),
	StopBiomeSpreadPower(bool),
	SpawnRateSliderPerPlayerPower,
}

impl CreativePower {
	pub fn decode_file(r: &mut FileReader) -> Result<CreativePower, WorldDecodeError> {
		match r.read_i16()? {
			0 => Ok(CreativePower::FreezeTime(r.read_bool()?)),
			1 => Ok(CreativePower::StartDayImmediately),
			2 => Ok(CreativePower::StartNoonImmediately),
			3 => Ok(CreativePower::StartNightImmediately),
			4 => Ok(CreativePower::StartMidnightImmediately),
			5 => Ok(CreativePower::GodmodePower),
			6 => Ok(CreativePower::ModifyWindDirectionAndStrength),
			7 => Ok(CreativePower::ModifyRainPower),
			8 => Ok(CreativePower::ModifyTimeRate(r.read_f32()?)),
			9 => Ok(CreativePower::FreezeRainPower(r.read_bool()?)),
			10 => Ok(CreativePower::FreezeWindDirectionAndStrength(r.read_bool()?)),
			11 => Ok(CreativePower::FarPlacementRangePower),
			12 => Ok(CreativePower::DifficultySliderPower(r.read_f32()?)),
			13 => Ok(CreativePower::StopBiomeSpreadPower(r.read_bool()?)),
			14 => Ok(CreativePower::SpawnRateSliderPerPlayerPower),
			_ => Err(WorldDecodeError::InvalidCreativePower),
		}
	}
}
