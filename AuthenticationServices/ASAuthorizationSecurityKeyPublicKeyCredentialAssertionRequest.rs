//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest;

    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        type Super = ASAuthorizationRequest;
    }
);

extern_methods!(
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        #[method_id(@__retain_semantics Other allowedCredentials)]
        pub unsafe fn allowedCredentials(
            &self,
        ) -> Id<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>, Shared>;

        #[method(setAllowedCredentials:)]
        pub unsafe fn setAllowedCredentials(
            &self,
            allowedCredentials: &NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>,
        );
    }
);
