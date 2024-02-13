#![allow(warnings)]

use macros::message_encoder_decoder;
use crate::binary::types::{RGB, Text};
use crate::binary::writer::Writer;
use crate::binary::reader::Reader;

use tokio::io::{AsyncWrite, AsyncWriteExt};

use std::convert::{TryFrom, TryInto};
use std::io::Write;
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

#[message_encoder_decoder]
pub enum Message<'a> {
	/// 1 <-
	VersionIdentifier(String),
	/// 2 ->
	ConnectionRefuse(Text),
	/// 3 ->
	ConnectionApprove {
		pub client_id: u8,
		pub flag: bool, // "ServerWantsToRunCheckBytesInClientLoopThread" flag. Seems to be always false.
	},
	/// 4 <->
	PlayerDetails {
		pub client_id: u8,
		pub skin_variant: u8,
		pub hair: u8,
		pub name: String,
		pub hair_dye: u8,
		pub hide_accessory: u16,
		pub hide_misc: u8,
		pub hair_color: RGB,
		pub skin_color: RGB,
		pub eye_color: RGB,
		pub shirt_color: RGB,
		pub undershirt_color: RGB,
		pub pants_color: RGB,
		pub shoe_color: RGB,
		/**
			if (player.difficulty == 1) flags_1[0] = true;
			else if (player1.difficulty == 2) flags_1[1] = true;
			else if (player1.difficulty == 3) flags_1[3] = true;
			flags_1[2] = player1.extraAccessory;
		 */
		pub flags_1: u8,
		/**
			flags_2[0] = player1.UsingBiomeTorches;
			flags_2[1] = player1.happyFunTorchTime;
			flags_2[2] = player1.unlockedBiomeTorches;
			flags_2[3] = player1.unlockedSuperCart;
			flags_2[4] = player1.enabledSuperCart;
		 */
		pub flags_2: u8,
		/**
			flags_3[0] = player1.usedAegisCrystal;
			flags_3[1] = player1.usedAegisFruit;
			flags_3[2] = player1.usedArcaneCrystal;
			flags_3[3] = player1.usedGalaxyPearl;
			flags_3[4] = player1.usedGummyWorm;
			flags_3[5] = player1.usedAmbrosia;
			flags_3[6] = player1.ateArtisanBread;
		 */
		pub flags_3: u8,
	},
	/// 5 <->
	PlayerInventorySlot {
		pub client_id: u8,
		pub slot_id: i16,
		pub amount: i16,
		pub prefix: u8,
		pub item_id: i16,
	},
	/// 6 <-
	WorldRequest,
	/// 8 <-
	SpawnRequest {
		pub x: i32,
		pub y: i32,
	},
	/// 16 <->
	PlayerHealth {
		pub client_id: u8,
		pub current: i16,
		pub maximum: i16,
	},
	/// 37 ->
	PasswordRequest,
	/// 38 <-
	PasswordResponse(String),
	/// 42 <-
	PlayerMana {
		pub client_id: u8,
		pub current: i16,
		pub maximum: i16,
	},
	/// 50 <-
	PlayerBuffs {
		pub client_id: u8,
		pub buffs: [u16; MAX_BUFFS],
	},
	/// 68 <-
	UUID(String),
	/// 83 ->
	KillCount {
		pub id: u16,
		pub amount: u32,
	},
	/// 101 ->
	PillarsStatus {
		pub solar: u16,
		pub vortex: u16,
		pub nebula: u16,
		pub stardust: u16,
	},
	/// 147 <->
	PlayerLoadout {
		pub client_id: u8,
		pub index: u8,
		pub hide_accessory: u16,
	},
	/// 0 <->
	Unknown(u8, &'a [u8]),
}

impl<'a> Message<'a> {
	pub async fn write(self, mut stream: Pin<&mut impl AsyncWrite>) -> Result<usize, &str> {
		let buffer: Vec<u8> = self.try_into()?;
		stream.write(&buffer).await.map_err(|_| "Error while writing")
	}
}
