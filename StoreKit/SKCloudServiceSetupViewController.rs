//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type SKCloudServiceSetupOptionsKey = NSString;

// NS_TYPED_ENUM
pub type SKCloudServiceSetupAction = NSString;

// NS_TYPED_ENUM
pub type SKCloudServiceSetupMessageIdentifier = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    pub struct SKCloudServiceSetupViewController;

    #[cfg(feature = "objc2-app-kit")]
    unsafe impl ClassType for SKCloudServiceSetupViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSCoding for SKCloudServiceSetupViewController {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSEditor for SKCloudServiceSetupViewController {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSObjectProtocol for SKCloudServiceSetupViewController {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSSeguePerforming for SKCloudServiceSetupViewController {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSUserInterfaceItemIdentification for SKCloudServiceSetupViewController {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl SKCloudServiceSetupViewController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn SKCloudServiceSetupViewControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn SKCloudServiceSetupViewControllerDelegate>>,
        );

        #[cfg(feature = "block2")]
        #[method(loadWithOptions:completionHandler:)]
        pub unsafe fn loadWithOptions_completionHandler(
            &self,
            options: &NSDictionary<SKCloudServiceSetupOptionsKey, AnyObject>,
            completion_handler: Option<&Block<dyn Fn(Bool, *mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl SKCloudServiceSetupViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl SKCloudServiceSetupViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl SKCloudServiceSetupViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait SKCloudServiceSetupViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "objc2-app-kit")]
        #[optional]
        #[method(cloudServiceSetupViewControllerDidDismiss:)]
        unsafe fn cloudServiceSetupViewControllerDidDismiss(
            &self,
            cloud_service_setup_view_controller: &SKCloudServiceSetupViewController,
        );
    }

    unsafe impl ProtocolType for dyn SKCloudServiceSetupViewControllerDelegate {}
);

extern "C" {
    pub static SKCloudServiceSetupOptionsActionKey: &'static SKCloudServiceSetupOptionsKey;
}

extern "C" {
    pub static SKCloudServiceSetupOptionsITunesItemIdentifierKey:
        &'static SKCloudServiceSetupOptionsKey;
}

extern "C" {
    pub static SKCloudServiceSetupOptionsAffiliateTokenKey: &'static SKCloudServiceSetupOptionsKey;
}

extern "C" {
    pub static SKCloudServiceSetupOptionsCampaignTokenKey: &'static SKCloudServiceSetupOptionsKey;
}

extern "C" {
    pub static SKCloudServiceSetupOptionsMessageIdentifierKey:
        &'static SKCloudServiceSetupOptionsKey;
}

extern "C" {
    pub static SKCloudServiceSetupActionSubscribe: &'static SKCloudServiceSetupAction;
}

extern "C" {
    pub static SKCloudServiceSetupMessageIdentifierJoin:
        &'static SKCloudServiceSetupMessageIdentifier;
}

extern "C" {
    pub static SKCloudServiceSetupMessageIdentifierConnect:
        &'static SKCloudServiceSetupMessageIdentifier;
}

extern "C" {
    pub static SKCloudServiceSetupMessageIdentifierAddMusic:
        &'static SKCloudServiceSetupMessageIdentifier;
}

extern "C" {
    pub static SKCloudServiceSetupMessageIdentifierPlayMusic:
        &'static SKCloudServiceSetupMessageIdentifier;
}
