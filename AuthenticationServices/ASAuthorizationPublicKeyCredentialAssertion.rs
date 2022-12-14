//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct ASAuthorizationPublicKeyCredentialAssertion;

    unsafe impl ProtocolType for ASAuthorizationPublicKeyCredentialAssertion {
        #[method_id(@__retain_semantics Other rawAuthenticatorData)]
        pub unsafe fn rawAuthenticatorData(&self) -> Id<NSData, Shared>;

        #[method_id(@__retain_semantics Other userID)]
        pub unsafe fn userID(&self) -> Id<NSData, Shared>;

        #[method_id(@__retain_semantics Other signature)]
        pub unsafe fn signature(&self) -> Id<NSData, Shared>;
    }
);
