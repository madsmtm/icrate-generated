//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialAssertionRequest",
    feature = "AuthenticationServices_ASAuthorizationRequest",
    feature = "Foundation_NSObject"
))]
unsafe impl ASAuthorizationPublicKeyCredentialAssertionRequest
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
{
}

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationRequest",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationRequest",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCopying for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationRequest",
    feature = "Foundation_NSObject"
))]
unsafe impl NSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other allowedCredentials)]
        pub unsafe fn allowedCredentials(
            &self,
        ) -> Id<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor",
            feature = "Foundation_NSArray"
        ))]
        #[method(setAllowedCredentials:)]
        pub unsafe fn setAllowedCredentials(
            &self,
            allowed_credentials: &NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `ASAuthorizationRequest`
    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}
);

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationRequest",
    feature = "AuthenticationServices_ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialAssertionRequest"
))]
unsafe impl ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialAssertionRequest
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
{
}
