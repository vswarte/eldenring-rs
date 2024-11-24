use std::ptr::NonNull;

use vtable_rs::VPtr;

#[vtable_rs::vtable]
pub trait NetworkSessionVmt {
    fn destructor(&mut self);

    /// Broadcasts a p2p packet to the entire lobby.
    fn broadcast_packet(&self, buffer: *const u8, length: u32, packet_type: u8);

    /// Sends a 0x120 hit struct to the remote party.
    fn send_hit(&self, buffer: *const u8);

    /// Attempts to dequeue any received packets of a given type.
    fn receive_packet(
        &mut self,
        allocation: &mut ReceivePacketAllocation,
        reader_out: &mut ReceivedPacketReader,
        packet_type: u8,
    ) -> NonNull<ReceivedPacketReader>;

    /// Dequeues packets of a type until there are none left and then returns that packet data.
    fn receive_latest_packet(
        &mut self,
        allocation: &mut ReceivePacketAllocation,
        reader_out: &mut ReceivedPacketReader,
        packet_type: u8,
    ) -> NonNull<ReceivedPacketReader>;

    /// Retrieves the last received packet 60.
    fn receive_latest_sp_effect_initialize_packet(
        &mut self,
        allocation: &mut ReceivePacketAllocation,
        buffer: &mut u64,
    );

    /// Retrieves the last received packet 61.
    fn receive_latest_sp_effect_sync_data(
        &mut self,
        allocation: &mut ReceivePacketAllocation,
        buffer: &mut u64,
        buffer_size: u32,
    );

    /// Ends connection with the remote party.
    fn kick(&mut self);

    /// Asks the remote party to leave the session.
    fn request_leave(&mut self);

    /// Returns a remote identity handle. For PC release this'll be the steam ID.
    fn remote_identity(&self) -> u64;
}

#[repr(C)]
pub struct ReceivePacketAllocation {
    /// Pointer to the write-able buffer of N bytes where N is determined by the size field.
    pub buffer: *mut u8,

    /// Amount of the data that can be written the buffer pointer at maximum.
    pub size: usize,
}

#[repr(C)]
pub struct ReceivedPacketReader {
    /// Pointer to the received data.
    pub buffer: *mut u8,

    /// Amount of the data that can be written the buffer pointer at maximum.
    pub allocation_size: usize,

    /// Amount of the data that was received and written to the buffer.
    pub received_size: usize,
}

/// Represents a network session with another player.
pub struct PlayerNetworkSession {
    pub vftable: VPtr<dyn NetworkSessionVmt, Self>,
    /// Steam ID for PC release.
    pub remote_identity: u64,
}
