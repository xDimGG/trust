pub mod binary;

use std::{fmt, fs::{self, File}, io, os::windows::fs::MetadataExt, path::Path, str};

use crate::binary::reader::Reader;

use self::binary::SafeReader;

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

const MAGIC_STRING: &[u8] = "relogic".as_bytes();

pub enum WorldParseError {
	UnexpectedEOF,
	InvalidNumber,
	InvalidString,
	BadFileSignature,
	ExpectedWorldType,
	PositionCheckFailed,
	UnsupportedVersion(i32),
	FSError(io::Error),
}

impl fmt::Display for WorldParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::UnexpectedEOF => write!(f, "Expected more data but reach end of file."),
			Self::InvalidNumber => write!(f, "Could not parse number"),
			Self::InvalidString => write!(f, "Could not parse string"),
			Self::BadFileSignature => write!(f, "Invalid file signature (expecting \"{}\")", str::from_utf8(MAGIC_STRING).unwrap()),
			Self::ExpectedWorldType => write!(f, "Expected file type to be world file"),
			Self::PositionCheckFailed => write!(f, "Position of buffer cursor does not match metadata position"),
			Self::UnsupportedVersion(v) => write!(f, "Unsupported file version: {}", v),
			Self::FSError(err) => write!(f, "{}", err),
		}
	}
}

#[derive(Debug, Clone)]
pub struct World {
	pub metadata: WorldMetadata,
	pub format: WorldFormat,
	pub header: WorldHeader,
}

#[derive(Debug, Clone)]
pub struct WorldMetadata {
	pub version: i32,
	pub file_type: FileType,
	pub revision: u32,
	pub favorite: bool,
}

const BG_COUNT: usize = 13;

#[derive(Debug, Clone)]
pub struct WorldHeader {
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
	pub spawn_tile_x: i32,
	pub spawn_tile_y: i32,
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
	pub peddlers_satchel_was_use: bool,
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
pub struct WorldFormat {
	pub importance: Vec<bool>,
	pub positions: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct WorldTile {

}

impl World {
	pub fn from_file(path: &Path) -> Result<World, WorldParseError> {
		let contents = fs::read(path).map_err(WorldParseError::FSError)?;
		let mut reader = SafeReader::new(contents.as_slice(), 0);
		let mut world = Self::from_reader(&mut reader)?;

		if world.metadata.version < 141 {
			let file_metadata = fs::metadata(path).map_err(WorldParseError::FSError)?;
			world.header.creation_time = file_metadata.creation_time() as i64;
		}

		Ok(world)
	}

	// TODO: implement Crc32 to get seed_text value
	// TODO: implement C# system.datetime.frombinary for parsing ticks

	pub fn from_reader(r: &mut SafeReader) -> Result<World, WorldParseError> {
		let version = r.read_i32()?;
		if version >= 88 {
			Self::read_world_v2(r)
		} else {
			todo!("implement v1 before release 88");
		}
	} 

	pub fn read_world_v2(r: &mut SafeReader) -> Result<World, WorldParseError> {
		r.seek(0);
		let metadata = Self::read_metadata(r)?;
		let format = Self::read_format(r)?;
		let header = Self::read_header(r, metadata.version)?;
		if r.get_cur() != format.positions[1] as usize {
			return Err(WorldParseError::PositionCheckFailed)
		}

		Ok(World { metadata, format, header })
	}

	pub fn read_metadata(r: &mut SafeReader) -> Result<WorldMetadata, WorldParseError> {
		let version = r.read_i32()?;
		let (file_type, revision, favorite) = if version >= 135 {
			let magic = r.read_bytes(7)?;
			if magic != MAGIC_STRING {
				return Err(WorldParseError::BadFileSignature)
			}

			(FileType::from(r.read_byte()?), r.read_u32()?, (r.read_u64()? & 1) == 1)
		} else {
			(FileType::World, 0, false)
		};

		if file_type != FileType::World {
			return Err(WorldParseError::ExpectedWorldType)
		}

		if version > 279 {
			return Err(WorldParseError::UnsupportedVersion(version))
		}

		Ok(WorldMetadata {
			version,
			file_type,
			revision,
			favorite,
		})
	}

	pub fn read_format(r: &mut SafeReader) -> Result<WorldFormat, WorldParseError> {
		let mut positions = vec![0; r.read_i16()? as usize];
		for p in &mut positions {
			*p = r.read_i32()?;
		}

		let mut importance = vec![false; r.read_u16()? as usize];
		let mut n1 = 0;
		let mut n2 = 128;
		for i in &mut importance {
			if n2 == 128 {
				n1 = r.read_byte()?;
				n2 = 1;
			} else {
				n2 <<= 1;
			}

			if (n1 & n2) == n2 {
				*i = true;
			}
		}

		Ok(WorldFormat { positions, importance })
	}

	pub fn read_header(r: &mut SafeReader, version: i32) -> Result<WorldHeader, WorldParseError> {
		let name = r.read_string()?;
		let (seed_text, worldgen_version) = if version >= 179 {
			(
				if version == 179 { r.read_i32()?.to_string() } else { r.read_string()? },
				r.read_u64()?
			)
		} else {
			("".to_owned(), 0)
		};

		let uuid = if version >= 181 { Some(r.read_bytes(16)?.try_into().unwrap()) } else { None };

		let id = r.read_i32()?;
		let left = r.read_i32()?;
		let right = r.read_i32()?;
		let top = r.read_i32()?;
		let bottom = r.read_i32()?;
		let height = r.read_i32()?;
		let width = r.read_i32()?;

		let game_mode = if version >= 209 {
			r.read_i32()?
		} else if version >= 112 {
			let mut gm = r.read_bool()? as i32;
			if version == 208 && r.read_bool()? {
				gm = 2;
			}
			gm
		} else {
			0
		};
		let game_mode: GameMode = (game_mode as u8).into();

		let world_drunk = version >= 227 && r.read_bool()?;
		let world_for_the_worthy = version >= 238 && r.read_bool()?;
		let world_anniversary = version >= 239 && r.read_bool()?;
		let world_dont_starve = version >= 241 && r.read_bool()?;
		let world_not_the_bees = version >= 249 && r.read_bool()?;
		let world_remix = version >= 266 && r.read_bool()?;
		let world_no_traps = version >= 266 && r.read_bool()?;
		let world_zenith = if version >= 267 { r.read_bool()? } else { world_drunk && world_remix };

		// TODO: parse ticks as time (https://learn.microsoft.com/en-us/dotnet/api/system.datetime.frombinary?view=net-8.0)
		let creation_time = if version >= 141 { r.read_i64()? } else { 0 };

		let moon_type = r.read_byte()? as i32;
		let tree_x = [r.read_i32()?, r.read_i32()?, r.read_i32()?];
		let tree_style = [r.read_i32()?, r.read_i32()?, r.read_i32()?, r.read_i32()?];
		let cave_back_x = [r.read_i32()?, r.read_i32()?, r.read_i32()?];
		let cave_back_style = [r.read_i32()?, r.read_i32()?, r.read_i32()?, r.read_i32()?];
		let ice_back_style = r.read_i32()?;
		let jungle_back_style = r.read_i32()?;
		let hell_back_style = r.read_i32()?;
		let spawn_tile_x = r.read_i32()?;
		let spawn_tile_y = r.read_i32()?;
		let world_surface = r.read_f64()?;
		let rock_layer = r.read_f64()?;
		let temp_time = r.read_f64()?;
		let temp_day_time = r.read_bool()?;
		let temp_moon_phase = r.read_i32()?;
		let temp_blood_moon = r.read_bool()?;
		let temp_eclipse = r.read_bool()?;
		let dungeon_x = r.read_i32()?;
		let dungeon_y = r.read_i32()?;
		let has_crimson = r.read_bool()?;
		let downed_boss_1 = r.read_bool()?;
		let downed_boss_2 = r.read_bool()?;
		let downed_boss_3 = r.read_bool()?;
		let downed_queen_bee = r.read_bool()?;
		let downed_mech_boss_1 = r.read_bool()?;
		let downed_mech_boss_2 = r.read_bool()?;
		let downed_mech_boss_3 = r.read_bool()?;
		let downed_mech_boss_any = r.read_bool()?;
		let downed_plant_boss = r.read_bool()?;
		let downed_golem_boss = r.read_bool()?;
		let downed_slime_king = version >= 118 && r.read_bool()?;
		let saved_goblin = r.read_bool()?;
		let saved_wizard = r.read_bool()?;
		let saved_mechanic = r.read_bool()?;
		let downed_goblins = r.read_bool()?;
		let downed_clown = r.read_bool()?;
		let downed_frost = r.read_bool()?;
		let downed_pirates = r.read_bool()?;
		let smashed_shadow_orb = r.read_bool()?;
		let spawn_meteor = r.read_bool()?;
		let shadow_orb_count = r.read_byte()? as i32;
		let altar_count = r.read_i32()?;
		let hard_mode = r.read_bool()?;
		let after_party_of_doom = version >= 257 && r.read_bool()?;
		let invasion_delay = r.read_i32()?;
		let invasion_size = r.read_i32()?;
		let invasion_type = r.read_i32()?;
		let invasion_x= r.read_f64()?;
		let slime_rain_time = if version >= 118 { r.read_f64()? } else { 0. };
		let sundial_cooldown = if version >= 113 { r.read_byte()? as i32 } else { 0 };
		let temp_raining = r.read_bool()?;
		let temp_rain_time = r.read_i32()?;
		let temp_max_rain = r.read_f32()?;
		let ore_tier_cobalt = r.read_i32()?;
		let ore_tier_mythril = r.read_i32()?;
		let ore_tier_adamantite = r.read_i32()?;
		let mut bg = [0u8; BG_COUNT];
		bg[..8].copy_from_slice(r.read_bytes(8)?);

		let cloud_bg_active = r.read_i32()? as f32;
		let cloud_bg_alpha = if cloud_bg_active < 1.0 { 0. } else { 1. };
		// Main.cloudBGActive = (float) -WorldGen.genRand.Next(8640, 86400);
		let num_clouds = r.read_i16()?;
		let wind_speed_target = r.read_f32()?;

		let angler_who_finished_today = if version >= 95 {
			let mut v = Vec::with_capacity(r.read_i32()? as usize);
			for _ in 0..v.capacity() {
				v.push(r.read_string()?)
			}
			v
		} else { vec![] };

		let saved_angler = version >= 99 && r.read_bool()?;
		let angler_quest = if version >= 101 { r.read_i32()? } else { 0 };
		let saved_stylist = version >= 104 && r.read_bool()?;
		let saved_tax_collector = version >= 129 && r.read_bool()?;
		let saved_golfer = version >= 201 && r.read_bool()?;
		let invasion_size_start = if version >= 107 { r.read_i32()? } else { 0 }; // TODO: mimc Main.FakeLoadInvasionStart
		let temp_cultist_delay = if version >= 108 { r.read_i32()? } else { 86400 };
		let npc_kill_counts = if version >= 109 {
			let mut kc = Vec::with_capacity(r.read_i16()? as usize);
			for _ in 0..kc.capacity() {
				kc.push(r.read_i32()?)
			}
			kc
		} else { vec![] };
		let fast_forward_time_to_dawn = version >= 128 && r.read_bool()?;

		let downed_fishron = version >= 131 && r.read_bool()?;
		let downed_martians = version >= 131 && r.read_bool()?;
		let downed_ancient_cultist = version >= 131 && r.read_bool()?;
		let downed_moonlord = version >= 131 && r.read_bool()?;
		let downed_halloween_king = version >= 131 && r.read_bool()?;
		let downed_halloween_tree = version >= 131 && r.read_bool()?;
		let downed_christmas_ice_queen = version >= 131 && r.read_bool()?;
		let downed_christmas_santank = version >= 131 && r.read_bool()?;
		let downed_christmas_tree = version >= 131 && r.read_bool()?;
		let downed_tower_solar = version >= 140 && r.read_bool()?;
		let downed_tower_vortex = version >= 140 && r.read_bool()?;
		let downed_tower_nebula = version >= 140 && r.read_bool()?;
		let downed_tower_stardust = version >= 140 && r.read_bool()?;
		let tower_active_solar = version >= 140 && r.read_bool()?;
		let tower_active_vortex = version >= 140 && r.read_bool()?;
		let tower_active_nebula = version >= 140 && r.read_bool()?;
		let tower_active_stardust = version >= 140 && r.read_bool()?;
		let lunar_apocalypse_is_up = version >= 140 && r.read_bool()?;

		let temp_party_manual = version >= 170 && r.read_bool()?;
		let temp_party_genuine = version >= 170 && r.read_bool()?;
		let temp_party_cooldown = if version >= 170 { r.read_i32()? } else { 0 };
		let temp_party_celebrating_npcs = if version >= 170 {
			let mut npcs = Vec::with_capacity(r.read_i32()? as usize);
			for _ in 0..npcs.capacity() {
				npcs.push(r.read_i32()?)
			}
			npcs
		} else { vec![] };

		let temp_sandstorm_happening = version >= 174 && r.read_bool()?;
		let temp_sandstorm_time_left = if version >= 174 { r.read_i32()? } else { 0 };
		let temp_sandstorm_severity = if version >= 174 { r.read_f32()? } else { 0. };
		let temp_sandstorm_intended_severity = if version >= 174 { r.read_f32()? } else { 0. };

		let saved_bartender = version >= 178 && r.read_bool()?;
		let downed_dd2_invasion_t1 = version >= 178 && r.read_bool()?;
		let downed_dd2_invasion_t2 = version >= 178 && r.read_bool()?;
		let downed_dd2_invasion_t3 = version >= 178 && r.read_bool()?;

		if version >= 193 { bg[8] = r.read_byte()? };
		if version >= 215 { bg[9] = r.read_byte()? };
		if version >= 194 { bg[10..].copy_from_slice(r.read_bytes(3)?) };

		let combat_book_was_used = version >= 204 && r.read_bool()?;

		let temp_lantern_night_cooldown = if version >= 207 { r.read_i32()? } else { 0 };
		let temp_lantern_night_genuine = version >= 207 && r.read_bool()?;
		let temp_lantern_night_manual = version >= 207 && r.read_bool()?;
		let temp_lantern_night_next_night_is_genuine = version >= 207 && r.read_bool()?;

		let tree_top_variations = if version >= 211 {
			let mut npcs = Vec::with_capacity(r.read_i32()? as usize);
			for _ in 0..npcs.capacity() {
				npcs.push(r.read_i32()?)
			}
			npcs
		} else {
			tree_style
				.into_iter()
				.chain(bg[1..=9].iter().map(|&e| e as i32))
				.collect::<Vec<i32>>()
		};

		let force_halloween_for_today = version >= 212 && r.read_bool()?;
		let force_xmas_for_today = version >= 212 && r.read_bool()?;

		let ore_tier_copper = if version >= 216 { r.read_i32()? } else { -1 };
		let ore_tier_iron = if version >= 216 { r.read_i32()? } else { -1 };
		let ore_tier_silver = if version >= 216 { r.read_i32()? } else { -1 };
		let ore_tier_gold = if version >= 216 { r.read_i32()? } else { -1 };

		let bought_cat = version >= 217 && r.read_bool()?;
		let bought_dog = version >= 217 && r.read_bool()?;
		let bought_bunny = version >= 217 && r.read_bool()?;

		let downed_empress_of_light = version >= 223 && r.read_bool()?;
		let downed_queen_slime = version >= 223 && r.read_bool()?;

		let downed_deerclops = version >= 240 && r.read_bool()?;

		let unlocked_slime_blue_spawn = version >= 251 && r.read_bool()?;

		let unlocked_merchant_spawn = version >= 251 && r.read_bool()?;
		let unlocked_demolition_spawn = version >= 251 && r.read_bool()?;
		let unlocked_party_girl_spawn = version >= 251 && r.read_bool()?;
		let unlocked_dye_trader_spawn = version >= 251 && r.read_bool()?;
		let unlocked_truffle_spawn = version >= 251 && r.read_bool()?;
		let unlocked_arms_dealer_spawn = version >= 251 && r.read_bool()?;
		let unlocked_nurse_spawn = version >= 251 && r.read_bool()?;
		let unlocked_princess_spawn = version >= 251 && r.read_bool()?;

		let combat_book_volume_two_was_used = version >= 259 && r.read_bool()?;

		let peddlers_satchel_was_use = version >= 260 && r.read_bool()?;

		let unlocked_slime_green_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_old_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_purple_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_rainbow_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_red_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_yellow_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_copper_spawn = version >= 261 && r.read_bool()?;

		let fast_forward_time_to_dusk = version >= 264 && r.read_bool()?;
		let moondial_cooldown = if version >= 264 { r.read_byte()? } else { 0 };

		Ok(WorldHeader {
			name,
			seed_text,
			worldgen_version,
			uuid,
			id,
			left,
			right,
			top,
			bottom,
			width,
			height,
			game_mode,
			world_drunk,
			world_for_the_worthy,
			world_anniversary,
			world_dont_starve,
			world_not_the_bees,
			world_remix,
			world_no_traps,
			world_zenith,
			creation_time,
			has_crimson,
			hard_mode,
			moon_type,
			tree_x,
			tree_style,
			cave_back_x,
			cave_back_style,
			ice_back_style,
			jungle_back_style,
			hell_back_style,
			spawn_tile_x,
			spawn_tile_y,
			world_surface,
			rock_layer,
			temp_time,
			temp_day_time,
			temp_moon_phase,
			temp_blood_moon,
			temp_eclipse,
			dungeon_x,
			dungeon_y,
			downed_boss_1,
			downed_boss_2,
			downed_boss_3,
			downed_queen_bee,
			downed_mech_boss_1,
			downed_mech_boss_2,
			downed_mech_boss_3,
			downed_mech_boss_any,
			downed_plant_boss,
			downed_golem_boss,
			downed_slime_king,
			saved_goblin,
			saved_wizard,
			saved_mechanic,
			downed_goblins,
			downed_clown,
			downed_frost,
			downed_pirates,
			smashed_shadow_orb,
			spawn_meteor,
			shadow_orb_count,
			altar_count,
			after_party_of_doom,
			invasion_delay,
			invasion_size,
			invasion_type,
			invasion_x,
			slime_rain_time,
			sundial_cooldown,
			temp_raining,
			temp_rain_time,
			temp_max_rain,
			ore_tier_cobalt,
			ore_tier_mythril,
			ore_tier_adamantite,
			bg,
			cloud_bg_active,
			cloud_bg_alpha,
			num_clouds,
			wind_speed_target,
			angler_who_finished_today,
			saved_angler,
			angler_quest,
			saved_stylist,
			saved_tax_collector,
			saved_golfer,
			invasion_size_start,
			temp_cultist_delay,
			npc_kill_counts,
			fast_forward_time_to_dawn,
			downed_fishron,
			downed_martians,
			downed_ancient_cultist,
			downed_moonlord,
			downed_halloween_king,
			downed_halloween_tree,
			downed_christmas_ice_queen,
			downed_christmas_santank,
			downed_christmas_tree,
			downed_tower_solar,
			downed_tower_vortex,
			downed_tower_nebula,
			downed_tower_stardust,
			tower_active_solar,
			tower_active_vortex,
			tower_active_nebula,
			tower_active_stardust,
			lunar_apocalypse_is_up,
			temp_party_manual,
			temp_party_genuine,
			temp_party_cooldown,
			temp_party_celebrating_npcs,
			temp_sandstorm_happening,
			temp_sandstorm_time_left,
			temp_sandstorm_severity,
			temp_sandstorm_intended_severity,
			saved_bartender,
			downed_dd2_invasion_t1,
			downed_dd2_invasion_t2,
			downed_dd2_invasion_t3,
			combat_book_was_used,
			temp_lantern_night_cooldown,
			temp_lantern_night_genuine,
			temp_lantern_night_manual,
			temp_lantern_night_next_night_is_genuine,
			tree_top_variations,
			force_halloween_for_today,
			force_xmas_for_today,
			ore_tier_copper,
			ore_tier_iron,
			ore_tier_silver,
			ore_tier_gold,
			bought_cat,
			bought_dog,
			bought_bunny,
			downed_empress_of_light,
			downed_queen_slime,
			downed_deerclops,
			unlocked_slime_blue_spawn,
			unlocked_merchant_spawn,
			unlocked_demolition_spawn,
			unlocked_party_girl_spawn,
			unlocked_dye_trader_spawn,
			unlocked_truffle_spawn,
			unlocked_arms_dealer_spawn,
			unlocked_nurse_spawn,
			unlocked_princess_spawn,
			combat_book_volume_two_was_used,
			peddlers_satchel_was_use,
			unlocked_slime_green_spawn,
			unlocked_slime_old_spawn,
			unlocked_slime_purple_spawn,
			unlocked_slime_rainbow_spawn,
			unlocked_slime_red_spawn,
			unlocked_slime_yellow_spawn,
			unlocked_slime_copper_spawn,
			fast_forward_time_to_dusk,
			moondial_cooldown,
		})
	}

	pub fn read_tiles(r: &mut SafeReader, format: &WorldFormat, header: &WorldHeader) -> Result<Vec<Vec<WorldTile>>, WorldParseError> {
		let mut map = Vec::with_capacity(header.width as usize);
		for x in 0..header.width {
			let mut column = Vec::with_capacity(header.height as usize);
			for y in 0..header.height {
				let flags = r.read_byte()?;
				if flags & 1 != 0 {
					let num5 = r.read_byte()?;
					if num5 & 1 != 0 {
						let num4 = r.read_byte()?;
						if num4 & 1 != 0 {
							let num3 = r.read_byte()?;
						}
					}
				}
			}

			map.push(column);
		}

		Ok(map)
	}
}

