#[doc = "Register `WSEN` reader"]
pub type R = crate::R<WsenSpec>;
#[doc = "Register `WSEN` writer"]
pub type W = crate::W<WsenSpec>;
#[doc = "Field `WSEN` reader - FMC wait state enable register"]
pub type WsenR = crate::BitReader;
#[doc = "Field `WSEN` writer - FMC wait state enable register"]
pub type WsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FMC wait state enable register"]
    #[inline(always)]
    pub fn wsen(&self) -> WsenR {
        WsenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMC wait state enable register"]
    #[inline(always)]
    #[must_use]
    pub fn wsen(&mut self) -> WsenW<WsenSpec> {
        WsenW::new(self, 0)
    }
}
#[doc = "Wait state enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`wsen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wsen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WsenSpec;
impl crate::RegisterSpec for WsenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wsen::R`](R) reader structure"]
impl crate::Readable for WsenSpec {}
#[doc = "`write(|w| ..)` method takes [`wsen::W`](W) writer structure"]
impl crate::Writable for WsenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WSEN to value 0"]
impl crate::Resettable for WsenSpec {
    const RESET_VALUE: u32 = 0;
}
