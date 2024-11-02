#[doc = "Register `BC` writer"]
pub type W = crate::W<BcSpec>;
#[doc = "Field `CR0` writer - Port 0 Clear bit"]
pub type Cr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR1` writer - Port 1 Clear bit"]
pub type Cr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR2` writer - Port 2 Clear bit"]
pub type Cr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR3` writer - Port 3 Clear bit"]
pub type Cr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR4` writer - Port 4 Clear bit"]
pub type Cr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR5` writer - Port 5 Clear bit"]
pub type Cr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR6` writer - Port 6 Clear bit"]
pub type Cr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR7` writer - Port 7 Clear bit"]
pub type Cr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR8` writer - Port 8 Clear bit"]
pub type Cr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR9` writer - Port 9 Clear bit"]
pub type Cr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR10` writer - Port 10 Clear bit"]
pub type Cr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR11` writer - Port 11 Clear bit"]
pub type Cr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR12` writer - Port 12 Clear bit"]
pub type Cr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR13` writer - Port 13 Clear bit"]
pub type Cr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR14` writer - Port 14 Clear bit"]
pub type Cr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR15` writer - Port 15 Clear bit"]
pub type Cr15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port 0 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr0(&mut self) -> Cr0W<BcSpec> {
        Cr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port 1 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr1(&mut self) -> Cr1W<BcSpec> {
        Cr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port 2 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr2(&mut self) -> Cr2W<BcSpec> {
        Cr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port 3 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr3(&mut self) -> Cr3W<BcSpec> {
        Cr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port 4 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr4(&mut self) -> Cr4W<BcSpec> {
        Cr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port 5 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr5(&mut self) -> Cr5W<BcSpec> {
        Cr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port 6 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr6(&mut self) -> Cr6W<BcSpec> {
        Cr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port 7 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr7(&mut self) -> Cr7W<BcSpec> {
        Cr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port 8 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr8(&mut self) -> Cr8W<BcSpec> {
        Cr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port 9 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr9(&mut self) -> Cr9W<BcSpec> {
        Cr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port 10 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr10(&mut self) -> Cr10W<BcSpec> {
        Cr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port 11 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr11(&mut self) -> Cr11W<BcSpec> {
        Cr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port 12 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr12(&mut self) -> Cr12W<BcSpec> {
        Cr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port 13 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr13(&mut self) -> Cr13W<BcSpec> {
        Cr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port 14 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr14(&mut self) -> Cr14W<BcSpec> {
        Cr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port 15 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr15(&mut self) -> Cr15W<BcSpec> {
        Cr15W::new(self, 15)
    }
}
#[doc = "Port bit clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcSpec;
impl crate::RegisterSpec for BcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bc::W`](W) writer structure"]
impl crate::Writable for BcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BC to value 0"]
impl crate::Resettable for BcSpec {
    const RESET_VALUE: u32 = 0;
}
