//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait ASAuthorizationWebBrowserPlatformPublicKeyCredentialAssertionRequest {
        #[cfg(feature = "AuthenticationServices_ASPublicKeyCredentialClientData")]
        #[method_id(@__retain_semantics Other clientData)]
        unsafe fn clientData(&self) -> Option<Id<ASPublicKeyCredentialClientData>>;

        #[method(shouldShowHybridTransport)]
        unsafe fn shouldShowHybridTransport(&self) -> bool;

        #[method(setShouldShowHybridTransport:)]
        unsafe fn setShouldShowHybridTransport(&self, should_show_hybrid_transport: bool);
    }

    unsafe impl ProtocolType
        for dyn ASAuthorizationWebBrowserPlatformPublicKeyCredentialAssertionRequest
    {
    }
);
