//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type ASAuthorizationOpenIDOperation = NSString;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static ASAuthorizationOperationImplicit: &'static ASAuthorizationOpenIDOperation;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static ASAuthorizationOperationLogin: &'static ASAuthorizationOpenIDOperation;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static ASAuthorizationOperationRefresh: &'static ASAuthorizationOpenIDOperation;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
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

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationRequest",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for ASAuthorizationOpenIDRequest {}

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationRequest",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCopying for ASAuthorizationOpenIDRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationOpenIDRequest {}

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationRequest",
    feature = "Foundation_NSObject"
))]
unsafe impl NSSecureCoding for ASAuthorizationOpenIDRequest {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    unsafe impl ASAuthorizationOpenIDRequest {
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorization",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other requestedScopes)]
        pub unsafe fn requestedScopes(&self) -> Option<Id<NSArray<ASAuthorizationScope>>>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorization",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method(setRequestedScopes:)]
        pub unsafe fn setRequestedScopes(
            &self,
            requested_scopes: Option<&NSArray<ASAuthorizationScope>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setState:)]
        pub unsafe fn setState(&self, state: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nonce)]
        pub unsafe fn nonce(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNonce:)]
        pub unsafe fn setNonce(&self, nonce: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other requestedOperation)]
        pub unsafe fn requestedOperation(&self) -> Id<ASAuthorizationOpenIDOperation>;

        #[cfg(feature = "Foundation_NSString")]
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
