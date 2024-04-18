//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CARemoteLayerServer;

    unsafe impl ClassType for CARemoteLayerServer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CARemoteLayerServer {}

extern_methods!(
    unsafe impl CARemoteLayerServer {
        #[method_id(@__retain_semantics Other sharedServer)]
        pub unsafe fn shared_server() -> Retained<CARemoteLayerServer>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CARemoteLayerServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// CARemoteLayerServer
    #[cfg(feature = "CALayer")]
    unsafe impl CALayer {
        #[method_id(@__retain_semantics Other layerWithRemoteClientId:)]
        pub unsafe fn layer_with_remote_client_id(client_id: u32) -> Retained<CALayer>;
    }
);
