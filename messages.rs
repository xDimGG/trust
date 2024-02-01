use proc_macros::message_read_write;

use tokio::net::TcpStream;
use tokio::prelude::*;

use std::convert::{TryFrom, TryInto};
use std::str;

#[derive(Debug)]
pub struct RGB (u8, u8, u8);

#[message_read_write]
pub enum Message<'a> {
    /// $01 ->
    VersionIdentifier(String),
    /// $02 <-
    ConnectionRefuse(String),
    /// $03 <-
    ConnectionApprove,
    /// $04 ->
    PlayerAppearance {
        id: u8,
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
        difficulty: u8,
        extra_accessory: u8,
    },
    /// $05 ->
    PlayerInventorySlot {
        client_id: u8,
        slot_id: u16,
        amount: u16,
        prefix: u8,
        item_id: u16,
    },
    /// $06 ->
    WorldRequest,
    /// $08 ->
    SpawnRequest {
        x: i32,
        y: i32,
    },
    /// $10 ->
    PlayerHealth {
        client_id: u8,
        current: u16,
        maximum: u16,
    },
    /// $25 <-
    PasswordRequest,
    /// $26 ->
    PasswordResponse(String),
    /// $2A ->
    PlayerMana {
        client_id: u8,
        current: u16,
        maximum: u16,
    },
    /// $32 ->
    PlayerBuffs {
        client_id: u8,
        buffs: [u16; 22],
    },
    /// $44 ->
    UUID(String),
    /// $53 <-
    KillCount {
        id: u16,
        amount: u32,
    },
    /// $65 <-
    PillarsStatus {
        solar: u16,
        vortex: u16,
        nebula: u16,
        stardust: u16,
    },
    /// $00 <->
    Unknown(u8, &'a [u8]),
}

struct MessageReader<'a> {
    buf: &'a [u8],
    cur: usize,
}

impl<'a> MessageReader<'a> {
    fn new(buf: &'a [u8], cur: usize) -> Self {
        Self { buf, cur }
    }

    fn take(&mut self, amount: usize) -> &[u8] {
        self.cur += amount;
        &self.buf[(self.cur - amount)..self.cur]
    }

    fn read_byte(&mut self) -> u8 {
        self.take(1)[0]
    }

    fn read_u16(&mut self) -> u16 {
        u16::from_le_bytes(self.take(2).try_into().unwrap())
    }

    fn read_i32(&mut self) -> i32 {
        i32::from_le_bytes(self.take(4).try_into().unwrap())
    }

    fn read_string(&mut self) -> String {
        let length = self.take(1)[0] as usize;
        str::from_utf8(self.take(length)).unwrap_or("").to_string()
    }

    fn read_rgb(&mut self) -> RGB {
        RGB(self.read_byte(), self.read_byte(), self.read_byte())
    }
}

impl<'a> Message<'a> {
    pub async fn write(self, stream: &mut TcpStream) -> Result<usize, &str> {
        let buffer: Vec<u8> = self.try_into()?;
        stream.write(&buffer).await.map_err(|_| "Error while writing")
    }
}

struct MessageWriter {
    buf: Vec<u8>,
}

impl<'a> MessageWriter {
    fn new(code: u8) -> Self {
        Self { buf: vec![0, 0, code] }
    }

    fn finalize(mut self) -> Vec<u8> {
        let [a, b] = (self.buf.len() as u16).to_le_bytes();
        self.buf[0] = a;
        self.buf[1] = b;
        self.buf
    }

    fn write_byte(mut self, byte: u8) -> Self {
        self.buf.push(byte);
        self
    }

    fn write_bytes(mut self, bytes: &[u8]) -> Self {
        self.buf.append(&mut bytes.to_vec());
        self
    }

    fn write_u16(self, num: u16) -> Self {
        self.write_bytes(&mut num.to_le_bytes().to_vec())
    }

    fn write_u32(self, num: u32) -> Self {
        self.write_bytes(&mut num.to_le_bytes().to_vec())
    }

    fn write_string(mut self, mut string: String) -> Self {
        self.buf.push(string.len() as u8);
        self.buf.append(unsafe { string.as_mut_vec() });
        self.buf.push(0);
        self
    }
}
