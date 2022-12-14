//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationRequest;

    unsafe impl ClassType for ASAuthorizationRequest {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl ASAuthorizationRequest {
        #[method_id(@__retain_semantics Other provider)]
        pub unsafe fn provider(&self) -> Id<ASAuthorizationProvider, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
