//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSByteCountFormatterUnits {
        NSByteCountFormatterUseDefault = 0,
        NSByteCountFormatterUseBytes = 1 << 0,
        NSByteCountFormatterUseKB = 1 << 1,
        NSByteCountFormatterUseMB = 1 << 2,
        NSByteCountFormatterUseGB = 1 << 3,
        NSByteCountFormatterUseTB = 1 << 4,
        NSByteCountFormatterUsePB = 1 << 5,
        NSByteCountFormatterUseEB = 1 << 6,
        NSByteCountFormatterUseZB = 1 << 7,
        NSByteCountFormatterUseYBOrHigher = 0x0FF << 8,
        NSByteCountFormatterUseAll = 0x0FFFF,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSByteCountFormatterCountStyle {
        NSByteCountFormatterCountStyleFile = 0,
        NSByteCountFormatterCountStyleMemory = 1,
        NSByteCountFormatterCountStyleDecimal = 2,
        NSByteCountFormatterCountStyleBinary = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSByteCountFormatter;

    unsafe impl ClassType for NSByteCountFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
    }
);

extern_methods!(
    unsafe impl NSByteCountFormatter {
        #[method_id(@__retain_semantics Other stringFromByteCount:countStyle:)]
        pub unsafe fn stringFromByteCount_countStyle(
            byteCount: c_longlong,
            countStyle: NSByteCountFormatterCountStyle,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringFromByteCount:)]
        pub unsafe fn stringFromByteCount(&self, byteCount: c_longlong) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringFromMeasurement:countStyle:)]
        pub unsafe fn stringFromMeasurement_countStyle(
            measurement: &NSMeasurement<NSUnitInformationStorage>,
            countStyle: NSByteCountFormatterCountStyle,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringFromMeasurement:)]
        pub unsafe fn stringFromMeasurement(
            &self,
            measurement: &NSMeasurement<NSUnitInformationStorage>,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&Object>,
        ) -> Option<Id<NSString, Shared>>;

        #[method(allowedUnits)]
        pub unsafe fn allowedUnits(&self) -> NSByteCountFormatterUnits;

        #[method(setAllowedUnits:)]
        pub unsafe fn setAllowedUnits(&self, allowedUnits: NSByteCountFormatterUnits);

        #[method(countStyle)]
        pub unsafe fn countStyle(&self) -> NSByteCountFormatterCountStyle;

        #[method(setCountStyle:)]
        pub unsafe fn setCountStyle(&self, countStyle: NSByteCountFormatterCountStyle);

        #[method(allowsNonnumericFormatting)]
        pub unsafe fn allowsNonnumericFormatting(&self) -> bool;

        #[method(setAllowsNonnumericFormatting:)]
        pub unsafe fn setAllowsNonnumericFormatting(&self, allowsNonnumericFormatting: bool);

        #[method(includesUnit)]
        pub unsafe fn includesUnit(&self) -> bool;

        #[method(setIncludesUnit:)]
        pub unsafe fn setIncludesUnit(&self, includesUnit: bool);

        #[method(includesCount)]
        pub unsafe fn includesCount(&self) -> bool;

        #[method(setIncludesCount:)]
        pub unsafe fn setIncludesCount(&self, includesCount: bool);

        #[method(includesActualByteCount)]
        pub unsafe fn includesActualByteCount(&self) -> bool;

        #[method(setIncludesActualByteCount:)]
        pub unsafe fn setIncludesActualByteCount(&self, includesActualByteCount: bool);

        #[method(isAdaptive)]
        pub unsafe fn isAdaptive(&self) -> bool;

        #[method(setAdaptive:)]
        pub unsafe fn setAdaptive(&self, adaptive: bool);

        #[method(zeroPadsFractionDigits)]
        pub unsafe fn zeroPadsFractionDigits(&self) -> bool;

        #[method(setZeroPadsFractionDigits:)]
        pub unsafe fn setZeroPadsFractionDigits(&self, zeroPadsFractionDigits: bool);

        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;

        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext);
    }
);
