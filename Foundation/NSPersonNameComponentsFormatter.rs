//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersonNameComponentsFormatterStyle(pub NSInteger);
impl NSPersonNameComponentsFormatterStyle {
    #[doc(alias = "NSPersonNameComponentsFormatterStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSPersonNameComponentsFormatterStyleShort")]
    pub const Short: Self = Self(1);
    #[doc(alias = "NSPersonNameComponentsFormatterStyleMedium")]
    pub const Medium: Self = Self(2);
    #[doc(alias = "NSPersonNameComponentsFormatterStyleLong")]
    pub const Long: Self = Self(3);
    #[doc(alias = "NSPersonNameComponentsFormatterStyleAbbreviated")]
    pub const Abbreviated: Self = Self(4);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSPersonNameComponentsFormatterStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSPersonNameComponentsFormatterStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersonNameComponentsFormatterOptions(pub NSUInteger);
impl NSPersonNameComponentsFormatterOptions {
    pub const NSPersonNameComponentsFormatterPhonetic: Self = Self(1 << 1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSPersonNameComponentsFormatterOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSPersonNameComponentsFormatterOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSFormatter")]
    pub struct NSPersonNameComponentsFormatter;

    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl ClassType for NSPersonNameComponentsFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSFormatter")]
unsafe impl Send for NSPersonNameComponentsFormatter {}

#[cfg(feature = "Foundation_NSFormatter")]
unsafe impl Sync for NSPersonNameComponentsFormatter {}

#[cfg(all(feature = "Foundation_NSFormatter", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for NSPersonNameComponentsFormatter {}

#[cfg(all(feature = "Foundation_NSFormatter", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for NSPersonNameComponentsFormatter {}

#[cfg(feature = "Foundation_NSFormatter")]
unsafe impl NSObjectProtocol for NSPersonNameComponentsFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl NSPersonNameComponentsFormatter {
        #[method(style)]
        pub unsafe fn style(&self) -> NSPersonNameComponentsFormatterStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSPersonNameComponentsFormatterStyle);

        #[method(isPhonetic)]
        pub unsafe fn isPhonetic(&self) -> bool;

        #[method(setPhonetic:)]
        pub unsafe fn setPhonetic(&self, phonetic: bool);

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(all(
            feature = "Foundation_NSPersonNameComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other localizedStringFromPersonNameComponents:style:options:)]
        pub unsafe fn localizedStringFromPersonNameComponents_style_options(
            components: &NSPersonNameComponents,
            name_format_style: NSPersonNameComponentsFormatterStyle,
            name_options: NSPersonNameComponentsFormatterOptions,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSPersonNameComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other stringFromPersonNameComponents:)]
        pub unsafe fn stringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSPersonNameComponents"
        ))]
        #[method_id(@__retain_semantics Other annotatedStringFromPersonNameComponents:)]
        pub unsafe fn annotatedStringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Id<NSAttributedString>;

        #[cfg(all(
            feature = "Foundation_NSPersonNameComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other personNameComponentsFromString:)]
        pub unsafe fn personNameComponentsFromString(
            &self,
            string: &NSString,
        ) -> Option<Id<NSPersonNameComponents>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<AnyObject>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString>>>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl NSPersonNameComponentsFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPersonNameComponentKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPersonNameComponentGivenName: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPersonNameComponentFamilyName: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPersonNameComponentMiddleName: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPersonNameComponentPrefix: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPersonNameComponentSuffix: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPersonNameComponentNickname: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPersonNameComponentDelimiter: &'static NSString;
}
