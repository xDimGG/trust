use std::net::SocketAddr;

use crate::world::transpiled::items;
use crate::world::types::World;
use crate::network::messages::{self, Message, MessageDecodeError};
use crate::network::utils::encode_tiles;

pub const MAX_INVENTORY_SLOTS: usize = 350;

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum ConnectionState {
	New,
	PendingAuthentication,
	Authenticated,
	DetailsReceived,
	Complete,
}

pub struct Client {
	pub addr: SocketAddr,
	pub state: ConnectionState,
	pub uuid: Option<String>,
	pub details: Option<messages::PlayerDetails>,
	pub health: Option<messages::PlayerHealth>,
	pub mana: Option<messages::PlayerMana>,
	pub buffs: Option<messages::PlayerBuffs>,
	pub loadout: Option<messages::PlayerLoadout>,
	pub inventory: Box<[Option<messages::PlayerInventorySlot>; MAX_INVENTORY_SLOTS]>,
	pub loaded_sections: Vec<Vec<bool>>,
	pub selected_item: u8,
}

impl Client {
	pub fn new(addr: SocketAddr, width: usize, height: usize) -> Self {
		// https://github.com/rust-lang/rust/issues/44796#issuecomment-967747810
		const INIT_SLOT_NONE: Option<messages::PlayerInventorySlot> = None;

		Self {
			addr,
			state: ConnectionState::New,
			details: None,
			uuid: None,
			health: None,
			buffs: None,
			mana: None,
			loadout: None,
			inventory: Box::new([INIT_SLOT_NONE; MAX_INVENTORY_SLOTS]),
			loaded_sections: vec![vec![false; height]; width],
			selected_item: 0,
		}
	}

	pub fn has_item(&self, id: i16) -> bool {
		self.inventory.iter().take(58).any(|slot| slot.as_ref().is_some_and(|slot| slot.item_id == id))
	}

	pub fn has_seed_weapon(&self) -> bool {
		self.has_item(items::BLOWPIPE) || self.has_item(items::BLOWGUN)
	}

	pub fn encode_sections(
		&mut self,
		world: &World,
		sec_x_start: usize,
		sec_x_end: usize,
		sec_y_start: usize,
		sec_y_end: usize,
	) -> Result<Vec<Message>, MessageDecodeError> {
		let mut msgs = vec![];
		for x in sec_x_start..=sec_x_end {
			for y in sec_y_start..=sec_y_end {
				if self.loaded_sections[x][y] {
					continue;
				}

				self.loaded_sections[x][y] = true;
				msgs.push(encode_tiles(world, x, y)?)
			}
		}

		Ok(msgs)
	}
}
