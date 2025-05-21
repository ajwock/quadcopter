#![no_std]
use core::result::Result;
mod rdy;
mod device_config;
mod signal_path_reset;
mod drive_config1;
mod drive_config2;
mod drive_config3;
mod int_config;
mod temp_data;
mod accel_data_x;
mod accel_data_y;
mod accel_data_z;
mod gyro_data_x;
mod gyro_data_y;
mod gyro_data_z;
mod tmst_fsynch;
mod apex_data4;
mod apex_data5;
mod pwr_mgmt0;
mod gyro_config0;
mod accel_config0;
mod temp_config0;
mod gyro_config1;
mod accel_config1;
mod apex_config0;
mod apex_config1;
mod wom_config;
mod fifo_config1;
mod fifo_config2;
mod fifo_config3;
mod int_source0;
mod int_source1;
mod int_source3;
mod int_source4;
mod fifo_lost_pkt0;
mod fifo_lost_pkt1;
mod apex_data0;
mod apex_data1;
mod apex_data2;
mod apex_data3;
mod intf_config0;
mod intf_config1;
mod int_status_drdy;
mod int_status;
mod int_status2;
mod int_status3;
mod fifo_count;
mod fifo_data;
mod who_am_i;
mod blk_sel_w;
mod maddr_w;
mod m_w;
mod blk_sel_r;
mod maddr_r;
mod m_r;
use regcomms::{RegComms, RegCommsError};
pub enum AccessProc {
    Standard,
}
pub struct Icm42670P<C: RegComms<1, u8>>(pub C);
impl<C: RegComms<1, u8>> Icm42670P<C> {
    pub fn comms_read(&mut self, reg_address: u8, buf: &mut [u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        self.0.comms_read(reg_address, buf)
    }
    pub fn comms_write(&mut self, reg_address: u8, buf: &[u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        self.0.comms_write(reg_address, buf)
    }
    pub async fn comms_read_async(&mut self, reg_address: u8, buf: &mut [u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        self.0.comms_read_async(reg_address, buf).await
    }
    pub async fn comms_write_async(&mut self, reg_address: u8, buf: &[u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        self.0.comms_write_async(reg_address, buf).await
    }
    pub fn rdy<'a>(&'a mut self) -> rdy::Rdy<'a, C> {
        rdy::Rdy(self)
    }
    pub fn device_config<'a>(&'a mut self) -> device_config::DeviceConfig<'a, C> {
        device_config::DeviceConfig(self)
    }
    pub fn signal_path_reset<'a>(&'a mut self) -> signal_path_reset::SignalPathReset<'a, C> {
        signal_path_reset::SignalPathReset(self)
    }
    pub fn drive_config1<'a>(&'a mut self) -> drive_config1::DriveConfig1<'a, C> {
        drive_config1::DriveConfig1(self)
    }
    pub fn drive_config2<'a>(&'a mut self) -> drive_config2::DriveConfig2<'a, C> {
        drive_config2::DriveConfig2(self)
    }
    pub fn drive_config3<'a>(&'a mut self) -> drive_config3::DriveConfig3<'a, C> {
        drive_config3::DriveConfig3(self)
    }
    pub fn int_config<'a>(&'a mut self) -> int_config::IntConfig<'a, C> {
        int_config::IntConfig(self)
    }
    pub fn temp_data<'a>(&'a mut self) -> temp_data::TempData<'a, C> {
        temp_data::TempData(self)
    }
    pub fn accel_data_x<'a>(&'a mut self) -> accel_data_x::AccelDataX<'a, C> {
        accel_data_x::AccelDataX(self)
    }
    pub fn accel_data_y<'a>(&'a mut self) -> accel_data_y::AccelDataY<'a, C> {
        accel_data_y::AccelDataY(self)
    }
    pub fn accel_data_z<'a>(&'a mut self) -> accel_data_z::AccelDataZ<'a, C> {
        accel_data_z::AccelDataZ(self)
    }
    pub fn gyro_data_x<'a>(&'a mut self) -> gyro_data_x::GyroDataX<'a, C> {
        gyro_data_x::GyroDataX(self)
    }
    pub fn gyro_data_y<'a>(&'a mut self) -> gyro_data_y::GyroDataY<'a, C> {
        gyro_data_y::GyroDataY(self)
    }
    pub fn gyro_data_z<'a>(&'a mut self) -> gyro_data_z::GyroDataZ<'a, C> {
        gyro_data_z::GyroDataZ(self)
    }
    pub fn tmst_fsynch<'a>(&'a mut self) -> tmst_fsynch::TmstFsynch<'a, C> {
        tmst_fsynch::TmstFsynch(self)
    }
    pub fn apex_data4<'a>(&'a mut self) -> apex_data4::ApexData4<'a, C> {
        apex_data4::ApexData4(self)
    }
    pub fn apex_data5<'a>(&'a mut self) -> apex_data5::ApexData5<'a, C> {
        apex_data5::ApexData5(self)
    }
    pub fn pwr_mgmt0<'a>(&'a mut self) -> pwr_mgmt0::PwrMgmt0<'a, C> {
        pwr_mgmt0::PwrMgmt0(self)
    }
    pub fn gyro_config0<'a>(&'a mut self) -> gyro_config0::GyroConfig0<'a, C> {
        gyro_config0::GyroConfig0(self)
    }
    pub fn accel_config0<'a>(&'a mut self) -> accel_config0::AccelConfig0<'a, C> {
        accel_config0::AccelConfig0(self)
    }
    pub fn temp_config0<'a>(&'a mut self) -> temp_config0::TempConfig0<'a, C> {
        temp_config0::TempConfig0(self)
    }
    pub fn gyro_config1<'a>(&'a mut self) -> gyro_config1::GyroConfig1<'a, C> {
        gyro_config1::GyroConfig1(self)
    }
    pub fn accel_config1<'a>(&'a mut self) -> accel_config1::AccelConfig1<'a, C> {
        accel_config1::AccelConfig1(self)
    }
    pub fn apex_config0<'a>(&'a mut self) -> apex_config0::ApexConfig0<'a, C> {
        apex_config0::ApexConfig0(self)
    }
    pub fn apex_config1<'a>(&'a mut self) -> apex_config1::ApexConfig1<'a, C> {
        apex_config1::ApexConfig1(self)
    }
    pub fn wom_config<'a>(&'a mut self) -> wom_config::WomConfig<'a, C> {
        wom_config::WomConfig(self)
    }
    pub fn fifo_config1<'a>(&'a mut self) -> fifo_config1::FifoConfig1<'a, C> {
        fifo_config1::FifoConfig1(self)
    }
    pub fn fifo_config2<'a>(&'a mut self) -> fifo_config2::FifoConfig2<'a, C> {
        fifo_config2::FifoConfig2(self)
    }
    pub fn fifo_config3<'a>(&'a mut self) -> fifo_config3::FifoConfig3<'a, C> {
        fifo_config3::FifoConfig3(self)
    }
    pub fn int_source0<'a>(&'a mut self) -> int_source0::IntSource0<'a, C> {
        int_source0::IntSource0(self)
    }
    pub fn int_source1<'a>(&'a mut self) -> int_source1::IntSource1<'a, C> {
        int_source1::IntSource1(self)
    }
    pub fn int_source3<'a>(&'a mut self) -> int_source3::IntSource3<'a, C> {
        int_source3::IntSource3(self)
    }
    pub fn int_source4<'a>(&'a mut self) -> int_source4::IntSource4<'a, C> {
        int_source4::IntSource4(self)
    }
    pub fn fifo_lost_pkt0<'a>(&'a mut self) -> fifo_lost_pkt0::FifoLostPkt0<'a, C> {
        fifo_lost_pkt0::FifoLostPkt0(self)
    }
    pub fn fifo_lost_pkt1<'a>(&'a mut self) -> fifo_lost_pkt1::FifoLostPkt1<'a, C> {
        fifo_lost_pkt1::FifoLostPkt1(self)
    }
    pub fn apex_data0<'a>(&'a mut self) -> apex_data0::ApexData0<'a, C> {
        apex_data0::ApexData0(self)
    }
    pub fn apex_data1<'a>(&'a mut self) -> apex_data1::ApexData1<'a, C> {
        apex_data1::ApexData1(self)
    }
    pub fn apex_data2<'a>(&'a mut self) -> apex_data2::ApexData2<'a, C> {
        apex_data2::ApexData2(self)
    }
    pub fn apex_data3<'a>(&'a mut self) -> apex_data3::ApexData3<'a, C> {
        apex_data3::ApexData3(self)
    }
    pub fn intf_config0<'a>(&'a mut self) -> intf_config0::IntfConfig0<'a, C> {
        intf_config0::IntfConfig0(self)
    }
    pub fn intf_config1<'a>(&'a mut self) -> intf_config1::IntfConfig1<'a, C> {
        intf_config1::IntfConfig1(self)
    }
    pub fn int_status_drdy<'a>(&'a mut self) -> int_status_drdy::IntStatusDrdy<'a, C> {
        int_status_drdy::IntStatusDrdy(self)
    }
    pub fn int_status<'a>(&'a mut self) -> int_status::IntStatus<'a, C> {
        int_status::IntStatus(self)
    }
    pub fn int_status2<'a>(&'a mut self) -> int_status2::IntStatus2<'a, C> {
        int_status2::IntStatus2(self)
    }
    pub fn int_status3<'a>(&'a mut self) -> int_status3::IntStatus3<'a, C> {
        int_status3::IntStatus3(self)
    }
    pub fn fifo_count<'a>(&'a mut self) -> fifo_count::FifoCount<'a, C> {
        fifo_count::FifoCount(self)
    }
    pub fn fifo_data<'a>(&'a mut self) -> fifo_data::FifoData<'a, C> {
        fifo_data::FifoData(self)
    }
    pub fn who_am_i<'a>(&'a mut self) -> who_am_i::WhoAmI<'a, C> {
        who_am_i::WhoAmI(self)
    }
    pub fn blk_sel_w<'a>(&'a mut self) -> blk_sel_w::BlkSelW<'a, C> {
        blk_sel_w::BlkSelW(self)
    }
    pub fn maddr_w<'a>(&'a mut self) -> maddr_w::MaddrW<'a, C> {
        maddr_w::MaddrW(self)
    }
    pub fn m_w<'a>(&'a mut self) -> m_w::MW<'a, C> {
        m_w::MW(self)
    }
    pub fn blk_sel_r<'a>(&'a mut self) -> blk_sel_r::BlkSelR<'a, C> {
        blk_sel_r::BlkSelR(self)
    }
    pub fn maddr_r<'a>(&'a mut self) -> maddr_r::MaddrR<'a, C> {
        maddr_r::MaddrR(self)
    }
    pub fn m_r<'a>(&'a mut self) -> m_r::MR<'a, C> {
        m_r::MR(self)
    }
}
