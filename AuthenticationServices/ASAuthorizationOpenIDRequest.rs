//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type ASAuthorizationOpenIDOperation = NSString;

extern "C" {
    pub static ASAuthorizationOperationImplicit: &'static ASAuthorizationOpenIDOperation;
}

extern "C" {
    pub static ASAuthorizationOperationLogin: &'static ASAuthorizationOpenIDOperation;
}

extern "C" {
    pub static ASAuthorizationOperationRefresh: &'static ASAuthorizationOpenIDOperation;
}

extern "C" {
    pub static ASAuthorizationOperationLogout: &'static ASAuthorizationOpenIDOperation;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    pub struct ASAuthorizationOpenIDRequest;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    unsafe impl ClassType for ASAuthorizationOpenIDRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
unsafe impl NSCoding for ASAuthorizationOpenIDRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
unsafe impl NSCopying for ASAuthorizationOpenIDRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationOpenIDRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
unsafe impl NSSecureCoding for ASAuthorizationOpenIDRequest {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    unsafe impl ASAuthorizationOpenIDRequest {
        #[cfg(feature = "AuthenticationServices_ASAuthorization")]
        #[method_id(@__retain_semantics Other requestedScopes)]
        pub unsafe fn requestedScopes(&self) -> Option<Id<NSArray<ASAuthorizationScope>>>;

        #[cfg(feature = "AuthenticationServices_ASAuthorization")]
        #[method(setRequestedScopes:)]
        pub unsafe fn setRequestedScopes(
            &self,
            requested_scopes: Option<&NSArray<ASAuthorizationScope>>,
        );

        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Option<Id<NSString>>;

        #[method(setState:)]
        pub unsafe fn setState(&self, state: Option<&NSString>);

        #[method_id(@__retain_semantics Other nonce)]
        pub unsafe fn nonce(&self) -> Option<Id<NSString>>;

        #[method(setNonce:)]
        pub unsafe fn setNonce(&self, nonce: Option<&NSString>);

        #[method_id(@__retain_semantics Other requestedOperation)]
        pub unsafe fn requestedOperation(&self) -> Id<ASAuthorizationOpenIDOperation>;

        #[method(setRequestedOperation:)]
        pub unsafe fn setRequestedOperation(
            &self,
            requested_operation: &ASAuthorizationOpenIDOperation,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `ASAuthorizationRequest`
    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    unsafe impl ASAuthorizationOpenIDRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
