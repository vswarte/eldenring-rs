#[repr(u32)]
#[derive(Debug)]
pub enum LobbyState {
    Offline = 0x0,
    CreatingLobby = 0x1,
    FailedCreatingLobby = 0x2,
    HostingLobby = 0x3,
    JoiningLobby = 0x4,
    FailedJoiningLobby = 0x5,
    InActiveLobby = 0x6,
    LeavingLobby = 0x7,
    FailedLeavingLobby = 0x8,
}

#[repr(u32)]
#[derive(Debug)]
pub enum ProtocolState {
    Inactive = 0x0,
    AwaitingWorldData = 0x1,
    Unk2 = 0x2,
    Unk3 = 0x3,
    Unk4 = 0x4,
    Unk5 = 0x5,
    InWorld = 0x6,
    Unk7 = 0x7,
}

#[repr(C)]
#[dlrf::singleton("CSSessionManager")]
pub struct CSSessionManager {
    vftable: usize,
    unk8: u32,
    pub lobby_state: LobbyState,
    pub protocol_state: ProtocolState,
    unk14: f32,
    unk18: u32,
    unk1c: u32,
}
