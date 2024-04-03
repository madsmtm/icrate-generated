//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AuthenticationServices_ASAuthorizationOpenIDRequest",
        feature = "AuthenticationServices_ASAuthorizationRequest"
    ))]
    pub struct ASAuthorizationAppleIDRequest;

    #[cfg(all(
        feature = "AuthenticationServices_ASAuthorizationOpenIDRequest",
        feature = "AuthenticationServices_ASAuthorizationRequest"
    ))]
    unsafe impl ClassType for ASAuthorizationAppleIDRequest {
        #[inherits(ASAuthorizationRequest, NSObject)]
        type Super = ASAuthorizationOpenIDRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationOpenIDRequest",
    feature = "AuthenticationServices_ASAuthorizationRequest"
))]
unsafe impl NSCoding for ASAuthorizationAppleIDRequest {}

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationOpenIDRequest",
    feature = "AuthenticationServices_ASAuthorizationRequest"
))]
unsafe impl NSCopying for ASAuthorizationAppleIDRequest {}

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationOpenIDRequest",
    feature = "AuthenticationServices_ASAuthorizationRequest"
))]
unsafe impl NSObjectProtocol for ASAuthorizationAppleIDRequest {}

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationOpenIDRequest",
    feature = "AuthenticationServices_ASAuthorizationRequest"
))]
unsafe impl NSSecureCoding for ASAuthorizationAppleIDRequest {}

extern_methods!(
    #[cfg(all(
        feature = "AuthenticationServices_ASAuthorizationOpenIDRequest",
        feature = "AuthenticationServices_ASAuthorizationRequest"
    ))]
    unsafe impl ASAuthorizationAppleIDRequest {
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Option<Id<NSString>>;

        #[method(setUser:)]
        pub unsafe fn setUser(&self, user: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `ASAuthorizationRequest`
    #[cfg(all(
        feature = "AuthenticationServices_ASAuthorizationOpenIDRequest",
        feature = "AuthenticationServices_ASAuthorizationRequest"
    ))]
    unsafe impl ASAuthorizationAppleIDRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
