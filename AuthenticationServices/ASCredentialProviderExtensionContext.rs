//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASCredentialProviderExtensionContext;

    unsafe impl ClassType for ASCredentialProviderExtensionContext {
        #[inherits(NSObject)]
        type Super = NSExtensionContext;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for ASCredentialProviderExtensionContext {}

extern_methods!(
    unsafe impl ASCredentialProviderExtensionContext {
        #[cfg(all(
            feature = "AuthenticationServices_ASPasswordCredential",
            feature = "block2"
        ))]
        #[method(completeRequestWithSelectedCredential:completionHandler:)]
        pub unsafe fn completeRequestWithSelectedCredential_completionHandler(
            &self,
            credential: &ASPasswordCredential,
            completion_handler: Option<&Block<dyn Fn(Bool)>>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASPasskeyAssertionCredential",
            feature = "block2"
        ))]
        #[method(completeAssertionRequestWithSelectedPasskeyCredential:completionHandler:)]
        pub unsafe fn completeAssertionRequestWithSelectedPasskeyCredential_completionHandler(
            &self,
            credential: &ASPasskeyAssertionCredential,
            completion_handler: Option<&Block<dyn Fn(Bool)>>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASPasskeyRegistrationCredential",
            feature = "block2"
        ))]
        #[method(completeRegistrationRequestWithSelectedPasskeyCredential:completionHandler:)]
        pub unsafe fn completeRegistrationRequestWithSelectedPasskeyCredential_completionHandler(
            &self,
            credential: &ASPasskeyRegistrationCredential,
            completion_handler: Option<&Block<dyn Fn(Bool)>>,
        );

        #[method(completeExtensionConfigurationRequest)]
        pub unsafe fn completeExtensionConfigurationRequest(&self);

        #[cfg(feature = "block2")]
        #[method(completeRequestReturningItems:completionHandler:)]
        pub unsafe fn completeRequestReturningItems_completionHandler(
            &self,
            items: Option<&NSArray>,
            completion_handler: Option<&Block<dyn Fn(Bool)>>,
        );

        #[method(cancelRequestWithError:)]
        pub unsafe fn cancelRequestWithError(&self, error: &NSError);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASCredentialProviderExtensionContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
