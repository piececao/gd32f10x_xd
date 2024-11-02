#[doc = "Register `STAT0` reader"]
pub type R = crate::R<Stat0Spec>;
#[doc = "Register `STAT0` writer"]
pub type W = crate::W<Stat0Spec>;
#[doc = "Field `BUSY` reader - The flash is busy bit"]
pub type BusyR = crate::BitReader;
#[doc = "Field `PGERR` reader - Program error flag bit"]
pub type PgerrR = crate::BitReader;
#[doc = "Field `PGERR` writer - Program error flag bit"]
pub type PgerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPERR` reader - Erase/Program protection error flag bit"]
pub type WperrR = crate::BitReader;
#[doc = "Field `WPERR` writer - Erase/Program protection error flag bit"]
pub type WperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDF` reader - End of operation flag bit"]
pub type EndfR = crate::BitReader;
#[doc = "Field `ENDF` writer - End of operation flag bit"]
pub type EndfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The flash is busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Program error flag bit"]
    #[inline(always)]
    pub fn pgerr(&self) -> PgerrR {
        PgerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    pub fn wperr(&self) -> WperrR {
        WperrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of operation flag bit"]
    #[inline(always)]
    pub fn endf(&self) -> EndfR {
        EndfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Program error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn pgerr(&mut self) -> PgerrW<Stat0Spec> {
        PgerrW::new(self, 2)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn wperr(&mut self) -> WperrW<Stat0Spec> {
        WperrW::new(self, 4)
    }
    #[doc = "Bit 5 - End of operation flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn endf(&mut self) -> EndfW<Stat0Spec> {
        EndfW::new(self, 5)
    }
}
#[doc = "Status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`stat0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat0Spec;
impl crate::RegisterSpec for Stat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for Stat0Spec {}
#[doc = "`write(|w| ..)` method takes [`stat0::W`](W) writer structure"]
impl crate::Writable for Stat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for Stat0Spec {
    const RESET_VALUE: u32 = 0;
}
