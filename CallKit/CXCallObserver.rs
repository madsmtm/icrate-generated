//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait CXCallObserverDelegate: NSObjectProtocol {
        #[cfg(feature = "CallKit_CXCall")]
        #[method(callObserver:callChanged:)]
        unsafe fn callObserver_callChanged(&self, call_observer: &CXCallObserver, call: &CXCall);
    }

    unsafe impl ProtocolType for dyn CXCallObserverDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXCallObserver;

    unsafe impl ClassType for CXCallObserver {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CXCallObserver {}

extern_methods!(
    unsafe impl CXCallObserver {
        #[cfg(feature = "CallKit_CXCall")]
        #[method_id(@__retain_semantics Other calls)]
        pub unsafe fn calls(&self) -> Id<NSArray<CXCall>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CXCallObserver {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
