// TODO:
// - Explicit CP437 Bytes/String parser
// - Replace position from float to i32 int for BlockAction, BlockLine

macro_rules! msgs {
    (
        #doc = {
            $( #[doc = $enum_doc:literal] )+
        }
        $(
            $(
                #[doc = $msg_doc:literal]
            )*
            $id:literal : $name:ident $( @ $bound:tt $size:literal )? ;
        )+
    ) => {
        $( #[doc = $enum_doc] )+
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum MessageKind {
            $(
                $( #[doc = $msg_doc] )*
                $name
            ),+
        }

        impl MessageKind {
            pub const fn id(&self) -> u8 {
                match self {
                    $(
                        Self::$name => $id
                    ),+
                }
            }

            pub const fn name(&self) -> &'static str {
                match self {
                    $(
                        Self::$name => stringify!($name)
                    ),+
                }
            }

            pub const fn size(&self) -> MessageSize {
                match self {
                    $(
                        Self::$name => msgs!( _len $( $bound $size )? )
                    ),+
                }
            }
        }
    };

    ( _len > $len:literal ) => {
        MessageSize::Minimum($len + 1)
    };

    ( _len = $len:literal ) => {
        MessageSize::Exact($len)
    };

    ( _len ) => {
        MessageSize::Unknown
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageSize {
    Exact(usize),
    Minimum(usize),
    Unknown,
}

// TODO: How to handle versions (have modules with all valid messages for a version?)
msgs! {
    #doc = {
        /// All accepted message types for the protocol.
        ///
        /// # NOTE
        /// Some messages use the same id for different versions of the protocol (e.g.
        /// MapStart75/MapStart76).
        ///
        /// Source: <https://www.piqueserver.org/aosprotocol/protocol075.html>
    }

    /// This packet is used to set the players position.
    ///
    /// Direction: `Client <-> Server`
     0: PositionData @ = 13;

    /// This packet is used to set the players orientation.
    ///
    /// Direction: `Server -> Client`
     1: OrientationData @ = 13;

    /// Updates position and orientation of all players.
    /// Always sends data for 32 players, with empty slots being all 0 (position: \[0,0,0\], orientation: \[0,0,0\]).
    ///
    /// Direction: `Server -> Client`
     2: WorldUpdate75 @ = 769;

    /// Updates position and orientation of all players.
    /// Unlike 0.75, this only sends information for the necessary players.
    ///
    /// Direction: `Server -> Client`
     2: WorldUpdate76 @ > 0;

    /// Contains the key-states of a player, packed into a byte.
    ///
    /// Direction: `Client <-> Server`
     3: InputData @ = 3;

    /// Contains the weapon input state(?).
    ///
    /// Direction: `Client <-> Server`
     4: WeaponInput @ = 3;

    /// Sent by the client when a hit is registered.
    /// The server should verify that this is possible to prevent abuse (such as hitting without shooting, facing the wrong way, etc).
    ///
    /// Direction: `Client -> Server`
     5: HitPacket @ = 3;

    /// Sent to the client when hurt.
    ///
    /// Direction: `Server -> Client`
     5: SetHP @ = 15;

    /// Spawns a grenade with the given information.
    ///
    /// Direction: `Client -> Server`?
     6: GrenadePacket @ = 30;

    /// Sets a player’s currently equipped tool/weapon.
    ///
    /// Direction: `Client -> Server`?
     7: SetTool @ = 3;

    /// Set the color of a player’s held block.
    ///
    /// Direction: `?`
     8: SetColor @ = 5;

    /// Set player’s team, weapon, etc.
    ///
    /// Direction: `Client <-> Server`
     9: ExisitingPlayer @ > 10;

    /// Like Existing Player, but with less information.
    ///
    /// Direction: `Client <-> Server`
    10: ShortPlayerData @ = 4;

    /// This packet is used to move various game objects like tents, intels and even grenades.
    /// When moving grenades in TC mode the voxlap client has a bug that changes grenades’ models to small tents.
    ///
    /// Direction: `?`
    11: MoveObject @ = 15;

    /// Send on respawn of a player.
    ///
    /// Direction: `Server -> Client`?
    12: CreatePlayer @ > 14;

    /// Sent when a block is placed/destroyed.
    ///
    /// Direction: `Client <-> Server`?
    13: BlockAction @ = 15;

    /// Create a line of blocks between 2 points.
    /// The block color is defined by the Set Color packet.
    ///
    /// Direction: `?`
    14: BlockLine @ = 26;

    // ?
    //
    // Direction: `Server -> Client`
    //
    // # NOTE (TODO)
    // This message has no id.
    // This packet is not a complete packet, as it is only sent after the initial data, where the gamemode is sent.
    // It could be considered as part of that initial data packet, but as what’s sent varies greatly depending on the gamemode, it is documented separately.
    //00: CTFState @ = 52;

    // ?
    //
    // Direction: `Server -> Client`
    //
    // # NOTE (TODO)
    // This message has no id.
    // This packet is not a complete packet, as it is only sent after the initial data, where the gamemode is sent.
    // It could be considered as part of that initial data packet, but as what’s sent varies greatly depending on the gamemode, it is documented separately.
    //00: TCState;

    /// Indicates that the map transfer is complete.
    /// Also informs the client of numerous game parameters.
    /// Be aware that CTFState or TCState may be appended to the packet after the gamemode id portion.
    ///
    /// Direction: `Server -> Client`
    15: StateData @ = 52;

    /// Notify the client of a player’s death.
    ///
    /// Direction: `Server -> Client`
    16: KillAction @ = 5;

    /// Reasonable limits must placed on length and frequency of chat messages.
    ///
    /// Direction: `Client <-> Server`
    17: ChatMessage @  > 1;

    /// Sent when a client connects, or a map is advanced for already existing connections.
    /// Should be the first packet received when a client connects.
    ///
    /// Direction: `Server -> Client`
    18: MapStart75 @ = 5;

    /// Sent when a client connects, or a map is advanced for already existing connections.
    /// Should be the first packet received when a client connects.
    ///
    /// Direction: `Server -> Client`
    18: MapStart76 @ > 7;

    /// Sent just after Map Start, repeatedly until the entire map is sent.
    /// Should always be the next sequence of packets after a Map Start packet.
    ///
    /// Direction: `Server -> Client`
    19: MapChunk;

    /// Sent when a player disconnects.
    ///
    /// Direction: `Server -> Client`
    20: PlayerLeft @ = 2;

    /// Sent when a player captures a Command Post in Territory Control mode.
    /// Captures have affects on the client.
    ///
    /// Direction: `Server -> Client`
    21: TerritoryCapture @ = 5;

    /// Display the TC progress bar.
    ///
    /// Direction: `Server -> Client`
    22: ProgressBar @ = 8;

    /// Sent when a player captures the intel, which is determined by the server.
    /// Winning captures have affects on the client.
    ///
    /// Direction: `Server -> Client`
    23: IntelCapture @ = 3;

    /// Sent when a player collects the intel, which is determined by the server.
    ///
    /// Direction: `Server -> Client`
    24: IntelPickup @ = 2;

    /// Sent when a player dropped the intel.
    /// This will update the intel position on the client.
    ///
    /// Direction: `Server -> Client`
    25: IntelDrop @ = 14;

    /// Id of the player who has been restocked.
    ///
    /// Direction: `Server -> Client`
    26: Restock @ = 2;

    /// Set the color of a player’s fog.
    ///
    /// Direction: `Server -> Client`
    27: FogColor @ = 5;

    /// Sent by the client when the player reloads their weapon, and relayed to other clients after protocol logic applied.
    /// This has no affect on animation, but is used to trigger sound effects on the other clients.
    ///
    /// Direction: `Client <-> Server`
    28: WeaponReload @ = 4;

    /// Sent by the client when the player changes team.
    /// Is not relayed to all clients directly, but instead uses Kill Action then Create Player to inform other clients of the team change.
    ///
    /// Direction: `Client -> Server`
    29: ChangeTeam @ = 3;

    /// Sent by the client when player changes weapon, and relayed to clients by server after filter_visibility logic is applied.
    /// Receiving clients will also be sent a preceding Kill Action to inform them the player has died both of which are sent as reliable packets.
    ///
    /// Direction: `Client -> Server`
    30: ChangeWeapon @ = 3;

    /// <https://github.com/yvt/openspades/blob/40fe69fa9a5216511e1f700c75817bab66540db9/Sources/Client/NetClient.cpp#L1267>
    ///
    /// Direction: `Client -> Server`
    31: MapCached @ = 2;

    /// Sent to the client for checking if client is compatible with version info (this isnt required to get version info).
    /// When sent, server waits for a with the challenge.
    ///
    /// Direction: `Server -> Client`
    31: VersionHandshakeInit @ = 5;

    /// Send back the challenge number to the server,
    /// for validating the client (this isnt required to get version info).
    ///
    /// Direction: `Client -> Server`
    32: VersionHandshakeResponse @ = 5;

    /// Ask the client to send the client and operational system infos.
    ///
    /// Direction: `Server -> Client`
    33: VersionGet @ = 1;

    /// Send the client and operational system infos.
    ///
    /// Direction: `Server -> Client`
    34: VersionResponse @ > 4;
}

// TODO: Move into separate file
pub mod model {
    use crate::error::VariantError;

    /// LE = Little Endian?
    ///
    /// TODO: Maybe replace bit enums with bitflags crate.

    pub type Byte = i8;
    pub type UByte = u8;
    pub type LEFloat = f32;
    pub type LEUint = u32;

    macro_rules! byte_enum {
        (
            $( #[doc = $doc:literal ] )*
            pub enum $name:ident {
                $(
                    $( #[doc = $variant_doc:literal ] )*
                    $variant_name:ident = $variant_value:literal ,
                )+
            }
        ) => {
            $( #[doc = $doc ] )*
            #[repr(u8)]
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum $name {
                $(
                    $( #[doc = $variant_doc ] )*
                    $variant_name = $variant_value
                ),+
            }

            impl ::std::convert::TryFrom<u8> for $name {
                #[allow(unused_qualifications)]
                type Error = crate::error::VariantError<u8>;

                fn try_from(value: u8) -> Result<Self, Self::Error> {
                    match value {
                        $(
                            $variant_value => Ok(Self::$variant_name),
                        )+
                        _ => Err(VariantError::new(stringify!($name), value)),
                    }
                }
            }
        };
    }

    byte_enum! {
        /// The version of the protocol/game.
        pub enum ProtocolVersion {
            V_0_75 = 3,
            V_0_76 = 4,
        }
    }

    impl ProtocolVersion {
        pub const fn to_number(self) -> u8 {
            self as u8
        }
    }

    byte_enum! {
        pub enum DisconnectReason {
            Banned               =  1,
            IpLimitExceeded      =  2,
            WrongProtocolVersion =  3,
            ServerFull           =  4,
            Kicked               = 10,
        }
    }

    impl DisconnectReason {
        pub const fn to_number(self) -> u8 {
            self as u8
        }

        pub const fn reason(&self) -> &'static str {
            match self {
                Self::Banned => "Banned",
                Self::IpLimitExceeded => "IP connection limit exceded",
                Self::WrongProtocolVersion => "Wrong protocol version",
                Self::ServerFull => "Server full",
                Self::Kicked => "Kicked",
            }
        }
    }

    /// In Ace of Spades the up-down axis is Z and it is inverted.
    /// This means 63 is water level and 0 is the highest point on a map.
    #[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Position {
        pub x: LEFloat,
        pub y: LEFloat,
        pub z: LEFloat,
    }

    impl Position {
        pub const fn new_xyz(x: LEFloat, y: LEFloat, z: LEFloat) -> Self {
            Self { x, y, z }
        }
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub struct PlayerPosition {
        pub position: Position,
        pub orientation: Position,
    }

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PlayerId(pub UByte);

    // TODO: Convert to enum?
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Team(pub Byte);

    impl Team {
        pub const SPECTATOR: Self = Self(-1);
        pub const BLUE: Self = Self(0);
        pub const GREEN: Self = Self(1);
        // From `Move Object`
        pub const NEUTRAL: Self = Self(2);
    }

    #[rustfmt::skip]
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum InputKey {
        Up     = 0b0000_0001,
        Down   = 0b0000_0010,
        Left   = 0b0000_0100,
        Right  = 0b0000_1000,
        Jump   = 0b0001_0000,
        Crouch = 0b0010_0000,
        Sneak  = 0b0100_0000,
        Sprint = 0b1000_0000,
    }

    impl InputKey {
        pub const fn to_mask(self) -> u8 {
            self as u8
        }
    }

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct KeyInput(pub UByte);

    impl KeyInput {
        pub const fn is_active(self, key: InputKey) -> bool {
            self.0 | key.to_mask() == 0b1
        }

        pub fn set_active(&mut self, key: InputKey) {
            self.0 |= key.to_mask();
        }
    }

    #[rustfmt::skip]
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum WeaponKey {
        Primary   = 0b0000_0001,
        Secondary = 0b0000_0010,
    }

    impl WeaponKey {
        pub const fn to_mask(self) -> u8 {
            self as u8
        }
    }

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct WeaponInput(pub UByte);

    impl WeaponInput {
        pub const fn is_active(self, key: WeaponKey) -> bool {
            self.0 | key.to_mask() == 0b1
        }

        pub fn set_active(&mut self, key: InputKey) {
            self.0 |= key.to_mask();
        }
    }

    byte_enum! {
        pub enum HitKind {
            Torso = 0,
            Head  = 1,
            Arms  = 2,
            Legs  = 3,
            Melee = 4,
        }
    }

    byte_enum! {
        /// Used in `Set HP` message.
        pub enum DamageKind {
            Fall   = 0,
            Weapon = 1,
        }
    }

    byte_enum! {
        pub enum ToolKind {
            Spade   = 0,
            Block   = 1,
            Gun     = 2,
            Grenade = 3,
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Color {
        r: UByte,
        g: UByte,
        b: UByte,
    }

    impl Color {
        pub const fn new_rgb(r: UByte, g: UByte, b: UByte) -> Self {
            Self { r, g, b }
        }
    }

    byte_enum! {
        pub enum GameMode {
            CTF = 0,
            TC  = 1,
        }
    }

    byte_enum! {
        // TODO: Better names?
        pub enum ActionKind {
            /// Places a block with the player’s selected color.
            Build      = 0,
            /// Bullet and spade(left button) destroy.
            BSLDestroy = 1,
            /// Spade(right button) destroy.
            ///
            /// Destroys 3 blocks, one above and below additionally.
            SRDestroy  = 2,
            /// Grenade destroy.
            ///
            /// Destroys all blocks within an 3x3x3 area.
            GDestroy   = 3,
        }
    }

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct IntelFlags(pub UByte);

    impl IntelFlags {
        pub fn team1_hold_state(&self) -> Result<HoldState, VariantError<u8>> {
            HoldState::try_from(self.0 & 0b01)
        }

        pub fn team2_hold_state(&self) -> Result<HoldState, VariantError<u8>> {
            HoldState::try_from((self.0 & 0b10) >> 1)
        }
    }

    byte_enum! {
        /// Used in `CTF State` message.
        pub enum HoldState {
            Dropped = 0,
            Holding = 1,
        }
    }

    /// The intel location data is 12 bytes long.
    /// If the intel is being held, the first byte is a UByte with the id of the holding player,
    /// then the rest are padding.
    /// If the intel is on the ground (not being held),
    /// the data will hold three LE Floats with its x, y and z position.
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub enum IntelLocation {
        Held(PlayerId),
        Dropped(Position),
    }

    byte_enum! {
        pub enum KillKind {
            /// WEAPON (body, limbs)
            Weapon = 0,
            /// HEADSHOT
            Headshot = 1,
            /// MELEE (spade)
            Melee = 2,
            /// GRENADE
            Grenade = 3,
            /// FALL
            Fall = 4,
            /// TEAM_CHANGE
            TeamChange = 5,
            /// CLASS_CHANGE
            ClassChange = 6,
        }
    }

    byte_enum! {
        pub enum ChatKind {
            /// Color: white
            All = 0,
            /// Color: team color, black for spectator
            Team = 1,
            /// Color: red
            System = 2,
        }
    }

    /// BGRA encoded
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FogColor {
        pub r: UByte,
        pub g: UByte,
        pub b: UByte,
        pub a: UByte,
    }

    impl FogColor {
        pub const fn from_color(color: Color, alpha: UByte) -> Self {
            Self {
                r: color.r,
                g: color.g,
                b: color.b,
                a: alpha,
            }
        }
    }

    byte_enum! {
        pub enum WeaponKind {
            Rifle   = 0,
            Smg     = 1,
            Shotgun = 2,
        }
    }

    byte_enum! {
        pub enum CachedKind {
            NotCached = 0,
            Cached    = 1,
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct TerritoryData {
        pub position: Position,
        // https://github.com/yvt/openspades/blob/40fe69fa9a5216511e1f700c75817bab66540db9/Sources/Client/NetClient.cpp#L1198
        pub owner_team: Team,
    }

    byte_enum! {
        pub enum CaptureKind {
            Winning = 0,
            Losing  = 1,
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Version {
        major: i8,
        minor: i8,
        revision: i8,
    }

    impl Version {
        pub const fn new(major: i8, minor: i8, revision: i8) -> Self {
            Self {
                major,
                minor,
                revision,
            }
        }
    }
}

#[allow(clippy::module_inception)]
pub mod msg {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::{all_consuming, map, map_parser},
        multi::{many0, many_m_n},
        number::complete::{le_f32, le_u32},
        sequence::{pair, preceded, tuple},
        Finish, IResult,
    };

    use super::{
        model::{
            ActionKind, CachedKind, CaptureKind, ChatKind, Color, DamageKind, GameMode, HitKind,
            IntelFlags, IntelLocation, KeyInput, KillKind, PlayerId, PlayerPosition, Position,
            Team, TerritoryData, ToolKind, Version, WeaponKind,
        },
        MessageKind,
    };

    pub trait Message {
        const KIND: MessageKind;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized;
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct PositionData {
        pub position: Position,
    }

    impl Message for PositionData {
        const KIND: MessageKind = MessageKind::PositionData;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let (i, position) =
                preceded(tag(&[<Self as Message>::KIND.id()]), super::parse::position)(i)?;

            Ok((i, Self { position }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct OrientationData {
        pub position: Position,
    }

    impl Message for OrientationData {
        const KIND: MessageKind = MessageKind::OrientationData;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let (i, position) =
                preceded(tag(&[<Self as Message>::KIND.id()]), super::parse::position)(i)?;

            Ok((i, Self { position }))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WorldUpdate75 {
        pub player_positions: [PlayerPosition; 32],
    }

    impl Message for WorldUpdate75 {
        const KIND: MessageKind = MessageKind::WorldUpdate75;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            use std::mem::MaybeUninit;

            let mut player_positions: [MaybeUninit<PlayerPosition>; 32] =
                MaybeUninit::uninit_array();

            let (mut i, _) = tag(&[<Self as Message>::KIND.id()])(i)?;

            for upp in player_positions.iter_mut() {
                let (ii, pp) = super::parse::player_position(i)?;
                i = ii;
                upp.write(pp);
            }

            // # SAFETY
            // Should be safe.
            let player_positions = unsafe { MaybeUninit::array_assume_init(player_positions) };

            Ok((i, Self { player_positions }))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WorldUpdate76 {
        // TODO: Move into own struct?
        pub player_positions: Vec<(PlayerId, PlayerPosition)>,
    }

    impl Message for WorldUpdate76 {
        const KIND: MessageKind = MessageKind::WorldUpdate76;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let pidpp = pair(super::parse::player_id, super::parse::player_position);

            let (i, player_positions) =
                preceded(tag(&[<Self as Message>::KIND.id()]), many0(pidpp))(i)?;

            Ok((i, Self { player_positions }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct InputData {
        pub player_id: PlayerId,
        pub state: KeyInput,
    }

    impl Message for InputData {
        const KIND: MessageKind = MessageKind::InputData;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = pair(super::parse::player_id, super::parse::key_input);

            let (i, (player_id, state)) = preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((i, Self { player_id, state }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WeaponInput {
        pub player_id: PlayerId,
        pub state: super::model::WeaponInput,
    }

    impl Message for WeaponInput {
        const KIND: MessageKind = MessageKind::WeaponInput;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = pair(super::parse::player_id, super::parse::weapon_input);

            let (i, (player_id, state)) = preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((i, Self { player_id, state }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HitPacket {
        pub player_id: PlayerId,
        pub kind: HitKind,
    }

    impl Message for HitPacket {
        const KIND: MessageKind = MessageKind::HitPacket;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = pair(super::parse::player_id, super::parse::hit_kind);

            let (i, (player_id, kind)) = preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((i, Self { player_id, kind }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct SetHP {
        pub hp: u8,
        pub damage_kind: DamageKind,
        pub source_position: Position,
    }

    impl Message for SetHP {
        const KIND: MessageKind = MessageKind::SetHP;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::next(),
                super::parse::damage_kind,
                super::parse::position,
            ));

            let (i, (hp, damage_kind, source_position)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    hp,
                    damage_kind,
                    source_position,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GrenadePacket {
        pub player_id: PlayerId,
        pub fuse_length: f32,
        pub position: Position,
        // TODO: Move into separate struct.
        pub velocity: Position,
    }

    impl Message for GrenadePacket {
        const KIND: MessageKind = MessageKind::GrenadePacket;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                le_f32,
                super::parse::position,
                super::parse::position,
            ));

            let (i, (player_id, fuse_length, position, velocity)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    fuse_length,
                    position,
                    velocity,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SetTool {
        pub player_id: PlayerId,
        pub kind: ToolKind,
    }

    impl Message for SetTool {
        const KIND: MessageKind = MessageKind::SetTool;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = pair(super::parse::player_id, super::parse::tool_kind);

            let (i, (player_id, kind)) = preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((i, Self { player_id, kind }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct SetColor {
        pub player_id: PlayerId,
        pub color: Color,
    }

    impl Message for SetColor {
        const KIND: MessageKind = MessageKind::SetColor;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = pair(super::parse::player_id, super::parse::color);

            let (i, (player_id, color)) = preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((i, Self { player_id, color }))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ExisitingPlayer {
        pub player_id: PlayerId,
        pub team: Team,
        pub weapon: WeaponKind,
        // TODO: Verify if correct enum.
        pub held_item: ToolKind,
        pub kills: u32,
        pub color: Color,
        pub name: String,
    }

    impl Message for ExisitingPlayer {
        const KIND: MessageKind = MessageKind::ExisitingPlayer;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                super::parse::team,
                super::parse::weapon_kind,
                super::parse::tool_kind,
                le_u32,
                super::parse::color,
                super::parse::string,
            ));

            let (i, (player_id, team, weapon, held_item, kills, color, name)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    team,
                    weapon,
                    held_item,
                    kills,
                    color,
                    name,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ShortPlayerData {
        pub player_id: PlayerId,
        pub team: Team,
        pub weapon: WeaponKind,
    }

    impl Message for ShortPlayerData {
        const KIND: MessageKind = MessageKind::ShortPlayerData;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                super::parse::team,
                super::parse::weapon_kind,
            ));

            let (i, (player_id, team, weapon)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    team,
                    weapon,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct MoveObject {
        pub player_id: PlayerId,
        pub team: Team,
        pub position: Position,
    }

    impl Message for MoveObject {
        const KIND: MessageKind = MessageKind::MoveObject;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                super::parse::team,
                super::parse::position,
            ));

            let (i, (player_id, team, position)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    team,
                    position,
                },
            ))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CreatePlayer {
        pub player_id: PlayerId,
        pub weapon: WeaponKind,
        pub team: Team,
        pub position: Position,
        pub name: String,
    }

    impl Message for CreatePlayer {
        const KIND: MessageKind = MessageKind::CreatePlayer;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                super::parse::weapon_kind,
                super::parse::team,
                super::parse::position,
                super::parse::string,
            ));

            let (i, (player_id, weapon, team, position, name)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    weapon,
                    team,
                    position,
                    name,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct BlockAction {
        pub player_id: PlayerId,
        pub kind: ActionKind,
        pub position: Position,
    }

    impl Message for BlockAction {
        const KIND: MessageKind = MessageKind::BlockAction;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                super::parse::action_kind,
                super::parse::position,
            ));

            let (i, (player_id, kind, position)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    kind,
                    position,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct BlockLine {
        pub player_id: PlayerId,
        pub start: Position,
        pub end: Position,
    }

    impl Message for BlockLine {
        const KIND: MessageKind = MessageKind::BlockLine;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                super::parse::position,
                super::parse::position,
            ));

            let (i, (player_id, start, end)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    start,
                    end,
                },
            ))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct StateData {
        pub player_id: PlayerId,
        pub fog_color: Color,
        pub team1_color: Color,
        pub team2_color: Color,
        // TODO: Improve (always len 10)
        pub team1_name: String,
        // TODO: Improve (always len 10)
        pub team2_name: String,
        pub gamemode: GameMode,
        pub addition: Option<StateDataAddition>,
    }

    impl Message for StateData {
        const KIND: MessageKind = MessageKind::StateData;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                super::parse::color,
                super::parse::color,
                super::parse::color,
                map_parser(take(10usize), super::parse::string),
                map_parser(take(10usize), super::parse::string),
                super::parse::gamemode,
            ));

            let (
                i,
                (player_id, fog_color, team1_color, team2_color, team1_name, team2_name, gamemode),
            ) = preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            let (i, addition) = StateDataAddition::parse(i, gamemode)?;

            Ok((
                i,
                Self {
                    player_id,
                    fog_color,
                    team1_color,
                    team2_color,
                    team1_name,
                    team2_name,
                    gamemode,
                    addition,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct CTFState {
        pub team1_score: u8,
        pub team2_score: u8,
        pub capture_limit: u8,
        pub intel_flags: IntelFlags,
        pub team1_intel_location: IntelLocation,
        pub team2_intel_location: IntelLocation,
        pub team1_base: Position,
        pub team2_base: Position,
    }

    impl CTFState {
        fn parse(i: &[u8]) -> IResult<&[u8], Self> {
            let (i, (team1_score, team2_score, capture_limit, intel_flags)) = tuple((
                super::parse::next(),
                super::parse::next(),
                super::parse::next(),
                super::parse::intel_flags,
            ))(i)?;

            let (i, team1_intel_location) =
                super::parse::intel_location(i, intel_flags.team1_hold_state().unwrap())?;
            let (i, team2_intel_location) =
                super::parse::intel_location(i, intel_flags.team2_hold_state().unwrap())?;

            let (i, (team1_base, team2_base)) =
                pair(super::parse::position, super::parse::position)(i)?;

            Ok((
                i,
                Self {
                    team1_score,
                    team2_score,
                    capture_limit,
                    intel_flags,
                    team1_intel_location,
                    team2_intel_location,
                    team1_base,
                    team2_base,
                },
            ))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TCState {
        pub territory_count: u8,
        pub territory_data: Vec<TerritoryData>,
    }

    impl TCState {
        fn parse(i: &[u8]) -> IResult<&[u8], Self> {
            let (i, territory_count) = super::parse::next()(i)?;
            let (i, territory_data) = many_m_n(
                territory_count as usize,
                territory_count as usize,
                super::parse::territory_data,
            )(i)?;

            Ok((
                i,
                Self {
                    territory_count,
                    territory_data,
                },
            ))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum StateDataAddition {
        CTFState(CTFState),
        TCState(TCState),
    }

    impl StateDataAddition {
        fn parse(i: &[u8], gamemode: GameMode) -> IResult<&[u8], Option<Self>> {
            if i.is_empty() {
                return Ok((i, None));
            }

            match gamemode {
                GameMode::CTF => {
                    let (i, ctf) = CTFState::parse(i)?;

                    Ok((i, Some(Self::CTFState(ctf))))
                }
                GameMode::TC => {
                    let (i, tc) = TCState::parse(i)?;

                    Ok((i, Some(Self::TCState(tc))))
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct KillAction {
        pub player_id: PlayerId,
        pub killer_id: PlayerId,
        pub kind: KillKind,
        pub respawn_time: u8,
    }

    impl Message for KillAction {
        const KIND: MessageKind = MessageKind::KillAction;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                super::parse::player_id,
                super::parse::kill_kind,
                super::parse::next(),
            ));

            let (i, (player_id, killer_id, kind, respawn_time)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    killer_id,
                    kind,
                    respawn_time,
                },
            ))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ChatMessage {
        pub player_id: PlayerId,
        pub kind: ChatKind,
        pub message: String,
    }

    impl Message for ChatMessage {
        const KIND: MessageKind = MessageKind::ChatMessage;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                super::parse::chat_kind,
                super::parse::string,
            ));

            let (i, (player_id, kind, message)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    kind,
                    message,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MapStart75 {
        pub size: u32,
    }

    impl Message for MapStart75 {
        const KIND: MessageKind = MessageKind::MapStart75;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let (i, size) = preceded(tag(&[<Self as Message>::KIND.id()]), le_u32)(i)?;

            Ok((i, Self { size }))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MapStart76 {
        pub size: u32,
        pub crc: u32,
        pub name: String,
    }

    impl Message for MapStart76 {
        const KIND: MessageKind = MessageKind::MapStart76;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((le_u32, le_u32, super::parse::string));

            let (i, (size, crc, name)) = preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((i, Self { size, crc, name }))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MapChunk {
        // DEFLATE/zlib encoded AOS map data.
        // zlib: <http://en.wikipedia.org/wiki/DEFLATE>
        // format: <http://silverspaceship.com/aosmap/aos_file_format.html>
        pub data: Vec<u8>,
    }

    impl Message for MapChunk {
        const KIND: MessageKind = MessageKind::MapChunk;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let (i, _) = tag(&[<Self as Message>::KIND.id()])(i)?;

            Ok((&[], Self { data: i.to_vec() }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PlayerLeft {
        pub player_id: PlayerId,
    }

    impl Message for PlayerLeft {
        const KIND: MessageKind = MessageKind::PlayerLeft;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let (i, player_id) = preceded(
                tag(&[<Self as Message>::KIND.id()]),
                super::parse::player_id,
            )(i)?;

            Ok((i, Self { player_id }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TerritoryCapture {
        pub player_id: PlayerId,
        pub entity_id: u8,
        pub kind: CaptureKind,
        pub team: Team,
    }

    impl Message for TerritoryCapture {
        const KIND: MessageKind = MessageKind::TerritoryCapture;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                super::parse::next(),
                super::parse::capture_kind,
                super::parse::team,
            ));

            let (i, (player_id, entity_id, kind, team)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    entity_id,
                    kind,
                    team,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ProgressBar {
        pub entity_id: u8,
        pub capturing_team: Team,
        pub rate: i8,
        pub progress: f32,
    }

    impl Message for ProgressBar {
        const KIND: MessageKind = MessageKind::ProgressBar;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::next(),
                super::parse::team,
                map(super::parse::next(), |b| b as i8),
                le_f32,
            ));

            let (i, (entity_id, capturing_team, rate, progress)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    entity_id,
                    capturing_team,
                    rate,
                    progress,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct IntelCapture {
        pub player_id: PlayerId,
        pub kind: CaptureKind,
    }

    impl Message for IntelCapture {
        const KIND: MessageKind = MessageKind::IntelCapture;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = pair(super::parse::player_id, super::parse::capture_kind);

            let (i, (player_id, kind)) = preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((i, Self { player_id, kind }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct IntelPickup {
        pub player_id: PlayerId,
    }

    impl Message for IntelPickup {
        const KIND: MessageKind = MessageKind::IntelPickup;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let (i, player_id) = preceded(
                tag(&[<Self as Message>::KIND.id()]),
                super::parse::player_id,
            )(i)?;

            Ok((i, Self { player_id }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct IntelDrop {
        pub player_id: PlayerId,
        pub position: Position,
    }

    impl Message for IntelDrop {
        const KIND: MessageKind = MessageKind::IntelDrop;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = pair(super::parse::player_id, super::parse::position);

            let (i, (player_id, position)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    position,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Restock {
        pub player_id: PlayerId,
    }

    impl Message for Restock {
        const KIND: MessageKind = MessageKind::Restock;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let (i, player_id) = preceded(
                tag(&[<Self as Message>::KIND.id()]),
                super::parse::player_id,
            )(i)?;

            Ok((i, Self { player_id }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FogColor {
        pub color: super::model::FogColor,
    }

    impl Message for FogColor {
        const KIND: MessageKind = MessageKind::Restock;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let (i, color) = preceded(
                tag(&[<Self as Message>::KIND.id()]),
                super::parse::fog_color,
            )(i)?;

            Ok((i, Self { color }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WeaponReload {
        pub player_id: PlayerId,
        pub clip_ammo: u8,
        pub reserve_ammo: u8,
    }

    impl Message for WeaponReload {
        const KIND: MessageKind = MessageKind::WeaponReload;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                super::parse::player_id,
                super::parse::next(),
                super::parse::next(),
            ));

            let (i, (player_id, clip_ammo, reserve_ammo)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    player_id,
                    clip_ammo,
                    reserve_ammo,
                },
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ChangeTeam {
        pub player_id: PlayerId,
        pub team: Team,
    }

    impl Message for ChangeTeam {
        const KIND: MessageKind = MessageKind::WeaponReload;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = pair(super::parse::player_id, super::parse::team);

            let (i, (player_id, team)) = preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((i, Self { player_id, team }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ChangeWeapon {
        pub player_id: PlayerId,
        pub kind: WeaponKind,
    }

    impl Message for ChangeWeapon {
        const KIND: MessageKind = MessageKind::ChangeWeapon;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = pair(super::parse::player_id, super::parse::weapon_kind);

            let (i, (player_id, kind)) = preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((i, Self { player_id, kind }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MapCached {
        pub kind: CachedKind,
    }

    impl Message for MapCached {
        const KIND: MessageKind = MessageKind::MapCached;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let (i, kind) = preceded(
                tag(&[<Self as Message>::KIND.id()]),
                super::parse::cached_kind,
            )(i)?;

            Ok((i, Self { kind }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VersionHandshakeInit {
        pub challenge: u32,
    }

    impl Message for VersionHandshakeInit {
        const KIND: MessageKind = MessageKind::VersionHandshakeInit;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let (i, challenge) = preceded(tag(&[<Self as Message>::KIND.id()]), le_u32)(i)?;

            Ok((i, Self { challenge }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VersionHandshakeResponse {
        pub challenge: u32,
    }

    impl Message for VersionHandshakeResponse {
        const KIND: MessageKind = MessageKind::VersionHandshakeResponse;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let (i, challenge) = preceded(tag(&[<Self as Message>::KIND.id()]), le_u32)(i)?;

            Ok((i, Self { challenge }))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VersionGet;

    impl Message for VersionGet {
        const KIND: MessageKind = MessageKind::VersionGet;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            Ok((i, Self))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VersionResponse {
        pub client_identifier: i8,
        pub version: Version,
        pub name: String,
    }

    impl Message for VersionResponse {
        const KIND: MessageKind = MessageKind::VersionResponse;

        fn parse(i: &[u8]) -> IResult<&[u8], Self>
        where
            Self: Sized,
        {
            let inner = tuple((
                map(super::parse::next(), |b| b as i8),
                super::parse::version,
                super::parse::string,
            ));

            let (i, (client_identifier, version, name)) =
                preceded(tag(&[<Self as Message>::KIND.id()]), inner)(i)?;

            Ok((
                i,
                Self {
                    client_identifier,
                    version,
                    name,
                },
            ))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Msg {
        PositionData(PositionData),
        OrientationData(OrientationData),
        WorldUpdate75(Box<WorldUpdate75>),
        WorldUpdate76(WorldUpdate76),
        InputData(InputData),
        WeaponInput(WeaponInput),
        HitPacket(HitPacket),
        SetHP(SetHP),
        GrenadePacket(GrenadePacket),
        SetTool(SetTool),
        SetColor(SetColor),
        ExisitingPlayer(ExisitingPlayer),
        ShortPlayerData(ShortPlayerData),
        MoveObject(MoveObject),
        CreatePlayer(CreatePlayer),
        BlockAction(BlockAction),
        BlockLine(BlockLine),
        StateData(StateData),
        KillAction(KillAction),
        ChatMessage(ChatMessage),
        MapStart75(MapStart75),
        MapStart76(MapStart76),
        MapChunk(MapChunk),
        PlayerLeft(PlayerLeft),
        TerritoryCapture(TerritoryCapture),
        ProgressBar(ProgressBar),
        IntelCapture(IntelCapture),
        IntelPickup(IntelPickup),
        IntelDrop(IntelDrop),
        Restock(Restock),
        FogColor(FogColor),
        WeaponReload(WeaponReload),
        ChangeTeam(ChangeTeam),
        ChangeWeapon(ChangeWeapon),
        MapCached(MapCached),
        VersionHandshakeInit(VersionHandshakeInit),
        VersionHandshakeResponse(VersionHandshakeResponse),
        VersionGet(VersionGet),
        VersionResponse(VersionResponse),
    }

    impl Msg {
        #[rustfmt::skip]
        pub fn parse_server(i: &[u8]) -> Result<Self, nom::error::Error<&[u8]>> {
            // TODO: Check all consumed
            let (_, msg) = all_consuming(alt((
                map(PositionData::parse, Self::PositionData),
                map(OrientationData::parse, Self::OrientationData),
                map(WorldUpdate75::parse, |msg| Self::WorldUpdate75(Box::new(msg))),
                //map(WorldUpdate76::parse, Self::WorldUpdate76),
                map(InputData::parse, Self::InputData),
                map(WeaponInput::parse, Self::WeaponInput),
                map(HitPacket::parse, Self::HitPacket),
                map(SetHP::parse, Self::SetHP),
                map(GrenadePacket::parse, Self::GrenadePacket),
                map(SetTool::parse, Self::SetTool),
                map(SetColor::parse, Self::SetColor),
                map(ExisitingPlayer::parse, Self::ExisitingPlayer),
                map(ShortPlayerData::parse, Self::ShortPlayerData),
                map(MoveObject::parse, Self::MoveObject),
                map(CreatePlayer::parse, Self::CreatePlayer),
                map(BlockAction::parse, Self::BlockAction),
                map(BlockLine::parse, Self::BlockLine),
                map(StateData::parse, Self::StateData),
                map(KillAction::parse, Self::KillAction),
                map(ChatMessage::parse, Self::ChatMessage),
                alt((
                    map(MapStart75::parse, Self::MapStart75),
                    //map(MapStart76::parse, Self::MapStart76),
                    map(MapChunk::parse, Self::MapChunk),
                    map(PlayerLeft::parse, Self::PlayerLeft),
                    map(TerritoryCapture::parse, Self::TerritoryCapture),
                    map(ProgressBar::parse, Self::ProgressBar),
                    map(IntelCapture::parse, Self::IntelCapture),
                    map(IntelPickup::parse, Self::IntelPickup),
                    map(IntelDrop::parse, Self::IntelDrop),
                    map(Restock::parse, Self::Restock),
                    map(FogColor::parse, Self::FogColor),
                    map(WeaponReload::parse, Self::WeaponReload),
                    map(VersionHandshakeInit::parse, Self::VersionHandshakeInit),
                    map(VersionGet::parse, Self::VersionGet),
                )),
            )))(i)
            .finish()?;

            Ok(msg)
        }
    }
}

pub mod parse {
    use nom::{
        bytes::complete::take,
        combinator::{map, map_res},
        number::complete::le_f32,
        sequence::{pair, terminated, tuple},
        IResult,
    };

    use super::model::{
        ActionKind, CachedKind, CaptureKind, ChatKind, Color, DamageKind, FogColor, GameMode,
        HitKind, HoldState, IntelFlags, IntelLocation, KeyInput, KillKind, PlayerId,
        PlayerPosition, Position, Team, TerritoryData, ToolKind, Version, WeaponInput, WeaponKind,
    };

    pub fn next<'a>() -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], u8> {
        map(take(1usize), |res: &'a [u8]| res[0])
    }

    pub fn try_from_byte<'a, T>() -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], T>
    where
        T: TryFrom<u8>,
    {
        map_res(next(), T::try_from)
    }

    // Discards a trailing 0 (c style strings).
    pub fn str(i: &[u8]) -> IResult<&[u8], &str> {
        // TODO: Remove unwrap
        let s = if i.last() == Some(&b'\0') {
            std::str::from_utf8(&i[..i.len() - 1]).unwrap()
        } else {
            std::str::from_utf8(i).unwrap()
        };

        Ok((&[], s))
    }

    pub fn string(i: &[u8]) -> IResult<&[u8], String> {
        map(str, |s: &str| s.to_owned())(i)
    }

    pub fn player_id(i: &[u8]) -> IResult<&[u8], PlayerId> {
        map(next(), PlayerId)(i)
    }

    pub fn position(i: &[u8]) -> IResult<&[u8], Position> {
        let (i, (x, y, z)) = tuple((le_f32, le_f32, le_f32))(i)?;

        Ok((i, Position::new_xyz(x, y, z)))
    }

    pub fn color(i: &[u8]) -> IResult<&[u8], Color> {
        let (i, (b, g, r)) = tuple((next(), next(), next()))(i)?;

        Ok((i, Color::new_rgb(r, g, b)))
    }

    pub fn player_position(i: &[u8]) -> IResult<&[u8], PlayerPosition> {
        let (i, (position, orientation)) = pair(position, position)(i)?;

        Ok((
            i,
            PlayerPosition {
                position,
                orientation,
            },
        ))
    }

    pub fn key_input(i: &[u8]) -> IResult<&[u8], KeyInput> {
        map(next(), KeyInput)(i)
    }

    pub fn weapon_input(i: &[u8]) -> IResult<&[u8], WeaponInput> {
        map(next(), WeaponInput)(i)
    }

    pub fn hit_kind(i: &[u8]) -> IResult<&[u8], HitKind> {
        try_from_byte()(i)
    }

    pub fn damage_kind(i: &[u8]) -> IResult<&[u8], DamageKind> {
        try_from_byte()(i)
    }

    pub fn tool_kind(i: &[u8]) -> IResult<&[u8], ToolKind> {
        try_from_byte()(i)
    }

    pub fn weapon_kind(i: &[u8]) -> IResult<&[u8], WeaponKind> {
        try_from_byte()(i)
    }

    pub fn team(i: &[u8]) -> IResult<&[u8], Team> {
        map(next(), |b| Team(b as i8))(i)
    }

    pub fn action_kind(i: &[u8]) -> IResult<&[u8], ActionKind> {
        try_from_byte()(i)
    }

    pub fn gamemode(i: &[u8]) -> IResult<&[u8], GameMode> {
        try_from_byte()(i)
    }

    pub fn intel_flags(i: &[u8]) -> IResult<&[u8], IntelFlags> {
        map(next(), IntelFlags)(i)
    }

    pub fn intel_location(i: &[u8], state: HoldState) -> IResult<&[u8], IntelLocation> {
        match state {
            HoldState::Dropped => map(position, IntelLocation::Dropped)(i),
            HoldState::Holding => map(terminated(player_id, take(11usize)), IntelLocation::Held)(i),
        }
    }

    pub fn territory_data(i: &[u8]) -> IResult<&[u8], TerritoryData> {
        let (i, (position, owner_team)) = pair(position, team)(i)?;

        Ok((
            i,
            TerritoryData {
                position,
                owner_team,
            },
        ))
    }

    pub fn kill_kind(i: &[u8]) -> IResult<&[u8], KillKind> {
        try_from_byte()(i)
    }

    pub fn chat_kind(i: &[u8]) -> IResult<&[u8], ChatKind> {
        try_from_byte()(i)
    }

    pub fn capture_kind(i: &[u8]) -> IResult<&[u8], CaptureKind> {
        try_from_byte()(i)
    }

    pub fn fog_color(i: &[u8]) -> IResult<&[u8], FogColor> {
        let (i, (color, alpha)) = pair(color, next())(i)?;

        Ok((i, FogColor::from_color(color, alpha)))
    }

    pub fn cached_kind(i: &[u8]) -> IResult<&[u8], CachedKind> {
        try_from_byte()(i)
    }

    pub fn version(i: &[u8]) -> IResult<&[u8], Version> {
        let (i, (major, minor, revision)) = tuple((
            map(next(), |b| b as i8),
            map(next(), |b| b as i8),
            map(next(), |b| b as i8),
        ))(i)?;

        Ok((i, Version::new(major, minor, revision)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types() {
        let t = MessageKind::ChangeWeapon;

        assert_eq!(t.id(), 30);
        assert_eq!(t.size(), MessageSize::Exact(3));
    }
}
