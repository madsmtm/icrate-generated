//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_TYPED_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type NSLocaleKey = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLocale;

    unsafe impl ClassType for NSLocale {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSLocale {}

unsafe impl Sync for NSLocale {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSLocale {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSLocale {}

unsafe impl NSObjectProtocol for NSLocale {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSLocale {}

extern_methods!(
    unsafe impl NSLocale {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(&self, key: &NSLocaleKey) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayNameForKey:value:)]
        pub unsafe fn displayNameForKey_value(
            &self,
            key: &NSLocaleKey,
            value: &AnyObject,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithLocaleIdentifier:)]
        pub unsafe fn initWithLocaleIdentifier(
            this: Allocated<Self>,
            string: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// NSExtendedLocale
    unsafe impl NSLocale {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localeIdentifier)]
        pub unsafe fn localeIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForLocaleIdentifier:)]
        pub unsafe fn localizedStringForLocaleIdentifier(
            &self,
            locale_identifier: &NSString,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other languageCode)]
        pub unsafe fn languageCode(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForLanguageCode:)]
        pub unsafe fn localizedStringForLanguageCode(
            &self,
            language_code: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other languageIdentifier)]
        pub unsafe fn languageIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForCountryCode:)]
        pub unsafe fn localizedStringForCountryCode(
            &self,
            country_code: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other regionCode)]
        pub unsafe fn regionCode(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other scriptCode)]
        pub unsafe fn scriptCode(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForScriptCode:)]
        pub unsafe fn localizedStringForScriptCode(
            &self,
            script_code: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other variantCode)]
        pub unsafe fn variantCode(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForVariantCode:)]
        pub unsafe fn localizedStringForVariantCode(
            &self,
            variant_code: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other exemplarCharacterSet)]
        pub unsafe fn exemplarCharacterSet(&self) -> Id<NSCharacterSet>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other calendarIdentifier)]
        pub unsafe fn calendarIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForCalendarIdentifier:)]
        pub unsafe fn localizedStringForCalendarIdentifier(
            &self,
            calendar_identifier: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other collationIdentifier)]
        pub unsafe fn collationIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForCollationIdentifier:)]
        pub unsafe fn localizedStringForCollationIdentifier(
            &self,
            collation_identifier: &NSString,
        ) -> Option<Id<NSString>>;

        #[method(usesMetricSystem)]
        pub unsafe fn usesMetricSystem(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other decimalSeparator)]
        pub unsafe fn decimalSeparator(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other groupingSeparator)]
        pub unsafe fn groupingSeparator(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other currencySymbol)]
        pub unsafe fn currencySymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other currencyCode)]
        pub unsafe fn currencyCode(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForCurrencyCode:)]
        pub unsafe fn localizedStringForCurrencyCode(
            &self,
            currency_code: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other collatorIdentifier)]
        pub unsafe fn collatorIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForCollatorIdentifier:)]
        pub unsafe fn localizedStringForCollatorIdentifier(
            &self,
            collator_identifier: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other quotationBeginDelimiter)]
        pub unsafe fn quotationBeginDelimiter(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other quotationEndDelimiter)]
        pub unsafe fn quotationEndDelimiter(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alternateQuotationBeginDelimiter)]
        pub unsafe fn alternateQuotationBeginDelimiter(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alternateQuotationEndDelimiter)]
        pub unsafe fn alternateQuotationEndDelimiter(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// NSLocaleCreation
    unsafe impl NSLocale {
        #[method_id(@__retain_semantics Other autoupdatingCurrentLocale)]
        pub unsafe fn autoupdatingCurrentLocale() -> Id<NSLocale>;

        #[method_id(@__retain_semantics Other currentLocale)]
        pub unsafe fn currentLocale() -> Id<NSLocale>;

        #[method_id(@__retain_semantics Other systemLocale)]
        pub unsafe fn systemLocale() -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localeWithLocaleIdentifier:)]
        pub unsafe fn localeWithLocaleIdentifier(ident: &NSString) -> Id<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLocaleLanguageDirection(pub NSUInteger);
impl NSLocaleLanguageDirection {
    #[doc(alias = "NSLocaleLanguageDirectionUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "NSLocaleLanguageDirectionLeftToRight")]
    pub const LeftToRight: Self = Self(1);
    #[doc(alias = "NSLocaleLanguageDirectionRightToLeft")]
    pub const RightToLeft: Self = Self(2);
    #[doc(alias = "NSLocaleLanguageDirectionTopToBottom")]
    pub const TopToBottom: Self = Self(3);
    #[doc(alias = "NSLocaleLanguageDirectionBottomToTop")]
    pub const BottomToTop: Self = Self(4);
}

unsafe impl Encode for NSLocaleLanguageDirection {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSLocaleLanguageDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSLocaleGeneralInfo
    unsafe impl NSLocale {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other availableLocaleIdentifiers)]
        pub unsafe fn availableLocaleIdentifiers() -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other ISOLanguageCodes)]
        pub unsafe fn ISOLanguageCodes() -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other ISOCountryCodes)]
        pub unsafe fn ISOCountryCodes() -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other ISOCurrencyCodes)]
        pub unsafe fn ISOCurrencyCodes() -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other commonISOCurrencyCodes)]
        pub unsafe fn commonISOCurrencyCodes() -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other preferredLanguages)]
        pub unsafe fn preferredLanguages() -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other componentsFromLocaleIdentifier:)]
        pub unsafe fn componentsFromLocaleIdentifier(
            string: &NSString,
        ) -> Id<NSDictionary<NSString, NSString>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localeIdentifierFromComponents:)]
        pub unsafe fn localeIdentifierFromComponents(
            dict: &NSDictionary<NSString, NSString>,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other canonicalLocaleIdentifierFromString:)]
        pub unsafe fn canonicalLocaleIdentifierFromString(string: &NSString) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other canonicalLanguageIdentifierFromString:)]
        pub unsafe fn canonicalLanguageIdentifierFromString(string: &NSString) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localeIdentifierFromWindowsLocaleCode:)]
        pub unsafe fn localeIdentifierFromWindowsLocaleCode(lcid: u32) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(windowsLocaleCodeFromLocaleIdentifier:)]
        pub unsafe fn windowsLocaleCodeFromLocaleIdentifier(locale_identifier: &NSString) -> u32;

        #[cfg(feature = "Foundation_NSString")]
        #[method(characterDirectionForLanguage:)]
        pub unsafe fn characterDirectionForLanguage(
            iso_lang_code: &NSString,
        ) -> NSLocaleLanguageDirection;

        #[cfg(feature = "Foundation_NSString")]
        #[method(lineDirectionForLanguage:)]
        pub unsafe fn lineDirectionForLanguage(
            iso_lang_code: &NSString,
        ) -> NSLocaleLanguageDirection;
    }
);

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static NSCurrentLocaleDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleIdentifier: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleLanguageCode: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleCountryCode: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleScriptCode: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleVariantCode: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleExemplarCharacterSet: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleCalendar: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleCollationIdentifier: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleUsesMetricSystem: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleMeasurementSystem: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleDecimalSeparator: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleGroupingSeparator: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleCurrencySymbol: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleCurrencyCode: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleCollatorIdentifier: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleQuotationBeginDelimiterKey: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleQuotationEndDelimiterKey: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleAlternateQuotationBeginDelimiterKey: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocaleAlternateQuotationEndDelimiterKey: &'static NSLocaleKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSGregorianCalendar: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSBuddhistCalendar: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSChineseCalendar: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSHebrewCalendar: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSIslamicCalendar: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSIslamicCivilCalendar: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSJapaneseCalendar: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSRepublicOfChinaCalendar: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPersianCalendar: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSIndianCalendar: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSISO8601Calendar: &'static NSString;
}
