use std::cmp::{max, min};
use std::io::Write;

use crate::binary::writer::Writer;
use crate::world::types::{Header, World};
use crate::world::tile::Tile;
use crate::network::messages::{Message, WorldHeader};
use flate2::write::ZlibEncoder;
use flate2::{Compress, Compression};

#[allow(clippy::too_many_arguments)]
// utility function for generating u8 bitmask from bools where first param is LSB
pub fn flags(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool, h: bool) -> u8 {
	a as u8 | (b as u8) << 1 | (c as u8) << 2 | (d as u8) << 3 | (e as u8) << 4 | (f as u8) << 5 | (g as u8) << 6 | (h as u8) << 7
}

const SECTION_WIDTH: usize = 200;
const SECTION_HEIGHT: usize = 150;

pub fn get_section_x(x: usize) -> usize {
	x / SECTION_WIDTH
}

pub fn get_section_y(y: usize) -> usize {
	y / SECTION_HEIGHT
}

pub fn get_sections_near(x: i32, y: i32, max_sec_x: usize, max_sec_y: usize) -> (usize, usize, usize, usize) {
	// -2, -1, +5, +3 are the values that the game uses. dont ask me
	let sec_x_start  = max(get_section_x(x as usize) - 2, 0);
	let sec_x_end = min(sec_x_start + 5, max_sec_x);
	let sec_y_start  = max(get_section_y(y as usize) - 1, 0);
	let sec_y_end = min(sec_y_start + 3, max_sec_y);

	(sec_x_start, sec_x_end, sec_y_start, sec_y_end)
}

pub fn encode_tiles(world: &World, sec_x: usize, sec_y: usize) -> Message {
	let x_start = sec_x * SECTION_WIDTH;
	let y_start = sec_y * SECTION_HEIGHT;
	let x_end = x_start + SECTION_WIDTH;
	let y_end = y_start + SECTION_HEIGHT;

	// todo: optimize this to reduce data copying

	let mut w = Writer::new();
	w.write_i32(x_start as i32);
	w.write_i32(y_start as i32);
	w.write_i16((x_end - x_start) as i16);
	w.write_i16((y_end - y_start) as i16);

	let mut last_tile = &Tile::default();
	let mut repeat_count = 0;

	let mut chest_tiles = vec![];
	let mut sign_tiles = vec![];
	let mut entity_tiles = vec![];

	for y in y_start..y_end {
		for x in x_start..x_end {
			let tile = &world.tiles[x][y];

			if !(x == x_start && y == y_start) {
				// todo: ensure isTheSameAs is like PartialEq
				// todo: automate this to use TileID.Sets.AllowsSaveCompressionBatching
				if tile == last_tile && tile.id != 423 && tile.id != 520 {
					repeat_count += 1;
					continue;
				}

				w.write_bytes(last_tile.encode(repeat_count, &world.format));
				repeat_count = 0;
			}

			last_tile = tile;

			if tile.id >= 0 && world.format.importance[tile.id as usize] {
				let fx = tile.frame_x;
				let fy = tile.frame_y;
				let is_chest = match tile.id {
					21 | 467 => fx % 36 == 0 && fy % 36 == 0,
					88 => fx % 54 == 0 && fy % 36 == 0,
					_ => false,
				};
				if is_chest {
					chest_tiles.push((x, y));
				} else {
					let is_sign = match tile.id {
						55 | 85 | 425 | 573 => fx % 36 == 0 && fy % 36 == 0,
						_ => false,
					};
					if is_sign {
						sign_tiles.push((x, y));
					} else {
						let is_entity = match tile.id {
							378 | 395 | 470 => fx % 36 == 0 && fy == 0,
							520 => fx % 18 == 0 && fy == 0,
							471 | 475 => fx % 54 == 0 && fy == 0,
							597 => fx % 54 == 0 && fy % 72 == 0,
							_ => false,
						};
						if is_entity {
							entity_tiles.push((x, y));
						}
					}
				}
			}
		}
	}

	w.write_bytes(last_tile.encode(repeat_count, &world.format));

	w.write_i16(chest_tiles.len() as i16);
	for (x, y) in chest_tiles {
		let (i, chest) = world.chests.iter().enumerate().find(|(_, c)| c.x as usize == x && c.y as usize == y).unwrap();
		w.write_i16(i as i16);
		w.write_i16(x as i16);
		w.write_i16(y as i16);
		w.write_string(chest.name.clone());
	}

	w.write_i16(sign_tiles.len() as i16);
	for (x, y) in sign_tiles {
		let (i, sign) = world.signs.iter().enumerate().find(|(_, s)| s.x as usize == x && s.y as usize == y).unwrap();
		w.write_i16(i as i16);
		w.write_i16(x as i16);
		w.write_i16(y as i16);
		w.write_string(sign.text.clone());
	}

	w.write_i16(entity_tiles.len() as i16);
	for (x, y) in entity_tiles {
		let entity = world.entities.iter().find(|c| c.x == x as i16 && c.y == y as i16).unwrap();
		entity.write(&mut w)
	}

	let mut out = ZlibEncoder::new_with_compress(vec![], Compress::new(Compression::default(), false));
	out.write_all(&w.finalize()).unwrap();

	Message::Custom(10, out.finish().unwrap())
}

pub fn encode_world_header(h: &Header) -> Message {
	Message::WorldHeader(WorldHeader {
		time: 0,
		time_flags: flags( h.temp_day_time, h.temp_blood_moon, h.temp_eclipse, false, false, false, false, false),
		moon_phase: h.temp_moon_phase as u8,
		width: h.width as i16,
		height: h.height as i16,
		spawn_x: h.spawn_x as i16,
		spawn_y: h.spawn_y as i16,
		world_surface: h.world_surface as i16,
		rock_layer: h.rock_layer as i16,
		id: h.id,
		name: h.name.clone(),
		game_mode: h.game_mode.clone() as u8,
		uuid: h.uuid.unwrap(),
		worldgen_version: h.worldgen_version,
		moon_type: h.moon_type as u8,
		bg_0: h.bg[0],
		bg_10: h.bg[10],
		bg_11: h.bg[11],
		bg_12: h.bg[12],
		bg_1: h.bg[1],
		bg_2: h.bg[2],
		bg_3: h.bg[3],
		bg_4: h.bg[4],
		bg_5: h.bg[5],
		bg_6: h.bg[6],
		bg_7: h.bg[7],
		bg_8: h.bg[8],
		bg_9: h.bg[9],
		ice_back_style: h.ice_back_style as u8,
		jungle_back_style: h.jungle_back_style as u8,
		hell_back_style: h.hell_back_style as u8,
		wind_speed_target: h.wind_speed_target,
		num_clouds: h.num_clouds as u8,
		tree_x: h.tree_x,
		tree_style: h.tree_style.iter().map(|n| *n as u8).collect::<Vec<u8>>().try_into().unwrap_or([0; 4]),
		cave_back_x: h.cave_back_x,
		cave_back_style: h.cave_back_style.iter().map(|n| *n as u8).collect::<Vec<u8>>().try_into().unwrap_or([0; 4]),
		tree_top_variations: h.tree_top_variations.iter().map(|n| *n as u8).collect::<Vec<u8>>().try_into().unwrap_or([0; 13]),
		max_raining: h.temp_max_rain,
		flags: [
			// todo: support for server-side characters
			flags(h.smashed_shadow_orb, h.downed_boss_1, h.downed_boss_2, h.downed_boss_3, h.hard_mode, h.downed_clown, false, h.downed_plant_boss),
			// todo: pumpkinMoon and snowMoon
			flags(h.downed_mech_boss_1, h.downed_mech_boss_2, h.downed_mech_boss_3, h.downed_mech_boss_any, h.cloud_bg_active == 1., h.has_crimson, false, false),
			// todo: int num7 = bitsByte7[2] ? 1 : 0;
			flags(false, h.fast_forward_time_to_dawn, false, h.downed_slime_king, h.downed_queen_bee, h.downed_fishron, h.downed_martians, h.downed_ancient_cultist),
			// todo: BirthdayParty
			flags(h.downed_moonlord, h.downed_halloween_king, h.downed_halloween_tree, h.downed_christmas_ice_queen, h.downed_christmas_santank, h.downed_christmas_tree, h.downed_golem_boss, false),
			// todo: DD2Event.Ongoing
			flags(h.downed_pirates, h.downed_frost, h.downed_goblins, h.temp_sandstorm_happening, false, h.downed_dd2_invasion_t1, h.downed_dd2_invasion_t2, h.downed_dd2_invasion_t3),
			flags(h.combat_book_was_used, h.temp_lantern_night_manual, h.downed_tower_solar, h.downed_tower_vortex, h.downed_tower_nebula, h.downed_tower_stardust, h.force_halloween_for_today, h.force_xmas_for_today),
			// todo: freeCake, getGodWorld
			flags(h.bought_cat, h.bought_dog, h.bought_bunny, false, h.world_drunk, h.downed_empress_of_light, h.downed_queen_slime, false),
			flags(h.world_anniversary, h.world_dont_starve, h.downed_deerclops, h.world_not_the_bees, h.world_remix, h.unlocked_slime_blue_spawn, h.combat_book_volume_two_was_used, h.peddlers_satchel_was_used),
			flags(h.unlocked_slime_green_spawn, h.unlocked_slime_old_spawn, h.unlocked_slime_purple_spawn, h.unlocked_slime_rainbow_spawn, h.unlocked_slime_red_spawn, h.unlocked_slime_yellow_spawn, h.unlocked_slime_copper_spawn, h.fast_forward_time_to_dusk),
			flags(h.world_no_traps, h.world_zenith, h.unlocked_truffle_spawn, false, false, false, false, false),
		],
		sundial_cooldown: h.sundial_cooldown as u8,
		moondial_cooldown: h.moondial_cooldown,
		ore_tier_copper: h.ore_tier_copper as i16,
		ore_tier_iron: h.ore_tier_iron as i16,
		ore_tier_silver: h.ore_tier_silver as i16,
		ore_tier_gold: h.ore_tier_gold as i16,
		ore_tier_cobalt: h.ore_tier_cobalt as i16,
		ore_tier_mythril: h.ore_tier_mythril as i16,
		ore_tier_adamantite: h.ore_tier_adamantite as i16,
		invasion_type: h.invasion_type as i8,
		lobby_id: 0,
		sandstorm_intended_severity: h.temp_sandstorm_intended_severity,
	})
}
