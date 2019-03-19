#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMER2CLKSTAT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED3R {
    bits: u32,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `STAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATR {
    #[doc = "SCLK_HF / 2"]
    SCLK_HFDIV2,
    #[doc = "SCLK_MF"]
    SCLK_MF,
    #[doc = "SCLK_LF"]
    SCLK_LF,
    #[doc = "No clock"]
    NONE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATR::SCLK_HFDIV2 => 4,
            STATR::SCLK_MF => 2,
            STATR::SCLK_LF => 1,
            STATR::NONE => 0,
            STATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATR {
        match value {
            4 => STATR::SCLK_HFDIV2,
            2 => STATR::SCLK_MF,
            1 => STATR::SCLK_LF,
            0 => STATR::NONE,
            i => STATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_HFDIV2`"]
    #[inline]
    pub fn is_sclk_hfdiv2(&self) -> bool {
        *self == STATR::SCLK_HFDIV2
    }
    #[doc = "Checks if the value of the field is `SCLK_MF`"]
    #[inline]
    pub fn is_sclk_mf(&self) -> bool {
        *self == STATR::SCLK_MF
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline]
    pub fn is_sclk_lf(&self) -> bool {
        *self == STATR::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == STATR::NONE
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STAT`"]
pub enum STATW {
    #[doc = "SCLK_HF / 2"]
    SCLK_HFDIV2,
    #[doc = "SCLK_MF"]
    SCLK_MF,
    #[doc = "SCLK_LF"]
    SCLK_LF,
    #[doc = "No clock"]
    NONE,
}
impl STATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STATW::SCLK_HFDIV2 => 4,
            STATW::SCLK_MF => 2,
            STATW::SCLK_LF => 1,
            STATW::NONE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATW<'a> {
    w: &'a mut W,
}
impl<'a> _STATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SCLK_HF / 2"]
    #[inline]
    pub fn sclk_hfdiv2(self) -> &'a mut W {
        self.variant(STATW::SCLK_HFDIV2)
    }
    #[doc = "SCLK_MF"]
    #[inline]
    pub fn sclk_mf(self) -> &'a mut W {
        self.variant(STATW::SCLK_MF)
    }
    #[doc = "SCLK_LF"]
    #[inline]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(STATW::SCLK_LF)
    }
    #[doc = "No clock"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(STATW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED3R { bits }
    }
    #[doc = "Bits 0:2 - 2:0\\] AUX_TIMER2 clock source status."]
    #[inline]
    pub fn stat(&self) -> STATR {
        STATR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] AUX_TIMER2 clock source status."]
    #[inline]
    pub fn stat(&mut self) -> _STATW {
        _STATW { w: self }
    }
}
