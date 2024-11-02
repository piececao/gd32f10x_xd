#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Field `GIF0` reader - Global interrupt flag of channel 0"]
pub type Gif0R = crate::BitReader;
#[doc = "Field `FTFIF0` reader - Full Transfer finish flag of channe 0"]
pub type Ftfif0R = crate::BitReader;
#[doc = "Field `HTFIF0` reader - Half transfer finish flag of channel 0"]
pub type Htfif0R = crate::BitReader;
#[doc = "Field `ERRIF0` reader - Error flag of channel 0"]
pub type Errif0R = crate::BitReader;
#[doc = "Field `GIF1` reader - Global interrupt flag of channel 1"]
pub type Gif1R = crate::BitReader;
#[doc = "Field `FTFIF1` reader - Full Transfer finish flag of channe 1"]
pub type Ftfif1R = crate::BitReader;
#[doc = "Field `HTFIF1` reader - Half transfer finish flag of channel 1"]
pub type Htfif1R = crate::BitReader;
#[doc = "Field `ERRIF1` reader - Error flag of channel 1"]
pub type Errif1R = crate::BitReader;
#[doc = "Field `GIF2` reader - Global interrupt flag of channel 2"]
pub type Gif2R = crate::BitReader;
#[doc = "Field `FTFIF2` reader - Full Transfer finish flag of channe 2"]
pub type Ftfif2R = crate::BitReader;
#[doc = "Field `HTFIF2` reader - Half transfer finish flag of channel 2"]
pub type Htfif2R = crate::BitReader;
#[doc = "Field `ERRIF2` reader - Error flag of channel 2"]
pub type Errif2R = crate::BitReader;
#[doc = "Field `GIF3` reader - Global interrupt flag of channel 3"]
pub type Gif3R = crate::BitReader;
#[doc = "Field `FTFIF3` reader - Full Transfer finish flag of channe 3"]
pub type Ftfif3R = crate::BitReader;
#[doc = "Field `HTFIF3` reader - Half transfer finish flag of channel 3"]
pub type Htfif3R = crate::BitReader;
#[doc = "Field `ERRIF3` reader - Error flag of channel 3"]
pub type Errif3R = crate::BitReader;
#[doc = "Field `GIF4` reader - Global interrupt flag of channel 4"]
pub type Gif4R = crate::BitReader;
#[doc = "Field `FTFIF4` reader - Full Transfer finish flag of channe 4"]
pub type Ftfif4R = crate::BitReader;
#[doc = "Field `HTFIF4` reader - Half transfer finish flag of channel 4"]
pub type Htfif4R = crate::BitReader;
#[doc = "Field `ERRIF4` reader - Error flag of channel 4"]
pub type Errif4R = crate::BitReader;
#[doc = "Field `GIF5` reader - Global interrupt flag of channel 5"]
pub type Gif5R = crate::BitReader;
#[doc = "Field `FTFIF5` reader - Full Transfer finish flag of channe 5"]
pub type Ftfif5R = crate::BitReader;
#[doc = "Field `HTFIF5` reader - Half transfer finish flag of channel 5"]
pub type Htfif5R = crate::BitReader;
#[doc = "Field `ERRIF5` reader - Error flag of channel 5"]
pub type Errif5R = crate::BitReader;
#[doc = "Field `GIF6` reader - Global interrupt flag of channel 6"]
pub type Gif6R = crate::BitReader;
#[doc = "Field `FTFIF6` reader - Full Transfer finish flag of channe 6"]
pub type Ftfif6R = crate::BitReader;
#[doc = "Field `HTFIF6` reader - Half transfer finish flag of channel 6"]
pub type Htfif6R = crate::BitReader;
#[doc = "Field `ERRIF6` reader - Error flag of channel 6"]
pub type Errif6R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Global interrupt flag of channel 0"]
    #[inline(always)]
    pub fn gif0(&self) -> Gif0R {
        Gif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full Transfer finish flag of channe 0"]
    #[inline(always)]
    pub fn ftfif0(&self) -> Ftfif0R {
        Ftfif0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer finish flag of channel 0"]
    #[inline(always)]
    pub fn htfif0(&self) -> Htfif0R {
        Htfif0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error flag of channel 0"]
    #[inline(always)]
    pub fn errif0(&self) -> Errif0R {
        Errif0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Global interrupt flag of channel 1"]
    #[inline(always)]
    pub fn gif1(&self) -> Gif1R {
        Gif1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full Transfer finish flag of channe 1"]
    #[inline(always)]
    pub fn ftfif1(&self) -> Ftfif1R {
        Ftfif1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Half transfer finish flag of channel 1"]
    #[inline(always)]
    pub fn htfif1(&self) -> Htfif1R {
        Htfif1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error flag of channel 1"]
    #[inline(always)]
    pub fn errif1(&self) -> Errif1R {
        Errif1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global interrupt flag of channel 2"]
    #[inline(always)]
    pub fn gif2(&self) -> Gif2R {
        Gif2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Full Transfer finish flag of channe 2"]
    #[inline(always)]
    pub fn ftfif2(&self) -> Ftfif2R {
        Ftfif2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Half transfer finish flag of channel 2"]
    #[inline(always)]
    pub fn htfif2(&self) -> Htfif2R {
        Htfif2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error flag of channel 2"]
    #[inline(always)]
    pub fn errif2(&self) -> Errif2R {
        Errif2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Global interrupt flag of channel 3"]
    #[inline(always)]
    pub fn gif3(&self) -> Gif3R {
        Gif3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Full Transfer finish flag of channe 3"]
    #[inline(always)]
    pub fn ftfif3(&self) -> Ftfif3R {
        Ftfif3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Half transfer finish flag of channel 3"]
    #[inline(always)]
    pub fn htfif3(&self) -> Htfif3R {
        Htfif3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error flag of channel 3"]
    #[inline(always)]
    pub fn errif3(&self) -> Errif3R {
        Errif3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Global interrupt flag of channel 4"]
    #[inline(always)]
    pub fn gif4(&self) -> Gif4R {
        Gif4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Full Transfer finish flag of channe 4"]
    #[inline(always)]
    pub fn ftfif4(&self) -> Ftfif4R {
        Ftfif4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Half transfer finish flag of channel 4"]
    #[inline(always)]
    pub fn htfif4(&self) -> Htfif4R {
        Htfif4R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Error flag of channel 4"]
    #[inline(always)]
    pub fn errif4(&self) -> Errif4R {
        Errif4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Global interrupt flag of channel 5"]
    #[inline(always)]
    pub fn gif5(&self) -> Gif5R {
        Gif5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Full Transfer finish flag of channe 5"]
    #[inline(always)]
    pub fn ftfif5(&self) -> Ftfif5R {
        Ftfif5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Half transfer finish flag of channel 5"]
    #[inline(always)]
    pub fn htfif5(&self) -> Htfif5R {
        Htfif5R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error flag of channel 5"]
    #[inline(always)]
    pub fn errif5(&self) -> Errif5R {
        Errif5R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Global interrupt flag of channel 6"]
    #[inline(always)]
    pub fn gif6(&self) -> Gif6R {
        Gif6R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Full Transfer finish flag of channe 6"]
    #[inline(always)]
    pub fn ftfif6(&self) -> Ftfif6R {
        Ftfif6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Half transfer finish flag of channel 6"]
    #[inline(always)]
    pub fn htfif6(&self) -> Htfif6R {
        Htfif6R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Error flag of channel 6"]
    #[inline(always)]
    pub fn errif6(&self) -> Errif6R {
        Errif6R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {
    const RESET_VALUE: u32 = 0;
}
