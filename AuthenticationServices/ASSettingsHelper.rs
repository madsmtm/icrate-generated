//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASSettingsHelper;

    unsafe impl ClassType for ASSettingsHelper {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for ASSettingsHelper {}

extern_methods!(
    unsafe impl ASSettingsHelper {
        #[cfg(feature = "block2")]
        #[method(openCredentialProviderAppSettingsWithCompletionHandler:)]
        pub unsafe fn openCredentialProviderAppSettingsWithCompletionHandler(
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(openVerificationCodeAppSettingsWithCompletionHandler:)]
        pub unsafe fn openVerificationCodeAppSettingsWithCompletionHandler(
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
