use regcomms::{RegCommsAccessProc, RegComms, RegCommsError};
use crate::Icm42670P;

#[derive(Default)]
pub struct Mreg1;

impl<C: RegComms<1, u8>> RegCommsAccessProc<Icm42670P<C>, 1, u8> for Mreg1 {
    fn proc_read(&self, peripheral: &mut Icm42670P<C>, reg_address: u8, buf: &mut [u8]) -> Result<(), RegCommsError> {
        assert!(buf.len() == 1);
        peripheral.blk_sel_r().modify(|mut val| {
            val.set(1);
            val
        })?;
        peripheral.maddr_r().modify(|mut val| {
            val.set(reg_address);
            val
        })?;
        // delay 10us
        let val = peripheral.m_r().read()?.get();
        // delay 10us
        buf[0] = val;
        peripheral.blk_sel_r().modify(|mut val| {
            val.set(0);
            val
        })
    }
    async fn proc_read_async(&self, peripheral: &mut Icm42670P<C>, reg_address: u8, buf: &mut [u8]) -> Result<(), RegCommsError> {
        assert!(buf.len() == 1);
        peripheral.blk_sel_r().modify_async(|mut val| {
            val.set(1);
            val
        }).await?;
        peripheral.maddr_r().modify_async(|mut val| {
            val.set(reg_address);
            val
        }).await?;
        // delay 10us
        let val = peripheral.m_r().read_async().await?.get();
        // delay 10us
        buf[0] = val;
        peripheral.blk_sel_r().modify_async(|mut val| {
            val.set(0);
            val
        }).await
    }

    fn proc_write(&self, peripheral: &mut Icm42670P<C>, reg_address: u8, buf: &[u8]) -> Result<(), RegCommsError> {
        assert!(buf.len() == 1);
        peripheral.blk_sel_w().modify(|mut val| {
            val.set(1);
            val
        })?;
        peripheral.maddr_w().modify(|mut val| {
            val.set(reg_address);
            val
        })?;
        // delay 10us
        peripheral.m_w().write_raw(buf[0])?;
        // delay 10us
        peripheral.blk_sel_w().modify(|mut val| {
            val.set(0);
            val
        })
    }
    async fn proc_write_async(&self, peripheral: &mut Icm42670P<C>, reg_address: u8, buf: &[u8]) -> Result<(), RegCommsError> {
        assert!(buf.len() == 1);
        peripheral.blk_sel_w().modify_async(|mut val| {
            val.set(1);
            val
        }).await?;
        peripheral.maddr_w().modify_async(|mut val| {
            val.set(reg_address);
            val
        }).await?;
        // delay 10us
        peripheral.m_w().write_raw_async(buf[0]).await?;
        // delay 10us
        peripheral.blk_sel_w().modify_async(|mut val| {
            val.set(0);
            val
        }).await
    }
}
