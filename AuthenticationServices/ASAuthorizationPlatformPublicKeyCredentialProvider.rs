//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct ASAuthorizationPlatformPublicKeyCredentialProvider;

    unsafe impl ClassType for ASAuthorizationPlatformPublicKeyCredentialProvider {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialProvider {
        #[method_id(@__retain_semantics Init initWithRelyingPartyIdentifier:)]
        pub unsafe fn initWithRelyingPartyIdentifier(
            this: Option<Allocated<Self>>,
            relyingPartyIdentifier: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other createCredentialRegistrationRequestWithChallenge:name:userID:)]
        pub unsafe fn createCredentialRegistrationRequestWithChallenge_name_userID(
            &self,
            challenge: &NSData,
            name: &NSString,
            userID: &NSData,
        ) -> Id<ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest, Shared>;

        #[method_id(@__retain_semantics Other createCredentialAssertionRequestWithChallenge:)]
        pub unsafe fn createCredentialAssertionRequestWithChallenge(
            &self,
            challenge: &NSData,
        ) -> Id<ASAuthorizationPlatformPublicKeyCredentialAssertionRequest, Shared>;

        #[method_id(@__retain_semantics Other relyingPartyIdentifier)]
        pub unsafe fn relyingPartyIdentifier(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
