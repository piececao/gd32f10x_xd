#[doc = "Register `CH1CTL` reader"]
pub type R = crate::R<Ch1ctlSpec>;
#[doc = "Register `CH1CTL` writer"]
pub type W = crate::W<Ch1ctlSpec>;
#[doc = "Field `CHEN` reader - Channel enable"]
pub type ChenR = crate::BitReader;
#[doc = "Field `CHEN` writer - Channel enable"]
pub type ChenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIE` reader - Enable bit for channel full transfer finish interrupt"]
pub type FtfieR = crate::BitReader;
#[doc = "Field `FTFIE` writer - Enable bit for channel full transfer finish interrupt"]
pub type FtfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIE` reader - Enable bit for channel half transfer finish interrupt"]
pub type HtfieR = crate::BitReader;
#[doc = "Field `HTFIE` writer - Enable bit for channel half transfer finish interrupt"]
pub type HtfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Enable bit for channel error interrupt"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Enable bit for channel error interrupt"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Transfer direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Transfer direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMEN` reader - Circular mode enable"]
pub type CmenR = crate::BitReader;
#[doc = "Field `CMEN` writer - Circular mode enable"]
pub type CmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNAGA` reader - Next address generation algorithm of peripheral"]
pub type PnagaR = crate::BitReader;
#[doc = "Field `PNAGA` writer - Next address generation algorithm of peripheral"]
pub type PnagaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNAGA` reader - Next address generation algorithm of memory"]
pub type MnagaR = crate::BitReader;
#[doc = "Field `MNAGA` writer - Next address generation algorithm of memory"]
pub type MnagaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWIDTH` reader - Transfer data size of peripheral"]
pub type PwidthR = crate::FieldReader;
#[doc = "Field `PWIDTH` writer - Transfer data size of peripheral"]
pub type PwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MWIDTH` reader - Transfer data size of memory"]
pub type MwidthR = crate::FieldReader;
#[doc = "Field `MWIDTH` writer - Transfer data size of memory"]
pub type MwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRIO` reader - Priority level"]
pub type PrioR = crate::FieldReader;
#[doc = "Field `PRIO` writer - Priority level"]
pub type PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M2M` reader - Memory to Memory Mode"]
pub type M2mR = crate::BitReader;
#[doc = "Field `M2M` writer - Memory to Memory Mode"]
pub type M2mW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&self) -> ChenR {
        ChenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable bit for channel full transfer finish interrupt"]
    #[inline(always)]
    pub fn ftfie(&self) -> FtfieR {
        FtfieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for channel half transfer finish interrupt"]
    #[inline(always)]
    pub fn htfie(&self) -> HtfieR {
        HtfieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable bit for channel error interrupt"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    pub fn cmen(&self) -> CmenR {
        CmenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&self) -> PnagaR {
        PnagaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&self) -> MnagaR {
        MnagaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    pub fn pwidth(&self) -> PwidthR {
        PwidthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    pub fn mwidth(&self) -> MwidthR {
        MwidthR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Priority level"]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to Memory Mode"]
    #[inline(always)]
    pub fn m2m(&self) -> M2mR {
        M2mR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> ChenW<Ch1ctlSpec> {
        ChenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable bit for channel full transfer finish interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ftfie(&mut self) -> FtfieW<Ch1ctlSpec> {
        FtfieW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable bit for channel half transfer finish interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn htfie(&mut self) -> HtfieW<Ch1ctlSpec> {
        HtfieW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable bit for channel error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<Ch1ctlSpec> {
        ErrieW::new(self, 3)
    }
    #[doc = "Bit 4 - Transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<Ch1ctlSpec> {
        DirW::new(self, 4)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmen(&mut self) -> CmenW<Ch1ctlSpec> {
        CmenW::new(self, 5)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn pnaga(&mut self) -> PnagaW<Ch1ctlSpec> {
        PnagaW::new(self, 6)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    #[must_use]
    pub fn mnaga(&mut self) -> MnagaW<Ch1ctlSpec> {
        MnagaW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PwidthW<Ch1ctlSpec> {
        PwidthW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    #[must_use]
    pub fn mwidth(&mut self) -> MwidthW<Ch1ctlSpec> {
        MwidthW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Priority level"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PrioW<Ch1ctlSpec> {
        PrioW::new(self, 12)
    }
    #[doc = "Bit 14 - Memory to Memory Mode"]
    #[inline(always)]
    #[must_use]
    pub fn m2m(&mut self) -> M2mW<Ch1ctlSpec> {
        M2mW::new(self, 14)
    }
}
#[doc = "Channel 1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1ctlSpec;
impl crate::RegisterSpec for Ch1ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1ctl::R`](R) reader structure"]
impl crate::Readable for Ch1ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1ctl::W`](W) writer structure"]
impl crate::Writable for Ch1ctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1CTL to value 0"]
impl crate::Resettable for Ch1ctlSpec {
    const RESET_VALUE: u32 = 0;
}
