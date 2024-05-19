//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASAuthorizationRequest")]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest;

    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "ASAuthorizationPublicKeyCredentialAssertionRequest",
    feature = "ASAuthorizationRequest"
))]
unsafe impl ASAuthorizationPublicKeyCredentialAssertionRequest
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
{
}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSCoding for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSCopying for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}

extern_methods!(
    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        #[cfg(feature = "ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor")]
        #[method_id(@__retain_semantics Other allowedCredentials)]
        pub unsafe fn allowedCredentials(
            &self,
        ) -> Id<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>>;

        #[cfg(feature = "ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor")]
        #[method(setAllowedCredentials:)]
        pub unsafe fn setAllowedCredentials(
            &self,
            allowed_credentials: &NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>,
        );

        #[method_id(@__retain_semantics Other appID)]
        pub unsafe fn appID(&self) -> Option<Id<NSString>>;

        #[method(setAppID:)]
        pub unsafe fn setAppID(&self, app_id: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `ASAuthorizationRequest`
    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}
);

#[cfg(all(
    feature = "ASAuthorizationRequest",
    feature = "ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialAssertionRequest"
))]
unsafe impl ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialAssertionRequest
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
{
}
