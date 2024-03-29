//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

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
        #[cfg(feature = "Foundation_NSError")]
        #[method(openCredentialProviderAppSettingsWithCompletionHandler:)]
        pub unsafe fn openCredentialProviderAppSettingsWithCompletionHandler(
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(openVerificationCodeAppSettingsWithCompletionHandler:)]
        pub unsafe fn openVerificationCodeAppSettingsWithCompletionHandler(
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
