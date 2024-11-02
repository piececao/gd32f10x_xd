#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: Id,
    ctl: Ctl,
}
impl RegisterBlock {
    #[doc = "0x00 - ID code register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x04 - Control register 0"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
}
#[doc = "ID (r) register accessor: ID code register\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "ID code register"]
pub mod id;
#[doc = "CTL (rw) register accessor: Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control register 0"]
pub mod ctl;
