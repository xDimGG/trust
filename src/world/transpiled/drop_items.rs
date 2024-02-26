// This file was mostly auto-generated by scripts/convert_drop_items.py
// Irregular cases were done by hand

use crate::world::tile::Tile;
use crate::network::client::Client;
use crate::world::transpiled::items;
use crate::world::transpiled::tiles;
use crate::world::types::World;
use crate::network::utils::rr;
use rand::random;

// From Minecart.cs
const TRACK_TYPE: &[i16] = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2];

impl Tile {
	pub fn get_dropped_item(&self, c: &Client, w: &World, stack: &mut i16, secondary_item: &mut i16, secondary_stack: &mut i16) -> i16 {
		*stack = 1;
		*secondary_item = 0;
		*secondary_stack = 1;
		match self.id {
			0 | 2 | 109 | 199 | 477 | 492 => 2,
			1 => 3,
			3 => {
				if self.frame_x == 144 {
					items::MUSHROOM
				} else if random() && c.has_seed_weapon() {
					items::SEED
				} else {
					0
				}
			}
			4 => {
				let num1 = self.frame_y / 22;
				match num1 {
					0 => 8,
					8 => 523,
					9 => 974,
					10 => 1245,
					11 => 1333,
					12 => 2274,
					13 => 3004,
					14 => 3045,
					15 => 3114,
					16 => 4383,
					17 => 4384,
					18 => 4385,
					19 => 4386,
					20 => 4387,
					21 => 4388,
					22 => 5293,
					23 => 5353,
					_ => 426 + num1,
				}
			}
			5 | 596 | 616 | 634 => unimplemented!("caller should implement this because there are multiple tile drops. source in WorldGen.cs:43272"),
			6 => 11,
			7 => 12,
			8 => 13,
			9 => 14,
			13 => match self.frame_y / 22 {
				1 => 28,
				2 => 110,
				3 => 350,
				4 => 351,
				5 => 2234,
				6 => 2244,
				7 => 2257,
				8 => 2258,
				_ => 31,
			}
			19 => {
				let num2 = self.frame_y / 18;
				match num2 {
					0 => 94,
					1 => 631,
					2 => 632,
					3 => 633,
					4 => 634,
					5 => 913,
					6 => 1384,
					7 => 1385,
					8 => 1386,
					9 => 1387,
					10 => 1388,
					11 => 1389,
					12 => 1418,
					13 => 1457,
					14 => 1702,
					15 => 1796,
					16 => 1818,
					17 => 2518,
					18 => 2549,
					19 => 2566,
					20 => 2581,
					21 => 2627,
					22 => 2628,
					23 => 2629,
					24 => 2630,
					25 => 2744,
					26 => 2822,
					27 => 3144,
					28 => 3146,
					29 => 3145,
					30..=35 => 3903 + num2 - 30,
					36 => 3945,
					37 => 3957,
					38 => 4159,
					39 => 4180,
					40 => 4201,
					41 => 4222,
					42 => 4311,
					43 => 4416,
					44 => 4580,
					45 => 5162,
					46 => 5183,
					47 => 5204,
					48 => 5292,
					_ => 0,
				}
			}
			22 => 56,
			23 => 2,
			24 => match self.frame_x {
				144 => 60,
				_ => 0,
			}
			25 => 61,
			30 => 9,
			33 => 105,
			36 => 1869,
			37 => 116,
			38 => 129,
			39 => 131,
			40 => 133,
			41 => 134,
			43 => 137,
			44 => 139,
			45 => 141,
			46 => 143,
			47 => 145,
			48 => 147,
			49 => 148,
			50 => match self.frame_x {
				90 => 165,
				_ => 149,
			}
			51 => 150,
			52 | 62 | 382 => if random() && c.has_equipped(items::CORDAGE_GUIDE) { items::VINE_ROPE } else { 0 }
			53 => 169,
			54 => 170,
			56 => 173,
			57 => 172,
			58 => 174,
			59 | 60 | 661 | 662 => 176,
			61 | 74 => {
				if self.id == tiles::JUNGLE_PLANTS {
					let item = match self.frame_x {
						144 => {
							*stack = rr(2, 3);
							items::JUNGLE_SPORES
						},
						162 => items::NATURES_GIFT,
						108..=125 => if random::<f64>() < 1. / 20. { items::JUNGLE_ROSE } else { 0 },
						_ => 0,
					};
					if item != 0 {
						return item
					}
				}

				if random::<f64>() < 1. / 100. {
					items::JUNGLE_GRASS_SEEDS
				} else {
					0
				}
			}
			63..=68 => self.id - 63 + 177,
			70 => 176,
			71 | 72 => {
				if random::<f64>() < 1. / 40. {
					items::MUSHROOM_GRASS_SEEDS
				} else if random() {
					items::GLOWING_MUSHROOM
				} else {
					0
				}
			}
			73 => {
				if random() && c.has_seed_weapon() {
					items::SEED
				} else {
					0
				}
			}
			75 => 192,
			76 => 214,
			78 => 222,
			80 => 276,
			81 => 275,
			83 | 84 => {
				let style = self.frame_x / 18;
				let (seed, plant) = if style == 6 {
					(items::SHIVERTHORN_SEEDS, items::SHIVERTHORN)
				} else {
					(items::DAYBLOOM_SEEDS + style, items::DAYBLOOM + style)
				};

				if c.has_in_hand(items::STAFF_OF_REGROWTH) || c.has_in_hand(items::ACORN_AXE) {
					*stack = rr(1, 2);
					*secondary_item = seed;
					*secondary_stack = rr(1, 5);
				} else {
					let harvestable = self.id == 84 || match style {
						0 => w.header.day_time,
						1 => !w.header.day_time,
						3 => !w.header.day_time && (w.header.blood_moon || w.header.moon_phase == 0),
						4 => w.header.raining || w.header.cloud_bg_alpha > 0.,
						5 => !w.header.raining && w.header.day_time && w.header.time > 40500.,
						_ => false,
					};
					if harvestable {
						*secondary_item = seed;
						*secondary_stack = rr(1, 3);
					}
				}

				plant
			}
			107 => 364,
			108 => 365,
			110 => match self.frame_x {
				144 => 5,
				_ => 0,
			}
			111 => 366,
			112 => 370,
			116 => 408,
			117 => 409,
			118 => 412,
			119 => 413,
			120 => 414,
			121 => 415,
			122 => 416,
			123 => 424,
			124 => 480,
			129 => if self.frame_x >= 324 { 4988 } else { 502 }
			130 => 511,
			131 => 512,
			135 => {
				let num5 = self.frame_y / 18;
				match num5 {
					0 => 529,
					1 => 541,
					2 => 542,
					3 => 543,
					4 => 852,
					5 => 853,
					6 => 1151,
					_ => 0,
				}
			}
			136 => 538,
			137 => {
				let num6 = self.frame_y / 18;
				match num6 {
					0 => 539,
					1 => 1146,
					2 => 1147,
					3 => 1148,
					4 => 1149,
					5 => 5135,
					_ => 0,
				}
			}
			140 => 577,
			141 => 580,
			144 => match self.frame_x {
				0 => 583,
				18 => 584,
				36 => 585,
				54 => 4484,
				72 => 4485,
				_ => 0,
			}
			145 => 586,
			146 => 591,
			147 => 593,
			148 => 594,
			149 => match self.frame_x {
				0 | 54 => items::BLUE_LIGHT,
				18 | 72 => items::RED_LIGHT,
				36 | 90 => items::GREEN_LIGHT,
				_ => 0,
			}
			150 => 604,
			151 => 607,
			152 => 609,
			153 => 611,
			154 => 612,
			155 => 613,
			156 => 614,
			157 => 619,
			158 => 620,
			159 => 621,
			160 => 662,
			161 => 664,
			163 => 833,
			164 => 834,
			166 => 699,
			167 => 700,
			168 => 701,
			169 => 702,
			170 => 1872,
			171 => unimplemented!("caller should implement this because there are multiple tile drops. source in source in WorldGen.cs:43957"),
			174 => 713,
			175 => 717,
			176 => 718,
			177 => 719,
			178 => match self.frame_x / 18 {
				0 => 181,
				1 => 180,
				2 => 177,
				3 => 179,
				4 => 178,
				5 => 182,
				6 => 999,
				_ => 0,
			}
			179 | 180 | 181 | 182 | 183 | 381 | 534 | 536 | 539 | 625 | 627 => 3,
			188 => 276,
			189 => 751,
			190 => 183,
			191 => 9,
			193 => 762,
			194 => 154,
			195 => 763,
			196 => 765,
			197 => 767,
			198 => 775,
			200 => 835,
			201 => match self.frame_x {
				270 => 2887,
				_ => 0,
			}
			202 => 824,
			203 => 836,
			204 => 880,
			206 => 883,
			208 => 911,
			210 => 937,
			211 => 947,
			213 => 965,
			214 => 85,
			221 => 1104,
			222 => 1105,
			223 => 1106,
			224 => 1103,
			225 => unimplemented!("caller should handle this because breaking this block may spawn mobs. source in WorldGen.cs:44094"),
			226 => 1101,
			227 => {
				let style = self.frame_x;
				if (8..=11).contains(&style) {
					items::STRANGE_PLANT_1 + style - 8
				} else {
					items::TEAL_MUSHROOM + style
				}
			}
			229 => 1125,
			230 => 1127,
			232 => 1150,
			234 => 1246,
			239 => {
				let num9 = self.frame_x / 18;
				match num9 {
					0 => 20,
					1 => 703,
					2 => 22,
					3 => 704,
					4 => 21,
					5 => 705,
					6 => 19,
					7 => 706,
					8 => 57,
					9 => 117,
					10 => 175,
					11 => 381,
					12 => 1184,
					13 => 382,
					14 => 1191,
					15 => 391,
					16 => 1198,
					17 => 1006,
					18 => 1225,
					19 => 1257,
					20 => 1552,
					21 => 3261,
					22 => 3467,
					_ => 0,
				}
			}
			248 => 1589,
			249 => 1591,
			250 => 1593,
			251 => 1725,
			252 => 1727,
			253 => 1729,
			255..=261 => 1970 + self.id - 255,
			262..=268 => 1970 + self.id - 262,
			272 => 1344,
			273 => 2119,
			274 => 2120,
			284 => 2173,
			311 => 2260,
			312 => 2261,
			313 => 2262,
			314 => match TRACK_TYPE[self.frame_x as usize] {
				0 => items::MINECART_TRACK,
				1 => items::PRESSURE_TRACK,
				2 => items::BOOSTER_TRACK,
				_ => 0,
			}
			315 => 2435,
			321 => 2503,
			322 => 2504,
			323 => {
				if w.header.world_anniversary {
					*stack += rr(2, 4)
				}
				if (88..=132).contains(&self.frame_x) {
					*secondary_item = items::ACORN;
				}
				items::PALM_WOOD

				// todo:
				// int index3 = x;
				// int index4 = y;
				// while (!Main.tile[index3, index4].active() || !Main.tileSolid[(int) Main.tile[index3, index4].type])
				//   ++index4;
				// if (!Main.tile[index3, index4].active())
				//   break;
				// switch (Main.tile[index3, index4].type)
				// {
				//   case 112:
				//     dropItem = 619;
				//     return;
				//   case 116:
				//     dropItem = 621;
				//     return;
				//   case 234:
				//     dropItem = 911;
				//     return;
				//   default:
				//     return;
			}
			324 => match self.frame_x / 18 {
				0 => 2625,
				1 => 2626,
				2 => 4072,
				3 => 4073,
				4 => 4071,
				_ => 0,
			}
			325 => 2692,
			326 => 2693,
			327 => 2694,
			328 => 2695,
			329 => 2697,
			330 => 71,
			331 => 72,
			332 => 73,
			333 => 74,
			336 => 2701,
			340 => 2751,
			341 => 2752,
			342 => 2753,
			343 => 2754,
			344 => 2755,
			345 => 2787,
			346 => 2792,
			347 => 2793,
			348 => 2794,
			350 => 2860,
			351 => 2868,
			353 => 2996,
			357 => 3066,
			365 => 3077,
			366 => 3078,
			367 => 3081,
			368 => 3086,
			369 => 3087,
			370 => 3100,
			371 => 3113,
			372 => 3117,
			379 => 3214,
			380 => 3215 + self.frame_y / 18,
			383 => 620,
			385 => 3234,
			396 => 3271,
			397 => 3272,
			398 => 3274,
			399 => 3275,
			400 => 3276,
			401 => 3277,
			402 => 3338,
			403 => 3339,
			404 => 3347,
			407 => 3380,
			408 => 3460,
			409 => 3461,
			415 => 3573,
			416 => 3574,
			417 => 3575,
			418 => 3576,
			419 => match self.frame_y / 18 {
				0 => 3602,
				1 => 3618,
				2 => 3663,
				_ => 0,
			}
			420 => match self.frame_y / 18 {
				0 => 3603,
				1 => 3604,
				2 => 3605,
				3 => 3606,
				4 => 3607,
				5 => 3608,
				_ => 0,
			}
			421 => 3609,
			422 => 3610,
			// TELogicSensor.Kill needs to be handled by parent
			423 => match self.frame_y / 18 {
				0 => items::LOGIC_SENSOR_SUN,
				1 => items::LOGIC_SENSOR_MOON,
				2 => items::LOGIC_SENSOR_ABOVE,
				3 => items::LOGIC_SENSOR_WATER,
				4 => items::LOGIC_SENSOR_LAVA,
				5 => items::LOGIC_SENSOR_HONEY,
				6 => items::LOGIC_SENSOR_LIQUID,
				_ => 0,
			}
			424 => 3616,
			426 => 3621,
			427 => 3622,
			// PressurePlateHelper.DestroyPlate(new Point(x, y)); needs to be handled by parent
			428 => match self.frame_y / 18 {
				0 => items::WEIGHTED_PRESSURE_PLATE_ORANGE,
				1 => items::WEIGHTED_PRESSURE_PLATE_CYAN,
				2 => items::WEIGHTED_PRESSURE_PLATE_PURPLE,
				3 => items::WEIGHTED_PRESSURE_PLATE_PINK,
				_ => 0,
			}
			429 => 3629,
			430 => 3633,
			431 => 3634,
			432 => 3635,
			433 => 3636,
			434 => 3637,
			435 => 3638,
			436 => 3639,
			437 => 3640,
			438 => 3641,
			439 => 3642,
			442 => 3707,
			445 => 3725,
			446 => 3736,
			447 => 3737,
			448 => 3738,
			449 => 3739,
			450 => 3740,
			451 => 3741,
			458 => 3754,
			459 => 3755,
			460 => 3756,
			472 => 3951,
			473 => 3953,
			474 => 3955,
			476 => 4040,
			478 => 4050,
			479 => 4051,
			494 => 4089,
			495 => 4090,
			496 => 4091,
			498 => 4139,
			500 => 4229,
			501 => 4230,
			502 => 4231,
			503 => 4232,
			507 => 4277,
			508 => 4278,
			512 | 513 | 514 | 515 | 516 | 517 | 535 | 537 | 540 | 626 | 628 => 129,
			519 => if self.frame_y == 90 && random() { 183 } else { 0 }
			520 => 4326,
			528 => if random() { 183 } else { 0 }
			541 => 4392,
			546 | 557 => 4422,
			561 => 4554,
			562 => 4564,
			563 => 4547,
			566 => 999,
			571 => {
				*stack = rr(1, 2);
				items::BAMBOO_BLOCK
			}
			574 => 4717,
			575 => 4718,
			576 => 4719,
			577 => 4720,
			578 => 4721,
			579 => 4761,
			583..=589 => {
				let gem_type = match self.id {
					583 => 180,
					584 => 181,
					585 => 177,
					586 => 179,
					587 => 178,
					588 => 182,
					589 => 999,
					_ => unreachable!(),
				};
				if self.frame_x >= 22 && self.frame_y >= 198 && random() {
					*secondary_item = self.id - 583 + items::GEM_TREE_TOPAZ_SEED;
				}
				if random::<f64>() < 1. / 10. {
					*stack = rr(1, 2);
					gem_type
				} else {
					items::STONE_BLOCK
				}
			}
			593 => 4868,
			618 => 4962,
			624 => 5114,
			630 => 5137,
			631 => 5138,
			633 => 172,
			635 => 5215,
			637 => if random::<f64>() < 1. / 100. { items::ASH_GRASS_SEEDS } else { 0 }
			641 => 5306,
			646 => 5322,
			650 => {
				let style = self.frame_x / 18;
				if style < 6 {
					items::STONE_BLOCK
				} else if style < 12 {
					items::DIRT_BLOCK
				} else if style < 28 {
					items::BONE
				} else if style < 36 {
					items::WOOD
				} else if style < 42 {
					items::SNOW_BLOCK
				} else if style < 48 {
					items::ICE_BLOCK
				} else if style < 54 {
					items::COBWEB
				} else if style < 60 {
					items::SANDSTONE
				} else if style < 66 {
					items::GRANITE
				} else if style < 72 {
					items::MARBLE
				} else if style < 73 {
					items::GRASS_SEEDS
				} else if style < 77 {
					items::SAND_BLOCK
				} else {
					0
				}
			}
			656 => 5333,
			659 => 5349,
			666 => 5395,
			667 => 5398,
			668 => 5400,
			669 => 5401,
			670 => 5402,
			671 => 5403,
			672 => 5404,
			673 => 5405,
			674 => 5406,
			675 => 5407,
			676 => 5408,
			677 => 5417,
			678 => 5419,
			679 => 5421,
			680 => 5423,
			681 => 5425,
			682 => 5427,
			683 => 5433,
			684 => 5435,
			685 => 5429,
			686 => 5431,
			687 => 5439,
			688 => 5440,
			689 => 5441,
			690 => 5442,
			691 => 5443,
			692 => 5444,
			_ => 0,
		}
	}
}
