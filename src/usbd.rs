#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ep0cs: Ep0cs,
    ep1cs: Ep1cs,
    ep2cs: Ep2cs,
    ep3cs: Ep3cs,
    ep4cs: Ep4cs,
    ep5cs: Ep5cs,
    ep6cs: Ep6cs,
    ep7cs: Ep7cs,
    _reserved8: [u8; 0x20],
    ctl: Ctl,
    intf: Intf,
    stat: Stat,
    daddr: Daddr,
    baddr: Baddr,
}
impl RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    #[inline(always)]
    pub const fn ep0cs(&self) -> &Ep0cs {
        &self.ep0cs
    }
    #[doc = "0x04 - endpoint 1 register"]
    #[inline(always)]
    pub const fn ep1cs(&self) -> &Ep1cs {
        &self.ep1cs
    }
    #[doc = "0x08 - endpoint 2 register"]
    #[inline(always)]
    pub const fn ep2cs(&self) -> &Ep2cs {
        &self.ep2cs
    }
    #[doc = "0x0c - endpoint 3 register"]
    #[inline(always)]
    pub const fn ep3cs(&self) -> &Ep3cs {
        &self.ep3cs
    }
    #[doc = "0x10 - endpoint 4 register"]
    #[inline(always)]
    pub const fn ep4cs(&self) -> &Ep4cs {
        &self.ep4cs
    }
    #[doc = "0x14 - endpoint 5 register"]
    #[inline(always)]
    pub const fn ep5cs(&self) -> &Ep5cs {
        &self.ep5cs
    }
    #[doc = "0x18 - endpoint 6 register"]
    #[inline(always)]
    pub const fn ep6cs(&self) -> &Ep6cs {
        &self.ep6cs
    }
    #[doc = "0x1c - endpoint 7 register"]
    #[inline(always)]
    pub const fn ep7cs(&self) -> &Ep7cs {
        &self.ep7cs
    }
    #[doc = "0x40 - control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x44 - interrupt flag register"]
    #[inline(always)]
    pub const fn intf(&self) -> &Intf {
        &self.intf
    }
    #[doc = "0x48 - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x4c - device address register"]
    #[inline(always)]
    pub const fn daddr(&self) -> &Daddr {
        &self.daddr
    }
    #[doc = "0x50 - Buffer address register"]
    #[inline(always)]
    pub const fn baddr(&self) -> &Baddr {
        &self.baddr
    }
}
#[doc = "EP0CS (rw) register accessor: endpoint 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0cs`]
module"]
#[doc(alias = "EP0CS")]
pub type Ep0cs = crate::Reg<ep0cs::Ep0csSpec>;
#[doc = "endpoint 0 register"]
pub mod ep0cs;
#[doc = "EP1CS (rw) register accessor: endpoint 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep1cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1cs`]
module"]
#[doc(alias = "EP1CS")]
pub type Ep1cs = crate::Reg<ep1cs::Ep1csSpec>;
#[doc = "endpoint 1 register"]
pub mod ep1cs;
#[doc = "EP2CS (rw) register accessor: endpoint 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep2cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2cs`]
module"]
#[doc(alias = "EP2CS")]
pub type Ep2cs = crate::Reg<ep2cs::Ep2csSpec>;
#[doc = "endpoint 2 register"]
pub mod ep2cs;
#[doc = "EP3CS (rw) register accessor: endpoint 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep3cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3cs`]
module"]
#[doc(alias = "EP3CS")]
pub type Ep3cs = crate::Reg<ep3cs::Ep3csSpec>;
#[doc = "endpoint 3 register"]
pub mod ep3cs;
#[doc = "EP4CS (rw) register accessor: endpoint 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep4cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4cs`]
module"]
#[doc(alias = "EP4CS")]
pub type Ep4cs = crate::Reg<ep4cs::Ep4csSpec>;
#[doc = "endpoint 4 register"]
pub mod ep4cs;
#[doc = "EP5CS (rw) register accessor: endpoint 5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep5cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep5cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep5cs`]
module"]
#[doc(alias = "EP5CS")]
pub type Ep5cs = crate::Reg<ep5cs::Ep5csSpec>;
#[doc = "endpoint 5 register"]
pub mod ep5cs;
#[doc = "EP6CS (rw) register accessor: endpoint 6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep6cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep6cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep6cs`]
module"]
#[doc(alias = "EP6CS")]
pub type Ep6cs = crate::Reg<ep6cs::Ep6csSpec>;
#[doc = "endpoint 6 register"]
pub mod ep6cs;
#[doc = "EP7CS (rw) register accessor: endpoint 7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep7cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep7cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep7cs`]
module"]
#[doc(alias = "EP7CS")]
pub type Ep7cs = crate::Reg<ep7cs::Ep7csSpec>;
#[doc = "endpoint 7 register"]
pub mod ep7cs;
#[doc = "CTL (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "control register"]
pub mod ctl;
#[doc = "INTF (rw) register accessor: interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
#[doc(alias = "INTF")]
pub type Intf = crate::Reg<intf::IntfSpec>;
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "STAT (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register"]
pub mod stat;
#[doc = "DADDR (rw) register accessor: device address register\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr`]
module"]
#[doc(alias = "DADDR")]
pub type Daddr = crate::Reg<daddr::DaddrSpec>;
#[doc = "device address register"]
pub mod daddr;
#[doc = "BADDR (rw) register accessor: Buffer address register\n\nYou can [`read`](crate::Reg::read) this register and get [`baddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baddr`]
module"]
#[doc(alias = "BADDR")]
pub type Baddr = crate::Reg<baddr::BaddrSpec>;
#[doc = "Buffer address register"]
pub mod baddr;
