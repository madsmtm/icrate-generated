//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    pub struct SKStoreProductViewController;

    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    unsafe impl ClassType for SKStoreProductViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "StoreKit_SKStoreProductViewController")]
unsafe impl NSCoding for SKStoreProductViewController {}

#[cfg(feature = "StoreKit_SKStoreProductViewController")]
unsafe impl NSEditor for SKStoreProductViewController {}

#[cfg(feature = "StoreKit_SKStoreProductViewController")]
unsafe impl NSObjectProtocol for SKStoreProductViewController {}

#[cfg(feature = "StoreKit_SKStoreProductViewController")]
unsafe impl NSSeguePerforming for SKStoreProductViewController {}

#[cfg(feature = "StoreKit_SKStoreProductViewController")]
unsafe impl NSUserInterfaceItemIdentification for SKStoreProductViewController {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    unsafe impl SKStoreProductViewController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn SKStoreProductViewControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&(impl SKStoreProductViewControllerDelegate + Message)>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(loadProductWithParameters:completionBlock:)]
        pub unsafe fn loadProductWithParameters_completionBlock(
            &self,
            parameters: &NSDictionary<NSString, AnyObject>,
            block: Option<&Block<(Bool, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "StoreKit_SKAdImpression"
        ))]
        #[method(loadProductWithParameters:impression:completionBlock:)]
        pub unsafe fn loadProductWithParameters_impression_completionBlock(
            &self,
            parameters: &NSDictionary<NSString, AnyObject>,
            impression: &SKAdImpression,
            block: Option<&Block<(Bool, *mut NSError), ()>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    unsafe impl SKStoreProductViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    unsafe impl SKStoreProductViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    unsafe impl SKStoreProductViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait SKStoreProductViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "StoreKit_SKStoreProductViewController")]
        #[optional]
        #[method(productViewControllerDidFinish:)]
        unsafe fn productViewControllerDidFinish(
            &self,
            view_controller: &SKStoreProductViewController,
        );
    }

    unsafe impl ProtocolType for dyn SKStoreProductViewControllerDelegate {}
);

extern_static!(SKStoreProductParameterITunesItemIdentifier: &'static NSString);

extern_static!(SKStoreProductParameterProductIdentifier: &'static NSString);

extern_static!(SKStoreProductParameterCustomProductPageIdentifier: &'static NSString);

extern_static!(SKStoreProductParameterAffiliateToken: &'static NSString);

extern_static!(SKStoreProductParameterCampaignToken: &'static NSString);

extern_static!(SKStoreProductParameterProviderToken: &'static NSString);

extern_static!(SKStoreProductParameterAdvertisingPartnerToken: &'static NSString);
