use core::result::Result;
use regcomms::{
    RegComms,
    RegCommsAddress,
    RegCommsError,
};
use smallvec::SmallVec;

pub struct SpiComms<Device: embedded_hal::spi::SpiBus> {
    pub comms: Device,
}

impl<Device: embedded_hal::spi::SpiBus> SpiComms<Device> {
    pub fn new(comms: Device) -> Self {
        Self {
            comms,
        }
    }
}

impl<Device: embedded_hal::spi::SpiBus, const N: usize, R: RegCommsAddress<N>> RegComms<N, R> for SpiComms<Device> {
    fn comms_read(&mut self, reg_address: R, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        let address_buf = reg_address.to_big_endian();
        let mut transaction_buf: SmallVec<[u8; 128]> = SmallVec::new();
        transaction_buf.extend(address_buf);
        transaction_buf.resize(address_buf.len() + buf.len(), 0);
        // Read op:  Set the first bit of the address buf to 1
        transaction_buf[0] |= 1 << 7;
        if let Err(_) = self.comms.transfer_in_place(&mut transaction_buf) {
            return Err(RegCommsError::Other)
        }
        buf.copy_from_slice(&transaction_buf[address_buf.len()..]);
        Ok(buf.len())
    }

    fn comms_write(&mut self, reg_address: R, buf: &[u8]) -> Result<usize, RegCommsError> {
        let address_buf = reg_address.to_big_endian();
        let mut transaction_buf: SmallVec<[u8; 128]> = SmallVec::new();
        transaction_buf.extend(address_buf);
        transaction_buf.extend_from_slice(buf);
        // Write op:  Set the first bit of the address buf to 0
        transaction_buf[0] &= !(1 << 7);
        if let Err(_) = self.comms.transfer_in_place(&mut transaction_buf) {
            return Err(RegCommsError::Other)
        }
        Ok(buf.len())
    }
}
