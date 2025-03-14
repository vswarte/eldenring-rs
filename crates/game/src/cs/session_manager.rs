use std::ptr::NonNull;

use windows::Win32::Foundation::FILETIME;

use crate::{
    dlcr::{AESDecrypter, AESEncrypter, DLSerialCipherKey},
    dlkr::{DLAllocatorBase, DLPlainLightMutex},
    dltx::{DLCodedString, DLInplaceStr},
    fd4::FD4Time,
    pointer::OwnedPtr,
    DoublyLinkedList, Vector,
};

use super::CSEzUpdateTask;

#[repr(u32)]
#[derive(Debug, PartialEq)]
/// Various states for an online lobby to be in.
///
/// Source of name: Sekiro Debug Menu
pub enum LobbyState {
    None = 0x0,
    TryToCreateSession = 0x1,
    FailedToCreateSession = 0x2,
    Host = 0x3,
    TryToJoinSession = 0x4,
    FailedToJoinSesion = 0x5,
    Client = 0x6,
    OnLeaveSession = 0x7,
    FailedToLeaveSession = 0x8,
}

#[repr(u32)]
#[derive(Debug)]
pub enum ProtocolState {
    Inactive = 0x0,
    Unk1 = 0x1,
    AwaitingWorldData = 0x2,
    Unk3 = 0x3,
    Unk4 = 0x4,
    Unk5 = 0x5,
    InWorld = 0x6,
    Unk7 = 0x7,
}

impl ProtocolState {
    /// Seems to be checked for packet 39,
    fn should_handle_some_packets(&self) -> bool {
        match self {
            ProtocolState::Inactive => false,
            ProtocolState::Unk1 => false,
            ProtocolState::AwaitingWorldData => false,
            ProtocolState::Unk3 => false,
            ProtocolState::Unk4 => true,
            ProtocolState::Unk5 => true,
            ProtocolState::InWorld => false,
            ProtocolState::Unk7 => true,
        }
    }
}

#[repr(C)]
#[dlrf::singleton("CSSessionManager")]
pub struct CSSessionManager {
    vftable: usize,
    unk8: u32,
    pub lobby_state: LobbyState,
    pub protocol_state: ProtocolState,
    unk14: f32,
    unk18: u8,
    unk19: u8,
    unk1a: u8,
    unk1b: u8,
    unk1c: u8,
    unk1d: u8,
    unk1e: u8,
    unk1f: u8,
    unk20: u32,
    unk24: u32,
    unk28: u32,
    unk2c: u32,
    unk30: usize,
    map_active_synchronizer: usize,
    voice_chat_manager: usize,
    allocator: NonNull<DLAllocatorBase>,
    unk50: NonNull<Self>,
    unk58: u32,
    unk5c: u32,
    manager_impl_steam: usize,
    unk68: usize,
    pub players: Vector<CSSessionManagerPlayerEntry>,
    pub host_player: CSSessionManagerPlayerEntry,
    unk190: usize,
    protocol_state_1_timeout: FD4Time,
    protocol_state_2_timeout: FD4Time,
    unk1b8: usize,
    unk1c0: u8,
    unk1c1: u8,
    unk1c2: u8,
    unk1c3: u8,
    unk1c4: f32,
    unk1c8: u16,
    unk1ca: u8,
    unk1cb: u8,
    unk1cc: f32,
    unk1d0: f32,
    unk1d4: f32,
    unk1d8: f32,
    unk1dc: u32,
    pub update_task: CSEzUpdateTask<Self>,
    unk208: CSEzUpdateTask<Self>,
    unk230: i8,
    unk231: u8,
    unk232: u8,
    unk233: u8,
    unk234: u32,
    pub serial_cipher_key: OwnedPtr<DLSerialCipherKey>,
    pub aes_encrypter: OwnedPtr<AESEncrypter>,
    pub aes_decrypter: OwnedPtr<AESDecrypter>,
    unk250: u32,
    unk254: u32,
    unk258: u32,
    unk25c: u32,
    /// P2P Send queue? Seems unused? Maybe left-over from DS2?
    p2p_send_queue: CSSessionManagerP2PSendQueue,
    pub mutex: DLPlainLightMutex,
    unk2d0: f32,
    unk2d4: f32,
    /// Contain statistics about the inbound packet queue, seems unused.
    p2p_inbound_queue_stats: Option<OwnedPtr<CSSessionManagerP2PInboundQueueStats>>,
    unk2e0: u32,
    /// Seems to be a total for the amount of packet bytes in some fashion?
    unk2e4: u32,
    unk2e8: u32,
    unk2ec: u8,
    unk2ed: bool,
    unk2ee: u8,
    unk2ef: u8,
    unk2f0: DoublyLinkedList<()>,
    unk308: u16,
    unk30a: u16,
    unk30c: u32,
    unk310: i32,
    unk314: u32,
    unk318: u32,
    unk31c: u32,
    unk320: u16,
    unk322: u16,
    unk324: i32,
    unk328: i32,
    unk32c: u32,
    /// Next fields seem to be some collection?
    unk330: NonNull<DLAllocatorBase>,
    unk338: Option<OwnedPtr<()>>,
    unk340: u32,
    unk344: u32,
    unk348: u16,
    unk34a: u16,
    unk34c: i32,
    unk350: f32,
    unk354: u32,
}

#[repr(C)]
pub struct CSSessionManagerPlayerEntry {
    internal_thread_steam_connection: usize,
    internal_thread_steam_socket: usize,
    pub steam_id: u64,
    pub steam_name: DLInplaceStr<1, 64>,
    connection_ref_info: usize,
    voice_chat_member_ref_info: usize,
    game_data_index: i32,
    chr_type: u32,
    unkd8: u32,
    unkdc: i32,
    unke0: u32,
    unke4: u32,
    unke8: u8,
    unke9: u8,
    // Seems to be used by the 250 "protocol" packet.
    p2p_control_byte: u8,
    unkeb: u8,
    unkec: u8,
    unked: u8,
    unkef: u8,
    unkf0: usize,
    unkf8: u32,
    team_type: u8,
    unkfd: u8,
    unkfe: u8,
    unkff: u8,
}

#[repr(C)]
pub struct CSSessionManagerP2PSendQueue {
    pub queue: Vector<CSSessionManagerP2PSendQueueEntry>,
    unk20: CSSessionManager0x20,
    rand_xor_shift: usize,
    unk38: u32,
    unk3c: u32,
}

#[repr(C)]
pub struct CSSessionManagerP2PSendQueueEntry {
    /// Recipient's steam ID.
    pub recipient: u64,
    pub packet_bytes: *mut u8,
    pub packet_length: u32,
    pub packet_type: u8,
    unk15: u8,
    _pad16: u16,
}

#[repr(C)]
pub struct CSSessionManager0x20 {
    time_1: FILETIME,
    time_2: FILETIME,
}

#[repr(C)]
pub struct CSSessionManagerP2PInboundQueueStats {
    /// Seems to keep track of the amount of packets waiting in-queue.
    pending_packet_count: u32,
    /// Seems to keep track of the amount of raw bytes in the queue.
    pending_packet_bytes: u32,
    unk8: u32,
    unkc: u32,
}
