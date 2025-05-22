#![no_std]
use core::result::Result;
use core::default::Default;
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
mod tmst_config1;
mod fifo_config5;
mod fifo_config6;
mod fsync_config;
mod int_config0;
mod int_config1;
mod sensor_config3;
mod st_config;
mod selftest;
mod intf_config6;
mod intf_config10;
mod intf_config7;
mod otp_config;
mod int_source6;
mod int_source7;
mod int_source8;
mod int_source9;
mod int_source10;
mod apex_config2;
mod apex_config3;
mod apex_config4;
mod apex_config5;
mod apex_config9;
mod apex_config10;
mod apex_config11;
mod accel_wom_x_thr;
mod accel_wom_y_thr;
mod accel_wom_z_thr;
mod offset_user0;
mod offset_user1;
mod offset_user2;
mod offset_user3;
mod offset_user4;
mod offset_user5;
mod offset_user6;
mod offset_user7;
mod offset_user8;
mod st_status1;
mod st_status2;
mod fdr_config;
mod apex_config12;
mod bank_access;
use regcomms::{RegComms, RegCommsError, RegCommsAccessProc};
use spin::once::Once;
#[derive(Default)]
pub struct StandardAccessProc;
impl<D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> RegCommsAccessProc<Icm42670P<D, C>, 1, u8> for StandardAccessProc {
    fn proc_read(&self, peripheral: &mut Icm42670P<D, C>, reg_address: u8, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        peripheral.comms.comms_read(reg_address, buf)
    }
    async fn proc_read_async(&self, peripheral: &mut Icm42670P<D, C>, reg_address: u8, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        peripheral.comms.comms_read_async(reg_address, buf).await
    }
    fn proc_write(&self, peripheral: &mut Icm42670P<D, C>, reg_address: u8, buf: &[u8]) -> Result<usize, RegCommsError> {
        peripheral.comms.comms_write(reg_address, buf)
    }
    async fn proc_write_async(&self, peripheral: &mut Icm42670P<D, C>, reg_address: u8, buf: &[u8]) -> Result<usize, RegCommsError> {
        peripheral.comms.comms_write_async(reg_address, buf).await
    }
}
static MREG_1: Once<crate::bank_access::Mreg1> = Once::new();
static STANDARD: Once<StandardAccessProc> = Once::new();
pub struct Icm42670P<D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> {
    delay: D,
    comms: C,
    mreg_1: &'static crate::bank_access::Mreg1,
    standard: &'static StandardAccessProc,
}
impl<D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> Icm42670P<D, C> {
    pub fn new(delay: D, comms: C) -> Self {
        Self {
             delay,
             comms,
            mreg_1: MREG_1.call_once(|| Default::default()),
            standard: STANDARD.call_once(|| Default::default()),
        }
    }
    pub fn rdy<'a>(&'a mut self) -> rdy::Rdy<'a, D, C> {
        rdy::Rdy(self)
    }
    pub fn device_config<'a>(&'a mut self) -> device_config::DeviceConfig<'a, D, C> {
        device_config::DeviceConfig(self)
    }
    pub fn signal_path_reset<'a>(&'a mut self) -> signal_path_reset::SignalPathReset<'a, D, C> {
        signal_path_reset::SignalPathReset(self)
    }
    pub fn drive_config1<'a>(&'a mut self) -> drive_config1::DriveConfig1<'a, D, C> {
        drive_config1::DriveConfig1(self)
    }
    pub fn drive_config2<'a>(&'a mut self) -> drive_config2::DriveConfig2<'a, D, C> {
        drive_config2::DriveConfig2(self)
    }
    pub fn drive_config3<'a>(&'a mut self) -> drive_config3::DriveConfig3<'a, D, C> {
        drive_config3::DriveConfig3(self)
    }
    pub fn int_config<'a>(&'a mut self) -> int_config::IntConfig<'a, D, C> {
        int_config::IntConfig(self)
    }
    pub fn temp_data<'a>(&'a mut self) -> temp_data::TempData<'a, D, C> {
        temp_data::TempData(self)
    }
    pub fn accel_data_x<'a>(&'a mut self) -> accel_data_x::AccelDataX<'a, D, C> {
        accel_data_x::AccelDataX(self)
    }
    pub fn accel_data_y<'a>(&'a mut self) -> accel_data_y::AccelDataY<'a, D, C> {
        accel_data_y::AccelDataY(self)
    }
    pub fn accel_data_z<'a>(&'a mut self) -> accel_data_z::AccelDataZ<'a, D, C> {
        accel_data_z::AccelDataZ(self)
    }
    pub fn gyro_data_x<'a>(&'a mut self) -> gyro_data_x::GyroDataX<'a, D, C> {
        gyro_data_x::GyroDataX(self)
    }
    pub fn gyro_data_y<'a>(&'a mut self) -> gyro_data_y::GyroDataY<'a, D, C> {
        gyro_data_y::GyroDataY(self)
    }
    pub fn gyro_data_z<'a>(&'a mut self) -> gyro_data_z::GyroDataZ<'a, D, C> {
        gyro_data_z::GyroDataZ(self)
    }
    pub fn tmst_fsynch<'a>(&'a mut self) -> tmst_fsynch::TmstFsynch<'a, D, C> {
        tmst_fsynch::TmstFsynch(self)
    }
    pub fn apex_data4<'a>(&'a mut self) -> apex_data4::ApexData4<'a, D, C> {
        apex_data4::ApexData4(self)
    }
    pub fn apex_data5<'a>(&'a mut self) -> apex_data5::ApexData5<'a, D, C> {
        apex_data5::ApexData5(self)
    }
    pub fn pwr_mgmt0<'a>(&'a mut self) -> pwr_mgmt0::PwrMgmt0<'a, D, C> {
        pwr_mgmt0::PwrMgmt0(self)
    }
    pub fn gyro_config0<'a>(&'a mut self) -> gyro_config0::GyroConfig0<'a, D, C> {
        gyro_config0::GyroConfig0(self)
    }
    pub fn accel_config0<'a>(&'a mut self) -> accel_config0::AccelConfig0<'a, D, C> {
        accel_config0::AccelConfig0(self)
    }
    pub fn temp_config0<'a>(&'a mut self) -> temp_config0::TempConfig0<'a, D, C> {
        temp_config0::TempConfig0(self)
    }
    pub fn gyro_config1<'a>(&'a mut self) -> gyro_config1::GyroConfig1<'a, D, C> {
        gyro_config1::GyroConfig1(self)
    }
    pub fn accel_config1<'a>(&'a mut self) -> accel_config1::AccelConfig1<'a, D, C> {
        accel_config1::AccelConfig1(self)
    }
    pub fn apex_config0<'a>(&'a mut self) -> apex_config0::ApexConfig0<'a, D, C> {
        apex_config0::ApexConfig0(self)
    }
    pub fn apex_config1<'a>(&'a mut self) -> apex_config1::ApexConfig1<'a, D, C> {
        apex_config1::ApexConfig1(self)
    }
    pub fn wom_config<'a>(&'a mut self) -> wom_config::WomConfig<'a, D, C> {
        wom_config::WomConfig(self)
    }
    pub fn fifo_config1<'a>(&'a mut self) -> fifo_config1::FifoConfig1<'a, D, C> {
        fifo_config1::FifoConfig1(self)
    }
    pub fn fifo_config2<'a>(&'a mut self) -> fifo_config2::FifoConfig2<'a, D, C> {
        fifo_config2::FifoConfig2(self)
    }
    pub fn fifo_config3<'a>(&'a mut self) -> fifo_config3::FifoConfig3<'a, D, C> {
        fifo_config3::FifoConfig3(self)
    }
    pub fn int_source0<'a>(&'a mut self) -> int_source0::IntSource0<'a, D, C> {
        int_source0::IntSource0(self)
    }
    pub fn int_source1<'a>(&'a mut self) -> int_source1::IntSource1<'a, D, C> {
        int_source1::IntSource1(self)
    }
    pub fn int_source3<'a>(&'a mut self) -> int_source3::IntSource3<'a, D, C> {
        int_source3::IntSource3(self)
    }
    pub fn int_source4<'a>(&'a mut self) -> int_source4::IntSource4<'a, D, C> {
        int_source4::IntSource4(self)
    }
    pub fn fifo_lost_pkt0<'a>(&'a mut self) -> fifo_lost_pkt0::FifoLostPkt0<'a, D, C> {
        fifo_lost_pkt0::FifoLostPkt0(self)
    }
    pub fn fifo_lost_pkt1<'a>(&'a mut self) -> fifo_lost_pkt1::FifoLostPkt1<'a, D, C> {
        fifo_lost_pkt1::FifoLostPkt1(self)
    }
    pub fn apex_data0<'a>(&'a mut self) -> apex_data0::ApexData0<'a, D, C> {
        apex_data0::ApexData0(self)
    }
    pub fn apex_data1<'a>(&'a mut self) -> apex_data1::ApexData1<'a, D, C> {
        apex_data1::ApexData1(self)
    }
    pub fn apex_data2<'a>(&'a mut self) -> apex_data2::ApexData2<'a, D, C> {
        apex_data2::ApexData2(self)
    }
    pub fn apex_data3<'a>(&'a mut self) -> apex_data3::ApexData3<'a, D, C> {
        apex_data3::ApexData3(self)
    }
    pub fn intf_config0<'a>(&'a mut self) -> intf_config0::IntfConfig0<'a, D, C> {
        intf_config0::IntfConfig0(self)
    }
    pub fn intf_config1<'a>(&'a mut self) -> intf_config1::IntfConfig1<'a, D, C> {
        intf_config1::IntfConfig1(self)
    }
    pub fn int_status_drdy<'a>(&'a mut self) -> int_status_drdy::IntStatusDrdy<'a, D, C> {
        int_status_drdy::IntStatusDrdy(self)
    }
    pub fn int_status<'a>(&'a mut self) -> int_status::IntStatus<'a, D, C> {
        int_status::IntStatus(self)
    }
    pub fn int_status2<'a>(&'a mut self) -> int_status2::IntStatus2<'a, D, C> {
        int_status2::IntStatus2(self)
    }
    pub fn int_status3<'a>(&'a mut self) -> int_status3::IntStatus3<'a, D, C> {
        int_status3::IntStatus3(self)
    }
    pub fn fifo_count<'a>(&'a mut self) -> fifo_count::FifoCount<'a, D, C> {
        fifo_count::FifoCount(self)
    }
    pub fn fifo_data<'a>(&'a mut self) -> fifo_data::FifoData<'a, D, C> {
        fifo_data::FifoData(self)
    }
    pub fn who_am_i<'a>(&'a mut self) -> who_am_i::WhoAmI<'a, D, C> {
        who_am_i::WhoAmI(self)
    }
    pub fn blk_sel_w<'a>(&'a mut self) -> blk_sel_w::BlkSelW<'a, D, C> {
        blk_sel_w::BlkSelW(self)
    }
    pub fn maddr_w<'a>(&'a mut self) -> maddr_w::MaddrW<'a, D, C> {
        maddr_w::MaddrW(self)
    }
    pub fn m_w<'a>(&'a mut self) -> m_w::MW<'a, D, C> {
        m_w::MW(self)
    }
    pub fn blk_sel_r<'a>(&'a mut self) -> blk_sel_r::BlkSelR<'a, D, C> {
        blk_sel_r::BlkSelR(self)
    }
    pub fn maddr_r<'a>(&'a mut self) -> maddr_r::MaddrR<'a, D, C> {
        maddr_r::MaddrR(self)
    }
    pub fn m_r<'a>(&'a mut self) -> m_r::MR<'a, D, C> {
        m_r::MR(self)
    }
    pub fn tmst_config1<'a>(&'a mut self) -> tmst_config1::TmstConfig1<'a, D, C> {
        tmst_config1::TmstConfig1(self)
    }
    pub fn fifo_config5<'a>(&'a mut self) -> fifo_config5::FifoConfig5<'a, D, C> {
        fifo_config5::FifoConfig5(self)
    }
    pub fn fifo_config6<'a>(&'a mut self) -> fifo_config6::FifoConfig6<'a, D, C> {
        fifo_config6::FifoConfig6(self)
    }
    pub fn fsync_config<'a>(&'a mut self) -> fsync_config::FsyncConfig<'a, D, C> {
        fsync_config::FsyncConfig(self)
    }
    pub fn int_config0<'a>(&'a mut self) -> int_config0::IntConfig0<'a, D, C> {
        int_config0::IntConfig0(self)
    }
    pub fn int_config1<'a>(&'a mut self) -> int_config1::IntConfig1<'a, D, C> {
        int_config1::IntConfig1(self)
    }
    pub fn sensor_config3<'a>(&'a mut self) -> sensor_config3::SensorConfig3<'a, D, C> {
        sensor_config3::SensorConfig3(self)
    }
    pub fn st_config<'a>(&'a mut self) -> st_config::StConfig<'a, D, C> {
        st_config::StConfig(self)
    }
    pub fn selftest<'a>(&'a mut self) -> selftest::Selftest<'a, D, C> {
        selftest::Selftest(self)
    }
    pub fn intf_config6<'a>(&'a mut self) -> intf_config6::IntfConfig6<'a, D, C> {
        intf_config6::IntfConfig6(self)
    }
    pub fn intf_config10<'a>(&'a mut self) -> intf_config10::IntfConfig10<'a, D, C> {
        intf_config10::IntfConfig10(self)
    }
    pub fn intf_config7<'a>(&'a mut self) -> intf_config7::IntfConfig7<'a, D, C> {
        intf_config7::IntfConfig7(self)
    }
    pub fn otp_config<'a>(&'a mut self) -> otp_config::OtpConfig<'a, D, C> {
        otp_config::OtpConfig(self)
    }
    pub fn int_source6<'a>(&'a mut self) -> int_source6::IntSource6<'a, D, C> {
        int_source6::IntSource6(self)
    }
    pub fn int_source7<'a>(&'a mut self) -> int_source7::IntSource7<'a, D, C> {
        int_source7::IntSource7(self)
    }
    pub fn int_source8<'a>(&'a mut self) -> int_source8::IntSource8<'a, D, C> {
        int_source8::IntSource8(self)
    }
    pub fn int_source9<'a>(&'a mut self) -> int_source9::IntSource9<'a, D, C> {
        int_source9::IntSource9(self)
    }
    pub fn int_source10<'a>(&'a mut self) -> int_source10::IntSource10<'a, D, C> {
        int_source10::IntSource10(self)
    }
    pub fn apex_config2<'a>(&'a mut self) -> apex_config2::ApexConfig2<'a, D, C> {
        apex_config2::ApexConfig2(self)
    }
    pub fn apex_config3<'a>(&'a mut self) -> apex_config3::ApexConfig3<'a, D, C> {
        apex_config3::ApexConfig3(self)
    }
    pub fn apex_config4<'a>(&'a mut self) -> apex_config4::ApexConfig4<'a, D, C> {
        apex_config4::ApexConfig4(self)
    }
    pub fn apex_config5<'a>(&'a mut self) -> apex_config5::ApexConfig5<'a, D, C> {
        apex_config5::ApexConfig5(self)
    }
    pub fn apex_config9<'a>(&'a mut self) -> apex_config9::ApexConfig9<'a, D, C> {
        apex_config9::ApexConfig9(self)
    }
    pub fn apex_config10<'a>(&'a mut self) -> apex_config10::ApexConfig10<'a, D, C> {
        apex_config10::ApexConfig10(self)
    }
    pub fn apex_config11<'a>(&'a mut self) -> apex_config11::ApexConfig11<'a, D, C> {
        apex_config11::ApexConfig11(self)
    }
    pub fn accel_wom_x_thr<'a>(&'a mut self) -> accel_wom_x_thr::AccelWomXThr<'a, D, C> {
        accel_wom_x_thr::AccelWomXThr(self)
    }
    pub fn accel_wom_y_thr<'a>(&'a mut self) -> accel_wom_y_thr::AccelWomYThr<'a, D, C> {
        accel_wom_y_thr::AccelWomYThr(self)
    }
    pub fn accel_wom_z_thr<'a>(&'a mut self) -> accel_wom_z_thr::AccelWomZThr<'a, D, C> {
        accel_wom_z_thr::AccelWomZThr(self)
    }
    pub fn offset_user0<'a>(&'a mut self) -> offset_user0::OffsetUser0<'a, D, C> {
        offset_user0::OffsetUser0(self)
    }
    pub fn offset_user1<'a>(&'a mut self) -> offset_user1::OffsetUser1<'a, D, C> {
        offset_user1::OffsetUser1(self)
    }
    pub fn offset_user2<'a>(&'a mut self) -> offset_user2::OffsetUser2<'a, D, C> {
        offset_user2::OffsetUser2(self)
    }
    pub fn offset_user3<'a>(&'a mut self) -> offset_user3::OffsetUser3<'a, D, C> {
        offset_user3::OffsetUser3(self)
    }
    pub fn offset_user4<'a>(&'a mut self) -> offset_user4::OffsetUser4<'a, D, C> {
        offset_user4::OffsetUser4(self)
    }
    pub fn offset_user5<'a>(&'a mut self) -> offset_user5::OffsetUser5<'a, D, C> {
        offset_user5::OffsetUser5(self)
    }
    pub fn offset_user6<'a>(&'a mut self) -> offset_user6::OffsetUser6<'a, D, C> {
        offset_user6::OffsetUser6(self)
    }
    pub fn offset_user7<'a>(&'a mut self) -> offset_user7::OffsetUser7<'a, D, C> {
        offset_user7::OffsetUser7(self)
    }
    pub fn offset_user8<'a>(&'a mut self) -> offset_user8::OffsetUser8<'a, D, C> {
        offset_user8::OffsetUser8(self)
    }
    pub fn st_status1<'a>(&'a mut self) -> st_status1::StStatus1<'a, D, C> {
        st_status1::StStatus1(self)
    }
    pub fn st_status2<'a>(&'a mut self) -> st_status2::StStatus2<'a, D, C> {
        st_status2::StStatus2(self)
    }
    pub fn fdr_config<'a>(&'a mut self) -> fdr_config::FdrConfig<'a, D, C> {
        fdr_config::FdrConfig(self)
    }
    pub fn apex_config12<'a>(&'a mut self) -> apex_config12::ApexConfig12<'a, D, C> {
        apex_config12::ApexConfig12(self)
    }
}
