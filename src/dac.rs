#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: Ctl0,
    swt: Swt,
    out0_r12dh: Out0R12dh,
    out0_l12dh: Out0L12dh,
    out0_r8dh: Out0R8dh,
    out1_r12dh: Out1R12dh,
    out1_l12dh: Out1L12dh,
    out1_r8dh: Out1R8dh,
    dacc_r12dh: DaccR12dh,
    dacc_l12dh: DaccL12dh,
    dacc_r8dh: DaccR8dh,
    out0_do: Out0Do,
    out1_do: Out1Do,
}
impl RegisterBlock {
    #[doc = "0x00 - DACx control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x04 - DACx software trigger register"]
    #[inline(always)]
    pub const fn swt(&self) -> &Swt {
        &self.swt
    }
    #[doc = "0x08 - DACx_OUT0 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn out0_r12dh(&self) -> &Out0R12dh {
        &self.out0_r12dh
    }
    #[doc = "0x0c - DACx_OUT0 12-bit left-aligned data holding register"]
    #[inline(always)]
    pub const fn out0_l12dh(&self) -> &Out0L12dh {
        &self.out0_l12dh
    }
    #[doc = "0x10 - DACx_OUT0 8-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn out0_r8dh(&self) -> &Out0R8dh {
        &self.out0_r8dh
    }
    #[doc = "0x14 - DACx_OUT1 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn out1_r12dh(&self) -> &Out1R12dh {
        &self.out1_r12dh
    }
    #[doc = "0x18 - DACx_OUT1 12-bit left-aligned data holding register"]
    #[inline(always)]
    pub const fn out1_l12dh(&self) -> &Out1L12dh {
        &self.out1_l12dh
    }
    #[doc = "0x1c - DACx_OUT1 8-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn out1_r8dh(&self) -> &Out1R8dh {
        &self.out1_r8dh
    }
    #[doc = "0x20 - DACx concurrent mode 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dacc_r12dh(&self) -> &DaccR12dh {
        &self.dacc_r12dh
    }
    #[doc = "0x24 - DACx concurrent mode 12-bit left-aligned data holding register"]
    #[inline(always)]
    pub const fn dacc_l12dh(&self) -> &DaccL12dh {
        &self.dacc_l12dh
    }
    #[doc = "0x28 - DACx concurrent mode 8-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dacc_r8dh(&self) -> &DaccR8dh {
        &self.dacc_r8dh
    }
    #[doc = "0x2c - DACx_OUT0 data output register"]
    #[inline(always)]
    pub const fn out0_do(&self) -> &Out0Do {
        &self.out0_do
    }
    #[doc = "0x30 - DACx_OUT1 data output register"]
    #[inline(always)]
    pub const fn out1_do(&self) -> &Out1Do {
        &self.out1_do
    }
}
#[doc = "CTL0 (rw) register accessor: DACx control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "DACx control register 0"]
pub mod ctl0;
#[doc = "SWT (w) register accessor: DACx software trigger register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swt`]
module"]
#[doc(alias = "SWT")]
pub type Swt = crate::Reg<swt::SwtSpec>;
#[doc = "DACx software trigger register"]
pub mod swt;
#[doc = "OUT0_R12DH (rw) register accessor: DACx_OUT0 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`out0_r12dh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out0_r12dh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out0_r12dh`]
module"]
#[doc(alias = "OUT0_R12DH")]
pub type Out0R12dh = crate::Reg<out0_r12dh::Out0R12dhSpec>;
#[doc = "DACx_OUT0 12-bit right-aligned data holding register"]
pub mod out0_r12dh;
#[doc = "OUT0_L12DH (rw) register accessor: DACx_OUT0 12-bit left-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`out0_l12dh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out0_l12dh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out0_l12dh`]
module"]
#[doc(alias = "OUT0_L12DH")]
pub type Out0L12dh = crate::Reg<out0_l12dh::Out0L12dhSpec>;
#[doc = "DACx_OUT0 12-bit left-aligned data holding register"]
pub mod out0_l12dh;
#[doc = "OUT0_R8DH (rw) register accessor: DACx_OUT0 8-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`out0_r8dh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out0_r8dh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out0_r8dh`]
module"]
#[doc(alias = "OUT0_R8DH")]
pub type Out0R8dh = crate::Reg<out0_r8dh::Out0R8dhSpec>;
#[doc = "DACx_OUT0 8-bit right-aligned data holding register"]
pub mod out0_r8dh;
#[doc = "OUT1_R12DH (rw) register accessor: DACx_OUT1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`out1_r12dh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_r12dh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_r12dh`]
module"]
#[doc(alias = "OUT1_R12DH")]
pub type Out1R12dh = crate::Reg<out1_r12dh::Out1R12dhSpec>;
#[doc = "DACx_OUT1 12-bit right-aligned data holding register"]
pub mod out1_r12dh;
#[doc = "OUT1_L12DH (rw) register accessor: DACx_OUT1 12-bit left-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`out1_l12dh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_l12dh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_l12dh`]
module"]
#[doc(alias = "OUT1_L12DH")]
pub type Out1L12dh = crate::Reg<out1_l12dh::Out1L12dhSpec>;
#[doc = "DACx_OUT1 12-bit left-aligned data holding register"]
pub mod out1_l12dh;
#[doc = "OUT1_R8DH (rw) register accessor: DACx_OUT1 8-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`out1_r8dh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_r8dh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_r8dh`]
module"]
#[doc(alias = "OUT1_R8DH")]
pub type Out1R8dh = crate::Reg<out1_r8dh::Out1R8dhSpec>;
#[doc = "DACx_OUT1 8-bit right-aligned data holding register"]
pub mod out1_r8dh;
#[doc = "DACC_R12DH (rw) register accessor: DACx concurrent mode 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacc_r12dh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacc_r12dh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacc_r12dh`]
module"]
#[doc(alias = "DACC_R12DH")]
pub type DaccR12dh = crate::Reg<dacc_r12dh::DaccR12dhSpec>;
#[doc = "DACx concurrent mode 12-bit right-aligned data holding register"]
pub mod dacc_r12dh;
#[doc = "DACC_L12DH (rw) register accessor: DACx concurrent mode 12-bit left-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacc_l12dh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacc_l12dh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacc_l12dh`]
module"]
#[doc(alias = "DACC_L12DH")]
pub type DaccL12dh = crate::Reg<dacc_l12dh::DaccL12dhSpec>;
#[doc = "DACx concurrent mode 12-bit left-aligned data holding register"]
pub mod dacc_l12dh;
#[doc = "DACC_R8DH (rw) register accessor: DACx concurrent mode 8-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacc_r8dh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacc_r8dh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacc_r8dh`]
module"]
#[doc(alias = "DACC_R8DH")]
pub type DaccR8dh = crate::Reg<dacc_r8dh::DaccR8dhSpec>;
#[doc = "DACx concurrent mode 8-bit right-aligned data holding register"]
pub mod dacc_r8dh;
#[doc = "OUT0_DO (r) register accessor: DACx_OUT0 data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`out0_do::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out0_do`]
module"]
#[doc(alias = "OUT0_DO")]
pub type Out0Do = crate::Reg<out0_do::Out0DoSpec>;
#[doc = "DACx_OUT0 data output register"]
pub mod out0_do;
#[doc = "OUT1_DO (r) register accessor: DACx_OUT1 data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`out1_do::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_do`]
module"]
#[doc(alias = "OUT1_DO")]
pub type Out1Do = crate::Reg<out1_do::Out1DoSpec>;
#[doc = "DACx_OUT1 data output register"]
pub mod out1_do;
