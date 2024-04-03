//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSProxy")]
    pub struct NSProtocolChecker;

    #[cfg(feature = "Foundation_NSProxy")]
    unsafe impl ClassType for NSProtocolChecker {
        type Super = NSProxy;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSProxy")]
unsafe impl NSObjectProtocol for NSProtocolChecker {}

extern_methods!(
    #[cfg(feature = "Foundation_NSProxy")]
    unsafe impl NSProtocolChecker {
        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Id<AnyProtocol>;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<NSObject>>;
    }
);

extern_methods!(
    /// NSProtocolCheckerCreation
    #[cfg(feature = "Foundation_NSProxy")]
    unsafe impl NSProtocolChecker {
        #[method_id(@__retain_semantics Other protocolCheckerWithTarget:protocol:)]
        pub unsafe fn protocolCheckerWithTarget_protocol(
            an_object: &NSObject,
            a_protocol: &AnyProtocol,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithTarget:protocol:)]
        pub unsafe fn initWithTarget_protocol(
            this: Allocated<Self>,
            an_object: &NSObject,
            a_protocol: &AnyProtocol,
        ) -> Id<Self>;
    }
);
