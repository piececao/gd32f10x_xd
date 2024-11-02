#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `MD8` reader - Port x mode bits (x = 8)"]
pub type Md8R = crate::FieldReader;
#[doc = "Field `MD8` writer - Port x mode bits (x = 8)"]
pub type Md8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL8` reader - Port x configuration bits (x = 8)"]
pub type Ctl8R = crate::FieldReader;
#[doc = "Field `CTL8` writer - Port x configuration bits (x = 8)"]
pub type Ctl8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD9` reader - Port x mode bits (x = 9)"]
pub type Md9R = crate::FieldReader;
#[doc = "Field `MD9` writer - Port x mode bits (x = 9)"]
pub type Md9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL9` reader - Port x configuration bits (x = 9)"]
pub type Ctl9R = crate::FieldReader;
#[doc = "Field `CTL9` writer - Port x configuration bits (x = 9)"]
pub type Ctl9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD10` reader - Port x mode bits (x = 10 )"]
pub type Md10R = crate::FieldReader;
#[doc = "Field `MD10` writer - Port x mode bits (x = 10 )"]
pub type Md10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL10` reader - Port x configuration bits (x = 10)"]
pub type Ctl10R = crate::FieldReader;
#[doc = "Field `CTL10` writer - Port x configuration bits (x = 10)"]
pub type Ctl10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD11` reader - Port x mode bits (x = 11 )"]
pub type Md11R = crate::FieldReader;
#[doc = "Field `MD11` writer - Port x mode bits (x = 11 )"]
pub type Md11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL11` reader - Port x configuration bits (x = 11)"]
pub type Ctl11R = crate::FieldReader;
#[doc = "Field `CTL11` writer - Port x configuration bits (x = 11)"]
pub type Ctl11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD12` reader - Port x mode bits (x = 12)"]
pub type Md12R = crate::FieldReader;
#[doc = "Field `MD12` writer - Port x mode bits (x = 12)"]
pub type Md12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL12` reader - Port x configuration bits (x = 12)"]
pub type Ctl12R = crate::FieldReader;
#[doc = "Field `CTL12` writer - Port x configuration bits (x = 12)"]
pub type Ctl12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD13` reader - Port x mode bits (x = 13)"]
pub type Md13R = crate::FieldReader;
#[doc = "Field `MD13` writer - Port x mode bits (x = 13)"]
pub type Md13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL13` reader - Port x configuration bits (x = 13)"]
pub type Ctl13R = crate::FieldReader;
#[doc = "Field `CTL13` writer - Port x configuration bits (x = 13)"]
pub type Ctl13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD14` reader - Port x mode bits (x = 14)"]
pub type Md14R = crate::FieldReader;
#[doc = "Field `MD14` writer - Port x mode bits (x = 14)"]
pub type Md14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL14` reader - Port x configuration bits (x = 14)"]
pub type Ctl14R = crate::FieldReader;
#[doc = "Field `CTL14` writer - Port x configuration bits (x = 14)"]
pub type Ctl14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MD15` reader - Port x mode bits (x = 15)"]
pub type Md15R = crate::FieldReader;
#[doc = "Field `MD15` writer - Port x mode bits (x = 15)"]
pub type Md15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL15` reader - Port x configuration bits (x = 15)"]
pub type Ctl15R = crate::FieldReader;
#[doc = "Field `CTL15` writer - Port x configuration bits (x = 15)"]
pub type Ctl15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline(always)]
    pub fn md8(&self) -> Md8R {
        Md8R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    pub fn ctl8(&self) -> Ctl8R {
        Ctl8R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline(always)]
    pub fn md9(&self) -> Md9R {
        Md9R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    pub fn ctl9(&self) -> Ctl9R {
        Ctl9R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline(always)]
    pub fn md10(&self) -> Md10R {
        Md10R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    pub fn ctl10(&self) -> Ctl10R {
        Ctl10R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline(always)]
    pub fn md11(&self) -> Md11R {
        Md11R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    pub fn ctl11(&self) -> Ctl11R {
        Ctl11R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline(always)]
    pub fn md12(&self) -> Md12R {
        Md12R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    pub fn ctl12(&self) -> Ctl12R {
        Ctl12R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline(always)]
    pub fn md13(&self) -> Md13R {
        Md13R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    pub fn ctl13(&self) -> Ctl13R {
        Ctl13R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline(always)]
    pub fn md14(&self) -> Md14R {
        Md14R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    pub fn ctl14(&self) -> Ctl14R {
        Ctl14R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline(always)]
    pub fn md15(&self) -> Md15R {
        Md15R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    pub fn ctl15(&self) -> Ctl15R {
        Ctl15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline(always)]
    #[must_use]
    pub fn md8(&mut self) -> Md8W<Ctl1Spec> {
        Md8W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl8(&mut self) -> Ctl8W<Ctl1Spec> {
        Ctl8W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline(always)]
    #[must_use]
    pub fn md9(&mut self) -> Md9W<Ctl1Spec> {
        Md9W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl9(&mut self) -> Ctl9W<Ctl1Spec> {
        Ctl9W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline(always)]
    #[must_use]
    pub fn md10(&mut self) -> Md10W<Ctl1Spec> {
        Md10W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl10(&mut self) -> Ctl10W<Ctl1Spec> {
        Ctl10W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline(always)]
    #[must_use]
    pub fn md11(&mut self) -> Md11W<Ctl1Spec> {
        Md11W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl11(&mut self) -> Ctl11W<Ctl1Spec> {
        Ctl11W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline(always)]
    #[must_use]
    pub fn md12(&mut self) -> Md12W<Ctl1Spec> {
        Md12W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl12(&mut self) -> Ctl12W<Ctl1Spec> {
        Ctl12W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline(always)]
    #[must_use]
    pub fn md13(&mut self) -> Md13W<Ctl1Spec> {
        Md13W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl13(&mut self) -> Ctl13W<Ctl1Spec> {
        Ctl13W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline(always)]
    #[must_use]
    pub fn md14(&mut self) -> Md14W<Ctl1Spec> {
        Md14W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl14(&mut self) -> Ctl14W<Ctl1Spec> {
        Ctl14W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline(always)]
    #[must_use]
    pub fn md15(&mut self) -> Md15W<Ctl1Spec> {
        Md15W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl15(&mut self) -> Ctl15W<Ctl1Spec> {
        Ctl15W::new(self, 30)
    }
}
#[doc = "port control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0x4444_4444"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
