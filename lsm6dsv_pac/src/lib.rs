#![no_std]
use core::result::Result;
use core::default::Default;
mod func_cfg_access;
mod pin_ctrl;
mod if_cfg;
mod odr_trig_cfg;
mod fifo_ctrl1;
mod fifo_ctrl2;
mod fifo_ctrl3;
mod fifo_ctrl4;
mod counter_bdr_reg1;
mod counter_bdr_reg2;
mod int1_ctrl;
mod int2_ctrl;
mod who_am_i;
mod ctrl1;
mod ctrl2;
mod ctrl3;
mod ctrl4;
mod ctrl5;
mod ctrl6;
mod ctrl7;
mod ctrl8;
mod ctrl9;
mod ctrl10;
mod ctrl_status;
mod fifo_status1;
mod fifo_status2;
mod all_int_src;
mod status_reg;
mod out_temp;
mod outx_g;
mod outy_g;
mod outz_g;
mod outx_a;
mod outy_a;
mod outz_a;
mod ui_outx_a_hg;
mod ui_outy_a_hg;
mod ui_outz_a_hg;
mod timestamp;
mod ui_status_reg;
mod wake_up_src;
mod tap_src;
mod d6_d_src;
mod status_controller_mainpage;
mod emb_func_status_mainpage;
mod fsm_status_mainpage;
mod mlc_status_mainpage;
mod hg_wake_up_src;
mod ctrl2_xl_hg;
mod ctrl1_xl_hg;
mod internal_freq_fine;
mod functions_enable;
mod hg_functions_enable;
mod hg_wake_up_ths;
mod inactivity_dur;
mod inactivity_ths;
mod tap_cfg0;
mod tap_cfg1;
mod tap_cfg2;
mod tap_ths_6_d;
mod tap_dur;
mod wake_up_ths;
mod wake_up_dur;
mod free_fall;
mod md1_cfg;
mod md2_cfg;
mod haodr_cfg;
mod emb_func_cfg;
mod xl_hg_x_ofs_usr;
mod xl_hg_y_ofs_usr;
mod xl_hg_z_ofs_usr;
mod x_ofs_usr;
mod y_ofs_usr;
mod z_ofs_usr;
mod fifo_data_out_tag;
mod fifo_data_out_x;
mod fifo_data_out_y;
mod fifo_data_out_z;
mod page_sel;
mod emb_func_en_a;
mod emb_func_en_b;
mod emb_func_exec_status;
mod page_address;
mod page_value;
mod emb_func_int1;
mod fsm_int1;
mod mlc_int1;
mod emb_func_int2;
mod fsm_int2;
mod mlc_int2;
mod emb_func_status;
mod fsm_status;
mod mlc_status;
mod page_rw;
mod sflp_gbiasx;
mod sflp_gbiasy;
mod sflp_gbiasz;
mod sflp_gravx;
mod sflp_gravy;
mod sflp_gravz;
mod sflp_quatw;
mod sflp_quatx;
mod sflp_quaty;
mod sflp_quatz;
mod sflp_biasx_init;
mod sflp_biasy_init;
mod sflp_biasz_init;
mod emb_func_fifo_en_a;
mod reg_mapping;
use regcomms::{RegComms, RegCommsError, RegCommsAccessProc};
use spin::once::Once;
#[derive(Default)]
pub struct StandardAccessProc;
impl<C: RegComms<1, u8>> RegCommsAccessProc<Lsm6Dsv<C>, 1, u8> for StandardAccessProc {
    fn proc_read(&self, peripheral: &mut Lsm6Dsv<C>, reg_address: u8, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        peripheral.comms.comms_read(reg_address, buf)
    }
    async fn proc_read_async(&self, peripheral: &mut Lsm6Dsv<C>, reg_address: u8, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        peripheral.comms.comms_read_async(reg_address, buf).await
    }
    fn proc_write(&self, peripheral: &mut Lsm6Dsv<C>, reg_address: u8, buf: &[u8]) -> Result<usize, RegCommsError> {
        peripheral.comms.comms_write(reg_address, buf)
    }
    async fn proc_write_async(&self, peripheral: &mut Lsm6Dsv<C>, reg_address: u8, buf: &[u8]) -> Result<usize, RegCommsError> {
        peripheral.comms.comms_write_async(reg_address, buf).await
    }
}
static EMBEDDED_FUNC: Once<crate::reg_mapping::EmbeddedFunction> = Once::new();
static STANDARD: Once<StandardAccessProc> = Once::new();
pub struct Lsm6Dsv<C: RegComms<1, u8>> {
    comms: C,
    embedded_func: &'static crate::reg_mapping::EmbeddedFunction,
    standard: &'static StandardAccessProc,
}
impl<C: RegComms<1, u8>> Lsm6Dsv<C> {
    pub fn new(comms: C) -> Self {
        Self {
             comms,
            embedded_func: EMBEDDED_FUNC.call_once(|| Default::default()),
            standard: STANDARD.call_once(|| Default::default()),
        }
    }
    pub fn func_cfg_access<'a>(&'a mut self) -> func_cfg_access::FuncCfgAccess<'a, C> {
        func_cfg_access::FuncCfgAccess(self)
    }
    pub fn pin_ctrl<'a>(&'a mut self) -> pin_ctrl::PinCtrl<'a, C> {
        pin_ctrl::PinCtrl(self)
    }
    pub fn if_cfg<'a>(&'a mut self) -> if_cfg::IfCfg<'a, C> {
        if_cfg::IfCfg(self)
    }
    pub fn odr_trig_cfg<'a>(&'a mut self) -> odr_trig_cfg::OdrTrigCfg<'a, C> {
        odr_trig_cfg::OdrTrigCfg(self)
    }
    pub fn fifo_ctrl1<'a>(&'a mut self) -> fifo_ctrl1::FifoCtrl1<'a, C> {
        fifo_ctrl1::FifoCtrl1(self)
    }
    pub fn fifo_ctrl2<'a>(&'a mut self) -> fifo_ctrl2::FifoCtrl2<'a, C> {
        fifo_ctrl2::FifoCtrl2(self)
    }
    pub fn fifo_ctrl3<'a>(&'a mut self) -> fifo_ctrl3::FifoCtrl3<'a, C> {
        fifo_ctrl3::FifoCtrl3(self)
    }
    pub fn fifo_ctrl4<'a>(&'a mut self) -> fifo_ctrl4::FifoCtrl4<'a, C> {
        fifo_ctrl4::FifoCtrl4(self)
    }
    pub fn counter_bdr_reg1<'a>(&'a mut self) -> counter_bdr_reg1::CounterBdrReg1<'a, C> {
        counter_bdr_reg1::CounterBdrReg1(self)
    }
    pub fn counter_bdr_reg2<'a>(&'a mut self) -> counter_bdr_reg2::CounterBdrReg2<'a, C> {
        counter_bdr_reg2::CounterBdrReg2(self)
    }
    pub fn int1_ctrl<'a>(&'a mut self) -> int1_ctrl::Int1Ctrl<'a, C> {
        int1_ctrl::Int1Ctrl(self)
    }
    pub fn int2_ctrl<'a>(&'a mut self) -> int2_ctrl::Int2Ctrl<'a, C> {
        int2_ctrl::Int2Ctrl(self)
    }
    pub fn who_am_i<'a>(&'a mut self) -> who_am_i::WhoAmI<'a, C> {
        who_am_i::WhoAmI(self)
    }
    pub fn ctrl1<'a>(&'a mut self) -> ctrl1::Ctrl1<'a, C> {
        ctrl1::Ctrl1(self)
    }
    pub fn ctrl2<'a>(&'a mut self) -> ctrl2::Ctrl2<'a, C> {
        ctrl2::Ctrl2(self)
    }
    pub fn ctrl3<'a>(&'a mut self) -> ctrl3::Ctrl3<'a, C> {
        ctrl3::Ctrl3(self)
    }
    pub fn ctrl4<'a>(&'a mut self) -> ctrl4::Ctrl4<'a, C> {
        ctrl4::Ctrl4(self)
    }
    pub fn ctrl5<'a>(&'a mut self) -> ctrl5::Ctrl5<'a, C> {
        ctrl5::Ctrl5(self)
    }
    pub fn ctrl6<'a>(&'a mut self) -> ctrl6::Ctrl6<'a, C> {
        ctrl6::Ctrl6(self)
    }
    pub fn ctrl7<'a>(&'a mut self) -> ctrl7::Ctrl7<'a, C> {
        ctrl7::Ctrl7(self)
    }
    pub fn ctrl8<'a>(&'a mut self) -> ctrl8::Ctrl8<'a, C> {
        ctrl8::Ctrl8(self)
    }
    pub fn ctrl9<'a>(&'a mut self) -> ctrl9::Ctrl9<'a, C> {
        ctrl9::Ctrl9(self)
    }
    pub fn ctrl10<'a>(&'a mut self) -> ctrl10::Ctrl10<'a, C> {
        ctrl10::Ctrl10(self)
    }
    pub fn ctrl_status<'a>(&'a mut self) -> ctrl_status::CtrlStatus<'a, C> {
        ctrl_status::CtrlStatus(self)
    }
    pub fn fifo_status1<'a>(&'a mut self) -> fifo_status1::FifoStatus1<'a, C> {
        fifo_status1::FifoStatus1(self)
    }
    pub fn fifo_status2<'a>(&'a mut self) -> fifo_status2::FifoStatus2<'a, C> {
        fifo_status2::FifoStatus2(self)
    }
    pub fn all_int_src<'a>(&'a mut self) -> all_int_src::AllIntSrc<'a, C> {
        all_int_src::AllIntSrc(self)
    }
    pub fn status_reg<'a>(&'a mut self) -> status_reg::StatusReg<'a, C> {
        status_reg::StatusReg(self)
    }
    pub fn out_temp<'a>(&'a mut self) -> out_temp::OutTemp<'a, C> {
        out_temp::OutTemp(self)
    }
    pub fn outx_g<'a>(&'a mut self) -> outx_g::OutxG<'a, C> {
        outx_g::OutxG(self)
    }
    pub fn outy_g<'a>(&'a mut self) -> outy_g::OutyG<'a, C> {
        outy_g::OutyG(self)
    }
    pub fn outz_g<'a>(&'a mut self) -> outz_g::OutzG<'a, C> {
        outz_g::OutzG(self)
    }
    pub fn outx_a<'a>(&'a mut self) -> outx_a::OutxA<'a, C> {
        outx_a::OutxA(self)
    }
    pub fn outy_a<'a>(&'a mut self) -> outy_a::OutyA<'a, C> {
        outy_a::OutyA(self)
    }
    pub fn outz_a<'a>(&'a mut self) -> outz_a::OutzA<'a, C> {
        outz_a::OutzA(self)
    }
    pub fn ui_outx_a_hg<'a>(&'a mut self) -> ui_outx_a_hg::UiOutxAHg<'a, C> {
        ui_outx_a_hg::UiOutxAHg(self)
    }
    pub fn ui_outy_a_hg<'a>(&'a mut self) -> ui_outy_a_hg::UiOutyAHg<'a, C> {
        ui_outy_a_hg::UiOutyAHg(self)
    }
    pub fn ui_outz_a_hg<'a>(&'a mut self) -> ui_outz_a_hg::UiOutzAHg<'a, C> {
        ui_outz_a_hg::UiOutzAHg(self)
    }
    pub fn timestamp<'a>(&'a mut self) -> timestamp::Timestamp<'a, C> {
        timestamp::Timestamp(self)
    }
    pub fn ui_status_reg<'a>(&'a mut self) -> ui_status_reg::UiStatusReg<'a, C> {
        ui_status_reg::UiStatusReg(self)
    }
    pub fn wake_up_src<'a>(&'a mut self) -> wake_up_src::WakeUpSrc<'a, C> {
        wake_up_src::WakeUpSrc(self)
    }
    pub fn tap_src<'a>(&'a mut self) -> tap_src::TapSrc<'a, C> {
        tap_src::TapSrc(self)
    }
    pub fn d6_d_src<'a>(&'a mut self) -> d6_d_src::D6DSrc<'a, C> {
        d6_d_src::D6DSrc(self)
    }
    pub fn status_controller_mainpage<'a>(&'a mut self) -> status_controller_mainpage::StatusControllerMainpage<'a, C> {
        status_controller_mainpage::StatusControllerMainpage(self)
    }
    pub fn emb_func_status_mainpage<'a>(&'a mut self) -> emb_func_status_mainpage::EmbFuncStatusMainpage<'a, C> {
        emb_func_status_mainpage::EmbFuncStatusMainpage(self)
    }
    pub fn fsm_status_mainpage<'a>(&'a mut self) -> fsm_status_mainpage::FsmStatusMainpage<'a, C> {
        fsm_status_mainpage::FsmStatusMainpage(self)
    }
    pub fn mlc_status_mainpage<'a>(&'a mut self) -> mlc_status_mainpage::MlcStatusMainpage<'a, C> {
        mlc_status_mainpage::MlcStatusMainpage(self)
    }
    pub fn hg_wake_up_src<'a>(&'a mut self) -> hg_wake_up_src::HgWakeUpSrc<'a, C> {
        hg_wake_up_src::HgWakeUpSrc(self)
    }
    pub fn ctrl2_xl_hg<'a>(&'a mut self) -> ctrl2_xl_hg::Ctrl2XlHg<'a, C> {
        ctrl2_xl_hg::Ctrl2XlHg(self)
    }
    pub fn ctrl1_xl_hg<'a>(&'a mut self) -> ctrl1_xl_hg::Ctrl1XlHg<'a, C> {
        ctrl1_xl_hg::Ctrl1XlHg(self)
    }
    pub fn internal_freq_fine<'a>(&'a mut self) -> internal_freq_fine::InternalFreqFine<'a, C> {
        internal_freq_fine::InternalFreqFine(self)
    }
    pub fn functions_enable<'a>(&'a mut self) -> functions_enable::FunctionsEnable<'a, C> {
        functions_enable::FunctionsEnable(self)
    }
    pub fn hg_functions_enable<'a>(&'a mut self) -> hg_functions_enable::HgFunctionsEnable<'a, C> {
        hg_functions_enable::HgFunctionsEnable(self)
    }
    pub fn hg_wake_up_ths<'a>(&'a mut self) -> hg_wake_up_ths::HgWakeUpThs<'a, C> {
        hg_wake_up_ths::HgWakeUpThs(self)
    }
    pub fn inactivity_dur<'a>(&'a mut self) -> inactivity_dur::InactivityDur<'a, C> {
        inactivity_dur::InactivityDur(self)
    }
    pub fn inactivity_ths<'a>(&'a mut self) -> inactivity_ths::InactivityThs<'a, C> {
        inactivity_ths::InactivityThs(self)
    }
    pub fn tap_cfg0<'a>(&'a mut self) -> tap_cfg0::TapCfg0<'a, C> {
        tap_cfg0::TapCfg0(self)
    }
    pub fn tap_cfg1<'a>(&'a mut self) -> tap_cfg1::TapCfg1<'a, C> {
        tap_cfg1::TapCfg1(self)
    }
    pub fn tap_cfg2<'a>(&'a mut self) -> tap_cfg2::TapCfg2<'a, C> {
        tap_cfg2::TapCfg2(self)
    }
    pub fn tap_ths_6_d<'a>(&'a mut self) -> tap_ths_6_d::TapThs6D<'a, C> {
        tap_ths_6_d::TapThs6D(self)
    }
    pub fn tap_dur<'a>(&'a mut self) -> tap_dur::TapDur<'a, C> {
        tap_dur::TapDur(self)
    }
    pub fn wake_up_ths<'a>(&'a mut self) -> wake_up_ths::WakeUpThs<'a, C> {
        wake_up_ths::WakeUpThs(self)
    }
    pub fn wake_up_dur<'a>(&'a mut self) -> wake_up_dur::WakeUpDur<'a, C> {
        wake_up_dur::WakeUpDur(self)
    }
    pub fn free_fall<'a>(&'a mut self) -> free_fall::FreeFall<'a, C> {
        free_fall::FreeFall(self)
    }
    pub fn md1_cfg<'a>(&'a mut self) -> md1_cfg::Md1Cfg<'a, C> {
        md1_cfg::Md1Cfg(self)
    }
    pub fn md2_cfg<'a>(&'a mut self) -> md2_cfg::Md2Cfg<'a, C> {
        md2_cfg::Md2Cfg(self)
    }
    pub fn haodr_cfg<'a>(&'a mut self) -> haodr_cfg::HaodrCfg<'a, C> {
        haodr_cfg::HaodrCfg(self)
    }
    pub fn emb_func_cfg<'a>(&'a mut self) -> emb_func_cfg::EmbFuncCfg<'a, C> {
        emb_func_cfg::EmbFuncCfg(self)
    }
    pub fn xl_hg_x_ofs_usr<'a>(&'a mut self) -> xl_hg_x_ofs_usr::XlHgXOfsUsr<'a, C> {
        xl_hg_x_ofs_usr::XlHgXOfsUsr(self)
    }
    pub fn xl_hg_y_ofs_usr<'a>(&'a mut self) -> xl_hg_y_ofs_usr::XlHgYOfsUsr<'a, C> {
        xl_hg_y_ofs_usr::XlHgYOfsUsr(self)
    }
    pub fn xl_hg_z_ofs_usr<'a>(&'a mut self) -> xl_hg_z_ofs_usr::XlHgZOfsUsr<'a, C> {
        xl_hg_z_ofs_usr::XlHgZOfsUsr(self)
    }
    pub fn x_ofs_usr<'a>(&'a mut self) -> x_ofs_usr::XOfsUsr<'a, C> {
        x_ofs_usr::XOfsUsr(self)
    }
    pub fn y_ofs_usr<'a>(&'a mut self) -> y_ofs_usr::YOfsUsr<'a, C> {
        y_ofs_usr::YOfsUsr(self)
    }
    pub fn z_ofs_usr<'a>(&'a mut self) -> z_ofs_usr::ZOfsUsr<'a, C> {
        z_ofs_usr::ZOfsUsr(self)
    }
    pub fn fifo_data_out_tag<'a>(&'a mut self) -> fifo_data_out_tag::FifoDataOutTag<'a, C> {
        fifo_data_out_tag::FifoDataOutTag(self)
    }
    pub fn fifo_data_out_x<'a>(&'a mut self) -> fifo_data_out_x::FifoDataOutX<'a, C> {
        fifo_data_out_x::FifoDataOutX(self)
    }
    pub fn fifo_data_out_y<'a>(&'a mut self) -> fifo_data_out_y::FifoDataOutY<'a, C> {
        fifo_data_out_y::FifoDataOutY(self)
    }
    pub fn fifo_data_out_z<'a>(&'a mut self) -> fifo_data_out_z::FifoDataOutZ<'a, C> {
        fifo_data_out_z::FifoDataOutZ(self)
    }
    pub fn page_sel<'a>(&'a mut self) -> page_sel::PageSel<'a, C> {
        page_sel::PageSel(self)
    }
    pub fn emb_func_en_a<'a>(&'a mut self) -> emb_func_en_a::EmbFuncEnA<'a, C> {
        emb_func_en_a::EmbFuncEnA(self)
    }
    pub fn emb_func_en_b<'a>(&'a mut self) -> emb_func_en_b::EmbFuncEnB<'a, C> {
        emb_func_en_b::EmbFuncEnB(self)
    }
    pub fn emb_func_exec_status<'a>(&'a mut self) -> emb_func_exec_status::EmbFuncExecStatus<'a, C> {
        emb_func_exec_status::EmbFuncExecStatus(self)
    }
    pub fn page_address<'a>(&'a mut self) -> page_address::PageAddress<'a, C> {
        page_address::PageAddress(self)
    }
    pub fn page_value<'a>(&'a mut self) -> page_value::PageValue<'a, C> {
        page_value::PageValue(self)
    }
    pub fn emb_func_int1<'a>(&'a mut self) -> emb_func_int1::EmbFuncInt1<'a, C> {
        emb_func_int1::EmbFuncInt1(self)
    }
    pub fn fsm_int1<'a>(&'a mut self) -> fsm_int1::FsmInt1<'a, C> {
        fsm_int1::FsmInt1(self)
    }
    pub fn mlc_int1<'a>(&'a mut self) -> mlc_int1::MlcInt1<'a, C> {
        mlc_int1::MlcInt1(self)
    }
    pub fn emb_func_int2<'a>(&'a mut self) -> emb_func_int2::EmbFuncInt2<'a, C> {
        emb_func_int2::EmbFuncInt2(self)
    }
    pub fn fsm_int2<'a>(&'a mut self) -> fsm_int2::FsmInt2<'a, C> {
        fsm_int2::FsmInt2(self)
    }
    pub fn mlc_int2<'a>(&'a mut self) -> mlc_int2::MlcInt2<'a, C> {
        mlc_int2::MlcInt2(self)
    }
    pub fn emb_func_status<'a>(&'a mut self) -> emb_func_status::EmbFuncStatus<'a, C> {
        emb_func_status::EmbFuncStatus(self)
    }
    pub fn fsm_status<'a>(&'a mut self) -> fsm_status::FsmStatus<'a, C> {
        fsm_status::FsmStatus(self)
    }
    pub fn mlc_status<'a>(&'a mut self) -> mlc_status::MlcStatus<'a, C> {
        mlc_status::MlcStatus(self)
    }
    pub fn page_rw<'a>(&'a mut self) -> page_rw::PageRw<'a, C> {
        page_rw::PageRw(self)
    }
    pub fn sflp_gbiasx<'a>(&'a mut self) -> sflp_gbiasx::SflpGbiasx<'a, C> {
        sflp_gbiasx::SflpGbiasx(self)
    }
    pub fn sflp_gbiasy<'a>(&'a mut self) -> sflp_gbiasy::SflpGbiasy<'a, C> {
        sflp_gbiasy::SflpGbiasy(self)
    }
    pub fn sflp_gbiasz<'a>(&'a mut self) -> sflp_gbiasz::SflpGbiasz<'a, C> {
        sflp_gbiasz::SflpGbiasz(self)
    }
    pub fn sflp_gravx<'a>(&'a mut self) -> sflp_gravx::SflpGravx<'a, C> {
        sflp_gravx::SflpGravx(self)
    }
    pub fn sflp_gravy<'a>(&'a mut self) -> sflp_gravy::SflpGravy<'a, C> {
        sflp_gravy::SflpGravy(self)
    }
    pub fn sflp_gravz<'a>(&'a mut self) -> sflp_gravz::SflpGravz<'a, C> {
        sflp_gravz::SflpGravz(self)
    }
    pub fn sflp_quatw<'a>(&'a mut self) -> sflp_quatw::SflpQuatw<'a, C> {
        sflp_quatw::SflpQuatw(self)
    }
    pub fn sflp_quatx<'a>(&'a mut self) -> sflp_quatx::SflpQuatx<'a, C> {
        sflp_quatx::SflpQuatx(self)
    }
    pub fn sflp_quaty<'a>(&'a mut self) -> sflp_quaty::SflpQuaty<'a, C> {
        sflp_quaty::SflpQuaty(self)
    }
    pub fn sflp_quatz<'a>(&'a mut self) -> sflp_quatz::SflpQuatz<'a, C> {
        sflp_quatz::SflpQuatz(self)
    }
    pub fn sflp_biasx_init<'a>(&'a mut self) -> sflp_biasx_init::SflpBiasxInit<'a, C> {
        sflp_biasx_init::SflpBiasxInit(self)
    }
    pub fn sflp_biasy_init<'a>(&'a mut self) -> sflp_biasy_init::SflpBiasyInit<'a, C> {
        sflp_biasy_init::SflpBiasyInit(self)
    }
    pub fn sflp_biasz_init<'a>(&'a mut self) -> sflp_biasz_init::SflpBiaszInit<'a, C> {
        sflp_biasz_init::SflpBiaszInit(self)
    }
    pub fn emb_func_fifo_en_a<'a>(&'a mut self) -> emb_func_fifo_en_a::EmbFuncFifoEnA<'a, C> {
        emb_func_fifo_en_a::EmbFuncFifoEnA(self)
    }
}
