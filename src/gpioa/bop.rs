#[doc = "Register `BOP` writer"]
pub type W = crate::W<BopSpec>;
#[doc = "Field `BOP0` writer - Port 0 Set bit"]
pub type Bop0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP1` writer - Port 1 Set bit"]
pub type Bop1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP2` writer - Port 2 Set bit"]
pub type Bop2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP3` writer - Port 3 Set bit"]
pub type Bop3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP4` writer - Port 4 Set bit"]
pub type Bop4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP5` writer - Port 5 Set bit"]
pub type Bop5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP6` writer - Port 6 Set bit"]
pub type Bop6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP7` writer - Port 7 Set bit"]
pub type Bop7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP8` writer - Port 8 Set bit"]
pub type Bop8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP9` writer - Port 9 Set bit"]
pub type Bop9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP10` writer - Port 10 Set bit"]
pub type Bop10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP11` writer - Port 11 Set bit"]
pub type Bop11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP12` writer - Port 12 Set bit"]
pub type Bop12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP13` writer - Port 13 Set bit"]
pub type Bop13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP14` writer - Port 14 Set bit"]
pub type Bop14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP15` writer - Port 15 Set bit"]
pub type Bop15W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - Port 0 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop0(&mut self) -> Bop0W<BopSpec> {
        Bop0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port 1 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop1(&mut self) -> Bop1W<BopSpec> {
        Bop1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port 2 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop2(&mut self) -> Bop2W<BopSpec> {
        Bop2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port 3 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop3(&mut self) -> Bop3W<BopSpec> {
        Bop3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port 4 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop4(&mut self) -> Bop4W<BopSpec> {
        Bop4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port 5 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop5(&mut self) -> Bop5W<BopSpec> {
        Bop5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port 6 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop6(&mut self) -> Bop6W<BopSpec> {
        Bop6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port 7 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop7(&mut self) -> Bop7W<BopSpec> {
        Bop7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port 8 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop8(&mut self) -> Bop8W<BopSpec> {
        Bop8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port 9 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop9(&mut self) -> Bop9W<BopSpec> {
        Bop9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port 10 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop10(&mut self) -> Bop10W<BopSpec> {
        Bop10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port 11 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop11(&mut self) -> Bop11W<BopSpec> {
        Bop11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port 12 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop12(&mut self) -> Bop12W<BopSpec> {
        Bop12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port 13 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop13(&mut self) -> Bop13W<BopSpec> {
        Bop13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port 14 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop14(&mut self) -> Bop14W<BopSpec> {
        Bop14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port 15 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop15(&mut self) -> Bop15W<BopSpec> {
        Bop15W::new(self, 15)
    }
    #[doc = "Bit 16 - Port 0 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr0(&mut self) -> Cr0W<BopSpec> {
        Cr0W::new(self, 16)
    }
    #[doc = "Bit 17 - Port 1 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr1(&mut self) -> Cr1W<BopSpec> {
        Cr1W::new(self, 17)
    }
    #[doc = "Bit 18 - Port 2 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr2(&mut self) -> Cr2W<BopSpec> {
        Cr2W::new(self, 18)
    }
    #[doc = "Bit 19 - Port 3 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr3(&mut self) -> Cr3W<BopSpec> {
        Cr3W::new(self, 19)
    }
    #[doc = "Bit 20 - Port 4 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr4(&mut self) -> Cr4W<BopSpec> {
        Cr4W::new(self, 20)
    }
    #[doc = "Bit 21 - Port 5 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr5(&mut self) -> Cr5W<BopSpec> {
        Cr5W::new(self, 21)
    }
    #[doc = "Bit 22 - Port 6 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr6(&mut self) -> Cr6W<BopSpec> {
        Cr6W::new(self, 22)
    }
    #[doc = "Bit 23 - Port 7 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr7(&mut self) -> Cr7W<BopSpec> {
        Cr7W::new(self, 23)
    }
    #[doc = "Bit 24 - Port 8 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr8(&mut self) -> Cr8W<BopSpec> {
        Cr8W::new(self, 24)
    }
    #[doc = "Bit 25 - Port 9 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr9(&mut self) -> Cr9W<BopSpec> {
        Cr9W::new(self, 25)
    }
    #[doc = "Bit 26 - Port 10 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr10(&mut self) -> Cr10W<BopSpec> {
        Cr10W::new(self, 26)
    }
    #[doc = "Bit 27 - Port 11 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr11(&mut self) -> Cr11W<BopSpec> {
        Cr11W::new(self, 27)
    }
    #[doc = "Bit 28 - Port 12 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr12(&mut self) -> Cr12W<BopSpec> {
        Cr12W::new(self, 28)
    }
    #[doc = "Bit 29 - Port 13 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr13(&mut self) -> Cr13W<BopSpec> {
        Cr13W::new(self, 29)
    }
    #[doc = "Bit 30 - Port 14 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr14(&mut self) -> Cr14W<BopSpec> {
        Cr14W::new(self, 30)
    }
    #[doc = "Bit 31 - Port 15 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr15(&mut self) -> Cr15W<BopSpec> {
        Cr15W::new(self, 31)
    }
}
#[doc = "Port bit operate register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BopSpec;
impl crate::RegisterSpec for BopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bop::W`](W) writer structure"]
impl crate::Writable for BopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOP to value 0"]
impl crate::Resettable for BopSpec {
    const RESET_VALUE: u32 = 0;
}
