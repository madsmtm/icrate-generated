//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPlatformPublicKeyCredentialProvider;

    unsafe impl ClassType for ASAuthorizationPlatformPublicKeyCredentialProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "ASAuthorizationProvider")]
unsafe impl ASAuthorizationProvider for ASAuthorizationPlatformPublicKeyCredentialProvider {}

unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialProvider {}

extern_methods!(
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialProvider {
        #[method_id(@__retain_semantics Init initWithRelyingPartyIdentifier:)]
        pub unsafe fn initWithRelyingPartyIdentifier(
            this: Allocated<Self>,
            relying_party_identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest",
            feature = "ASAuthorizationRequest"
        ))]
        #[method_id(@__retain_semantics Other createCredentialRegistrationRequestWithChallenge:name:userID:)]
        pub unsafe fn createCredentialRegistrationRequestWithChallenge_name_userID(
            &self,
            challenge: &NSData,
            name: &NSString,
            user_id: &NSData,
        ) -> Retained<ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest>;

        #[cfg(all(
            feature = "ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest",
            feature = "ASAuthorizationRequest"
        ))]
        #[method_id(@__retain_semantics Other createCredentialRegistrationRequestWithChallenge:name:userID:requestStyle:)]
        pub unsafe fn createCredentialRegistrationRequestWithChallenge_name_userID_requestStyle(
            &self,
            challenge: &NSData,
            name: &NSString,
            user_id: &NSData,
            request_style: ASAuthorizationPlatformPublicKeyCredentialRegistrationRequestStyle,
        ) -> Retained<ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest>;

        #[cfg(all(
            feature = "ASAuthorizationPlatformPublicKeyCredentialAssertionRequest",
            feature = "ASAuthorizationRequest"
        ))]
        #[method_id(@__retain_semantics Other createCredentialAssertionRequestWithChallenge:)]
        pub unsafe fn createCredentialAssertionRequestWithChallenge(
            &self,
            challenge: &NSData,
        ) -> Retained<ASAuthorizationPlatformPublicKeyCredentialAssertionRequest>;

        #[method_id(@__retain_semantics Other relyingPartyIdentifier)]
        pub unsafe fn relyingPartyIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialProvider {}
);

#[cfg(feature = "ASAuthorizationWebBrowserPlatformPublicKeyCredentialProvider")]
unsafe impl ASAuthorizationWebBrowserPlatformPublicKeyCredentialProvider
    for ASAuthorizationPlatformPublicKeyCredentialProvider
{
}
