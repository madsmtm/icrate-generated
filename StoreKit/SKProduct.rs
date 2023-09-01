//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum SKProductPeriodUnit {
        SKProductPeriodUnitDay = 0,
        SKProductPeriodUnitWeek = 1,
        SKProductPeriodUnitMonth = 2,
        SKProductPeriodUnitYear = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKProductSubscriptionPeriod")]
    pub struct SKProductSubscriptionPeriod;

    #[cfg(feature = "StoreKit_SKProductSubscriptionPeriod")]
    unsafe impl ClassType for SKProductSubscriptionPeriod {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKProductSubscriptionPeriod")]
unsafe impl Send for SKProductSubscriptionPeriod {}

#[cfg(feature = "StoreKit_SKProductSubscriptionPeriod")]
unsafe impl Sync for SKProductSubscriptionPeriod {}

#[cfg(feature = "StoreKit_SKProductSubscriptionPeriod")]
unsafe impl NSObjectProtocol for SKProductSubscriptionPeriod {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKProductSubscriptionPeriod")]
    unsafe impl SKProductSubscriptionPeriod {
        #[method(numberOfUnits)]
        pub unsafe fn numberOfUnits(&self) -> NSUInteger;

        #[method(unit)]
        pub unsafe fn unit(&self) -> SKProductPeriodUnit;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "StoreKit_SKProductSubscriptionPeriod")]
    unsafe impl SKProductSubscriptionPeriod {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKProduct")]
    pub struct SKProduct;

    #[cfg(feature = "StoreKit_SKProduct")]
    unsafe impl ClassType for SKProduct {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKProduct")]
unsafe impl Send for SKProduct {}

#[cfg(feature = "StoreKit_SKProduct")]
unsafe impl Sync for SKProduct {}

#[cfg(feature = "StoreKit_SKProduct")]
unsafe impl NSObjectProtocol for SKProduct {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKProduct")]
    unsafe impl SKProduct {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedTitle)]
        pub unsafe fn localizedTitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSDecimalNumber")]
        #[method_id(@__retain_semantics Other price)]
        pub unsafe fn price(&self) -> Id<NSDecimalNumber>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other priceLocale)]
        pub unsafe fn priceLocale(&self) -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other productIdentifier)]
        pub unsafe fn productIdentifier(&self) -> Id<NSString>;

        #[method(isDownloadable)]
        pub unsafe fn isDownloadable(&self) -> bool;

        #[deprecated]
        #[method(downloadable)]
        pub unsafe fn downloadable(&self) -> bool;

        #[method(isFamilyShareable)]
        pub unsafe fn isFamilyShareable(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentLengths)]
        pub unsafe fn contentLengths(&self) -> Id<NSArray<NSNumber>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other downloadContentLengths)]
        pub unsafe fn downloadContentLengths(&self) -> Id<NSArray<NSNumber>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentVersion)]
        pub unsafe fn contentVersion(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other downloadContentVersion)]
        pub unsafe fn downloadContentVersion(&self) -> Id<NSString>;

        #[cfg(feature = "StoreKit_SKProductSubscriptionPeriod")]
        #[method_id(@__retain_semantics Other subscriptionPeriod)]
        pub unsafe fn subscriptionPeriod(&self) -> Option<Id<SKProductSubscriptionPeriod>>;

        #[cfg(feature = "StoreKit_SKProductDiscount")]
        #[method_id(@__retain_semantics Other introductoryPrice)]
        pub unsafe fn introductoryPrice(&self) -> Option<Id<SKProductDiscount>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subscriptionGroupIdentifier)]
        pub unsafe fn subscriptionGroupIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "StoreKit_SKProductDiscount"))]
        #[method_id(@__retain_semantics Other discounts)]
        pub unsafe fn discounts(&self) -> Id<NSArray<SKProductDiscount>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "StoreKit_SKProduct")]
    unsafe impl SKProduct {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
