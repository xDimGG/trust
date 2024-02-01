#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
mod messages {
    use proc_macros::message_read_write;
    use tokio::net::TcpStream;
    use tokio::prelude::*;
    use std::convert::{TryFrom, TryInto};
    use std::str;
    pub struct RGB(u8, u8, u8);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for RGB {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                RGB(ref __self_0_0, ref __self_0_1, ref __self_0_2) => {
                    let mut debug_trait_builder = f.debug_tuple("RGB");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    let _ = debug_trait_builder.field(&&(*__self_0_1));
                    let _ = debug_trait_builder.field(&&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub struct PlayerAppearance {
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
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PlayerAppearance {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                PlayerAppearance {
                    id: ref __self_0_0,
                    skin_variant: ref __self_0_1,
                    hair: ref __self_0_2,
                    name: ref __self_0_3,
                    hair_dye: ref __self_0_4,
                    hide_accessory: ref __self_0_5,
                    hide_misc: ref __self_0_6,
                    hair_color: ref __self_0_7,
                    skin_color: ref __self_0_8,
                    eye_color: ref __self_0_9,
                    shirt_color: ref __self_0_10,
                    undershirt_color: ref __self_0_11,
                    pants_color: ref __self_0_12,
                    shoe_color: ref __self_0_13,
                    difficulty: ref __self_0_14,
                    extra_accessory: ref __self_0_15,
                } => {
                    let mut debug_trait_builder = f.debug_struct("PlayerAppearance");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("skin_variant", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("hair", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("name", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("hair_dye", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("hide_accessory", &&(*__self_0_5));
                    let _ = debug_trait_builder.field("hide_misc", &&(*__self_0_6));
                    let _ = debug_trait_builder.field("hair_color", &&(*__self_0_7));
                    let _ = debug_trait_builder.field("skin_color", &&(*__self_0_8));
                    let _ = debug_trait_builder.field("eye_color", &&(*__self_0_9));
                    let _ = debug_trait_builder.field("shirt_color", &&(*__self_0_10));
                    let _ = debug_trait_builder.field("undershirt_color", &&(*__self_0_11));
                    let _ = debug_trait_builder.field("pants_color", &&(*__self_0_12));
                    let _ = debug_trait_builder.field("shoe_color", &&(*__self_0_13));
                    let _ = debug_trait_builder.field("difficulty", &&(*__self_0_14));
                    let _ = debug_trait_builder.field("extra_accessory", &&(*__self_0_15));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub struct PlayerInventorySlot {
        client_id: u8,
        slot_id: u16,
        amount: u16,
        prefix: u8,
        item_id: u16,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PlayerInventorySlot {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                PlayerInventorySlot {
                    client_id: ref __self_0_0,
                    slot_id: ref __self_0_1,
                    amount: ref __self_0_2,
                    prefix: ref __self_0_3,
                    item_id: ref __self_0_4,
                } => {
                    let mut debug_trait_builder = f.debug_struct("PlayerInventorySlot");
                    let _ = debug_trait_builder.field("client_id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("slot_id", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("amount", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("prefix", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("item_id", &&(*__self_0_4));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub struct SpawnRequest {
        x: i32,
        y: i32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SpawnRequest {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SpawnRequest {
                    x: ref __self_0_0,
                    y: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("SpawnRequest");
                    let _ = debug_trait_builder.field("x", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("y", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub struct PlayerHealth {
        client_id: u8,
        current: u16,
        maximum: u16,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PlayerHealth {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                PlayerHealth {
                    client_id: ref __self_0_0,
                    current: ref __self_0_1,
                    maximum: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("PlayerHealth");
                    let _ = debug_trait_builder.field("client_id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("current", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("maximum", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub struct PlayerMana {
        client_id: u8,
        current: u16,
        maximum: u16,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PlayerMana {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                PlayerMana {
                    client_id: ref __self_0_0,
                    current: ref __self_0_1,
                    maximum: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("PlayerMana");
                    let _ = debug_trait_builder.field("client_id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("current", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("maximum", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub struct PlayerBuffs {
        client_id: u8,
        buffs: [u16; 22],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PlayerBuffs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                PlayerBuffs {
                    client_id: ref __self_0_0,
                    buffs: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("PlayerBuffs");
                    let _ = debug_trait_builder.field("client_id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("buffs", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub struct KillCount {
        id: u16,
        amount: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for KillCount {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                KillCount {
                    id: ref __self_0_0,
                    amount: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("KillCount");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("amount", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub struct PillarsStatus {
        solar: u16,
        vortex: u16,
        nebula: u16,
        stardust: u16,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PillarsStatus {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                PillarsStatus {
                    solar: ref __self_0_0,
                    vortex: ref __self_0_1,
                    nebula: ref __self_0_2,
                    stardust: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("PillarsStatus");
                    let _ = debug_trait_builder.field("solar", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("vortex", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("nebula", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("stardust", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub enum Message<'a> {
        /// $01 ->
        VersionIdentifier(String),
        /// $02 <-
        ConnectionRefuse(String),
        /// $03 <-
        ConnectionApprove,
        PlayerAppearance(PlayerAppearance),
        PlayerInventorySlot(PlayerInventorySlot),
        /// $06 ->
        WorldRequest,
        SpawnRequest(SpawnRequest),
        PlayerHealth(PlayerHealth),
        /// $25 <-
        PasswordRequest,
        /// $26 ->
        PasswordResponse(String),
        PlayerMana(PlayerMana),
        PlayerBuffs(PlayerBuffs),
        /// $44 ->
        UUID(String),
        KillCount(KillCount),
        PillarsStatus(PillarsStatus),
        /// $00 <->
        Unknown(u8, &'a [u8]),
    }
    impl<'a> From<&'a [u8]> for Message<'a> {
        fn from(buf: &'a [u8]) -> Self {
            let mut mr = MessageReader::new(buf, 0);
            match mr.read_byte() {
                1u8 => Self::VersionIdentifier(mr.read_string()),
                4u8 => Self::PlayerAppearance(PlayerAppearance {
                    id: mr.read_byte(),
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
                    difficulty: mr.read_byte(),
                    extra_accessory: mr.read_byte(),
                }),
                5u8 => Self::PlayerInventorySlot(PlayerInventorySlot {
                    client_id: mr.read_byte(),
                    slot_id: mr.read_u16(),
                    amount: mr.read_u16(),
                    prefix: mr.read_byte(),
                    item_id: mr.read_u16(),
                }),
                6u8 => Self::WorldRequest,
                8u8 => Self::SpawnRequest(SpawnRequest {
                    x: mr.read_i32(),
                    y: mr.read_i32(),
                }),
                16u8 => Self::PlayerHealth(PlayerHealth {
                    client_id: mr.read_byte(),
                    current: mr.read_u16(),
                    maximum: mr.read_u16(),
                }),
                38u8 => Self::PasswordResponse(mr.read_string()),
                42u8 => Self::PlayerMana(PlayerMana {
                    client_id: mr.read_byte(),
                    current: mr.read_u16(),
                    maximum: mr.read_u16(),
                }),
                50u8 => Self::PlayerBuffs(PlayerBuffs {
                    client_id: mr.read_byte(),
                    buffs: {
                        let mut buf = [0u16; 22];
                        for num in &mut buf {
                            *num = u16::from_le_bytes(mr.take(2).try_into().unwrap())
                        }
                        buf
                    },
                }),
                68u8 => Self::UUID(mr.read_string()),
                code => Self::Unknown(code, &buf[1..]),
            }
        }
    }
    impl<'a> TryFrom<Message<'a>> for Vec<u8> {
        type Error = &'static str;
        fn try_from(msg: Message) -> Result<Self, Self::Error> {
            match msg {
                Message::ConnectionRefuse(field0) => {
                    Ok(MessageWriter::new(2u8).write_string(field0).finalize())
                }
                Message::ConnectionApprove => Ok(MessageWriter::new(3u8).finalize()),
                Message::PasswordRequest => Ok(MessageWriter::new(37u8).finalize()),
                Message::KillCount(data) => Ok(MessageWriter::new(83u8)
                    .write_u16(data.id)
                    .write_u32(data.amount)
                    .finalize()),
                Message::PillarsStatus(data) => Ok(MessageWriter::new(101u8)
                    .write_u16(data.solar)
                    .write_u16(data.vortex)
                    .write_u16(data.nebula)
                    .write_u16(data.stardust)
                    .finalize()),
                Message::Unknown(code, buf) => {
                    Ok(MessageWriter::new(code).write_bytes(buf).finalize())
                }
                _ => Err("Unserializable message. Consider using Message::Unknown"),
            }
        }
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
            stream
                .write(&buffer)
                .await
                .map_err(|_| "Error while writing")
        }
    }
    struct MessageWriter {
        buf: Vec<u8>,
    }
    impl<'a> MessageWriter {
        fn new(code: u8) -> Self {
            Self {
                buf: <[_]>::into_vec(box [0, 0, code]),
            }
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
}
mod server {
    use tokio::net::{TcpListener, TcpStream};
    use tokio::prelude::*;
    use tokio::io;
    use tokio::sync::{Mutex, RwLock};
    use std::sync::Arc;
    use crate::messages::Message;
    const GAME_VERSION: &'static str = "Terraria230";
    pub struct Server {
        password: RwLock<String>,
        listener: Mutex<TcpListener>,
    }
    impl Server {
        pub async fn new(address: &str, password: &str) -> io::Result<Server> {
            let listener = Mutex::new(TcpListener::bind(address).await?);
            Ok(Server {
                password: RwLock::new(password.to_owned()),
                listener,
            })
        }
        pub async fn start(self: Arc<Self>) -> io::Result<()> {
            loop {
                let srv = Arc::clone(&self);
                let (stream, _) = srv.listener.lock().await.accept().await?;
                tokio::spawn(async move { srv.accept(stream).await });
            }
        }
        async fn accept(&self, mut stream: TcpStream) -> io::Result<()> {
            loop {
                let mut length = [0u8; 2];
                stream.read(&mut length).await?;
                let length = u16::from_le_bytes(length);
                if length < 2 {
                    continue;
                }
                let mut buffer = ::alloc::vec::from_elem(0u8, length as usize - 2);
                stream.read(&mut buffer).await?;
                match Message::from(buffer.as_slice()) {
                    Message::VersionIdentifier(version) => {
                        if version == GAME_VERSION {
                            let password = self.password.read().await;
                            if password.is_empty() {
                                Message::ConnectionApprove.write(&mut stream).await.unwrap();
                            } else {
                                Message::PasswordRequest.write(&mut stream).await.unwrap();
                            }
                        } else {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["Player tried joining with version ", "\n"],
                                    &match (&version,) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    },
                                ));
                            };
                            Message::ConnectionRefuse("LegacyMultiplayer.4".to_owned())
                                .write(&mut stream)
                                .await
                                .unwrap();
                        }
                    }
                    Message::PlayerAppearance(pa) => {
                        match pa {
                            tmp => {
                                {
                                    ::std::io::_eprint(::core::fmt::Arguments::new_v1_formatted(
                                        &["[", ":", "] ", " = ", "\n"],
                                        &match (&"src\\server.rs", &65u32, &"pa", &&tmp) {
                                            (arg0, arg1, arg2, arg3) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg3,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                            ],
                                        },
                                        &[
                                            ::core::fmt::rt::v1::Argument {
                                                position: 0usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 1usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 2usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 3usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 4u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                        ],
                                    ));
                                };
                                tmp
                            }
                        };
                    }
                    Message::PlayerInventorySlot(pis) => {
                        match pis {
                            tmp => {
                                {
                                    ::std::io::_eprint(::core::fmt::Arguments::new_v1_formatted(
                                        &["[", ":", "] ", " = ", "\n"],
                                        &match (&"src\\server.rs", &66u32, &"pis", &&tmp) {
                                            (arg0, arg1, arg2, arg3) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg3,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                            ],
                                        },
                                        &[
                                            ::core::fmt::rt::v1::Argument {
                                                position: 0usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 1usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 2usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 3usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 4u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                        ],
                                    ));
                                };
                                tmp
                            }
                        };
                    }
                    Message::WorldRequest => {
                        Message :: Unknown ( 0x07 , b"\xb2\x6a\x00\x00\x00\x00\x68\x10\xb0\x04\x33\x08\xef\x00\x50\x01\xb0\x01\x37\xd4\x43\x51\x05trust\x03\xad\x39\xad\x7f\x7e\x13\x3f\x46\x9f\x72\x8d\xcc\xca\x4c\xc0\xd7\x01\x00\x00\x00\xe4\x00\x00\x00\x06\x07\x0a\x08\x01\x01\x05\x05\x01\x05\x03\x04\x02\x00\x02\x01\x01\x13\x83\x40\xbd\x00\x94\x06\x00\x00\x68\x10\x00\x00\x68\x10\x00\x00\x00\x02\x00\x00\x34\x04\x00\x00\x68\x10\x00\x00\x68\x10\x00\x00\x02\x05\x03\x07\x00\x02\x00\x00\x01\x05\x05\x01\x05\x03\x04\x02\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x07\x00\xa7\x00\x09\x00\x08\x00\xff\xff\xff\xff\xff\xff\x00\x00\x00\x00\x00\x00\x00\x00\x00\x46\xe7\x19\x3e" ) . write ( & mut stream ) . await . unwrap ( ) ;
                    }
                    Message::PasswordResponse(pass) => {
                        let password = self.password.read().await;
                        if pass == password.as_str() {
                            Message::ConnectionApprove.write(&mut stream).await.unwrap();
                        } else {
                            Message::ConnectionRefuse("LegacyMultiplayer.1".to_owned())
                                .write(&mut stream)
                                .await
                                .unwrap();
                        }
                    }
                    Message::PlayerHealth(ph) => {
                        match ph {
                            tmp => {
                                {
                                    ::std::io::_eprint(::core::fmt::Arguments::new_v1_formatted(
                                        &["[", ":", "] ", " = ", "\n"],
                                        &match (&"src\\server.rs", &98u32, &"ph", &&tmp) {
                                            (arg0, arg1, arg2, arg3) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg3,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                            ],
                                        },
                                        &[
                                            ::core::fmt::rt::v1::Argument {
                                                position: 0usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 1usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 2usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 3usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 4u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                        ],
                                    ));
                                };
                                tmp
                            }
                        };
                    }
                    Message::UUID(uuid) => {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["Got UUID: ", "\n"],
                            &match (&uuid,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                    }
                    Message::PlayerMana(pm) => {
                        match pm {
                            tmp => {
                                {
                                    ::std::io::_eprint(::core::fmt::Arguments::new_v1_formatted(
                                        &["[", ":", "] ", " = ", "\n"],
                                        &match (&"src\\server.rs", &100u32, &"pm", &&tmp) {
                                            (arg0, arg1, arg2, arg3) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg3,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                            ],
                                        },
                                        &[
                                            ::core::fmt::rt::v1::Argument {
                                                position: 0usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 1usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 2usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 3usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 4u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                        ],
                                    ));
                                };
                                tmp
                            }
                        };
                    }
                    Message::PlayerBuffs(pb) => {
                        match pb {
                            tmp => {
                                {
                                    ::std::io::_eprint(::core::fmt::Arguments::new_v1_formatted(
                                        &["[", ":", "] ", " = ", "\n"],
                                        &match (&"src\\server.rs", &101u32, &"pb", &&tmp) {
                                            (arg0, arg1, arg2, arg3) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg3,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                            ],
                                        },
                                        &[
                                            ::core::fmt::rt::v1::Argument {
                                                position: 0usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 1usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 2usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 3usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 4u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                        ],
                                    ));
                                };
                                tmp
                            }
                        };
                    }
                    Message::Unknown(code, buf) => {
                        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                            &["Unknown (", "): ", "\n"],
                            &match (&code, &buf) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::LowerHex::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 1usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                        ));
                    }
                    _ => {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["Got unknown packet: ", "\n"],
                            &match (&buffer,) {
                                (arg0,) => {
                                    [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                                }
                            },
                        ));
                    }
                }
            }
        }
    }
}
use std::sync::Arc;
fn main() {
    tokio::runtime::Builder::new()
        .basic_scheduler()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            {
                let srv = server::Server::new("127.0.0.1:7777", "").await.unwrap();
                Arc::new(srv).start().await.unwrap();
            }
        })
}
