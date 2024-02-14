#![feature(prelude_import)]
#![allow(clippy::upper_case_acronyms)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod binary {
    pub mod reader {
        use crate::binary::types::{Text, RGB};
        pub struct Reader<'a> {
            buf: &'a [u8],
            cur: usize,
        }
        #[allow(dead_code)]
        impl<'a> Reader<'a> {
            pub fn new(buf: &'a [u8]) -> Self {
                Self { buf, cur: 0 }
            }
            pub fn read_bytes(&mut self, amount: usize) -> &[u8] {
                self.cur += amount;
                &self.buf[(self.cur - amount)..self.cur]
            }
            pub fn read_byte(&mut self) -> u8 {
                self.read_bytes(1)[0]
            }
            pub fn read_bool(&mut self) -> bool {
                self.read_byte() != 0
            }
            pub fn read_i8(&mut self) -> i8 {
                self.read_byte() as i8
            }
            pub fn read_u16(&mut self) -> u16 {
                u16::from_le_bytes(self.read_bytes(2).try_into().unwrap())
            }
            pub fn read_i16(&mut self) -> i16 {
                i16::from_le_bytes(self.read_bytes(2).try_into().unwrap())
            }
            pub fn read_u32(&mut self) -> u32 {
                u32::from_le_bytes(self.read_bytes(4).try_into().unwrap())
            }
            pub fn read_i32(&mut self) -> i32 {
                i32::from_le_bytes(self.read_bytes(4).try_into().unwrap())
            }
            pub fn read_u64(&mut self) -> u64 {
                u64::from_le_bytes(self.read_bytes(8).try_into().unwrap())
            }
            pub fn read_i64(&mut self) -> i64 {
                i64::from_le_bytes(self.read_bytes(8).try_into().unwrap())
            }
            pub fn read_length(&mut self) -> usize {
                let mut length = self.read_byte() as usize;
                let mut shift = 7;
                while length & (1 << shift) != 0 {
                    length &= !(1 << shift);
                    length |= (self.read_byte() as usize) << shift;
                    shift += 7;
                }
                length
            }
            pub fn read_string(&mut self) -> String {
                let length = self.read_length();
                std::str::from_utf8(self.read_bytes(length)).unwrap_or("").to_string()
            }
            pub fn read_text(&mut self) -> Text {
                Text(self.read_byte().into(), self.read_string())
            }
            pub fn read_rgb(&mut self) -> RGB {
                RGB(self.read_byte(), self.read_byte(), self.read_byte())
            }
        }
    }
    pub mod writer {
        use crate::binary::types::{Text, RGB};
        pub struct Writer {
            buf: Vec<u8>,
        }
        #[allow(dead_code)]
        impl Writer {
            pub fn new(code: u8) -> Self {
                Self {
                    buf: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([0, 0, code]),
                    ),
                }
            }
            pub fn finalize(mut self) -> Vec<u8> {
                let [a, b] = (self.buf.len() as u16).to_le_bytes();
                self.buf[0] = a;
                self.buf[1] = b;
                self.buf
            }
            pub fn write_bytes(mut self, bytes: &[u8]) -> Self {
                self.buf.append(&mut bytes.to_vec());
                self
            }
            pub fn write_byte(mut self, byte: u8) -> Self {
                self.buf.push(byte);
                self
            }
            pub fn write_bool(self, b: bool) -> Self {
                self.write_byte(b as u8)
            }
            pub fn write_i8(self, num: i8) -> Self {
                self.write_bytes(&num.to_le_bytes())
            }
            pub fn write_u16(self, num: u16) -> Self {
                self.write_bytes(&num.to_le_bytes())
            }
            pub fn write_i16(self, num: i16) -> Self {
                self.write_bytes(&num.to_le_bytes())
            }
            pub fn write_u32(self, num: u32) -> Self {
                self.write_bytes(&num.to_le_bytes())
            }
            pub fn write_i32(self, num: i32) -> Self {
                self.write_bytes(&num.to_le_bytes())
            }
            pub fn write_u64(self, num: u64) -> Self {
                self.write_bytes(&num.to_le_bytes())
            }
            pub fn write_i64(self, num: i64) -> Self {
                self.write_bytes(&num.to_le_bytes())
            }
            pub fn write_length(mut self, mut len: usize) -> Self {
                while len >= (1 << 7) {
                    self = self.write_byte((len & 0b1111111) as u8 | (1 << 7));
                    len >>= 7;
                }
                self.write_byte(len as u8)
            }
            pub fn write_string(self, string: String) -> Self {
                self.write_length(string.len())
                    .write_bytes(string.as_bytes())
                    .write_byte(0)
            }
            pub fn write_text(self, text: Text) -> Self {
                self.write_byte(text.0 as u8).write_string(text.1)
            }
            pub fn write_rgb(self, rgb: RGB) -> Self {
                self.write_byte(rgb.0).write_byte(rgb.1).write_byte(rgb.2)
            }
        }
    }
    pub mod types {
        pub struct RGB(pub u8, pub u8, pub u8);
        #[automatically_derived]
        impl ::core::fmt::Debug for RGB {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field3_finish(
                    f,
                    "RGB",
                    &self.0,
                    &self.1,
                    &&self.2,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RGB {
            #[inline]
            fn clone(&self) -> RGB {
                RGB(
                    ::core::clone::Clone::clone(&self.0),
                    ::core::clone::Clone::clone(&self.1),
                    ::core::clone::Clone::clone(&self.2),
                )
            }
        }
        pub struct Text(pub TextMode, pub String);
        #[automatically_derived]
        impl ::core::fmt::Debug for Text {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Text",
                    &self.0,
                    &&self.1,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Text {
            #[inline]
            fn clone(&self) -> Text {
                Text(
                    ::core::clone::Clone::clone(&self.0),
                    ::core::clone::Clone::clone(&self.1),
                )
            }
        }
        pub struct Vector2(pub f32, pub f32);
        #[automatically_derived]
        impl ::core::fmt::Debug for Vector2 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Vector2",
                    &self.0,
                    &&self.1,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Vector2 {
            #[inline]
            fn clone(&self) -> Vector2 {
                Vector2(
                    ::core::clone::Clone::clone(&self.0),
                    ::core::clone::Clone::clone(&self.1),
                )
            }
        }
        #[repr(u8)]
        pub enum TextMode {
            Literal,
            Formattable,
            LocalizationKey,
            Invalid,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TextMode {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        TextMode::Literal => "Literal",
                        TextMode::Formattable => "Formattable",
                        TextMode::LocalizationKey => "LocalizationKey",
                        TextMode::Invalid => "Invalid",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TextMode {
            #[inline]
            fn clone(&self) -> TextMode {
                match self {
                    TextMode::Literal => TextMode::Literal,
                    TextMode::Formattable => TextMode::Formattable,
                    TextMode::LocalizationKey => TextMode::LocalizationKey,
                    TextMode::Invalid => TextMode::Invalid,
                }
            }
        }
        impl From<u8> for TextMode {
            fn from(value: u8) -> Self {
                match value {
                    0 => Self::Literal,
                    1 => Self::Formattable,
                    2 => Self::LocalizationKey,
                    _ => Self::Invalid,
                }
            }
        }
    }
}
mod network {
    pub mod messages {
        use macros::message_encoder_decoder;
        use crate::binary::types::{RGB, Text};
        use crate::binary::writer::Writer;
        use crate::binary::reader::Reader;
        use tokio::io::{AsyncWrite, AsyncWriteExt};
        use std::convert::{TryFrom, TryInto};
        use std::pin::Pin;
        use std::str;
        pub trait Sanitize {
            fn sanitize(&mut self, src: u8);
        }
        const MAX_VARIANT_COUNT: u8 = 12;
        const MAX_HAIR: u8 = 165;
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
        const MIN_MAXIMUM_HEALTH: i16 = 100;
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
        const MAX_BUFFS: usize = 44;
        pub struct ConnectionApprove {
            pub client_id: u8,
            pub flag: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ConnectionApprove {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ConnectionApprove",
                    "client_id",
                    &self.client_id,
                    "flag",
                    &&self.flag,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ConnectionApprove {
            #[inline]
            fn clone(&self) -> ConnectionApprove {
                ConnectionApprove {
                    client_id: ::core::clone::Clone::clone(&self.client_id),
                    flag: ::core::clone::Clone::clone(&self.flag),
                }
            }
        }
        pub struct PlayerDetails {
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
            pub flags_1: u8,
            pub flags_2: u8,
            pub flags_3: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PlayerDetails {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "client_id",
                    "skin_variant",
                    "hair",
                    "name",
                    "hair_dye",
                    "hide_accessory",
                    "hide_misc",
                    "hair_color",
                    "skin_color",
                    "eye_color",
                    "shirt_color",
                    "undershirt_color",
                    "pants_color",
                    "shoe_color",
                    "flags_1",
                    "flags_2",
                    "flags_3",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.client_id,
                    &self.skin_variant,
                    &self.hair,
                    &self.name,
                    &self.hair_dye,
                    &self.hide_accessory,
                    &self.hide_misc,
                    &self.hair_color,
                    &self.skin_color,
                    &self.eye_color,
                    &self.shirt_color,
                    &self.undershirt_color,
                    &self.pants_color,
                    &self.shoe_color,
                    &self.flags_1,
                    &self.flags_2,
                    &&self.flags_3,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "PlayerDetails",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PlayerDetails {
            #[inline]
            fn clone(&self) -> PlayerDetails {
                PlayerDetails {
                    client_id: ::core::clone::Clone::clone(&self.client_id),
                    skin_variant: ::core::clone::Clone::clone(&self.skin_variant),
                    hair: ::core::clone::Clone::clone(&self.hair),
                    name: ::core::clone::Clone::clone(&self.name),
                    hair_dye: ::core::clone::Clone::clone(&self.hair_dye),
                    hide_accessory: ::core::clone::Clone::clone(&self.hide_accessory),
                    hide_misc: ::core::clone::Clone::clone(&self.hide_misc),
                    hair_color: ::core::clone::Clone::clone(&self.hair_color),
                    skin_color: ::core::clone::Clone::clone(&self.skin_color),
                    eye_color: ::core::clone::Clone::clone(&self.eye_color),
                    shirt_color: ::core::clone::Clone::clone(&self.shirt_color),
                    undershirt_color: ::core::clone::Clone::clone(
                        &self.undershirt_color,
                    ),
                    pants_color: ::core::clone::Clone::clone(&self.pants_color),
                    shoe_color: ::core::clone::Clone::clone(&self.shoe_color),
                    flags_1: ::core::clone::Clone::clone(&self.flags_1),
                    flags_2: ::core::clone::Clone::clone(&self.flags_2),
                    flags_3: ::core::clone::Clone::clone(&self.flags_3),
                }
            }
        }
        pub struct PlayerInventorySlot {
            pub client_id: u8,
            pub slot_id: i16,
            pub amount: i16,
            pub prefix: u8,
            pub item_id: i16,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PlayerInventorySlot {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "PlayerInventorySlot",
                    "client_id",
                    &self.client_id,
                    "slot_id",
                    &self.slot_id,
                    "amount",
                    &self.amount,
                    "prefix",
                    &self.prefix,
                    "item_id",
                    &&self.item_id,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PlayerInventorySlot {
            #[inline]
            fn clone(&self) -> PlayerInventorySlot {
                PlayerInventorySlot {
                    client_id: ::core::clone::Clone::clone(&self.client_id),
                    slot_id: ::core::clone::Clone::clone(&self.slot_id),
                    amount: ::core::clone::Clone::clone(&self.amount),
                    prefix: ::core::clone::Clone::clone(&self.prefix),
                    item_id: ::core::clone::Clone::clone(&self.item_id),
                }
            }
        }
        pub struct SpawnRequest {
            pub x: i32,
            pub y: i32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SpawnRequest {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "SpawnRequest",
                    "x",
                    &self.x,
                    "y",
                    &&self.y,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SpawnRequest {
            #[inline]
            fn clone(&self) -> SpawnRequest {
                SpawnRequest {
                    x: ::core::clone::Clone::clone(&self.x),
                    y: ::core::clone::Clone::clone(&self.y),
                }
            }
        }
        pub struct PlayerHealth {
            pub client_id: u8,
            pub current: i16,
            pub maximum: i16,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PlayerHealth {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "PlayerHealth",
                    "client_id",
                    &self.client_id,
                    "current",
                    &self.current,
                    "maximum",
                    &&self.maximum,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PlayerHealth {
            #[inline]
            fn clone(&self) -> PlayerHealth {
                PlayerHealth {
                    client_id: ::core::clone::Clone::clone(&self.client_id),
                    current: ::core::clone::Clone::clone(&self.current),
                    maximum: ::core::clone::Clone::clone(&self.maximum),
                }
            }
        }
        pub struct PlayerMana {
            pub client_id: u8,
            pub current: i16,
            pub maximum: i16,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PlayerMana {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "PlayerMana",
                    "client_id",
                    &self.client_id,
                    "current",
                    &self.current,
                    "maximum",
                    &&self.maximum,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PlayerMana {
            #[inline]
            fn clone(&self) -> PlayerMana {
                PlayerMana {
                    client_id: ::core::clone::Clone::clone(&self.client_id),
                    current: ::core::clone::Clone::clone(&self.current),
                    maximum: ::core::clone::Clone::clone(&self.maximum),
                }
            }
        }
        pub struct PlayerBuffs {
            pub client_id: u8,
            pub buffs: [u16; MAX_BUFFS],
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PlayerBuffs {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "PlayerBuffs",
                    "client_id",
                    &self.client_id,
                    "buffs",
                    &&self.buffs,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PlayerBuffs {
            #[inline]
            fn clone(&self) -> PlayerBuffs {
                PlayerBuffs {
                    client_id: ::core::clone::Clone::clone(&self.client_id),
                    buffs: ::core::clone::Clone::clone(&self.buffs),
                }
            }
        }
        pub struct InvasionProgress {
            pub progress: i32,
            pub progress_max: i32,
            pub icon: i8,
            pub progress_wave: i8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InvasionProgress {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "InvasionProgress",
                    "progress",
                    &self.progress,
                    "progress_max",
                    &self.progress_max,
                    "icon",
                    &self.icon,
                    "progress_wave",
                    &&self.progress_wave,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for InvasionProgress {
            #[inline]
            fn clone(&self) -> InvasionProgress {
                InvasionProgress {
                    progress: ::core::clone::Clone::clone(&self.progress),
                    progress_max: ::core::clone::Clone::clone(&self.progress_max),
                    icon: ::core::clone::Clone::clone(&self.icon),
                    progress_wave: ::core::clone::Clone::clone(&self.progress_wave),
                }
            }
        }
        pub struct KillCount {
            pub id: u16,
            pub amount: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for KillCount {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "KillCount",
                    "id",
                    &self.id,
                    "amount",
                    &&self.amount,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for KillCount {
            #[inline]
            fn clone(&self) -> KillCount {
                KillCount {
                    id: ::core::clone::Clone::clone(&self.id),
                    amount: ::core::clone::Clone::clone(&self.amount),
                }
            }
        }
        pub struct PillarsStatus {
            pub solar: u16,
            pub vortex: u16,
            pub nebula: u16,
            pub stardust: u16,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PillarsStatus {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "PillarsStatus",
                    "solar",
                    &self.solar,
                    "vortex",
                    &self.vortex,
                    "nebula",
                    &self.nebula,
                    "stardust",
                    &&self.stardust,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PillarsStatus {
            #[inline]
            fn clone(&self) -> PillarsStatus {
                PillarsStatus {
                    solar: ::core::clone::Clone::clone(&self.solar),
                    vortex: ::core::clone::Clone::clone(&self.vortex),
                    nebula: ::core::clone::Clone::clone(&self.nebula),
                    stardust: ::core::clone::Clone::clone(&self.stardust),
                }
            }
        }
        pub struct PlayerLoadout {
            pub client_id: u8,
            pub index: u8,
            pub hide_accessory: u16,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PlayerLoadout {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "PlayerLoadout",
                    "client_id",
                    &self.client_id,
                    "index",
                    &self.index,
                    "hide_accessory",
                    &&self.hide_accessory,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PlayerLoadout {
            #[inline]
            fn clone(&self) -> PlayerLoadout {
                PlayerLoadout {
                    client_id: ::core::clone::Clone::clone(&self.client_id),
                    index: ::core::clone::Clone::clone(&self.index),
                    hide_accessory: ::core::clone::Clone::clone(&self.hide_accessory),
                }
            }
        }
        pub enum Message<'a> {
            /// 1 <-
            VersionIdentifier(String),
            /// 2 ->
            ConnectionRefuse(Text),
            ConnectionApprove(ConnectionApprove),
            PlayerDetails(PlayerDetails),
            PlayerInventorySlot(PlayerInventorySlot),
            /// 6 <-
            WorldRequest,
            SpawnRequest(SpawnRequest),
            PlayerHealth(PlayerHealth),
            /// 37 ->
            PasswordRequest,
            /// 38 <-
            PasswordResponse(String),
            PlayerMana(PlayerMana),
            PlayerBuffs(PlayerBuffs),
            /// 68 <-
            UUID(String),
            InvasionProgress(InvasionProgress),
            KillCount(KillCount),
            PillarsStatus(PillarsStatus),
            PlayerLoadout(PlayerLoadout),
            /// 0 <->
            Unknown(u8, &'a [u8]),
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for Message<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Message::VersionIdentifier(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "VersionIdentifier",
                            &__self_0,
                        )
                    }
                    Message::ConnectionRefuse(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ConnectionRefuse",
                            &__self_0,
                        )
                    }
                    Message::ConnectionApprove(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ConnectionApprove",
                            &__self_0,
                        )
                    }
                    Message::PlayerDetails(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PlayerDetails",
                            &__self_0,
                        )
                    }
                    Message::PlayerInventorySlot(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PlayerInventorySlot",
                            &__self_0,
                        )
                    }
                    Message::WorldRequest => {
                        ::core::fmt::Formatter::write_str(f, "WorldRequest")
                    }
                    Message::SpawnRequest(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "SpawnRequest",
                            &__self_0,
                        )
                    }
                    Message::PlayerHealth(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PlayerHealth",
                            &__self_0,
                        )
                    }
                    Message::PasswordRequest => {
                        ::core::fmt::Formatter::write_str(f, "PasswordRequest")
                    }
                    Message::PasswordResponse(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PasswordResponse",
                            &__self_0,
                        )
                    }
                    Message::PlayerMana(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PlayerMana",
                            &__self_0,
                        )
                    }
                    Message::PlayerBuffs(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PlayerBuffs",
                            &__self_0,
                        )
                    }
                    Message::UUID(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "UUID",
                            &__self_0,
                        )
                    }
                    Message::InvasionProgress(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "InvasionProgress",
                            &__self_0,
                        )
                    }
                    Message::KillCount(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "KillCount",
                            &__self_0,
                        )
                    }
                    Message::PillarsStatus(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PillarsStatus",
                            &__self_0,
                        )
                    }
                    Message::PlayerLoadout(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PlayerLoadout",
                            &__self_0,
                        )
                    }
                    Message::Unknown(__self_0, __self_1) => {
                        ::core::fmt::Formatter::debug_tuple_field2_finish(
                            f,
                            "Unknown",
                            __self_0,
                            &__self_1,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for Message<'a> {
            #[inline]
            fn clone(&self) -> Message<'a> {
                match self {
                    Message::VersionIdentifier(__self_0) => {
                        Message::VersionIdentifier(::core::clone::Clone::clone(__self_0))
                    }
                    Message::ConnectionRefuse(__self_0) => {
                        Message::ConnectionRefuse(::core::clone::Clone::clone(__self_0))
                    }
                    Message::ConnectionApprove(__self_0) => {
                        Message::ConnectionApprove(::core::clone::Clone::clone(__self_0))
                    }
                    Message::PlayerDetails(__self_0) => {
                        Message::PlayerDetails(::core::clone::Clone::clone(__self_0))
                    }
                    Message::PlayerInventorySlot(__self_0) => {
                        Message::PlayerInventorySlot(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    Message::WorldRequest => Message::WorldRequest,
                    Message::SpawnRequest(__self_0) => {
                        Message::SpawnRequest(::core::clone::Clone::clone(__self_0))
                    }
                    Message::PlayerHealth(__self_0) => {
                        Message::PlayerHealth(::core::clone::Clone::clone(__self_0))
                    }
                    Message::PasswordRequest => Message::PasswordRequest,
                    Message::PasswordResponse(__self_0) => {
                        Message::PasswordResponse(::core::clone::Clone::clone(__self_0))
                    }
                    Message::PlayerMana(__self_0) => {
                        Message::PlayerMana(::core::clone::Clone::clone(__self_0))
                    }
                    Message::PlayerBuffs(__self_0) => {
                        Message::PlayerBuffs(::core::clone::Clone::clone(__self_0))
                    }
                    Message::UUID(__self_0) => {
                        Message::UUID(::core::clone::Clone::clone(__self_0))
                    }
                    Message::InvasionProgress(__self_0) => {
                        Message::InvasionProgress(::core::clone::Clone::clone(__self_0))
                    }
                    Message::KillCount(__self_0) => {
                        Message::KillCount(::core::clone::Clone::clone(__self_0))
                    }
                    Message::PillarsStatus(__self_0) => {
                        Message::PillarsStatus(::core::clone::Clone::clone(__self_0))
                    }
                    Message::PlayerLoadout(__self_0) => {
                        Message::PlayerLoadout(::core::clone::Clone::clone(__self_0))
                    }
                    Message::Unknown(__self_0, __self_1) => {
                        Message::Unknown(
                            ::core::clone::Clone::clone(__self_0),
                            ::core::clone::Clone::clone(__self_1),
                        )
                    }
                }
            }
        }
        impl<'a> TryFrom<Message<'a>> for Vec<u8> {
            type Error = &'static str;
            fn try_from(msg: Message) -> Result<Self, Self::Error> {
                match msg {
                    Message::ConnectionRefuse(field0) => {
                        Ok(Writer::new(2u8).write_text(field0).finalize())
                    }
                    Message::ConnectionApprove(data) => {
                        Ok(
                            Writer::new(3u8)
                                .write_byte(data.client_id)
                                .write_bool(data.flag)
                                .finalize(),
                        )
                    }
                    Message::PlayerDetails(data) => {
                        Ok(
                            Writer::new(4u8)
                                .write_byte(data.client_id)
                                .write_byte(data.skin_variant)
                                .write_byte(data.hair)
                                .write_string(data.name)
                                .write_byte(data.hair_dye)
                                .write_u16(data.hide_accessory)
                                .write_byte(data.hide_misc)
                                .write_rgb(data.hair_color)
                                .write_rgb(data.skin_color)
                                .write_rgb(data.eye_color)
                                .write_rgb(data.shirt_color)
                                .write_rgb(data.undershirt_color)
                                .write_rgb(data.pants_color)
                                .write_rgb(data.shoe_color)
                                .write_byte(data.flags_1)
                                .write_byte(data.flags_2)
                                .write_byte(data.flags_3)
                                .finalize(),
                        )
                    }
                    Message::PlayerInventorySlot(data) => {
                        Ok(
                            Writer::new(5u8)
                                .write_byte(data.client_id)
                                .write_i16(data.slot_id)
                                .write_i16(data.amount)
                                .write_byte(data.prefix)
                                .write_i16(data.item_id)
                                .finalize(),
                        )
                    }
                    Message::PlayerHealth(data) => {
                        Ok(
                            Writer::new(16u8)
                                .write_byte(data.client_id)
                                .write_i16(data.current)
                                .write_i16(data.maximum)
                                .finalize(),
                        )
                    }
                    Message::PasswordRequest => Ok(Writer::new(37u8).finalize()),
                    Message::InvasionProgress(data) => {
                        Ok(
                            Writer::new(78u8)
                                .write_i32(data.progress)
                                .write_i32(data.progress_max)
                                .write_i8(data.icon)
                                .write_i8(data.progress_wave)
                                .finalize(),
                        )
                    }
                    Message::KillCount(data) => {
                        Ok(
                            Writer::new(83u8)
                                .write_u16(data.id)
                                .write_u32(data.amount)
                                .finalize(),
                        )
                    }
                    Message::PillarsStatus(data) => {
                        Ok(
                            Writer::new(101u8)
                                .write_u16(data.solar)
                                .write_u16(data.vortex)
                                .write_u16(data.nebula)
                                .write_u16(data.stardust)
                                .finalize(),
                        )
                    }
                    Message::PlayerLoadout(data) => {
                        Ok(
                            Writer::new(147u8)
                                .write_byte(data.client_id)
                                .write_byte(data.index)
                                .write_u16(data.hide_accessory)
                                .finalize(),
                        )
                    }
                    Message::Unknown(code, buf) => {
                        Ok(Writer::new(code).write_bytes(buf).finalize())
                    }
                    _ => Err("Unserializable message. Consider using Message::Unknown"),
                }
            }
        }
        impl<'a> From<&'a [u8]> for Message<'a> {
            fn from(buf: &'a [u8]) -> Self {
                let mut mr = Reader::new(buf);
                match mr.read_byte() {
                    1u8 => Self::VersionIdentifier(mr.read_string()),
                    4u8 => {
                        Self::PlayerDetails(PlayerDetails {
                            client_id: mr.read_byte(),
                            skin_variant: mr.read_byte(),
                            hair: mr.read_byte(),
                            name: mr.read_string(),
                            hair_dye: mr.read_byte(),
                            hide_accessory: mr.read_u16(),
                            hide_misc: mr.read_byte(),
                            hair_color: mr.read_rgb(),
                            skin_color: mr.read_rgb(),
                            eye_color: mr.read_rgb(),
                            shirt_color: mr.read_rgb(),
                            undershirt_color: mr.read_rgb(),
                            pants_color: mr.read_rgb(),
                            shoe_color: mr.read_rgb(),
                            flags_1: mr.read_byte(),
                            flags_2: mr.read_byte(),
                            flags_3: mr.read_byte(),
                        })
                    }
                    5u8 => {
                        Self::PlayerInventorySlot(PlayerInventorySlot {
                            client_id: mr.read_byte(),
                            slot_id: mr.read_i16(),
                            amount: mr.read_i16(),
                            prefix: mr.read_byte(),
                            item_id: mr.read_i16(),
                        })
                    }
                    6u8 => Self::WorldRequest,
                    8u8 => {
                        Self::SpawnRequest(SpawnRequest {
                            x: mr.read_i32(),
                            y: mr.read_i32(),
                        })
                    }
                    16u8 => {
                        Self::PlayerHealth(PlayerHealth {
                            client_id: mr.read_byte(),
                            current: mr.read_i16(),
                            maximum: mr.read_i16(),
                        })
                    }
                    38u8 => Self::PasswordResponse(mr.read_string()),
                    42u8 => {
                        Self::PlayerMana(PlayerMana {
                            client_id: mr.read_byte(),
                            current: mr.read_i16(),
                            maximum: mr.read_i16(),
                        })
                    }
                    50u8 => {
                        Self::PlayerBuffs(PlayerBuffs {
                            client_id: mr.read_byte(),
                            buffs: {
                                let mut buf = [0u16; MAX_BUFFS];
                                for num in &mut buf {
                                    *num = u16::from_le_bytes(
                                        mr.read_bytes(2).try_into().unwrap(),
                                    );
                                }
                                buf
                            },
                        })
                    }
                    68u8 => Self::UUID(mr.read_string()),
                    147u8 => {
                        Self::PlayerLoadout(PlayerLoadout {
                            client_id: mr.read_byte(),
                            index: mr.read_byte(),
                            hide_accessory: mr.read_u16(),
                        })
                    }
                    code => Self::Unknown(code, &buf[1..]),
                }
            }
        }
        impl<'a> Message<'a> {
            pub async fn write(
                self,
                mut stream: Pin<&mut impl AsyncWrite>,
            ) -> Result<usize, &str> {
                let buffer: Vec<u8> = self.try_into()?;
                stream.write(&buffer).await.map_err(|_| "Error while writing")
            }
        }
    }
    pub mod server {
        use std::net::SocketAddr;
        use std::pin::Pin;
        use tokio::net::{TcpListener, TcpStream};
        use tokio::io::{Result, AsyncReadExt};
        use tokio::sync::{Mutex, RwLock, broadcast};
        use tokio::select;
        use std::sync::Arc;
        use crate::network::messages::{self, Sanitize, Message, ConnectionApprove};
        use crate::binary::types::{Text, TextMode};
        const GAME_VERSION: &str = "Terraria279";
        const MAX_CLIENTS: usize = 256;
        const MAX_NAME_LEN: usize = 20;
        pub enum ConnectionState {
            New,
            PendingAuthentication,
            Authenticated,
            Complete,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ConnectionState {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ConnectionState {
            #[inline]
            fn eq(&self, other: &ConnectionState) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for ConnectionState {}
        #[automatically_derived]
        impl ::core::cmp::Eq for ConnectionState {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        const MAX_INVENTORY_SLOTS: usize = 350;
        pub struct Client {
            pub addr: SocketAddr,
            pub state: ConnectionState,
            pub uuid: Option<String>,
            pub details: Option<messages::PlayerDetails>,
            pub health: Option<messages::PlayerHealth>,
            pub mana: Option<messages::PlayerMana>,
            pub buffs: Option<messages::PlayerBuffs>,
            pub loadout: Option<messages::PlayerLoadout>,
            pub inventory: Arc<
                Mutex<[Option<messages::PlayerInventorySlot>; MAX_INVENTORY_SLOTS]>,
            >,
        }
        impl Client {
            fn new(addr: SocketAddr) -> Self {
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
                    inventory: Arc::new(
                        Mutex::new([INIT_SLOT_NONE; MAX_INVENTORY_SLOTS]),
                    ),
                }
            }
        }
        pub struct Server {
            pub clients: Arc<Mutex<[Option<Client>; MAX_CLIENTS]>>,
            pub password: RwLock<String>,
            pub broadcast: broadcast::Sender<(Message<'static>, Option<usize>)>,
        }
        impl Server {
            pub fn new(password: &str) -> Server {
                const INIT_CLIENT_NONE: Option<Client> = None;
                let (tx, _) = broadcast::channel(1024);
                Server {
                    password: RwLock::new(password.to_owned()),
                    clients: Arc::new(Mutex::new([INIT_CLIENT_NONE; MAX_CLIENTS])),
                    broadcast: tx,
                }
            }
            pub async fn listen(self, address: &str) -> Result<()> {
                let listener = TcpListener::bind(address).await?;
                let arc = Arc::new(self);
                loop {
                    let (mut stream, addr) = listener.accept().await?;
                    let rc = arc.clone();
                    tokio::spawn(async move { rc.accept(&mut stream, addr).await });
                }
            }
            async fn accept(
                &self,
                stream: &mut TcpStream,
                addr: SocketAddr,
            ) -> Result<()> {
                let (mut rh, mut wh) = stream.split();
                let mut tx = self.broadcast.clone();
                let mut rx = self.broadcast.subscribe();
                let src = {
                    let mut clients = self.clients.lock().await;
                    let Some(id) = clients.iter().position(Option::is_none) else {
                        Message::ConnectionRefuse(
                                Text(
                                    TextMode::LocalizationKey,
                                    "CLI.ServerIsFull".to_owned(),
                                ),
                            )
                            .write(Pin::new(&mut wh))
                            .await
                            .unwrap();
                        return Ok(());
                    };
                    clients[id] = Some(Client::new(addr));
                    id
                };
                loop {
                    let mut length = [0u8; 2];
                    {
                        #[doc(hidden)]
                        mod __tokio_select_util {
                            pub(super) enum Out<_0, _1> {
                                _0(_0),
                                _1(_1),
                                Disabled,
                            }
                            pub(super) type Mask = u8;
                        }
                        use ::tokio::macros::support::Future;
                        use ::tokio::macros::support::Pin;
                        use ::tokio::macros::support::Poll::{Ready, Pending};
                        const BRANCHES: u32 = 2;
                        let mut disabled: __tokio_select_util::Mask = Default::default();
                        if !true {
                            let mask: __tokio_select_util::Mask = 1 << 0;
                            disabled |= mask;
                        }
                        if !true {
                            let mask: __tokio_select_util::Mask = 1 << 1;
                            disabled |= mask;
                        }
                        let mut output = {
                            let mut futures = (rh.read(&mut length), rx.recv());
                            let mut futures = &mut futures;
                            ::tokio::macros::support::poll_fn(|cx| {
                                    let mut is_pending = false;
                                    let start = {
                                        ::tokio::macros::support::thread_rng_n(BRANCHES)
                                    };
                                    for i in 0..BRANCHES {
                                        let branch;
                                        #[allow(clippy::modulo_one)]
                                        {
                                            branch = (start + i) % BRANCHES;
                                        }
                                        match branch {
                                            #[allow(unreachable_code)]
                                            0 => {
                                                let mask = 1 << branch;
                                                if disabled & mask == mask {
                                                    continue;
                                                }
                                                let (fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                let out = match Future::poll(fut, cx) {
                                                    Ready(out) => out,
                                                    Pending => {
                                                        is_pending = true;
                                                        continue;
                                                    }
                                                };
                                                disabled |= mask;
                                                #[allow(unused_variables)] #[allow(unused_mut)]
                                                match &out {
                                                    read_result => {}
                                                    _ => continue,
                                                }
                                                return Ready(__tokio_select_util::Out::_0(out));
                                            }
                                            #[allow(unreachable_code)]
                                            1 => {
                                                let mask = 1 << branch;
                                                if disabled & mask == mask {
                                                    continue;
                                                }
                                                let (_, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                let out = match Future::poll(fut, cx) {
                                                    Ready(out) => out,
                                                    Pending => {
                                                        is_pending = true;
                                                        continue;
                                                    }
                                                };
                                                disabled |= mask;
                                                #[allow(unused_variables)] #[allow(unused_mut)]
                                                match &out {
                                                    content => {}
                                                    _ => continue,
                                                }
                                                return Ready(__tokio_select_util::Out::_1(out));
                                            }
                                            _ => {
                                                ::core::panicking::panic_fmt(
                                                    format_args!(
                                                        "internal error: entered unreachable code: {0}",
                                                        format_args!(
                                                            "reaching this means there probably is an off by one bug",
                                                        ),
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                    if is_pending {
                                        Pending
                                    } else {
                                        Ready(__tokio_select_util::Out::Disabled)
                                    }
                                })
                                .await
                        };
                        match output {
                            __tokio_select_util::Out::_0(read_result) => {
                                read_result?;
                                let length = u16::from_le_bytes(length);
                                if length < 2 {
                                    continue;
                                }
                                let mut buffer = ::alloc::vec::from_elem(
                                    0u8,
                                    length as usize - 2,
                                );
                                rh.read_exact(&mut buffer).await?;
                                if let Some(msg) = self
                                    .handle_message(
                                        Message::from(buffer.as_slice()),
                                        src,
                                        &mut tx,
                                    )
                                    .await?
                                {
                                    msg.write(Pin::new(&mut wh)).await.unwrap();
                                }
                            }
                            __tokio_select_util::Out::_1(content) => {
                                let (content, ignore_id) = content.unwrap();
                                if ignore_id.map_or(true, |id| id != src) {
                                    content.write(Pin::new(&mut wh)).await.unwrap();
                                }
                            }
                            __tokio_select_util::Out::Disabled => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "all branches are disabled and there is no else branch",
                                    ),
                                );
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("failed to match bind"),
                                    ),
                                );
                            }
                        }
                    }
                }
            }
            async fn handle_message(
                &self,
                msg: Message<'_>,
                src: usize,
                tx: &mut broadcast::Sender<(Message<'static>, Option<usize>)>,
            ) -> Result<Option<Message<'static>>> {
                let mut clients = self.clients.lock().await;
                match msg {
                    Message::VersionIdentifier(version) => {
                        if clients[src].as_ref().unwrap().state != ConnectionState::New {
                            return Ok(None);
                        }
                        if version == GAME_VERSION {
                            let password = self.password.read().await;
                            if password.is_empty() {
                                clients[src]
                                    .as_mut()
                                    .unwrap()
                                    .state = ConnectionState::Authenticated;
                                Ok(
                                    Some(
                                        Message::ConnectionApprove(ConnectionApprove {
                                            client_id: src as u8,
                                            flag: false,
                                        }),
                                    ),
                                )
                            } else {
                                clients[src]
                                    .as_mut()
                                    .unwrap()
                                    .state = ConnectionState::PendingAuthentication;
                                Ok(Some(Message::PasswordRequest))
                            }
                        } else {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "Player tried joining with unsupported version {0}\n",
                                        version,
                                    ),
                                );
                            };
                            Ok(
                                Some(
                                    Message::ConnectionRefuse(
                                        Text(
                                            TextMode::LocalizationKey,
                                            "LegacyMultiplayer.4".to_owned(),
                                        ),
                                    ),
                                ),
                            )
                        }
                    }
                    Message::PasswordResponse(pass) => {
                        let password = self.password.read().await;
                        if pass == password.as_str() {
                            clients[src]
                                .as_mut()
                                .unwrap()
                                .state = ConnectionState::Authenticated;
                            Ok(
                                Some(
                                    Message::ConnectionApprove(ConnectionApprove {
                                        client_id: src as u8,
                                        flag: false,
                                    }),
                                ),
                            )
                        } else {
                            Ok(
                                Some(
                                    Message::ConnectionRefuse(
                                        Text(
                                            TextMode::LocalizationKey,
                                            "LegacyMultiplayer.1".to_owned(),
                                        ),
                                    ),
                                ),
                            )
                        }
                    }
                    Message::UUID(uuid) => {
                        clients[src].as_mut().unwrap().uuid = Some(uuid);
                        Ok(None)
                    }
                    Message::PlayerDetails(mut pd) => {
                        if clients[src].as_ref().unwrap().state
                            != ConnectionState::Authenticated
                        {
                            return Ok(
                                Some(
                                    Message::ConnectionRefuse(
                                        Text(
                                            TextMode::LocalizationKey,
                                            "LegacyMultiplayer.1".to_owned(),
                                        ),
                                    ),
                                ),
                            );
                        }
                        if clients[src].as_ref().unwrap().state
                            != ConnectionState::Complete
                        {
                            let exists_same_name = clients
                                .iter()
                                .any(|c_opt| {
                                    c_opt
                                        .as_ref()
                                        .map_or(
                                            false,
                                            |c| c.details.as_ref().map_or(false, |d| d.name == pd.name),
                                        )
                                });
                            if exists_same_name {
                                return Ok(
                                    Some(
                                        Message::ConnectionRefuse(
                                            Text(
                                                TextMode::LocalizationKey,
                                                "LegacyMultiplayer.5".to_owned(),
                                            ),
                                        ),
                                    ),
                                );
                            }
                        }
                        if pd.name.len() > MAX_NAME_LEN {
                            return Ok(
                                Some(
                                    Message::ConnectionRefuse(
                                        Text(
                                            TextMode::LocalizationKey,
                                            "Net.NameTooLong".to_owned(),
                                        ),
                                    ),
                                ),
                            );
                        }
                        if pd.name.is_empty() {
                            return Ok(
                                Some(
                                    Message::ConnectionRefuse(
                                        Text(TextMode::LocalizationKey, "Net.EmptyName".to_owned()),
                                    ),
                                ),
                            );
                        }
                        pd.sanitize(src as u8);
                        tx.send((Message::PlayerDetails(pd.clone()), Some(src)))
                            .unwrap();
                        clients[src].as_mut().unwrap().details = Some(pd);
                        Ok(None)
                    }
                    Message::PlayerHealth(mut ph) => {
                        ph.sanitize(src as u8);
                        tx.send((Message::PlayerHealth(ph.clone()), Some(src))).unwrap();
                        clients[src].as_mut().unwrap().health = Some(ph);
                        Ok(None)
                    }
                    Message::PlayerMana(mut pm) => {
                        pm.sanitize(src as u8);
                        clients[src].as_mut().unwrap().mana = Some(pm);
                        Ok(None)
                    }
                    Message::PlayerBuffs(mut pb) => {
                        pb.sanitize(src as u8);
                        tx.send((Message::PlayerBuffs(pb.clone()), Some(src))).unwrap();
                        clients[src].as_mut().unwrap().buffs = Some(pb);
                        Ok(None)
                    }
                    Message::PlayerLoadout(mut psl) => {
                        psl.sanitize(src as u8);
                        tx.send((Message::PlayerLoadout(psl.clone()), Some(src)))
                            .unwrap();
                        clients[src].as_mut().unwrap().loadout = Some(psl);
                        Ok(None)
                    }
                    Message::PlayerInventorySlot(mut pis) => {
                        pis.sanitize(src as u8);
                        let idx = pis.slot_id as usize;
                        if idx < MAX_INVENTORY_SLOTS {
                            tx.send((
                                    Message::PlayerInventorySlot(pis.clone()),
                                    Some(src),
                                ))
                                .unwrap();
                            clients[src]
                                .as_mut()
                                .unwrap()
                                .inventory
                                .as_ref()
                                .lock()
                                .await[idx] = Some(pis);
                        }
                        Ok(None)
                    }
                    Message::Unknown(code, buf) => {
                        {
                            ::std::io::_print(
                                format_args!("Unknown ({0}): {1:?}\n", code, buf),
                            );
                        };
                        Ok(None)
                    }
                    pkt => {
                        {
                            ::std::io::_print(
                                format_args!("Not yet implemented packet: {0:?}\n", pkt),
                            );
                        };
                        Ok(None)
                    }
                }
            }
        }
    }
    pub mod utils {}
}
mod world {
    pub mod binary {
        use crate::binary::types::{Text, RGB, Vector2};
        use crate::world::WorldParseError;
        pub struct SafeReader {
            buf: Vec<u8>,
            cur: usize,
        }
        type R<T> = Result<T, WorldParseError>;
        #[allow(dead_code)]
        impl SafeReader {
            pub fn new(buf: Vec<u8>) -> Self {
                Self { buf, cur: 0 }
            }
            pub fn seek(&mut self, index: usize) {
                self.cur = index;
            }
            pub fn skip(&mut self, delta: usize) {
                self.cur += delta;
            }
            pub fn get_cur(&self) -> usize {
                self.cur
            }
            pub fn read_bytes(&mut self, amount: usize) -> R<&[u8]> {
                self.cur += amount;
                if self.cur < self.buf.len() {
                    Ok(&self.buf[(self.cur - amount)..self.cur])
                } else {
                    Err(WorldParseError::UnexpectedEOF)
                }
            }
            pub fn read_byte(&mut self) -> R<u8> {
                Ok(self.read_bytes(1)?[0])
            }
            pub fn read_bool(&mut self) -> R<bool> {
                Ok(self.read_byte()? != 0)
            }
            pub fn read_i8(&mut self) -> R<i8> {
                Ok(self.read_byte()? as i8)
            }
            pub fn read_u16(&mut self) -> R<u16> {
                Ok(
                    u16::from_le_bytes(
                        self
                            .read_bytes(2)?
                            .try_into()
                            .map_err(|_| WorldParseError::InvalidNumber)?,
                    ),
                )
            }
            pub fn read_i16(&mut self) -> R<i16> {
                Ok(
                    i16::from_le_bytes(
                        self
                            .read_bytes(2)?
                            .try_into()
                            .map_err(|_| WorldParseError::InvalidNumber)?,
                    ),
                )
            }
            pub fn read_u32(&mut self) -> R<u32> {
                Ok(
                    u32::from_le_bytes(
                        self
                            .read_bytes(4)?
                            .try_into()
                            .map_err(|_| WorldParseError::InvalidNumber)?,
                    ),
                )
            }
            pub fn read_i32(&mut self) -> R<i32> {
                Ok(
                    i32::from_le_bytes(
                        self
                            .read_bytes(4)?
                            .try_into()
                            .map_err(|_| WorldParseError::InvalidNumber)?,
                    ),
                )
            }
            pub fn read_u64(&mut self) -> R<u64> {
                Ok(
                    u64::from_le_bytes(
                        self
                            .read_bytes(8)?
                            .try_into()
                            .map_err(|_| WorldParseError::InvalidNumber)?,
                    ),
                )
            }
            pub fn read_i64(&mut self) -> R<i64> {
                Ok(
                    i64::from_le_bytes(
                        self
                            .read_bytes(8)?
                            .try_into()
                            .map_err(|_| WorldParseError::InvalidNumber)?,
                    ),
                )
            }
            pub fn read_f32(&mut self) -> R<f32> {
                Ok(
                    f32::from_le_bytes(
                        self
                            .read_bytes(4)?
                            .try_into()
                            .map_err(|_| WorldParseError::InvalidNumber)?,
                    ),
                )
            }
            pub fn read_f64(&mut self) -> R<f64> {
                Ok(
                    f64::from_le_bytes(
                        self
                            .read_bytes(8)?
                            .try_into()
                            .map_err(|_| WorldParseError::InvalidNumber)?,
                    ),
                )
            }
            pub fn read_vector2(&mut self) -> R<Vector2> {
                Ok(Vector2(self.read_f32()?, self.read_f32()?))
            }
            pub fn read_length(&mut self) -> R<usize> {
                let mut length = self.read_byte()? as usize;
                let mut shift = 7;
                while length & (1 << shift) != 0 {
                    length &= !(1 << shift);
                    length |= (self.read_byte()? as usize) << shift;
                    shift += 7;
                }
                Ok(length)
            }
            pub fn read_string(&mut self) -> R<String> {
                let length = self.read_length()?;
                Ok(unsafe {
                    std::str::from_utf8_unchecked(self.read_bytes(length)?).to_owned()
                })
            }
            pub fn read_text(&mut self) -> R<Text> {
                Ok(Text(self.read_byte()?.into(), self.read_string()?))
            }
            pub fn read_rgb(&mut self) -> R<RGB> {
                Ok(RGB(self.read_byte()?, self.read_byte()?, self.read_byte()?))
            }
        }
    }
    use std::{
        collections::HashSet, fmt, fs, io, os::windows::fs::MetadataExt, path::Path,
        str::{self, Utf8Error},
    };
    use crate::binary::types::Vector2;
    use self::binary::SafeReader;
    #[repr(u8)]
    pub enum FileType {
        None,
        Map,
        World,
        Player,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FileType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FileType {
        #[inline]
        fn eq(&self, other: &FileType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FileType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    FileType::None => "None",
                    FileType::Map => "Map",
                    FileType::World => "World",
                    FileType::Player => "Player",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FileType {
        #[inline]
        fn clone(&self) -> FileType {
            match self {
                FileType::None => FileType::None,
                FileType::Map => FileType::Map,
                FileType::World => FileType::World,
                FileType::Player => FileType::Player,
            }
        }
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
    #[repr(u8)]
    pub enum GameMode {
        Normal,
        Expert,
        Master,
        Creative,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for GameMode {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for GameMode {
        #[inline]
        fn eq(&self, other: &GameMode) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GameMode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    GameMode::Normal => "Normal",
                    GameMode::Expert => "Expert",
                    GameMode::Master => "Master",
                    GameMode::Creative => "Creative",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GameMode {
        #[inline]
        fn clone(&self) -> GameMode {
            match self {
                GameMode::Normal => GameMode::Normal,
                GameMode::Expert => GameMode::Expert,
                GameMode::Master => GameMode::Master,
                GameMode::Creative => GameMode::Creative,
            }
        }
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
        BadFileSignature,
        ExpectedWorldType,
        InvalidString(Utf8Error),
        PositionCheckFailed(String),
        UnsupportedVersion(i32),
        FSError(io::Error),
    }
    impl fmt::Display for WorldParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::UnexpectedEOF => {
                    f.write_fmt(
                        format_args!("Expected more data but reach end of file."),
                    )
                }
                Self::InvalidNumber => {
                    f.write_fmt(format_args!("Could not parse number"))
                }
                Self::InvalidString(err) => {
                    f.write_fmt(format_args!("Could not parse string, got {0}", err))
                }
                Self::BadFileSignature => {
                    f.write_fmt(
                        format_args!(
                            "Invalid file signature (expecting \"{0}\")",
                            str::from_utf8(MAGIC_STRING).unwrap(),
                        ),
                    )
                }
                Self::ExpectedWorldType => {
                    f.write_fmt(format_args!("Expected file type to be world file"))
                }
                Self::PositionCheckFailed(s) => {
                    f.write_fmt(
                        format_args!(
                            "Position of buffer cursor does not match metadata position for field {0}",
                            s,
                        ),
                    )
                }
                Self::UnsupportedVersion(v) => {
                    f.write_fmt(format_args!("Unsupported file version: {0}", v))
                }
                Self::FSError(err) => f.write_fmt(format_args!("{0}", err)),
            }
        }
    }
    pub struct World {
        pub metadata: Metadata,
        pub format: Format,
        pub header: Header,
        pub tiles: Vec<Vec<Tile>>,
        pub chests: Vec<Chest>,
        pub signs: Vec<Sign>,
        pub npcs: Vec<NPC>,
        pub tile_entities: Vec<TileEntity>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for World {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "metadata",
                "format",
                "header",
                "tiles",
                "chests",
                "signs",
                "npcs",
                "tile_entities",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.metadata,
                &self.format,
                &self.header,
                &self.tiles,
                &self.chests,
                &self.signs,
                &self.npcs,
                &&self.tile_entities,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "World", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for World {
        #[inline]
        fn clone(&self) -> World {
            World {
                metadata: ::core::clone::Clone::clone(&self.metadata),
                format: ::core::clone::Clone::clone(&self.format),
                header: ::core::clone::Clone::clone(&self.header),
                tiles: ::core::clone::Clone::clone(&self.tiles),
                chests: ::core::clone::Clone::clone(&self.chests),
                signs: ::core::clone::Clone::clone(&self.signs),
                npcs: ::core::clone::Clone::clone(&self.npcs),
                tile_entities: ::core::clone::Clone::clone(&self.tile_entities),
            }
        }
    }
    pub struct Metadata {
        pub version: i32,
        pub file_type: FileType,
        pub revision: u32,
        pub favorite: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Metadata {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Metadata",
                "version",
                &self.version,
                "file_type",
                &self.file_type,
                "revision",
                &self.revision,
                "favorite",
                &&self.favorite,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Metadata {
        #[inline]
        fn clone(&self) -> Metadata {
            Metadata {
                version: ::core::clone::Clone::clone(&self.version),
                file_type: ::core::clone::Clone::clone(&self.file_type),
                revision: ::core::clone::Clone::clone(&self.revision),
                favorite: ::core::clone::Clone::clone(&self.favorite),
            }
        }
    }
    const BG_COUNT: usize = 13;
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
    #[automatically_derived]
    impl ::core::fmt::Debug for Header {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "name",
                "seed_text",
                "worldgen_version",
                "uuid",
                "id",
                "left",
                "right",
                "top",
                "bottom",
                "width",
                "height",
                "game_mode",
                "world_drunk",
                "world_for_the_worthy",
                "world_anniversary",
                "world_dont_starve",
                "world_not_the_bees",
                "world_remix",
                "world_no_traps",
                "world_zenith",
                "creation_time",
                "has_crimson",
                "hard_mode",
                "moon_type",
                "tree_x",
                "tree_style",
                "cave_back_x",
                "cave_back_style",
                "ice_back_style",
                "jungle_back_style",
                "hell_back_style",
                "spawn_tile_x",
                "spawn_tile_y",
                "world_surface",
                "rock_layer",
                "temp_time",
                "temp_day_time",
                "temp_moon_phase",
                "temp_blood_moon",
                "temp_eclipse",
                "dungeon_x",
                "dungeon_y",
                "downed_boss_1",
                "downed_boss_2",
                "downed_boss_3",
                "downed_queen_bee",
                "downed_mech_boss_1",
                "downed_mech_boss_2",
                "downed_mech_boss_3",
                "downed_mech_boss_any",
                "downed_plant_boss",
                "downed_golem_boss",
                "downed_slime_king",
                "saved_goblin",
                "saved_wizard",
                "saved_mechanic",
                "downed_goblins",
                "downed_clown",
                "downed_frost",
                "downed_pirates",
                "smashed_shadow_orb",
                "spawn_meteor",
                "shadow_orb_count",
                "altar_count",
                "after_party_of_doom",
                "invasion_delay",
                "invasion_size",
                "invasion_type",
                "invasion_x",
                "slime_rain_time",
                "sundial_cooldown",
                "temp_raining",
                "temp_rain_time",
                "temp_max_rain",
                "ore_tier_cobalt",
                "ore_tier_mythril",
                "ore_tier_adamantite",
                "bg",
                "cloud_bg_active",
                "cloud_bg_alpha",
                "num_clouds",
                "wind_speed_target",
                "angler_who_finished_today",
                "saved_angler",
                "angler_quest",
                "saved_stylist",
                "saved_tax_collector",
                "saved_golfer",
                "invasion_size_start",
                "temp_cultist_delay",
                "npc_kill_counts",
                "fast_forward_time_to_dawn",
                "downed_fishron",
                "downed_martians",
                "downed_ancient_cultist",
                "downed_moonlord",
                "downed_halloween_king",
                "downed_halloween_tree",
                "downed_christmas_ice_queen",
                "downed_christmas_santank",
                "downed_christmas_tree",
                "downed_tower_solar",
                "downed_tower_vortex",
                "downed_tower_nebula",
                "downed_tower_stardust",
                "tower_active_solar",
                "tower_active_vortex",
                "tower_active_nebula",
                "tower_active_stardust",
                "lunar_apocalypse_is_up",
                "temp_party_manual",
                "temp_party_genuine",
                "temp_party_cooldown",
                "temp_party_celebrating_npcs",
                "temp_sandstorm_happening",
                "temp_sandstorm_time_left",
                "temp_sandstorm_severity",
                "temp_sandstorm_intended_severity",
                "saved_bartender",
                "downed_dd2_invasion_t1",
                "downed_dd2_invasion_t2",
                "downed_dd2_invasion_t3",
                "combat_book_was_used",
                "temp_lantern_night_cooldown",
                "temp_lantern_night_genuine",
                "temp_lantern_night_manual",
                "temp_lantern_night_next_night_is_genuine",
                "tree_top_variations",
                "force_halloween_for_today",
                "force_xmas_for_today",
                "ore_tier_copper",
                "ore_tier_iron",
                "ore_tier_silver",
                "ore_tier_gold",
                "bought_cat",
                "bought_dog",
                "bought_bunny",
                "downed_empress_of_light",
                "downed_queen_slime",
                "downed_deerclops",
                "unlocked_slime_blue_spawn",
                "unlocked_merchant_spawn",
                "unlocked_demolition_spawn",
                "unlocked_party_girl_spawn",
                "unlocked_dye_trader_spawn",
                "unlocked_truffle_spawn",
                "unlocked_arms_dealer_spawn",
                "unlocked_nurse_spawn",
                "unlocked_princess_spawn",
                "combat_book_volume_two_was_used",
                "peddlers_satchel_was_use",
                "unlocked_slime_green_spawn",
                "unlocked_slime_old_spawn",
                "unlocked_slime_purple_spawn",
                "unlocked_slime_rainbow_spawn",
                "unlocked_slime_red_spawn",
                "unlocked_slime_yellow_spawn",
                "unlocked_slime_copper_spawn",
                "fast_forward_time_to_dusk",
                "moondial_cooldown",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.name,
                &self.seed_text,
                &self.worldgen_version,
                &self.uuid,
                &self.id,
                &self.left,
                &self.right,
                &self.top,
                &self.bottom,
                &self.width,
                &self.height,
                &self.game_mode,
                &self.world_drunk,
                &self.world_for_the_worthy,
                &self.world_anniversary,
                &self.world_dont_starve,
                &self.world_not_the_bees,
                &self.world_remix,
                &self.world_no_traps,
                &self.world_zenith,
                &self.creation_time,
                &self.has_crimson,
                &self.hard_mode,
                &self.moon_type,
                &self.tree_x,
                &self.tree_style,
                &self.cave_back_x,
                &self.cave_back_style,
                &self.ice_back_style,
                &self.jungle_back_style,
                &self.hell_back_style,
                &self.spawn_tile_x,
                &self.spawn_tile_y,
                &self.world_surface,
                &self.rock_layer,
                &self.temp_time,
                &self.temp_day_time,
                &self.temp_moon_phase,
                &self.temp_blood_moon,
                &self.temp_eclipse,
                &self.dungeon_x,
                &self.dungeon_y,
                &self.downed_boss_1,
                &self.downed_boss_2,
                &self.downed_boss_3,
                &self.downed_queen_bee,
                &self.downed_mech_boss_1,
                &self.downed_mech_boss_2,
                &self.downed_mech_boss_3,
                &self.downed_mech_boss_any,
                &self.downed_plant_boss,
                &self.downed_golem_boss,
                &self.downed_slime_king,
                &self.saved_goblin,
                &self.saved_wizard,
                &self.saved_mechanic,
                &self.downed_goblins,
                &self.downed_clown,
                &self.downed_frost,
                &self.downed_pirates,
                &self.smashed_shadow_orb,
                &self.spawn_meteor,
                &self.shadow_orb_count,
                &self.altar_count,
                &self.after_party_of_doom,
                &self.invasion_delay,
                &self.invasion_size,
                &self.invasion_type,
                &self.invasion_x,
                &self.slime_rain_time,
                &self.sundial_cooldown,
                &self.temp_raining,
                &self.temp_rain_time,
                &self.temp_max_rain,
                &self.ore_tier_cobalt,
                &self.ore_tier_mythril,
                &self.ore_tier_adamantite,
                &self.bg,
                &self.cloud_bg_active,
                &self.cloud_bg_alpha,
                &self.num_clouds,
                &self.wind_speed_target,
                &self.angler_who_finished_today,
                &self.saved_angler,
                &self.angler_quest,
                &self.saved_stylist,
                &self.saved_tax_collector,
                &self.saved_golfer,
                &self.invasion_size_start,
                &self.temp_cultist_delay,
                &self.npc_kill_counts,
                &self.fast_forward_time_to_dawn,
                &self.downed_fishron,
                &self.downed_martians,
                &self.downed_ancient_cultist,
                &self.downed_moonlord,
                &self.downed_halloween_king,
                &self.downed_halloween_tree,
                &self.downed_christmas_ice_queen,
                &self.downed_christmas_santank,
                &self.downed_christmas_tree,
                &self.downed_tower_solar,
                &self.downed_tower_vortex,
                &self.downed_tower_nebula,
                &self.downed_tower_stardust,
                &self.tower_active_solar,
                &self.tower_active_vortex,
                &self.tower_active_nebula,
                &self.tower_active_stardust,
                &self.lunar_apocalypse_is_up,
                &self.temp_party_manual,
                &self.temp_party_genuine,
                &self.temp_party_cooldown,
                &self.temp_party_celebrating_npcs,
                &self.temp_sandstorm_happening,
                &self.temp_sandstorm_time_left,
                &self.temp_sandstorm_severity,
                &self.temp_sandstorm_intended_severity,
                &self.saved_bartender,
                &self.downed_dd2_invasion_t1,
                &self.downed_dd2_invasion_t2,
                &self.downed_dd2_invasion_t3,
                &self.combat_book_was_used,
                &self.temp_lantern_night_cooldown,
                &self.temp_lantern_night_genuine,
                &self.temp_lantern_night_manual,
                &self.temp_lantern_night_next_night_is_genuine,
                &self.tree_top_variations,
                &self.force_halloween_for_today,
                &self.force_xmas_for_today,
                &self.ore_tier_copper,
                &self.ore_tier_iron,
                &self.ore_tier_silver,
                &self.ore_tier_gold,
                &self.bought_cat,
                &self.bought_dog,
                &self.bought_bunny,
                &self.downed_empress_of_light,
                &self.downed_queen_slime,
                &self.downed_deerclops,
                &self.unlocked_slime_blue_spawn,
                &self.unlocked_merchant_spawn,
                &self.unlocked_demolition_spawn,
                &self.unlocked_party_girl_spawn,
                &self.unlocked_dye_trader_spawn,
                &self.unlocked_truffle_spawn,
                &self.unlocked_arms_dealer_spawn,
                &self.unlocked_nurse_spawn,
                &self.unlocked_princess_spawn,
                &self.combat_book_volume_two_was_used,
                &self.peddlers_satchel_was_use,
                &self.unlocked_slime_green_spawn,
                &self.unlocked_slime_old_spawn,
                &self.unlocked_slime_purple_spawn,
                &self.unlocked_slime_rainbow_spawn,
                &self.unlocked_slime_red_spawn,
                &self.unlocked_slime_yellow_spawn,
                &self.unlocked_slime_copper_spawn,
                &self.fast_forward_time_to_dusk,
                &&self.moondial_cooldown,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Header",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Header {
        #[inline]
        fn clone(&self) -> Header {
            Header {
                name: ::core::clone::Clone::clone(&self.name),
                seed_text: ::core::clone::Clone::clone(&self.seed_text),
                worldgen_version: ::core::clone::Clone::clone(&self.worldgen_version),
                uuid: ::core::clone::Clone::clone(&self.uuid),
                id: ::core::clone::Clone::clone(&self.id),
                left: ::core::clone::Clone::clone(&self.left),
                right: ::core::clone::Clone::clone(&self.right),
                top: ::core::clone::Clone::clone(&self.top),
                bottom: ::core::clone::Clone::clone(&self.bottom),
                width: ::core::clone::Clone::clone(&self.width),
                height: ::core::clone::Clone::clone(&self.height),
                game_mode: ::core::clone::Clone::clone(&self.game_mode),
                world_drunk: ::core::clone::Clone::clone(&self.world_drunk),
                world_for_the_worthy: ::core::clone::Clone::clone(
                    &self.world_for_the_worthy,
                ),
                world_anniversary: ::core::clone::Clone::clone(&self.world_anniversary),
                world_dont_starve: ::core::clone::Clone::clone(&self.world_dont_starve),
                world_not_the_bees: ::core::clone::Clone::clone(
                    &self.world_not_the_bees,
                ),
                world_remix: ::core::clone::Clone::clone(&self.world_remix),
                world_no_traps: ::core::clone::Clone::clone(&self.world_no_traps),
                world_zenith: ::core::clone::Clone::clone(&self.world_zenith),
                creation_time: ::core::clone::Clone::clone(&self.creation_time),
                has_crimson: ::core::clone::Clone::clone(&self.has_crimson),
                hard_mode: ::core::clone::Clone::clone(&self.hard_mode),
                moon_type: ::core::clone::Clone::clone(&self.moon_type),
                tree_x: ::core::clone::Clone::clone(&self.tree_x),
                tree_style: ::core::clone::Clone::clone(&self.tree_style),
                cave_back_x: ::core::clone::Clone::clone(&self.cave_back_x),
                cave_back_style: ::core::clone::Clone::clone(&self.cave_back_style),
                ice_back_style: ::core::clone::Clone::clone(&self.ice_back_style),
                jungle_back_style: ::core::clone::Clone::clone(&self.jungle_back_style),
                hell_back_style: ::core::clone::Clone::clone(&self.hell_back_style),
                spawn_tile_x: ::core::clone::Clone::clone(&self.spawn_tile_x),
                spawn_tile_y: ::core::clone::Clone::clone(&self.spawn_tile_y),
                world_surface: ::core::clone::Clone::clone(&self.world_surface),
                rock_layer: ::core::clone::Clone::clone(&self.rock_layer),
                temp_time: ::core::clone::Clone::clone(&self.temp_time),
                temp_day_time: ::core::clone::Clone::clone(&self.temp_day_time),
                temp_moon_phase: ::core::clone::Clone::clone(&self.temp_moon_phase),
                temp_blood_moon: ::core::clone::Clone::clone(&self.temp_blood_moon),
                temp_eclipse: ::core::clone::Clone::clone(&self.temp_eclipse),
                dungeon_x: ::core::clone::Clone::clone(&self.dungeon_x),
                dungeon_y: ::core::clone::Clone::clone(&self.dungeon_y),
                downed_boss_1: ::core::clone::Clone::clone(&self.downed_boss_1),
                downed_boss_2: ::core::clone::Clone::clone(&self.downed_boss_2),
                downed_boss_3: ::core::clone::Clone::clone(&self.downed_boss_3),
                downed_queen_bee: ::core::clone::Clone::clone(&self.downed_queen_bee),
                downed_mech_boss_1: ::core::clone::Clone::clone(
                    &self.downed_mech_boss_1,
                ),
                downed_mech_boss_2: ::core::clone::Clone::clone(
                    &self.downed_mech_boss_2,
                ),
                downed_mech_boss_3: ::core::clone::Clone::clone(
                    &self.downed_mech_boss_3,
                ),
                downed_mech_boss_any: ::core::clone::Clone::clone(
                    &self.downed_mech_boss_any,
                ),
                downed_plant_boss: ::core::clone::Clone::clone(&self.downed_plant_boss),
                downed_golem_boss: ::core::clone::Clone::clone(&self.downed_golem_boss),
                downed_slime_king: ::core::clone::Clone::clone(&self.downed_slime_king),
                saved_goblin: ::core::clone::Clone::clone(&self.saved_goblin),
                saved_wizard: ::core::clone::Clone::clone(&self.saved_wizard),
                saved_mechanic: ::core::clone::Clone::clone(&self.saved_mechanic),
                downed_goblins: ::core::clone::Clone::clone(&self.downed_goblins),
                downed_clown: ::core::clone::Clone::clone(&self.downed_clown),
                downed_frost: ::core::clone::Clone::clone(&self.downed_frost),
                downed_pirates: ::core::clone::Clone::clone(&self.downed_pirates),
                smashed_shadow_orb: ::core::clone::Clone::clone(
                    &self.smashed_shadow_orb,
                ),
                spawn_meteor: ::core::clone::Clone::clone(&self.spawn_meteor),
                shadow_orb_count: ::core::clone::Clone::clone(&self.shadow_orb_count),
                altar_count: ::core::clone::Clone::clone(&self.altar_count),
                after_party_of_doom: ::core::clone::Clone::clone(
                    &self.after_party_of_doom,
                ),
                invasion_delay: ::core::clone::Clone::clone(&self.invasion_delay),
                invasion_size: ::core::clone::Clone::clone(&self.invasion_size),
                invasion_type: ::core::clone::Clone::clone(&self.invasion_type),
                invasion_x: ::core::clone::Clone::clone(&self.invasion_x),
                slime_rain_time: ::core::clone::Clone::clone(&self.slime_rain_time),
                sundial_cooldown: ::core::clone::Clone::clone(&self.sundial_cooldown),
                temp_raining: ::core::clone::Clone::clone(&self.temp_raining),
                temp_rain_time: ::core::clone::Clone::clone(&self.temp_rain_time),
                temp_max_rain: ::core::clone::Clone::clone(&self.temp_max_rain),
                ore_tier_cobalt: ::core::clone::Clone::clone(&self.ore_tier_cobalt),
                ore_tier_mythril: ::core::clone::Clone::clone(&self.ore_tier_mythril),
                ore_tier_adamantite: ::core::clone::Clone::clone(
                    &self.ore_tier_adamantite,
                ),
                bg: ::core::clone::Clone::clone(&self.bg),
                cloud_bg_active: ::core::clone::Clone::clone(&self.cloud_bg_active),
                cloud_bg_alpha: ::core::clone::Clone::clone(&self.cloud_bg_alpha),
                num_clouds: ::core::clone::Clone::clone(&self.num_clouds),
                wind_speed_target: ::core::clone::Clone::clone(&self.wind_speed_target),
                angler_who_finished_today: ::core::clone::Clone::clone(
                    &self.angler_who_finished_today,
                ),
                saved_angler: ::core::clone::Clone::clone(&self.saved_angler),
                angler_quest: ::core::clone::Clone::clone(&self.angler_quest),
                saved_stylist: ::core::clone::Clone::clone(&self.saved_stylist),
                saved_tax_collector: ::core::clone::Clone::clone(
                    &self.saved_tax_collector,
                ),
                saved_golfer: ::core::clone::Clone::clone(&self.saved_golfer),
                invasion_size_start: ::core::clone::Clone::clone(
                    &self.invasion_size_start,
                ),
                temp_cultist_delay: ::core::clone::Clone::clone(
                    &self.temp_cultist_delay,
                ),
                npc_kill_counts: ::core::clone::Clone::clone(&self.npc_kill_counts),
                fast_forward_time_to_dawn: ::core::clone::Clone::clone(
                    &self.fast_forward_time_to_dawn,
                ),
                downed_fishron: ::core::clone::Clone::clone(&self.downed_fishron),
                downed_martians: ::core::clone::Clone::clone(&self.downed_martians),
                downed_ancient_cultist: ::core::clone::Clone::clone(
                    &self.downed_ancient_cultist,
                ),
                downed_moonlord: ::core::clone::Clone::clone(&self.downed_moonlord),
                downed_halloween_king: ::core::clone::Clone::clone(
                    &self.downed_halloween_king,
                ),
                downed_halloween_tree: ::core::clone::Clone::clone(
                    &self.downed_halloween_tree,
                ),
                downed_christmas_ice_queen: ::core::clone::Clone::clone(
                    &self.downed_christmas_ice_queen,
                ),
                downed_christmas_santank: ::core::clone::Clone::clone(
                    &self.downed_christmas_santank,
                ),
                downed_christmas_tree: ::core::clone::Clone::clone(
                    &self.downed_christmas_tree,
                ),
                downed_tower_solar: ::core::clone::Clone::clone(
                    &self.downed_tower_solar,
                ),
                downed_tower_vortex: ::core::clone::Clone::clone(
                    &self.downed_tower_vortex,
                ),
                downed_tower_nebula: ::core::clone::Clone::clone(
                    &self.downed_tower_nebula,
                ),
                downed_tower_stardust: ::core::clone::Clone::clone(
                    &self.downed_tower_stardust,
                ),
                tower_active_solar: ::core::clone::Clone::clone(
                    &self.tower_active_solar,
                ),
                tower_active_vortex: ::core::clone::Clone::clone(
                    &self.tower_active_vortex,
                ),
                tower_active_nebula: ::core::clone::Clone::clone(
                    &self.tower_active_nebula,
                ),
                tower_active_stardust: ::core::clone::Clone::clone(
                    &self.tower_active_stardust,
                ),
                lunar_apocalypse_is_up: ::core::clone::Clone::clone(
                    &self.lunar_apocalypse_is_up,
                ),
                temp_party_manual: ::core::clone::Clone::clone(&self.temp_party_manual),
                temp_party_genuine: ::core::clone::Clone::clone(
                    &self.temp_party_genuine,
                ),
                temp_party_cooldown: ::core::clone::Clone::clone(
                    &self.temp_party_cooldown,
                ),
                temp_party_celebrating_npcs: ::core::clone::Clone::clone(
                    &self.temp_party_celebrating_npcs,
                ),
                temp_sandstorm_happening: ::core::clone::Clone::clone(
                    &self.temp_sandstorm_happening,
                ),
                temp_sandstorm_time_left: ::core::clone::Clone::clone(
                    &self.temp_sandstorm_time_left,
                ),
                temp_sandstorm_severity: ::core::clone::Clone::clone(
                    &self.temp_sandstorm_severity,
                ),
                temp_sandstorm_intended_severity: ::core::clone::Clone::clone(
                    &self.temp_sandstorm_intended_severity,
                ),
                saved_bartender: ::core::clone::Clone::clone(&self.saved_bartender),
                downed_dd2_invasion_t1: ::core::clone::Clone::clone(
                    &self.downed_dd2_invasion_t1,
                ),
                downed_dd2_invasion_t2: ::core::clone::Clone::clone(
                    &self.downed_dd2_invasion_t2,
                ),
                downed_dd2_invasion_t3: ::core::clone::Clone::clone(
                    &self.downed_dd2_invasion_t3,
                ),
                combat_book_was_used: ::core::clone::Clone::clone(
                    &self.combat_book_was_used,
                ),
                temp_lantern_night_cooldown: ::core::clone::Clone::clone(
                    &self.temp_lantern_night_cooldown,
                ),
                temp_lantern_night_genuine: ::core::clone::Clone::clone(
                    &self.temp_lantern_night_genuine,
                ),
                temp_lantern_night_manual: ::core::clone::Clone::clone(
                    &self.temp_lantern_night_manual,
                ),
                temp_lantern_night_next_night_is_genuine: ::core::clone::Clone::clone(
                    &self.temp_lantern_night_next_night_is_genuine,
                ),
                tree_top_variations: ::core::clone::Clone::clone(
                    &self.tree_top_variations,
                ),
                force_halloween_for_today: ::core::clone::Clone::clone(
                    &self.force_halloween_for_today,
                ),
                force_xmas_for_today: ::core::clone::Clone::clone(
                    &self.force_xmas_for_today,
                ),
                ore_tier_copper: ::core::clone::Clone::clone(&self.ore_tier_copper),
                ore_tier_iron: ::core::clone::Clone::clone(&self.ore_tier_iron),
                ore_tier_silver: ::core::clone::Clone::clone(&self.ore_tier_silver),
                ore_tier_gold: ::core::clone::Clone::clone(&self.ore_tier_gold),
                bought_cat: ::core::clone::Clone::clone(&self.bought_cat),
                bought_dog: ::core::clone::Clone::clone(&self.bought_dog),
                bought_bunny: ::core::clone::Clone::clone(&self.bought_bunny),
                downed_empress_of_light: ::core::clone::Clone::clone(
                    &self.downed_empress_of_light,
                ),
                downed_queen_slime: ::core::clone::Clone::clone(
                    &self.downed_queen_slime,
                ),
                downed_deerclops: ::core::clone::Clone::clone(&self.downed_deerclops),
                unlocked_slime_blue_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_slime_blue_spawn,
                ),
                unlocked_merchant_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_merchant_spawn,
                ),
                unlocked_demolition_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_demolition_spawn,
                ),
                unlocked_party_girl_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_party_girl_spawn,
                ),
                unlocked_dye_trader_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_dye_trader_spawn,
                ),
                unlocked_truffle_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_truffle_spawn,
                ),
                unlocked_arms_dealer_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_arms_dealer_spawn,
                ),
                unlocked_nurse_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_nurse_spawn,
                ),
                unlocked_princess_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_princess_spawn,
                ),
                combat_book_volume_two_was_used: ::core::clone::Clone::clone(
                    &self.combat_book_volume_two_was_used,
                ),
                peddlers_satchel_was_use: ::core::clone::Clone::clone(
                    &self.peddlers_satchel_was_use,
                ),
                unlocked_slime_green_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_slime_green_spawn,
                ),
                unlocked_slime_old_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_slime_old_spawn,
                ),
                unlocked_slime_purple_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_slime_purple_spawn,
                ),
                unlocked_slime_rainbow_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_slime_rainbow_spawn,
                ),
                unlocked_slime_red_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_slime_red_spawn,
                ),
                unlocked_slime_yellow_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_slime_yellow_spawn,
                ),
                unlocked_slime_copper_spawn: ::core::clone::Clone::clone(
                    &self.unlocked_slime_copper_spawn,
                ),
                fast_forward_time_to_dusk: ::core::clone::Clone::clone(
                    &self.fast_forward_time_to_dusk,
                ),
                moondial_cooldown: ::core::clone::Clone::clone(&self.moondial_cooldown),
            }
        }
    }
    pub struct Format {
        pub importance: Vec<bool>,
        pub positions: Vec<i32>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Format {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Format",
                "importance",
                &self.importance,
                "positions",
                &&self.positions,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Format {
        #[inline]
        fn clone(&self) -> Format {
            Format {
                importance: ::core::clone::Clone::clone(&self.importance),
                positions: ::core::clone::Clone::clone(&self.positions),
            }
        }
    }
    const WALL_COUNT: u16 = 347;
    pub struct Tile {
        pub header: [u8; 4],
        pub id: i16,
        pub active: bool,
        pub frame_x: i16,
        pub frame_y: i16,
        pub color: u8,
        pub wall: u16,
        pub wall_color: u16,
        pub liquid: u8,
        pub liquid_header: u8,
        pub wire_1: bool,
        pub wire_2: bool,
        pub wire_3: bool,
        pub wire_4: bool,
        pub actuator: bool,
        pub in_active: bool,
        pub invisible_block: bool,
        pub invisible_wall: bool,
        pub fullbright_block: bool,
        pub fullbright_wall: bool,
        pub half_brick: bool,
        pub slope: u8,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Tile {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "header",
                "id",
                "active",
                "frame_x",
                "frame_y",
                "color",
                "wall",
                "wall_color",
                "liquid",
                "liquid_header",
                "wire_1",
                "wire_2",
                "wire_3",
                "wire_4",
                "actuator",
                "in_active",
                "invisible_block",
                "invisible_wall",
                "fullbright_block",
                "fullbright_wall",
                "half_brick",
                "slope",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.header,
                &self.id,
                &self.active,
                &self.frame_x,
                &self.frame_y,
                &self.color,
                &self.wall,
                &self.wall_color,
                &self.liquid,
                &self.liquid_header,
                &self.wire_1,
                &self.wire_2,
                &self.wire_3,
                &self.wire_4,
                &self.actuator,
                &self.in_active,
                &self.invisible_block,
                &self.invisible_wall,
                &self.fullbright_block,
                &self.fullbright_wall,
                &self.half_brick,
                &&self.slope,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Tile", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Tile {
        #[inline]
        fn clone(&self) -> Tile {
            Tile {
                header: ::core::clone::Clone::clone(&self.header),
                id: ::core::clone::Clone::clone(&self.id),
                active: ::core::clone::Clone::clone(&self.active),
                frame_x: ::core::clone::Clone::clone(&self.frame_x),
                frame_y: ::core::clone::Clone::clone(&self.frame_y),
                color: ::core::clone::Clone::clone(&self.color),
                wall: ::core::clone::Clone::clone(&self.wall),
                wall_color: ::core::clone::Clone::clone(&self.wall_color),
                liquid: ::core::clone::Clone::clone(&self.liquid),
                liquid_header: ::core::clone::Clone::clone(&self.liquid_header),
                wire_1: ::core::clone::Clone::clone(&self.wire_1),
                wire_2: ::core::clone::Clone::clone(&self.wire_2),
                wire_3: ::core::clone::Clone::clone(&self.wire_3),
                wire_4: ::core::clone::Clone::clone(&self.wire_4),
                actuator: ::core::clone::Clone::clone(&self.actuator),
                in_active: ::core::clone::Clone::clone(&self.in_active),
                invisible_block: ::core::clone::Clone::clone(&self.invisible_block),
                invisible_wall: ::core::clone::Clone::clone(&self.invisible_wall),
                fullbright_block: ::core::clone::Clone::clone(&self.fullbright_block),
                fullbright_wall: ::core::clone::Clone::clone(&self.fullbright_wall),
                half_brick: ::core::clone::Clone::clone(&self.half_brick),
                slope: ::core::clone::Clone::clone(&self.slope),
            }
        }
    }
    pub struct Chest {
        pub x: i32,
        pub y: i32,
        pub name: String,
        pub items: Vec<ChestItem>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Chest {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Chest",
                "x",
                &self.x,
                "y",
                &self.y,
                "name",
                &self.name,
                "items",
                &&self.items,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Chest {
        #[inline]
        fn clone(&self) -> Chest {
            Chest {
                x: ::core::clone::Clone::clone(&self.x),
                y: ::core::clone::Clone::clone(&self.y),
                name: ::core::clone::Clone::clone(&self.name),
                items: ::core::clone::Clone::clone(&self.items),
            }
        }
    }
    pub struct ChestItem {
        pub id: i32,
        pub stack: i16,
        pub prefix: u8,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ChestItem {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ChestItem",
                "id",
                &self.id,
                "stack",
                &self.stack,
                "prefix",
                &&self.prefix,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ChestItem {
        #[inline]
        fn clone(&self) -> ChestItem {
            ChestItem {
                id: ::core::clone::Clone::clone(&self.id),
                stack: ::core::clone::Clone::clone(&self.stack),
                prefix: ::core::clone::Clone::clone(&self.prefix),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for ChestItem {
        #[inline]
        fn default() -> ChestItem {
            ChestItem {
                id: ::core::default::Default::default(),
                stack: ::core::default::Default::default(),
                prefix: ::core::default::Default::default(),
            }
        }
    }
    pub struct Sign {
        pub x: i32,
        pub y: i32,
        pub text: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Sign {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Sign",
                "x",
                &self.x,
                "y",
                &self.y,
                "text",
                &&self.text,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Sign {
        #[inline]
        fn clone(&self) -> Sign {
            Sign {
                x: ::core::clone::Clone::clone(&self.x),
                y: ::core::clone::Clone::clone(&self.y),
                text: ::core::clone::Clone::clone(&self.text),
            }
        }
    }
    pub struct NPC {
        pub id: i32,
        pub name: String,
        pub x: f32,
        pub y: f32,
        pub homeless: bool,
        pub shimmer: bool,
        pub home_x: i32,
        pub home_y: i32,
        pub variation: i32,
        pub position: Option<Vector2>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for NPC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "name",
                "x",
                "y",
                "homeless",
                "shimmer",
                "home_x",
                "home_y",
                "variation",
                "position",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.name,
                &self.x,
                &self.y,
                &self.homeless,
                &self.shimmer,
                &self.home_x,
                &self.home_y,
                &self.variation,
                &&self.position,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "NPC", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NPC {
        #[inline]
        fn clone(&self) -> NPC {
            NPC {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                x: ::core::clone::Clone::clone(&self.x),
                y: ::core::clone::Clone::clone(&self.y),
                homeless: ::core::clone::Clone::clone(&self.homeless),
                shimmer: ::core::clone::Clone::clone(&self.shimmer),
                home_x: ::core::clone::Clone::clone(&self.home_x),
                home_y: ::core::clone::Clone::clone(&self.home_y),
                variation: ::core::clone::Clone::clone(&self.variation),
                position: ::core::clone::Clone::clone(&self.position),
            }
        }
    }
    pub struct TileEntity {
        pub id: i32,
        pub x: i16,
        pub y: i16,
        pub entity: TileEntityExtra,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TileEntity {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "TileEntity",
                "id",
                &self.id,
                "x",
                &self.x,
                "y",
                &self.y,
                "entity",
                &&self.entity,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TileEntity {
        #[inline]
        fn clone(&self) -> TileEntity {
            TileEntity {
                id: ::core::clone::Clone::clone(&self.id),
                x: ::core::clone::Clone::clone(&self.x),
                y: ::core::clone::Clone::clone(&self.y),
                entity: ::core::clone::Clone::clone(&self.entity),
            }
        }
    }
    impl World {
        pub fn from_file(path: &Path) -> Result<World, WorldParseError> {
            let contents = fs::read(path).map_err(WorldParseError::FSError)?;
            let mut reader = SafeReader::new(contents);
            let mut world = Self::from_reader(&mut reader)?;
            if world.metadata.version < 141 {
                let file_metadata = fs::metadata(path)
                    .map_err(WorldParseError::FSError)?;
                world.header.creation_time = file_metadata.creation_time() as i64;
            }
            Ok(world)
        }
        pub fn from_reader(r: &mut SafeReader) -> Result<World, WorldParseError> {
            let version = r.read_i32()?;
            if version >= 88 {
                Self::read_world_v2(r)
            } else {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "not yet implemented: {0}",
                            format_args!("implement v1 before release 88"),
                        ),
                    );
                };
            }
        }
        pub fn read_world_v2(r: &mut SafeReader) -> Result<World, WorldParseError> {
            r.seek(0);
            let metadata = Self::read_metadata(r)?;
            let format = Self::read_format(r)?;
            if r.get_cur() != format.positions[0] as usize {
                return Err(WorldParseError::PositionCheckFailed("format".to_owned()));
            }
            let header = Self::read_header(r, &metadata)?;
            if r.get_cur() != format.positions[1] as usize {
                return Err(WorldParseError::PositionCheckFailed("header".to_owned()));
            }
            let tiles = Self::read_tiles(r, &format, &header)?;
            if r.get_cur() != format.positions[2] as usize {
                return Err(WorldParseError::PositionCheckFailed("tiles".to_owned()));
            }
            let chests = Self::read_chests(r)?;
            if r.get_cur() != format.positions[3] as usize {
                return Err(WorldParseError::PositionCheckFailed("chests".to_owned()));
            }
            let signs = Self::read_signs(r, &tiles)?;
            if r.get_cur() != format.positions[4] as usize {
                return Err(WorldParseError::PositionCheckFailed("signs".to_owned()));
            }
            let npcs = Self::read_npcs(r, &metadata)?;
            if r.get_cur() != format.positions[5] as usize {
                return Err(WorldParseError::PositionCheckFailed("npcs".to_owned()));
            }
            let version = metadata.version;
            let tile_entities = if version >= 116 {
                let te = if version >= 122 {
                    Self::read_tile_entities(r)?
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "not yet implemented: {0}",
                                format_args!(
                                    "implement WorldFile.LoadDummies for older versions",
                                ),
                            ),
                        );
                    }
                };
                (
                    match r.get_cur() {
                        tmp => {
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "[{0}:{1}] {2} = {3:#?}\n",
                                        "src\\world\\mod.rs",
                                        408u32,
                                        "r.get_cur()",
                                        &tmp,
                                    ),
                                );
                            };
                            tmp
                        }
                    },
                    match format.positions[6] {
                        tmp => {
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "[{0}:{1}] {2} = {3:#?}\n",
                                        "src\\world\\mod.rs",
                                        408u32,
                                        "format.positions[6]",
                                        &tmp,
                                    ),
                                );
                            };
                            tmp
                        }
                    },
                );
                if r.get_cur() != format.positions[6] as usize {
                    return Err(
                        WorldParseError::PositionCheckFailed("tile_entities".to_owned()),
                    );
                }
                te
            } else {
                ::alloc::vec::Vec::new()
            };
            Ok(World {
                metadata,
                format,
                header,
                tiles,
                chests,
                signs,
                npcs,
                tile_entities,
            })
        }
        pub fn read_metadata(r: &mut SafeReader) -> Result<Metadata, WorldParseError> {
            let version = r.read_i32()?;
            let (file_type, revision, favorite) = if version >= 135 {
                let magic = r.read_bytes(7)?;
                if magic != MAGIC_STRING {
                    return Err(WorldParseError::BadFileSignature);
                }
                (FileType::from(r.read_byte()?), r.read_u32()?, (r.read_u64()? & 1) == 1)
            } else {
                (FileType::World, 0, false)
            };
            if file_type != FileType::World {
                return Err(WorldParseError::ExpectedWorldType);
            }
            if version > 279 {
                return Err(WorldParseError::UnsupportedVersion(version));
            }
            Ok(Metadata {
                version,
                file_type,
                revision,
                favorite,
            })
        }
        pub fn read_format(r: &mut SafeReader) -> Result<Format, WorldParseError> {
            let mut positions = ::alloc::vec::from_elem(0, r.read_i16()? as usize);
            for p in &mut positions {
                *p = r.read_i32()?;
            }
            let mut importance = ::alloc::vec::from_elem(false, r.read_u16()? as usize);
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
            Ok(Format { positions, importance })
        }
        pub fn read_header(
            r: &mut SafeReader,
            metadata: &Metadata,
        ) -> Result<Header, WorldParseError> {
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
            let world_zenith = if version >= 267 {
                r.read_bool()?
            } else {
                world_drunk && world_remix
            };
            let creation_time = if version >= 141 { r.read_i64()? } else { 0 };
            let moon_type = r.read_byte()? as i32;
            let tree_x = [r.read_i32()?, r.read_i32()?, r.read_i32()?];
            let tree_style = [
                r.read_i32()?,
                r.read_i32()?,
                r.read_i32()?,
                r.read_i32()?,
            ];
            let cave_back_x = [r.read_i32()?, r.read_i32()?, r.read_i32()?];
            let cave_back_style = [
                r.read_i32()?,
                r.read_i32()?,
                r.read_i32()?,
                r.read_i32()?,
            ];
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
            let invasion_x = r.read_f64()?;
            let slime_rain_time = if version >= 118 { r.read_f64()? } else { 0. };
            let sundial_cooldown = if version >= 113 {
                r.read_byte()? as i32
            } else {
                0
            };
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
            let num_clouds = r.read_i16()?;
            let wind_speed_target = r.read_f32()?;
            let angler_who_finished_today = if version >= 95 {
                let mut v = Vec::with_capacity(r.read_i32()? as usize);
                for _ in 0..v.capacity() {
                    v.push(r.read_string()?)
                }
                v
            } else {
                ::alloc::vec::Vec::new()
            };
            let saved_angler = version >= 99 && r.read_bool()?;
            let angler_quest = if version >= 101 { r.read_i32()? } else { 0 };
            let saved_stylist = version >= 104 && r.read_bool()?;
            let saved_tax_collector = version >= 129 && r.read_bool()?;
            let saved_golfer = version >= 201 && r.read_bool()?;
            let invasion_size_start = if version >= 107 { r.read_i32()? } else { 0 };
            let temp_cultist_delay = if version >= 108 { r.read_i32()? } else { 86400 };
            let npc_kill_counts = if version >= 109 {
                let mut kc = Vec::with_capacity(r.read_i16()? as usize);
                for _ in 0..kc.capacity() {
                    kc.push(r.read_i32()?)
                }
                kc
            } else {
                ::alloc::vec::Vec::new()
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
            let temp_party_manual = version >= 170 && r.read_bool()?;
            let temp_party_genuine = version >= 170 && r.read_bool()?;
            let temp_party_cooldown = if version >= 170 { r.read_i32()? } else { 0 };
            let temp_party_celebrating_npcs = if version >= 170 {
                let mut npcs = Vec::with_capacity(r.read_i32()? as usize);
                for _ in 0..npcs.capacity() {
                    npcs.push(r.read_i32()?)
                }
                npcs
            } else {
                ::alloc::vec::Vec::new()
            };
            let temp_sandstorm_happening = version >= 174 && r.read_bool()?;
            let temp_sandstorm_time_left = if version >= 174 {
                r.read_i32()?
            } else {
                0
            };
            let temp_sandstorm_severity = if version >= 174 {
                r.read_f32()?
            } else {
                0.
            };
            let temp_sandstorm_intended_severity = if version >= 174 {
                r.read_f32()?
            } else {
                0.
            };
            let saved_bartender = version >= 178 && r.read_bool()?;
            let downed_dd2_invasion_t1 = version >= 178 && r.read_bool()?;
            let downed_dd2_invasion_t2 = version >= 178 && r.read_bool()?;
            let downed_dd2_invasion_t3 = version >= 178 && r.read_bool()?;
            if version >= 193 {
                bg[8] = r.read_byte()?;
            }
            if version >= 215 {
                bg[9] = r.read_byte()?;
            }
            if version >= 194 {
                bg[10..].copy_from_slice(r.read_bytes(3)?)
            }
            let combat_book_was_used = version >= 204 && r.read_bool()?;
            let temp_lantern_night_cooldown = if version >= 207 {
                r.read_i32()?
            } else {
                0
            };
            let temp_lantern_night_genuine = version >= 207 && r.read_bool()?;
            let temp_lantern_night_manual = version >= 207 && r.read_bool()?;
            let temp_lantern_night_next_night_is_genuine = version >= 207
                && r.read_bool()?;
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
        pub fn read_tiles(
            r: &mut SafeReader,
            format: &Format,
            header: &Header,
        ) -> Result<Vec<Vec<Tile>>, WorldParseError> {
            let mut map = Vec::with_capacity(header.width as usize);
            for _ in 0..header.width {
                let mut column = Vec::with_capacity(header.height as usize);
                let mut y = 0;
                while y < header.height {
                    let b_1 = r.read_byte()?;
                    let b_2 = if b_1 & 1 == 1 { r.read_byte()? } else { 0 };
                    let b_3 = if b_2 & 1 == 1 { r.read_byte()? } else { 0 };
                    let b_4 = if b_3 & 1 == 1 { r.read_byte()? } else { 0 };
                    let (active, id, frame_x, frame_y, color) = if b_1 & 2 == 2 {
                        let id = if b_1 & 32 == 32 {
                            r.read_i16()?
                        } else {
                            r.read_byte()? as i16
                        };
                        let (x, y) = if format.importance[id as usize] {
                            let x = r.read_i16()?;
                            let y = r.read_i16()?;
                            (x, if id == 144 { 0 } else { y })
                        } else {
                            (-1, -1)
                        };
                        let col = if b_3 & 8 == 8 { r.read_byte()? } else { 0 };
                        (true, id, x, y, col)
                    } else {
                        (false, -1, 0, 0, 0)
                    };
                    let (wall, wall_color) = if b_1 & 4 == 4 {
                        (
                            r.read_byte()? as u16,
                            if b_3 & 16 == 16 { r.read_byte()? as u16 } else { 0 },
                        )
                    } else {
                        (0, 0)
                    };
                    let liquid_bits = (b_1 & 0b11000) >> 3;
                    let (liquid, liquid_header) = if liquid_bits != 0 {
                        let liquid_header = r.read_byte()?;
                        (
                            if b_3 & 128 == 128 {
                                3
                            } else {
                                match liquid_bits {
                                    2 => 1,
                                    3 => 2,
                                    _ => 0,
                                }
                            },
                            liquid_header,
                        )
                    } else {
                        (0, 0)
                    };
                    let (wire_1, wire_2, wire_3, half_brick, slope) = if b_2 > 1 {
                        let n_9 = (b_2 & 0b1110000) >> 4;
                        let (hb, sl) = if n_9 != 0 {
                            (n_9 == 1, n_9 - 1)
                        } else {
                            (false, 0)
                        };
                        (b_2 & 2 == 2, b_2 & 4 == 4, b_2 & 8 == 8, hb, sl)
                    } else {
                        (false, false, false, false, 0)
                    };
                    let (actuator, in_active, wire_4, wall) = if b_3 > 1 {
                        let wall_extended = if b_3 & 64 == 64 {
                            let new_wall = (r.read_byte()? as u16) << 8 | wall;
                            if new_wall < WALL_COUNT { new_wall } else { 0 }
                        } else {
                            wall
                        };
                        (b_3 & 2 == 2, b_3 & 4 == 4, b_3 & 32 == 32, wall_extended)
                    } else {
                        (false, false, false, wall)
                    };
                    let (
                        invisible_block,
                        invisible_wall,
                        fullbright_block,
                        fullbright_wall,
                    ) = if b_4 > 1 {
                        (b_4 & 2 == 2, b_4 & 4 == 4, b_4 & 8 == 8, b_4 & 16 == 16)
                    } else {
                        (false, false, false, false)
                    };
                    let tile = Tile {
                        header: [b_1, b_2, b_3, b_4],
                        id,
                        active,
                        frame_x,
                        frame_y,
                        color,
                        wall,
                        wall_color,
                        liquid,
                        liquid_header,
                        wire_1,
                        wire_2,
                        wire_3,
                        wire_4,
                        actuator,
                        in_active,
                        invisible_block,
                        invisible_wall,
                        fullbright_block,
                        fullbright_wall,
                        half_brick,
                        slope,
                    };
                    let repeat = match (b_1 & 0b11000000) >> 6 {
                        0 => 0,
                        1 => r.read_byte()? as i32,
                        _ => r.read_i16()? as i32,
                    };
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
        pub fn read_chests(r: &mut SafeReader) -> Result<Vec<Chest>, WorldParseError> {
            let mut chests = Vec::with_capacity(r.read_i16()? as usize);
            let n_2 = r.read_i16()?;
            let n_3 = if n_2 < 40 { n_2 } else { 40 };
            let n_4 = if n_2 < 40 { 0 } else { n_2 - 40 };
            for _ in 0..chests.capacity() {
                let x = r.read_i32()?;
                let y = r.read_i32()?;
                let name = r.read_string()?;
                let mut items = ::alloc::vec::Vec::new();
                for _ in 0..n_3 {
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
            r: &mut SafeReader,
            tiles: &[Vec<Tile>],
        ) -> Result<Vec<Sign>, WorldParseError> {
            let mut signs = Vec::with_capacity(r.read_i16()? as usize);
            for _ in 0..signs.capacity() {
                let text = r.read_string()?;
                let x = r.read_i32()?;
                let y = r.read_i32()?;
                let t = &tiles[x as usize][y as usize];
                if t.active && (t.id == 55 || t.id == 85 || t.id == 425 || t.id == 573) {
                    signs.push(Sign { x, y, text })
                }
            }
            Ok(signs)
        }
        pub fn read_npcs(
            r: &mut SafeReader,
            metadata: &Metadata,
        ) -> Result<Vec<NPC>, WorldParseError> {
            let version = metadata.version;
            let mut shimmers = HashSet::new();
            if version >= 268 {
                for _ in 0..r.read_i32()? {
                    shimmers.insert(r.read_i32()?);
                }
            }
            let mut npcs = ::alloc::vec::Vec::new();
            while r.read_bool()? {
                let id = if version >= 190 {
                    r.read_i32()?
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "not yet implemented: {0}",
                                format_args!(
                                    "implement NPCID.FromLegacyName(reader.ReadString())",
                                ),
                            ),
                        );
                    }
                };
                let name = r.read_string()?;
                let x = r.read_f32()?;
                let y = r.read_f32()?;
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
                    x,
                    y,
                    homeless,
                    home_x,
                    home_y,
                    variation,
                    shimmer: shimmers.contains(&id),
                    position: None,
                })
            }
            if version >= 140 {
                let mut iter = npcs.iter_mut();
                while r.read_bool()? {
                    let Some(npc) = iter.next() else { break };
                    if version >= 190 {
                        npc.id = r.read_i32()?;
                    } else {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!(
                                        "implement NPCID.FromLegacyName(reader.ReadString())",
                                    ),
                                ),
                            );
                        }
                    }
                    npc.position = Some(r.read_vector2()?);
                }
            }
            Ok(npcs)
        }
        pub fn read_tile_entities(
            r: &mut SafeReader,
        ) -> Result<Vec<TileEntity>, WorldParseError> {
            let mut tile_entities = Vec::with_capacity(r.read_i32()? as usize);
            for _ in 0..tile_entities.capacity() {
                let entity_type = r.read_byte()?;
                let id = r.read_i32()?;
                let x = r.read_i16()?;
                let y = r.read_i16()?;
                let entity = match entity_type {
                    0 => {
                        TileEntityExtra::Dummy {
                            npc: r.read_i16()?,
                        }
                    }
                    1 => {
                        TileEntityExtra::ItemFrame {
                            id: r.read_i16()?,
                            prefix: r.read_byte()?,
                            stack: r.read_i16()?,
                        }
                    }
                    2 => {
                        TileEntityExtra::LogicSensor {
                            logic_check: r.read_byte()?,
                            on: r.read_bool()?,
                        }
                    }
                    _ => {
                        continue;
                    }
                };
                tile_entities.push(TileEntity { id, x, y, entity })
            }
            Ok(tile_entities)
        }
    }
    pub enum TileEntityExtra {
        Dummy { npc: i16 },
        ItemFrame { id: i16, stack: i16, prefix: u8 },
        LogicSensor { logic_check: u8, on: bool },
        DisplayDoll {},
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TileEntityExtra {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                TileEntityExtra::Dummy { npc: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Dummy",
                        "npc",
                        &__self_0,
                    )
                }
                TileEntityExtra::ItemFrame {
                    id: __self_0,
                    stack: __self_1,
                    prefix: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ItemFrame",
                        "id",
                        __self_0,
                        "stack",
                        __self_1,
                        "prefix",
                        &__self_2,
                    )
                }
                TileEntityExtra::LogicSensor { logic_check: __self_0, on: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "LogicSensor",
                        "logic_check",
                        __self_0,
                        "on",
                        &__self_1,
                    )
                }
                TileEntityExtra::DisplayDoll {} => {
                    ::core::fmt::Formatter::write_str(f, "DisplayDoll")
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TileEntityExtra {
        #[inline]
        fn clone(&self) -> TileEntityExtra {
            match self {
                TileEntityExtra::Dummy { npc: __self_0 } => {
                    TileEntityExtra::Dummy {
                        npc: ::core::clone::Clone::clone(__self_0),
                    }
                }
                TileEntityExtra::ItemFrame {
                    id: __self_0,
                    stack: __self_1,
                    prefix: __self_2,
                } => {
                    TileEntityExtra::ItemFrame {
                        id: ::core::clone::Clone::clone(__self_0),
                        stack: ::core::clone::Clone::clone(__self_1),
                        prefix: ::core::clone::Clone::clone(__self_2),
                    }
                }
                TileEntityExtra::LogicSensor { logic_check: __self_0, on: __self_1 } => {
                    TileEntityExtra::LogicSensor {
                        logic_check: ::core::clone::Clone::clone(__self_0),
                        on: ::core::clone::Clone::clone(__self_1),
                    }
                }
                TileEntityExtra::DisplayDoll {} => TileEntityExtra::DisplayDoll {},
            }
        }
    }
}
use network::server::Server;
use directories::UserDirs;
use std::{fs, path::Path};
use world::World;
fn main() {
    let body = async {
        match World::from_file(
            Path::new(
                "C:\\Users\\Dim\\Documents\\My Games\\Terraria\\Worlds\\shimmer.wld",
            ),
        ) {
            Ok(w) => {
                match &w.npcs {
                    tmp => {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "[{0}:{1}] {2} = {3:#?}\n",
                                    "src\\main.rs",
                                    31u32,
                                    "&w.npcs",
                                    &tmp,
                                ),
                            );
                        };
                        tmp
                    }
                };
            }
            Err(e) => {
                {
                    ::std::io::_print(format_args!("Parse Error: {0}\n", e));
                };
            }
        };
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
