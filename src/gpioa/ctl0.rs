#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `MD0` reader - Port x mode bits (x = 0)"]
pub type Md0R = crate::FieldReader;
#[doc = "Field `MD0` writer - Port x mode bits (x = 0)"]
pub type Md0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL0` reader - Port x configuration bits (x = 0)"]
pub type Ctl0R = crate::FieldReader;
#[doc = "Field `CTL0` writer - Port x configuration bits (x = 0)"]
pub type Ctl0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD1` reader - Port x mode bits (x = 1)"]
pub type Md1R = crate::FieldReader;
#[doc = "Field `MD1` writer - Port x mode bits (x = 1)"]
pub type Md1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL1` reader - Port x configuration bits (x = 1)"]
pub type Ctl1R = crate::FieldReader;
#[doc = "Field `CTL1` writer - Port x configuration bits (x = 1)"]
pub type Ctl1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD2` reader - Port x mode bits (x = 2 )"]
pub type Md2R = crate::FieldReader;
#[doc = "Field `MD2` writer - Port x mode bits (x = 2 )"]
pub type Md2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL2` reader - Port x configuration bits (x = 2)"]
pub type Ctl2R = crate::FieldReader;
#[doc = "Field `CTL2` writer - Port x configuration bits (x = 2)"]
pub type Ctl2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD3` reader - Port x mode bits (x = 3 )"]
pub type Md3R = crate::FieldReader;
#[doc = "Field `MD3` writer - Port x mode bits (x = 3 )"]
pub type Md3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL3` reader - Port x configuration bits (x = 3)"]
pub type Ctl3R = crate::FieldReader;
#[doc = "Field `CTL3` writer - Port x configuration bits (x = 3)"]
pub type Ctl3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD4` reader - Port x mode bits (x = 4)"]
pub type Md4R = crate::FieldReader;
#[doc = "Field `MD4` writer - Port x mode bits (x = 4)"]
pub type Md4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL4` reader - Port x configuration bits (x = 4)"]
pub type Ctl4R = crate::FieldReader;
#[doc = "Field `CTL4` writer - Port x configuration bits (x = 4)"]
pub type Ctl4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD5` reader - Port x mode bits (x = 5)"]
pub type Md5R = crate::FieldReader;
#[doc = "Field `MD5` writer - Port x mode bits (x = 5)"]
pub type Md5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL5` reader - Port x configuration bits (x = 5)"]
pub type Ctl5R = crate::FieldReader;
#[doc = "Field `CTL5` writer - Port x configuration bits (x = 5)"]
pub type Ctl5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD6` reader - Port x mode bits (x = 6)"]
pub type Md6R = crate::FieldReader;
#[doc = "Field `MD6` writer - Port x mode bits (x = 6)"]
pub type Md6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL6` reader - Port x configuration bits (x = 6)"]
pub type Ctl6R = crate::FieldReader;
#[doc = "Field `CTL6` writer - Port x configuration bits (x = 6)"]
pub type Ctl6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD7` reader - Port x mode bits (x = 7)"]
pub type Md7R = crate::FieldReader;
#[doc = "Field `MD7` writer - Port x mode bits (x = 7)"]
pub type Md7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL7` reader - Port x configuration bits (x = 7)"]
pub type Ctl7R = crate::FieldReader;
#[doc = "Field `CTL7` writer - Port x configuration bits (x = 7)"]
pub type Ctl7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x mode bits (x = 0)"]
    #[inline(always)]
    pub fn md0(&self) -> Md0R {
        Md0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    pub fn ctl0(&self) -> Ctl0R {
        Ctl0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 1)"]
    #[inline(always)]
    pub fn md1(&self) -> Md1R {
        Md1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    pub fn ctl1(&self) -> Ctl1R {
        Ctl1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 2 )"]
    #[inline(always)]
    pub fn md2(&self) -> Md2R {
        Md2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    pub fn ctl2(&self) -> Ctl2R {
        Ctl2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 3 )"]
    #[inline(always)]
    pub fn md3(&self) -> Md3R {
        Md3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    pub fn ctl3(&self) -> Ctl3R {
        Ctl3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 4)"]
    #[inline(always)]
    pub fn md4(&self) -> Md4R {
        Md4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 4)"]
    #[inline(always)]
    pub fn ctl4(&self) -> Ctl4R {
        Ctl4R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 5)"]
    #[inline(always)]
    pub fn md5(&self) -> Md5R {
        Md5R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    pub fn ctl5(&self) -> Ctl5R {
        Ctl5R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 6)"]
    #[inline(always)]
    pub fn md6(&self) -> Md6R {
        Md6R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 6)"]
    #[inline(always)]
    pub fn ctl6(&self) -> Ctl6R {
        Ctl6R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 7)"]
    #[inline(always)]
    pub fn md7(&self) -> Md7R {
        Md7R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    pub fn ctl7(&self) -> Ctl7R {
        Ctl7R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x mode bits (x = 0)"]
    #[inline(always)]
    #[must_use]
    pub fn md0(&mut self) -> Md0W<Ctl0Spec> {
        Md0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl0(&mut self) -> Ctl0W<Ctl0Spec> {
        Ctl0W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn md1(&mut self) -> Md1W<Ctl0Spec> {
        Md1W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl1(&mut self) -> Ctl1W<Ctl0Spec> {
        Ctl1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 2 )"]
    #[inline(always)]
    #[must_use]
    pub fn md2(&mut self) -> Md2W<Ctl0Spec> {
        Md2W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl2(&mut self) -> Ctl2W<Ctl0Spec> {
        Ctl2W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 3 )"]
    #[inline(always)]
    #[must_use]
    pub fn md3(&mut self) -> Md3W<Ctl0Spec> {
        Md3W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl3(&mut self) -> Ctl3W<Ctl0Spec> {
        Ctl3W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 4)"]
    #[inline(always)]
    #[must_use]
    pub fn md4(&mut self) -> Md4W<Ctl0Spec> {
        Md4W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 4)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl4(&mut self) -> Ctl4W<Ctl0Spec> {
        Ctl4W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 5)"]
    #[inline(always)]
    #[must_use]
    pub fn md5(&mut self) -> Md5W<Ctl0Spec> {
        Md5W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl5(&mut self) -> Ctl5W<Ctl0Spec> {
        Ctl5W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 6)"]
    #[inline(always)]
    #[must_use]
    pub fn md6(&mut self) -> Md6W<Ctl0Spec> {
        Md6W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 6)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl6(&mut self) -> Ctl6W<Ctl0Spec> {
        Ctl6W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 7)"]
    #[inline(always)]
    #[must_use]
    pub fn md7(&mut self) -> Md7W<Ctl0Spec> {
        Md7W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl7(&mut self) -> Ctl7W<Ctl0Spec> {
        Ctl7W::new(self, 30)
    }
}
#[doc = "port control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0x4444_4444"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
