use macros::message_encoder_decoder;
use crate::binary::types::{Text, Vector2, RGB};
use crate::binary::writer::Writer;
use crate::binary::reader::Reader;

use tokio::io::{AsyncWrite, AsyncWriteExt};

use std::convert::{TryFrom, TryInto};
use std::pin::Pin;
use std::str;

pub trait Sanitize {
	fn sanitize(&mut self, src: u8);
}

const MAX_VARIANT_COUNT: u8 = 12; // PlayerVariantID.Count
const MAX_HAIR: u8 = 165; // from MessageBuffer case 4

impl Sanitize for PlayerDetails {
	fn sanitize(&mut self, src: u8) {
		self.client_id = src;
		if self.skin_variant >= MAX_VARIANT_COUNT {
			self.skin_variant = MAX_VARIANT_COUNT - 1;
		}
		if self.hair >= MAX_HAIR {
			self.hair = 0;
		}
	}
}

const MIN_MAXIMUM_HEALTH: i16 = 100; // from MessageBuffer case 16

impl Sanitize for PlayerHealth {
	fn sanitize(&mut self, src: u8) {
		self.client_id = src;
		if self.maximum >= MIN_MAXIMUM_HEALTH {
			self.maximum = MIN_MAXIMUM_HEALTH;
		}
	}
}

impl Sanitize for PlayerMana {
	fn sanitize(&mut self, src: u8) {
		self.client_id = src;
	}
}

impl Sanitize for PlayerBuffs {
	fn sanitize(&mut self, src: u8) {
		self.client_id = src;
	}
}

impl Sanitize for PlayerLoadout {
	fn sanitize(&mut self, src: u8) {
		self.client_id = src;
	}
}

impl Sanitize for PlayerInventorySlot {
	fn sanitize(&mut self, src: u8) {
		self.client_id = src;
	}
}

const MAX_BUFFS: usize = 44; // from Player.maxBuffs
const AREA_ID_COUNT: usize = 13; // from TreeTopsInfo.AreaId.Count

#[message_encoder_decoder]
pub enum Message {
	/// 1 <-
	VersionIdentifier(String),
	/// 2 ->
	ConnectionRefuse(Text),
	/// 3 ->
	ConnectionApprove {
		client_id: u8,
		flag: bool, // "ServerWantsToRunCheckBytesInClientLoopThread" flag. Seems to be always false.
	},
	/// 4 <->
	PlayerDetails {
		client_id: u8,
		skin_variant: u8,
		hair: u8,
		name: String,
		hair_dye: u8,
		hide_accessory: u16,
		hide_misc: u8,
		hair_color: RGB,
		skin_color: RGB,
		eye_color: RGB,
		shirt_color: RGB,
		undershirt_color: RGB,
		pants_color: RGB,
		shoe_color: RGB,
		/**
			if (player.difficulty == 1) flags_1[0] = true;
			else if (player1.difficulty == 2) flags_1[1] = true;
			else if (player1.difficulty == 3) flags_1[3] = true;
			flags_1[2] = player1.extraAccessory;
		 */
		flags_1: u8,
		/**
			flags_2[0] = player1.UsingBiomeTorches;
			flags_2[1] = player1.happyFunTorchTime;
			flags_2[2] = player1.unlockedBiomeTorches;
			flags_2[3] = player1.unlockedSuperCart;
			flags_2[4] = player1.enabledSuperCart;
		 */
		flags_2: u8,
		/**
			flags_3[0] = player1.usedAegisCrystal;
			flags_3[1] = player1.usedAegisFruit;
			flags_3[2] = player1.usedArcaneCrystal;
			flags_3[3] = player1.usedGalaxyPearl;
			flags_3[4] = player1.usedGummyWorm;
			flags_3[5] = player1.usedAmbrosia;
			flags_3[6] = player1.ateArtisanBread;
		 */
		flags_3: u8,
	},
	/// 5 <->
	PlayerInventorySlot {
		client_id: u8,
		slot_id: i16,
		amount: i16,
		prefix: u8,
		item_id: i16,
	},
	/// 6 <-
	WorldRequest,
	/// 7 ->
	WorldHeader {
		time: i32,
		time_flags: u8,
		moon_phase: u8,
		width: i16,
		height: i16,
		spawn_x: i16,
		spawn_y: i16,
		world_surface: i16,
		rock_layer: i16,
		id: i32,
		name: String,
		game_mode: u8,
		uuid: [u8; 16],
		worldgen_version: u64,
		moon_type: u8,
		bg_0: u8,
		bg_10: u8,
		bg_11: u8,
		bg_12: u8,
		bg_1: u8,
		bg_2: u8,
		bg_3: u8,
		bg_4: u8,
		bg_5: u8,
		bg_6: u8,
		bg_7: u8,
		bg_8: u8,
		bg_9: u8,
		ice_back_style: u8,
		jungle_back_style: u8,
		hell_back_style: u8,
		wind_speed_target: f32,
		num_clouds: u8,
		tree_x: [i32; 3],
		tree_style: [u8; 4],
		cave_back_x: [i32; 3],
		cave_back_style: [u8; 4],
		tree_top_variations: [u8; AREA_ID_COUNT],
		max_raining: f32,
		flags: [u8; 10],
		sundial_cooldown: u8,
		moondial_cooldown: u8,
		ore_tier_copper: i16,
		ore_tier_iron: i16,
		ore_tier_silver: i16,
		ore_tier_gold: i16,
		ore_tier_cobalt: i16,
		ore_tier_mythril: i16,
		ore_tier_adamantite: i16,
		invasion_type: i8,
		lobby_id: u64,
		sandstorm_intended_severity: f32,
	},
	/// 8 <-
	SpawnRequest {
		x: i32,
		y: i32,
	},
	/// 9 ->
	SpawnResponse {
		status: i32,
		text: Text,
		flags: u8, // HideStatusTextPercent | StatusTextHasShadows << 1 | ServerWantsToRunCheckBytesInClientLoopThread << 2
	},
	/// 16 <->
	PlayerHealth {
		client_id: u8,
		current: i16,
		maximum: i16,
	},
	/// 23 ->
	NPCInfo {
		id: i16,
		position: Vector2,
		velocity: Vector2,
		target: u16,
		flags_1: u8,
		flags_2: u8,
		npc_ai: Vec<u8>,
		id_2: i16,
		stats_scaled_for_n_players: Option<u8>,
		strength_multiplier: Option<u8>,
		
	},
	/// 37 ->
	PasswordRequest,
	/// 38 <-
	PasswordResponse(String),
	/// 42 <-
	PlayerMana {
		client_id: u8,
		current: i16,
		maximum: i16,
	},
	/// 50 <-
	PlayerBuffs {
		client_id: u8,
		buffs: [u16; MAX_BUFFS],
	},
	/// 68 <-
	UUID(String),
	/// 78 ->
	InvasionProgress {
		progress: i32,
		progress_max: i32,
		icon: i8,
		progress_wave: i8,
	},
	/// 83 ->
	KillCount {
		id: u16,
		amount: u32,
	},
	/// 101 ->
	PillarsStatus {
		solar: u16,
		vortex: u16,
		nebula: u16,
		stardust: u16,
	},
	/// 147 <->
	PlayerLoadout {
		client_id: u8,
		index: u8,
		hide_accessory: u16,
	},
	/// 0 <->
	Custom(u8, Vec<u8>),
}

impl Message {
	pub async fn write(self, mut stream: Pin<&mut impl AsyncWrite>) -> Result<usize, &str> {
		let buffer: Vec<u8> = self.try_into()?;
		stream.write(&buffer).await.map_err(|_| "Error while writing")
	}
}
