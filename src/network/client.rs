use std::net::SocketAddr;

use crate::world::transpiled::items;
use crate::world::types::World;
use crate::network::messages::{self, PlayerItemSlot, Message, MessageDecodeError};
use crate::network::utils::encode_tiles;
use crate::network::transpiled::item_slots;

pub const MAX_ITEM_SLOTS: usize = 350;

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
	pub items: Box<[Option<PlayerItemSlot>; MAX_ITEM_SLOTS]>,
	pub loaded_sections: Vec<Vec<bool>>,
	pub selected_item: u8,
	pub selected_loadout: u8,
}

impl Client {
	pub fn new(addr: SocketAddr, width: usize, height: usize) -> Self {
		// https://github.com/rust-lang/rust/issues/44796#issuecomment-967747810
		const INIT_SLOT_NONE: Option<PlayerItemSlot> = None;

		Self {
			addr,
			state: ConnectionState::New,
			details: None,
			uuid: None,
			health: None,
			buffs: None,
			mana: None,
			items: Box::new([INIT_SLOT_NONE; MAX_ITEM_SLOTS]),
			loaded_sections: vec![vec![false; height]; width],
			selected_item: 0,
			selected_loadout: 0,
		}
	}

	pub fn iter_inventory(&self) -> impl Iterator<Item = &Option<PlayerItemSlot>> {
		self.items.iter().take(item_slots::INVENTORY_END + 1)
	}

	// pub fn iter_loadout(&self) -> impl Iterator<Item = &Option<PlayerItemSlot>> {
	// 	let start = item_slots::LOADOUTS_START[self.selected_loadout as usize];
	// 	let end = item_slots::LOADOUTS_END[self.selected_loadout as usize];
	// 	self.items.iter().skip(start).take(end - start + 1)
	// }

	pub fn iter_equipment(&self) -> impl Iterator<Item = &Option<PlayerItemSlot>> {
		let start = item_slots::ARMOR_LOADOUTS_START[self.selected_loadout as usize];
		let end = item_slots::ARMOR_LOADOUTS_END[self.selected_loadout as usize];
		self.items.iter().skip(start).take(end - start + 1)
	}

	// pub fn iter_dyes(&self) -> impl Iterator<Item = &Option<PlayerItemSlot>> {
	// 	let start = item_slots::DYE_LOADOUTS_START[self.selected_loadout as usize];
	// 	let end = item_slots::DYE_LOADOUTS_END[self.selected_loadout as usize];
	// 	self.items.iter().skip(start).take(end - start + 1)
	// }

	pub fn has_equipped(&self, id: i16) -> bool {
		self.iter_equipment().any(|slot| slot.as_ref().is_some_and(|slot| slot.item_id == id))
	}

	pub fn has_in_hand(&self, id: i16) -> bool {
		self.items[self.selected_item as usize].as_ref().is_some_and(|slot| slot.item_id == id)
	}

	pub fn has_in_inventory(&self, id: i16) -> bool {
		self.iter_inventory().any(|slot| slot.as_ref().is_some_and(|slot| slot.item_id == id))
	}

	pub fn has_seed_weapon(&self) -> bool {
		self.has_in_inventory(items::BLOWPIPE) || self.has_in_inventory(items::BLOWGUN)
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
