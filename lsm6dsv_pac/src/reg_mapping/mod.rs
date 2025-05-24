use regcomms::{RegCommsAccessProc, RegComms, RegCommsError};
use crate::Lsm6Dsv;

#[derive(Default)]
pub struct EmbeddedFunction;

fn access_embedded_func<C, F>(lsm: &mut Lsm6Dsv<C>, f: F) -> Result<usize, RegCommsError>
where
    C: RegComms<1, u8>,
    F: FnOnce(&mut Lsm6Dsv<C>) -> Result<usize, RegCommsError>,
{
        lsm.func_cfg_access().modify(|mut val| {
            val.emb_func_reg_access().set_bit();
            val
        })?;
        let len = f(lsm)?;
        lsm.func_cfg_access().modify(|mut val| {
            val.emb_func_reg_access().set_bit();
            val
        })?;
        Ok(len)
}

impl<C: RegComms<1, u8>> RegCommsAccessProc<Lsm6Dsv<C>, 1, u8> for EmbeddedFunction {
    fn proc_read(&self, peripheral: &mut Lsm6Dsv<C>, reg_address: u8, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        access_embedded_func(peripheral, |p| {
            let proc = p.standard;
            proc.proc_read(p, reg_address, buf)
        })
    }
    fn proc_write(&self, peripheral: &mut Lsm6Dsv<C>, reg_address: u8, buf: &[u8]) -> Result<usize, RegCommsError> {
        access_embedded_func(peripheral, |p| {
            let proc = p.standard;
            proc.proc_write(p, reg_address, buf)
        })
    }
    async fn proc_read_async(&self, peripheral: &mut Lsm6Dsv<C>, reg_address: u8, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        peripheral.func_cfg_access().modify_async(|mut val| {
            val.emb_func_reg_access().set_bit();
            val
        }).await?;
        let proc = peripheral.standard;
        let len = proc.proc_read_async(peripheral, reg_address, buf).await?;
        peripheral.func_cfg_access().modify_async(|mut val| {
            val.emb_func_reg_access().set_bit();
            val
        }).await?;
        Ok(len)
    }
    async fn proc_write_async(&self, peripheral: &mut Lsm6Dsv<C>, reg_address: u8, buf: &[u8]) -> Result<usize, RegCommsError> {
        peripheral.func_cfg_access().modify_async(|mut val| {
            val.emb_func_reg_access().set_bit();
            val
        }).await?;
        let proc = peripheral.standard;
        let len = proc.proc_write_async(peripheral, reg_address, buf).await?;
        peripheral.func_cfg_access().modify_async(|mut val| {
            val.emb_func_reg_access().set_bit();
            val
        }).await?;
        Ok(len)
    }
}
