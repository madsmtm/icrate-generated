//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSTimeZone")]
    pub struct NSTimeZone;

    #[cfg(feature = "Foundation_NSTimeZone")]
    unsafe impl ClassType for NSTimeZone {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSTimeZone")]
    unsafe impl NSTimeZone {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Id<NSData, Shared>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(secondsFromGMTForDate:)]
        pub unsafe fn secondsFromGMTForDate(&self, aDate: &NSDate) -> NSInteger;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other abbreviationForDate:)]
        pub unsafe fn abbreviationForDate(&self, aDate: &NSDate) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(isDaylightSavingTimeForDate:)]
        pub unsafe fn isDaylightSavingTimeForDate(&self, aDate: &NSDate) -> bool;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(daylightSavingTimeOffsetForDate:)]
        pub unsafe fn daylightSavingTimeOffsetForDate(&self, aDate: &NSDate) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other nextDaylightSavingTimeTransitionAfterDate:)]
        pub unsafe fn nextDaylightSavingTimeTransitionAfterDate(
            &self,
            aDate: &NSDate,
        ) -> Option<Id<NSDate, Shared>>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTimeZoneNameStyle {
        NSTimeZoneNameStyleStandard = 0,
        NSTimeZoneNameStyleShortStandard = 1,
        NSTimeZoneNameStyleDaylightSaving = 2,
        NSTimeZoneNameStyleShortDaylightSaving = 3,
        NSTimeZoneNameStyleGeneric = 4,
        NSTimeZoneNameStyleShortGeneric = 5,
    }
);

extern_methods!(
    /// NSExtendedTimeZone
    #[cfg(feature = "Foundation_NSTimeZone")]
    unsafe impl NSTimeZone {
        #[method_id(@__retain_semantics Other systemTimeZone)]
        pub unsafe fn systemTimeZone() -> Id<NSTimeZone, Shared>;

        #[method(resetSystemTimeZone)]
        pub unsafe fn resetSystemTimeZone();

        #[method_id(@__retain_semantics Other defaultTimeZone)]
        pub unsafe fn defaultTimeZone() -> Id<NSTimeZone, Shared>;

        #[method(setDefaultTimeZone:)]
        pub unsafe fn setDefaultTimeZone(defaultTimeZone: &NSTimeZone);

        #[method_id(@__retain_semantics Other localTimeZone)]
        pub unsafe fn localTimeZone() -> Id<NSTimeZone, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other knownTimeZoneNames)]
        pub unsafe fn knownTimeZoneNames() -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other abbreviationDictionary)]
        pub unsafe fn abbreviationDictionary() -> Id<NSDictionary<NSString, NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setAbbreviationDictionary:)]
        pub unsafe fn setAbbreviationDictionary(
            abbreviationDictionary: &NSDictionary<NSString, NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other timeZoneDataVersion)]
        pub unsafe fn timeZoneDataVersion() -> Id<NSString, Shared>;

        #[method(secondsFromGMT)]
        pub unsafe fn secondsFromGMT(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other abbreviation)]
        pub unsafe fn abbreviation(&self) -> Option<Id<NSString, Shared>>;

        #[method(isDaylightSavingTime)]
        pub unsafe fn isDaylightSavingTime(&self) -> bool;

        #[method(daylightSavingTimeOffset)]
        pub unsafe fn daylightSavingTimeOffset(&self) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other nextDaylightSavingTimeTransition)]
        pub unsafe fn nextDaylightSavingTimeTransition(&self) -> Option<Id<NSDate, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;

        #[method(isEqualToTimeZone:)]
        pub unsafe fn isEqualToTimeZone(&self, aTimeZone: &NSTimeZone) -> bool;

        #[cfg(all(feature = "Foundation_NSLocale", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedName:locale:)]
        pub unsafe fn localizedName_locale(
            &self,
            style: NSTimeZoneNameStyle,
            locale: Option<&NSLocale>,
        ) -> Option<Id<NSString, Shared>>;
    }
);

extern_methods!(
    /// NSTimeZoneCreation
    #[cfg(feature = "Foundation_NSTimeZone")]
    unsafe impl NSTimeZone {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other timeZoneWithName:)]
        pub unsafe fn timeZoneWithName(tzName: &NSString) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other timeZoneWithName:data:)]
        pub unsafe fn timeZoneWithName_data(
            tzName: &NSString,
            aData: Option<&NSData>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(
            this: Option<Allocated<Self>>,
            tzName: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithName:data:)]
        pub unsafe fn initWithName_data(
            this: Option<Allocated<Self>>,
            tzName: &NSString,
            aData: Option<&NSData>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other timeZoneForSecondsFromGMT:)]
        pub unsafe fn timeZoneForSecondsFromGMT(seconds: NSInteger) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other timeZoneWithAbbreviation:)]
        pub unsafe fn timeZoneWithAbbreviation(abbreviation: &NSString)
            -> Option<Id<Self, Shared>>;
    }
);

extern_static!(NSSystemTimeZoneDidChangeNotification: &'static NSNotificationName);
