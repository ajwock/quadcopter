#![no_std]
mod accel_config;
mod gyro_config;
mod fifo_config;
mod generic_config;
mod config;

pub use config::Config;
pub use accel_config::{AccelConfig, AccelRange};
pub use gyro_config::{GyroConfig, GyroRange};
pub use fifo_config::FifoConfig;
pub use generic_config::{
    ODR,
    DLPF,
};

use icm42670p_pac::Icm42670P;
use regcomms::{RegComms, RegCommsError};
use embedded_hal_async::delay::DelayNs;

pub struct Icm42670<D: DelayNs, C: RegComms<1, u8>> {
    pub p: Icm42670P<D, C>,
    conf: Config,
    delay: D,
}

impl<D: DelayNs + Clone, C: RegComms<1, u8>> Icm42670<D, C> {
    pub fn new(comms: C, delay: D) -> Self {
        Self {
            p: Icm42670P::new(delay.clone(), comms),
            conf: Default::default(),
            delay,
        }
    }

    pub async fn verify_identity(&mut self) -> Result<bool, RegCommsError> {
        let id = self.p.who_am_i().read_async().await?.get();
        Ok(id == 0x67)
    }

    pub async fn poweron_idle(&mut self) -> Result<(), RegCommsError> {
        self.p.pwr_mgmt0().write_raw_async(0b00010000).await
    }

    pub async fn configure_accelerometer(&mut self, accel_cnf: AccelConfig) -> Result<(), RegCommsError> {
        self.p.accel_config0().modify_async(|mut val| {
            val
                .accel_ui_fs_sel().set(accel_cnf.accel_range.to_bits())
                .accel_odr().set(accel_cnf.accel_odr.to_bits());
            val
        }).await?;
        self.p.accel_config1().modify_async(|mut val| {
            val
                .accel_ui_filt_bw().set(accel_cnf.accel_range.to_bits());
            val
        }).await?;
        Ok(())   
    }

    pub async fn configure_gyrometer(&mut self, gyro_cnf: GyroConfig) -> Result<(), RegCommsError> {
        self.p.gyro_config0().modify_async(|mut val| {
            val
                 .gyro_ui_fs_sel().set(gyro_cnf.gyro_range.to_bits())
                 .gyro_odr().set(gyro_cnf.gyro_odr.to_bits());
            val
        }).await?;
        self.p.gyro_config1().modify_async(|mut val| {
            val
                .gyro_ui_filt_bw().set(gyro_cnf.gyro_range.to_bits());
            val
        }).await?;
        Ok(())
    }

    pub async fn fifo_configure(&mut self, conf: Config) -> Result<(), RegCommsError> {
        let Some(fifo_config) = conf.fifo_config else {
            return Ok(())
        };

        self.p.tmst_config1().modify_async(|mut val| {
            val
                .tmst_fsync_en().clear_bit()
                .tmst_en().set_bit();
            val
        }).await?;
        // We keep most defaults but tmst in fifo is not fsync
        let mut saved_val = 0;
        self.p.fifo_config5().modify_async(|mut val| {
            val
                .fifo_accel_en().assign(conf.accel_config.is_some())
                .fifo_gyro_en().assign(conf.gyro_config.is_some())
                .fifo_tmst_fsync_en().clear_bit();
            saved_val = val.get();
            val
        }).await?;
        let readback_val = self.p.fifo_config5().read_async().await?.get();
        if saved_val != readback_val {
            panic!("Alert!  saved_val 0x{:x} != 0x{:x} readback value for fifo_config5", saved_val, readback_val);
        }
        self.p.fifo_config1().modify_async(|mut val| {
            val
                .fifo_mode().assign(fifo_config.mode.to_bit());
            val
        }).await?;
        Ok(())
    }

    pub async fn enable(&mut self) -> Result<(), RegCommsError> {
        self.p.pwr_mgmt0().modify_async(|mut val| {
            let gyro_mode = if self.conf.gyro_config.is_some() {
                0b11
            } else {
                0b00
            };
            let accel_mode = if self.conf.accel_config.is_some() {
                0b11
            } else {
                0b00
            };
            val
                .gyro_mode().set(gyro_mode)
                .accel_mode().set(accel_mode);
            val
        }).await?;
        // Enable fifo
        if self.conf.fifo_config.is_some() {
            self.p.fifo_config1().modify_async(|mut val| {
                val.fifo_bypass().clear_bit();
                val
            }).await?;
        }
        Ok(())
    }

    pub async fn flush_fifo(&mut self) -> Result<(), RegCommsError> {
        self.p.signal_path_reset().modify_async(|mut val| {
            val.fifo_flush().set_bit();
            val
        }).await?;
        self.delay.delay_ns(1500).await;
        if self.p.signal_path_reset().read_async().await?.fifo_flush().bit() {
            // Fifo flush failed
            Err(RegCommsError::Other)
        } else {
            Ok(())
        }
    }

    pub async fn configure(&mut self, conf: Config) -> Result<(), RegCommsError> {
        self.poweron_idle().await?;
        if !self.verify_identity().await? {
            return Err(RegCommsError::Other)
        }
        if let Some(accelerometer_config) = conf.accel_config {
            self.configure_accelerometer(accelerometer_config).await?;
        }
        if let Some(gyrometer_config) = conf.gyro_config {
            self.configure_gyrometer(gyrometer_config).await?;
        }
        self.fifo_configure(conf).await?;
        self.conf = conf;
        Ok(())
    }

}
