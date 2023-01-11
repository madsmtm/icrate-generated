//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters")]
    pub struct ASAuthorizationPublicKeyCredentialParameters;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters")]
    unsafe impl ClassType for ASAuthorizationPublicKeyCredentialParameters {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters")]
    unsafe impl ASAuthorizationPublicKeyCredentialParameters {
        #[method_id(@__retain_semantics Init initWithAlgorithm:)]
        pub unsafe fn initWithAlgorithm(
            this: Option<Allocated<Self>>,
            algorithm: ASCOSEAlgorithmIdentifier,
        ) -> Id<Self, Shared>;

        #[method(algorithm)]
        pub unsafe fn algorithm(&self) -> ASCOSEAlgorithmIdentifier;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
