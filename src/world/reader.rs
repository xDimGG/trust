use crate::world::binary::FileReader;
use crate::world::entity::*;
use crate::world::tile::*;
use crate::world::types::*;
use std::time::{Duration, UNIX_EPOCH};
use std::{collections::HashSet, fs, path::Path};

use super::transpiled::tile_flags::SIGN;

pub const EPOCH_DIFFERENCE: u64 = 719162 * 24 * 60 * 60 * 1000;

impl World {
	pub fn from_file(path: &Path) -> Result<World, WorldDecodeError> {
		let contents = fs::read(path)?;
		let mut reader = FileReader::new(contents);
		let mut world = Self::from_reader(&mut reader)?;

		if world.metadata.version < 141 {
			let file_metadata = fs::metadata(path)?;
			world.header.creation_time = file_metadata.created()?
		}

		Ok(world)
	}

	// TODO: implement Crc32 to get seed_text value

	pub fn from_reader(r: &mut FileReader) -> Result<World, WorldDecodeError> {
		let version = r.read_i32()?;
		if version >= 88 {
			Self::read_world_v2(r)
		} else {
			todo!("implement v1 before release 88");
		}
	}

	pub fn read_world_v2(r: &mut FileReader) -> Result<World, WorldDecodeError> {
		r.seek(0);
		let metadata = Self::read_metadata(r)?;
		let format = Self::read_format(r)?;
		if r.get_cur() != format.positions[0] as usize {
			return Err(WorldDecodeError::PositionCheckFailed("format".to_owned()));
		}

		let header = Self::read_header(r, &metadata)?;
		if r.get_cur() != format.positions[1] as usize {
			return Err(WorldDecodeError::PositionCheckFailed("header".to_owned()));
		}

		let tiles = Self::read_tiles(r, &header)?;
		if r.get_cur() != format.positions[2] as usize {
			return Err(WorldDecodeError::PositionCheckFailed("tiles".to_owned()));
		}

		let chests = Self::read_chests(r)?;
		if r.get_cur() != format.positions[3] as usize {
			return Err(WorldDecodeError::PositionCheckFailed("chests".to_owned()));
		}

		let signs = Self::read_signs(r, &tiles)?;
		if r.get_cur() != format.positions[4] as usize {
			return Err(WorldDecodeError::PositionCheckFailed("signs".to_owned()));
		}

		let npcs = Self::read_npcs(r, &metadata)?;
		if r.get_cur() != format.positions[5] as usize {
			return Err(WorldDecodeError::PositionCheckFailed("npcs".to_owned()));
		}

		let version = metadata.version;

		let entities = if version >= 116 {
			let te = if version >= 122 {
				Self::read_entities(r)?
			} else {
				todo!("implement WorldFile.LoadDummies for older versions")
			};

			if r.get_cur() != format.positions[6] as usize {
				return Err(WorldDecodeError::PositionCheckFailed("entities".to_owned()));
			}

			te
		} else {
			vec![]
		};

		let weighted_pressure_plates = if version >= 170 {
			let wpp = Self::read_weighted_pressure_plates(r)?;
			if r.get_cur() != format.positions[7] as usize {
				return Err(WorldDecodeError::PositionCheckFailed(
					"weighted_pressure_plates".to_owned(),
				));
			}

			wpp
		} else {
			vec![]
		};

		let room_locations = if version >= 189 {
			let rl = Self::read_room_locations(r)?;
			if r.get_cur() != format.positions[8] as usize {
				return Err(WorldDecodeError::PositionCheckFailed(
					"room_locations".to_owned(),
				));
			}

			rl
		} else {
			vec![]
		};

		let bestiary = if version >= 210 {
			let be = Self::read_bestiary(r)?;
			if r.get_cur() != format.positions[9] as usize {
				return Err(WorldDecodeError::PositionCheckFailed("bestiary".to_owned()));
			}

			be
		} else {
			todo!("WorldFile.LoadBestiaryForVersionsBefore210()")
		};

		let creative_powers = if version >= 220 {
			let be = Self::read_creative_powers(r)?;
			if r.get_cur() != format.positions[10] as usize {
				return Err(WorldDecodeError::PositionCheckFailed(
					"creative_powers".to_owned(),
				));
			}

			be
		} else {
			vec![]
		};

		if Self::validate_footer(r, &header)? {
			Ok(World {
				metadata,
				format,
				header,
				tiles,
				chests,
				signs,
				npcs,
				entities,
				weighted_pressure_plates,
				room_locations,
				bestiary,
				creative_powers,
			})
		} else {
			Err(WorldDecodeError::InvalidFooter)
		}
	}

	pub fn read_metadata(r: &mut FileReader) -> Result<Metadata, WorldDecodeError> {
		let version = r.read_i32()?;
		let (file_type, revision, favorite) = if version >= 135 {
			let magic = r.read_bytes(7)?;
			if magic != MAGIC_STRING {
				return Err(WorldDecodeError::BadFileSignature);
			}

			(
				FileType::from(r.read_byte()?),
				r.read_u32()?,
				(r.read_u64()? & 1) == 1,
			)
		} else {
			(FileType::World, 0, false)
		};

		if file_type != FileType::World {
			return Err(WorldDecodeError::ExpectedWorldType);
		}

		if version > 279 {
			return Err(WorldDecodeError::UnsupportedVersion(version));
		}

		Ok(Metadata {
			version,
			file_type,
			revision,
			favorite,
		})
	}

	pub fn read_format(r: &mut FileReader) -> Result<Format, WorldDecodeError> {
		let mut positions = vec![0; r.read_i16()? as usize];
		for p in &mut positions {
			*p = r.read_i32()?;
		}

		let mut importance = vec![false; r.read_u16()? as usize];
		let mut byte = 0;
		let mut mask = 128;
		for i in &mut importance {
			if mask == 128 {
				byte = r.read_byte()?;
				mask = 1;
			} else {
				mask <<= 1;
			}

			if (byte & mask) == mask {
				*i = true;
			}
		}

		Ok(Format {
			positions,
			importance,
		})
	}

	pub fn read_header(
		r: &mut FileReader,
		metadata: &Metadata,
	) -> Result<Header, WorldDecodeError> {
		let version = metadata.version;
		let name = r.read_string()?;
		let (seed_text, worldgen_version) = if version >= 179 {
			(
				if version == 179 {
					r.read_i32()?.to_string()
				} else {
					r.read_string()?
				},
				r.read_u64()?,
			)
		} else {
			("".to_owned(), 0)
		};

		let uuid = if version >= 181 {
			Some(r.read_bytes(16)?.try_into().unwrap())
		} else {
			None
		};

		let id = r.read_i32()?;
		let left = r.read_i32()?;
		let right = r.read_i32()?;
		let top = r.read_i32()?;
		let bottom = r.read_i32()?;
		let height = r.read_i32()?;
		let width = r.read_i32()?;

		let game_mode: GameMode = (if version >= 209 {
			r.read_i32()? as u8
		} else {
			let gm = if version >= 112 {
				r.read_bool()? as u8
			} else {
				0
			};
			if version == 208 && r.read_bool()? {
				2
			} else {
				gm
			}
		})
		.into();

		let world_drunk = version >= 222 && r.read_bool()?;
		let world_for_the_worthy = version >= 227 && r.read_bool()?;
		let world_anniversary = version >= 238 && r.read_bool()?;
		let world_dont_starve = version >= 239 && r.read_bool()?;
		let world_not_the_bees = version >= 241 && r.read_bool()?;
		let world_remix = version >= 249 && r.read_bool()?;
		let world_no_traps = version >= 266 && r.read_bool()?;
		let world_zenith = if version >= 267 {
			r.read_bool()?
		} else {
			world_drunk && world_remix
		};

		// https://learn.microsoft.com/en-us/dotnet/api/system.datetime.now?view=net-8.0#remarks
		let creation_time = if version >= 141 {
			let b = r.read_u64()? << 2 >> 2;
			UNIX_EPOCH + Duration::from_millis(b / 10_000 - EPOCH_DIFFERENCE)
		} else {
			UNIX_EPOCH
		};

		let moon_type = r.read_byte()? as i32;
		let tree_x = [r.read_i32()?, r.read_i32()?, r.read_i32()?];
		let tree_style = [r.read_i32()?, r.read_i32()?, r.read_i32()?, r.read_i32()?];
		let cave_back_x = [r.read_i32()?, r.read_i32()?, r.read_i32()?];
		let cave_back_style = [r.read_i32()?, r.read_i32()?, r.read_i32()?, r.read_i32()?];
		let ice_back_style = r.read_i32()?;
		let jungle_back_style = r.read_i32()?;
		let hell_back_style = r.read_i32()?;
		let spawn_x = r.read_i32()?;
		let spawn_y = r.read_i32()?;
		let world_surface = r.read_f64()?;
		let rock_layer = r.read_f64()?;
		let time = r.read_f64()?;
		let day_time = r.read_bool()?;
		let moon_phase = r.read_i32()?;
		let blood_moon = r.read_bool()?;
		let eclipse = r.read_bool()?;
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
		let invasion_x = r.read_f64()?;
		let slime_rain_time = if version >= 118 { r.read_f64()? } else { 0. };
		let sundial_cooldown = if version >= 113 {
			r.read_byte()? as i32
		} else {
			0
		};
		let raining = r.read_bool()?;
		let rain_time = r.read_i32()?;
		let max_rain = r.read_f32()?;
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
		} else {
			vec![]
		};

		let saved_angler = version >= 99 && r.read_bool()?;
		let angler_quest = if version >= 101 { r.read_i32()? } else { 0 };
		let saved_stylist = version >= 104 && r.read_bool()?;
		let saved_tax_collector = version >= 129 && r.read_bool()?;
		let saved_golfer = version >= 201 && r.read_bool()?;
		let invasion_size_start = if version >= 107 { r.read_i32()? } else { 0 }; // TODO: mimc Main.FakeLoadInvasionStart
		let cultist_delay = if version >= 108 { r.read_i32()? } else { 86400 };
		let npc_kill_counts = if version >= 109 {
			let mut kc = Vec::with_capacity(r.read_i16()? as usize);
			for _ in 0..kc.capacity() {
				kc.push(r.read_i32()?)
			}
			kc
		} else {
			vec![]
		};
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

		let party_manual = version >= 170 && r.read_bool()?;
		let party_genuine = version >= 170 && r.read_bool()?;
		let party_cooldown = if version >= 170 { r.read_i32()? } else { 0 };
		let party_celebrating_npcs = if version >= 170 {
			let mut npcs = Vec::with_capacity(r.read_i32()? as usize);
			for _ in 0..npcs.capacity() {
				npcs.push(r.read_i32()?)
			}
			npcs
		} else {
			vec![]
		};

		let sandstorm_happening = version >= 174 && r.read_bool()?;
		let sandstorm_time_left = if version >= 174 { r.read_i32()? } else { 0 };
		let sandstorm_severity = if version >= 174 { r.read_f32()? } else { 0. };
		let sandstorm_intended_severity = if version >= 174 { r.read_f32()? } else { 0. };

		let saved_bartender = version >= 178 && r.read_bool()?;
		let downed_dd2_invasion_t1 = version >= 178 && r.read_bool()?;
		let downed_dd2_invasion_t2 = version >= 178 && r.read_bool()?;
		let downed_dd2_invasion_t3 = version >= 178 && r.read_bool()?;

		if version >= 193 {
			bg[8] = r.read_byte()?
		};
		if version >= 215 {
			bg[9] = r.read_byte()?
		};
		if version >= 194 {
			bg[10..].copy_from_slice(r.read_bytes(3)?)
		};

		let combat_book_was_used = version >= 204 && r.read_bool()?;

		let lantern_night_cooldown = if version >= 207 { r.read_i32()? } else { 0 };
		let lantern_night_genuine = version >= 207 && r.read_bool()?;
		let lantern_night_manual = version >= 207 && r.read_bool()?;
		let lantern_night_next_night_is_genuine = version >= 207 && r.read_bool()?;

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

		let peddlers_satchel_was_used = version >= 260 && r.read_bool()?;

		let unlocked_slime_green_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_old_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_purple_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_rainbow_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_red_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_yellow_spawn = version >= 261 && r.read_bool()?;
		let unlocked_slime_copper_spawn = version >= 261 && r.read_bool()?;

		let fast_forward_time_to_dusk = version >= 264 && r.read_bool()?;
		let moondial_cooldown = if version >= 264 { r.read_byte()? } else { 0 };

		Ok(Header {
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
			spawn_x,
			spawn_y,
			world_surface,
			rock_layer,
			time,
			day_time,
			moon_phase,
			blood_moon,
			eclipse,
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
			raining,
			rain_time,
			max_rain,
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
			cultist_delay,
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
			party_manual,
			party_genuine,
			party_cooldown,
			party_celebrating_npcs,
			sandstorm_happening,
			sandstorm_time_left,
			sandstorm_severity,
			sandstorm_intended_severity,
			saved_bartender,
			downed_dd2_invasion_t1,
			downed_dd2_invasion_t2,
			downed_dd2_invasion_t3,
			combat_book_was_used,
			lantern_night_cooldown,
			lantern_night_genuine,
			lantern_night_manual,
			lantern_night_next_night_is_genuine,
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
			peddlers_satchel_was_used,
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

	pub fn read_tiles(
		r: &mut FileReader,
		header: &Header,
	) -> Result<Vec<Vec<Tile>>, WorldDecodeError> {
		let mut map = Vec::with_capacity(header.width as usize);
		for _ in 0..header.width {
			let mut column = Vec::with_capacity(header.height as usize);
			let mut y = 0;
			while y < header.height as usize {
				// Header bytes
				let (tile, repeat) = Tile::decode(r)?;
				for _ in 0..repeat {
					column.push(tile.clone());
				}
				column.push(tile);

				y += repeat + 1;
			}

			map.push(column);
		}

		Ok(map)
	}

	pub fn read_chests(r: &mut FileReader) -> Result<Vec<Chest>, WorldDecodeError> {
		let mut chests = Vec::with_capacity(r.read_i16()? as usize);

		let n_2 = r.read_i16()?;
		let n_3 = if n_2 < 40 { n_2 } else { 40 };
		let n_4 = if n_2 < 40 { 0 } else { n_2 - 40 };

		for _ in 0..chests.capacity() {
			let x = r.read_i32()?;
			let y = r.read_i32()?;
			let name = r.read_string()?;
			let mut items = Vec::with_capacity(n_3 as usize);
			for _ in 0..items.capacity() {
				let stack = r.read_i16()?;
				let item = if stack == 0 {
					ChestItem::default()
				} else {
					ChestItem {
						id: r.read_i32()?,
						stack: if stack > 0 { stack } else { 1 },
						prefix: r.read_byte()?,
					}
				};
				items.push(item)
			}

			for _ in 0..n_4 {
				if r.read_i16()? > 0 {
					r.skip(5)
				}
			}

			chests.push(Chest { x, y, name, items })
		}

		Ok(chests)
	}

	pub fn read_signs(
		r: &mut FileReader,
		tiles: &[Vec<Tile>],
	) -> Result<Vec<Sign>, WorldDecodeError> {
		let mut signs = Vec::with_capacity(r.read_i16()? as usize);

		for _ in 0..signs.capacity() {
			let text = r.read_string()?;
			let x = r.read_i32()?;
			let y = r.read_i32()?;

			let t = &tiles[x as usize][y as usize];
			if t.active && SIGN[t.id as usize] {
				signs.push(Sign { x, y, text })
			}
		}

		Ok(signs)
	}

	pub fn read_npcs(
		r: &mut FileReader,
		metadata: &Metadata,
	) -> Result<Vec<NPC>, WorldDecodeError> {
		let version = metadata.version;
		let mut shimmers = HashSet::new();
		if version >= 268 {
			for _ in 0..r.read_i32()? {
				shimmers.insert(r.read_i32()?);
			}
		}

		let mut npcs = vec![];

		while r.read_bool()? {
			let id = if version >= 190 {
				r.read_i32()?
			} else {
				todo!("implement NPCID.FromLegacyName(reader.ReadString())")
			};

			let name = r.read_string()?;
			let position = r.read_vector2()?;
			let homeless = r.read_bool()?;
			let home_x = r.read_i32()?;
			let home_y = r.read_i32()?;
			let variation = if version >= 213 && r.read_byte()? & 1 == 1 {
				r.read_i32()?
			} else {
				0
			};

			npcs.push(NPC {
				id,
				name,
				position,
				homeless,
				home_x,
				home_y,
				variation,
				shimmer: shimmers.contains(&id),
			})
		}

		if version >= 140 {
			let mut iter = npcs.iter_mut();
			while r.read_bool()? {
				let Some(npc) = iter.next() else { break };
				if version >= 190 {
					npc.id = r.read_i32()?;
				} else {
					todo!("implement NPCID.FromLegacyName(reader.ReadString())")
				}

				npc.position = r.read_vector2()?
			}
		}

		Ok(npcs)
	}

	pub fn read_entities(r: &mut FileReader) -> Result<Vec<Entity>, WorldDecodeError> {
		let mut entities = Vec::with_capacity(r.read_i32()? as usize);
		for _ in 0..entities.capacity() {
			entities.push(Entity::decode(r)?)
		}

		Ok(entities)
	}

	pub fn read_weighted_pressure_plates(
		r: &mut FileReader,
	) -> Result<Vec<WeightedPressurePlate>, WorldDecodeError> {
		let mut wpp = Vec::with_capacity(r.read_i32()? as usize);
		for _ in 0..wpp.capacity() {
			wpp.push(WeightedPressurePlate {
				x: r.read_i32()?,
				y: r.read_i32()?,
			})
		}

		Ok(wpp)
	}

	pub fn read_room_locations(r: &mut FileReader) -> Result<Vec<RoomLocation>, WorldDecodeError> {
		let mut rl = Vec::with_capacity(r.read_i32()? as usize);
		for _ in 0..rl.capacity() {
			rl.push(RoomLocation {
				id: r.read_i32()?,
				x: r.read_i32()?,
				y: r.read_i32()?,
			})
		}

		Ok(rl)
	}

	pub fn read_bestiary(r: &mut FileReader) -> Result<Bestiary, WorldDecodeError> {
		let mut kills = Vec::with_capacity(r.read_i32()? as usize);
		for _ in 0..kills.capacity() {
			kills.push((r.read_string()?, r.read_i32()?));
		}

		let mut sights = Vec::with_capacity(r.read_i32()? as usize);
		for _ in 0..sights.capacity() {
			sights.push(r.read_string()?);
		}

		let mut chats = Vec::with_capacity(r.read_i32()? as usize);
		for _ in 0..chats.capacity() {
			chats.push(r.read_string()?);
		}

		Ok(Bestiary {
			kills,
			sights,
			chats,
		})
	}

	pub fn read_creative_powers(
		r: &mut FileReader,
	) -> Result<Vec<CreativePower>, WorldDecodeError> {
		let mut powers = vec![];
		while r.read_bool()? {
			let power = CreativePower::decode_file(r)?;
			powers.push(power);
		}

		Ok(powers)
	}

	pub fn validate_footer(r: &mut FileReader, header: &Header) -> Result<bool, WorldDecodeError> {
		Ok(r.read_bool()? && r.read_string()? == header.name && r.read_i32()? == header.id)
	}
}
