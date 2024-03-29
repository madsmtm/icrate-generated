//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    #[cfg(all(
        feature = "AuthenticationServices_ASAuthorizationCredential",
        feature = "AuthenticationServices_ASPublicKeyCredential",
        feature = "Foundation_NSObject"
    ))]
    pub unsafe trait ASAuthorizationPublicKeyCredentialRegistration:
        ASPublicKeyCredential
    {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other rawAttestationObject)]
        unsafe fn rawAttestationObject(&self) -> Option<Id<NSData>>;
    }

    #[cfg(all(
        feature = "AuthenticationServices_ASAuthorizationCredential",
        feature = "AuthenticationServices_ASPublicKeyCredential",
        feature = "Foundation_NSObject"
    ))]
    unsafe impl ProtocolType for dyn ASAuthorizationPublicKeyCredentialRegistration {}
);
