//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPasswordProvider;

    unsafe impl ClassType for ASAuthorizationPasswordProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationProvider")]
unsafe impl ASAuthorizationProvider for ASAuthorizationPasswordProvider {}

unsafe impl NSObjectProtocol for ASAuthorizationPasswordProvider {}

extern_methods!(
    unsafe impl ASAuthorizationPasswordProvider {
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationPasswordRequest",
            feature = "AuthenticationServices_ASAuthorizationRequest"
        ))]
        #[method_id(@__retain_semantics Other createRequest)]
        pub unsafe fn createRequest(&self) -> Id<ASAuthorizationPasswordRequest>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAuthorizationPasswordProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
