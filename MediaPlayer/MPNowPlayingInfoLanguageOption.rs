//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static MPLanguageOptionCharacteristicIsMainProgramContent: &'static NSString;
}

extern "C" {
    pub static MPLanguageOptionCharacteristicIsAuxiliaryContent: &'static NSString;
}

extern "C" {
    pub static MPLanguageOptionCharacteristicContainsOnlyForcedSubtitles: &'static NSString;
}

extern "C" {
    pub static MPLanguageOptionCharacteristicTranscribesSpokenDialog: &'static NSString;
}

extern "C" {
    pub static MPLanguageOptionCharacteristicDescribesMusicAndSound: &'static NSString;
}

extern "C" {
    pub static MPLanguageOptionCharacteristicEasyToRead: &'static NSString;
}

extern "C" {
    pub static MPLanguageOptionCharacteristicDescribesVideo: &'static NSString;
}

extern "C" {
    pub static MPLanguageOptionCharacteristicLanguageTranslation: &'static NSString;
}

extern "C" {
    pub static MPLanguageOptionCharacteristicDubbedTranslation: &'static NSString;
}

extern "C" {
    pub static MPLanguageOptionCharacteristicVoiceOverTranslation: &'static NSString;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPNowPlayingInfoLanguageOptionType(pub NSUInteger);
impl MPNowPlayingInfoLanguageOptionType {
    #[doc(alias = "MPNowPlayingInfoLanguageOptionTypeAudible")]
    pub const Audible: Self = Self(0);
    #[doc(alias = "MPNowPlayingInfoLanguageOptionTypeLegible")]
    pub const Legible: Self = Self(1);
}

unsafe impl Encode for MPNowPlayingInfoLanguageOptionType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPNowPlayingInfoLanguageOptionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPNowPlayingInfoLanguageOption;

    unsafe impl ClassType for MPNowPlayingInfoLanguageOption {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPNowPlayingInfoLanguageOption {}

extern_methods!(
    unsafe impl MPNowPlayingInfoLanguageOption {
        #[method_id(@__retain_semantics Init initWithType:languageTag:characteristics:displayName:identifier:)]
        pub unsafe fn initWithType_languageTag_characteristics_displayName_identifier(
            this: Allocated<Self>,
            language_option_type: MPNowPlayingInfoLanguageOptionType,
            language_tag: &NSString,
            language_option_characteristics: Option<&NSArray<NSString>>,
            display_name: &NSString,
            identifier: &NSString,
        ) -> Id<Self>;

        #[method(isAutomaticLegibleLanguageOption)]
        pub unsafe fn isAutomaticLegibleLanguageOption(&self) -> bool;

        #[method(isAutomaticAudibleLanguageOption)]
        pub unsafe fn isAutomaticAudibleLanguageOption(&self) -> bool;

        #[method(languageOptionType)]
        pub unsafe fn languageOptionType(&self) -> MPNowPlayingInfoLanguageOptionType;

        #[method_id(@__retain_semantics Other languageTag)]
        pub unsafe fn languageTag(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other languageOptionCharacteristics)]
        pub unsafe fn languageOptionCharacteristics(&self) -> Option<Id<NSArray<NSString>>>;

        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPNowPlayingInfoLanguageOption {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPNowPlayingInfoLanguageOptionGroup;

    unsafe impl ClassType for MPNowPlayingInfoLanguageOptionGroup {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPNowPlayingInfoLanguageOptionGroup {}

extern_methods!(
    unsafe impl MPNowPlayingInfoLanguageOptionGroup {
        #[method_id(@__retain_semantics Init initWithLanguageOptions:defaultLanguageOption:allowEmptySelection:)]
        pub unsafe fn initWithLanguageOptions_defaultLanguageOption_allowEmptySelection(
            this: Allocated<Self>,
            language_options: &NSArray<MPNowPlayingInfoLanguageOption>,
            default_language_option: Option<&MPNowPlayingInfoLanguageOption>,
            allow_empty_selection: bool,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other languageOptions)]
        pub unsafe fn languageOptions(&self) -> Id<NSArray<MPNowPlayingInfoLanguageOption>>;

        #[method_id(@__retain_semantics Other defaultLanguageOption)]
        pub unsafe fn defaultLanguageOption(&self) -> Option<Id<MPNowPlayingInfoLanguageOption>>;

        #[method(allowEmptySelection)]
        pub unsafe fn allowEmptySelection(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPNowPlayingInfoLanguageOptionGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
