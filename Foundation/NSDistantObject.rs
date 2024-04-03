//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSProxy")]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSDistantObject;

    #[cfg(feature = "Foundation_NSProxy")]
    unsafe impl ClassType for NSDistantObject {
        type Super = NSProxy;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "Foundation_NSObject", feature = "Foundation_NSProxy"))]
unsafe impl NSCoding for NSDistantObject {}

#[cfg(feature = "Foundation_NSProxy")]
unsafe impl NSObjectProtocol for NSDistantObject {}

extern_methods!(
    #[cfg(feature = "Foundation_NSProxy")]
    unsafe impl NSDistantObject {
        #[cfg(feature = "Foundation_NSConnection")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other proxyWithTarget:connection:)]
        pub unsafe fn proxyWithTarget_connection(
            target: &AnyObject,
            connection: &NSConnection,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSConnection")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Init initWithTarget:connection:)]
        pub unsafe fn initWithTarget_connection(
            this: Allocated<Self>,
            target: &AnyObject,
            connection: &NSConnection,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSConnection")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other proxyWithLocal:connection:)]
        pub unsafe fn proxyWithLocal_connection(
            target: &AnyObject,
            connection: &NSConnection,
        ) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSConnection")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Init initWithLocal:connection:)]
        pub unsafe fn initWithLocal_connection(
            this: Allocated<Self>,
            target: &AnyObject,
            connection: &NSConnection,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setProtocolForProxy:)]
        pub unsafe fn setProtocolForProxy(&self, proto: Option<&AnyProtocol>);

        #[cfg(feature = "Foundation_NSConnection")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other connectionForProxy)]
        pub unsafe fn connectionForProxy(&self) -> Id<NSConnection>;
    }
);
