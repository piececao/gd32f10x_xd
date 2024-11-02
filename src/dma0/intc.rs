#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "Field `GIFC0` writer - Clear global interrupt flag of channel 0"]
pub type Gifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC0` writer - Clear bit for full transfer finish flag of channel 0"]
pub type Ftfifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC0` writer - Clear bit for half transfer finish flag of channel 0"]
pub type Htfifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC0` writer - Clear bit for error flag of channel 0"]
pub type Errifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC1` writer - Clear global interrupt flag of channel 1"]
pub type Gifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC1` writer - Clear bit for full transfer finish flag of channel 1"]
pub type Ftfifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC1` writer - Clear bit for half transfer finish flag of channel 1"]
pub type Htfifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC1` writer - Clear bit for error flag of channel 1"]
pub type Errifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC2` writer - Clear global interrupt flag of channel 2"]
pub type Gifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC2` writer - Clear bit for full transfer finish flag of channel 2"]
pub type Ftfifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC2` writer - Clear bit for half transfer finish flag of channel 2"]
pub type Htfifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC2` writer - Clear bit for error flag of channel 2"]
pub type Errifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC3` writer - Clear global interrupt flag of channel 3"]
pub type Gifc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC3` writer - Clear bit for full transfer finish flag of channel 3"]
pub type Ftfifc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC3` writer - Clear bit for half transfer finish flag of channel 3"]
pub type Htfifc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC3` writer - Clear bit for error flag of channel 3"]
pub type Errifc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC4` writer - Clear global interrupt flag of channel 4"]
pub type Gifc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC4` writer - Clear bit for full transfer finish flag of channel 4"]
pub type Ftfifc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC4` writer - Clear bit for half transfer finish flag of channel 4"]
pub type Htfifc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC4` writer - Clear bit for error flag of channel 4"]
pub type Errifc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC5` writer - Clear global interrupt flag of channel 5"]
pub type Gifc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC5` writer - Clear bit for full transfer finish flag of channel 5"]
pub type Ftfifc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC5` writer - Clear bit for half transfer finish flag of channel 5"]
pub type Htfifc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC5` writer - Clear bit for error flag of channel 5"]
pub type Errifc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC6` writer - Clear global interrupt flag of channel 6"]
pub type Gifc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC6` writer - Clear bit for full transfer finish flag of channel 6"]
pub type Ftfifc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC6` writer - Clear bit for half transfer finish flag of channel 6"]
pub type Htfifc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC6` writer - Clear bit for error flag of channel 6"]
pub type Errifc6W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear global interrupt flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn gifc0(&mut self) -> Gifc0W<IntcSpec> {
        Gifc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear bit for full transfer finish flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc0(&mut self) -> Ftfifc0W<IntcSpec> {
        Ftfifc0W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear bit for half transfer finish flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc0(&mut self) -> Htfifc0W<IntcSpec> {
        Htfifc0W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit for error flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn errifc0(&mut self) -> Errifc0W<IntcSpec> {
        Errifc0W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear global interrupt flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn gifc1(&mut self) -> Gifc1W<IntcSpec> {
        Gifc1W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear bit for full transfer finish flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc1(&mut self) -> Ftfifc1W<IntcSpec> {
        Ftfifc1W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear bit for half transfer finish flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc1(&mut self) -> Htfifc1W<IntcSpec> {
        Htfifc1W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear bit for error flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn errifc1(&mut self) -> Errifc1W<IntcSpec> {
        Errifc1W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear global interrupt flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn gifc2(&mut self) -> Gifc2W<IntcSpec> {
        Gifc2W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear bit for full transfer finish flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc2(&mut self) -> Ftfifc2W<IntcSpec> {
        Ftfifc2W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear bit for half transfer finish flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc2(&mut self) -> Htfifc2W<IntcSpec> {
        Htfifc2W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear bit for error flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn errifc2(&mut self) -> Errifc2W<IntcSpec> {
        Errifc2W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear global interrupt flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn gifc3(&mut self) -> Gifc3W<IntcSpec> {
        Gifc3W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear bit for full transfer finish flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc3(&mut self) -> Ftfifc3W<IntcSpec> {
        Ftfifc3W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear bit for half transfer finish flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc3(&mut self) -> Htfifc3W<IntcSpec> {
        Htfifc3W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear bit for error flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn errifc3(&mut self) -> Errifc3W<IntcSpec> {
        Errifc3W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear global interrupt flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn gifc4(&mut self) -> Gifc4W<IntcSpec> {
        Gifc4W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear bit for full transfer finish flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc4(&mut self) -> Ftfifc4W<IntcSpec> {
        Ftfifc4W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear bit for half transfer finish flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc4(&mut self) -> Htfifc4W<IntcSpec> {
        Htfifc4W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear bit for error flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn errifc4(&mut self) -> Errifc4W<IntcSpec> {
        Errifc4W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear global interrupt flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn gifc5(&mut self) -> Gifc5W<IntcSpec> {
        Gifc5W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear bit for full transfer finish flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc5(&mut self) -> Ftfifc5W<IntcSpec> {
        Ftfifc5W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear bit for half transfer finish flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc5(&mut self) -> Htfifc5W<IntcSpec> {
        Htfifc5W::new(self, 22)
    }
    #[doc = "Bit 23 - Clear bit for error flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn errifc5(&mut self) -> Errifc5W<IntcSpec> {
        Errifc5W::new(self, 23)
    }
    #[doc = "Bit 24 - Clear global interrupt flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn gifc6(&mut self) -> Gifc6W<IntcSpec> {
        Gifc6W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear bit for full transfer finish flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc6(&mut self) -> Ftfifc6W<IntcSpec> {
        Ftfifc6W::new(self, 25)
    }
    #[doc = "Bit 26 - Clear bit for half transfer finish flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc6(&mut self) -> Htfifc6W<IntcSpec> {
        Htfifc6W::new(self, 26)
    }
    #[doc = "Bit 27 - Clear bit for error flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn errifc6(&mut self) -> Errifc6W<IntcSpec> {
        Errifc6W::new(self, 27)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcSpec;
impl crate::RegisterSpec for IntcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for IntcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for IntcSpec {
    const RESET_VALUE: u32 = 0;
}
