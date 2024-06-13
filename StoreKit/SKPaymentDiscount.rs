//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
    pub struct SKPaymentDiscount;

    unsafe impl ClassType for SKPaymentDiscount {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for SKPaymentDiscount {}

unsafe impl Sync for SKPaymentDiscount {}

unsafe impl NSObjectProtocol for SKPaymentDiscount {}

extern_methods!(
    unsafe impl SKPaymentDiscount {
        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[method_id(@__retain_semantics Init initWithIdentifier:keyIdentifier:nonce:signature:timestamp:)]
        pub unsafe fn initWithIdentifier_keyIdentifier_nonce_signature_timestamp(
            this: Allocated<Self>,
            identifier: &NSString,
            key_identifier: &NSString,
            nonce: &NSUUID,
            signature: &NSString,
            timestamp: &NSNumber,
        ) -> Retained<Self>;

        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[method_id(@__retain_semantics Other keyIdentifier)]
        pub unsafe fn keyIdentifier(&self) -> Retained<NSString>;

        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[method_id(@__retain_semantics Other nonce)]
        pub unsafe fn nonce(&self) -> Retained<NSUUID>;

        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[method_id(@__retain_semantics Other signature)]
        pub unsafe fn signature(&self) -> Retained<NSString>;

        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Retained<NSNumber>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKPaymentDiscount {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
