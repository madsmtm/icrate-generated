//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNPostalAddress;

    unsafe impl ClassType for CNPostalAddress {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for CNPostalAddress {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for CNPostalAddress {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSMutableCopying for CNPostalAddress {}

unsafe impl NSObjectProtocol for CNPostalAddress {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for CNPostalAddress {}

extern_methods!(
    unsafe impl CNPostalAddress {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other street)]
        pub unsafe fn street(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subLocality)]
        pub unsafe fn subLocality(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other city)]
        pub unsafe fn city(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subAdministrativeArea)]
        pub unsafe fn subAdministrativeArea(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other postalCode)]
        pub unsafe fn postalCode(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other country)]
        pub unsafe fn country(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other ISOCountryCode)]
        pub unsafe fn ISOCountryCode(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForKey:)]
        pub unsafe fn localizedStringForKey(key: &NSString) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNPostalAddress {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static CNPostalAddressStreetKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static CNPostalAddressSubLocalityKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static CNPostalAddressCityKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static CNPostalAddressSubAdministrativeAreaKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static CNPostalAddressStateKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static CNPostalAddressPostalCodeKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static CNPostalAddressCountryKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static CNPostalAddressISOCountryCodeKey: &'static NSString;
}
