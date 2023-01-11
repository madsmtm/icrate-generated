//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
    pub struct CAReplicatorLayer;

    #[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
    unsafe impl ClassType for CAReplicatorLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
    }
);

extern_methods!(
    #[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
    unsafe impl CAReplicatorLayer {
        #[method(instanceCount)]
        pub unsafe fn instanceCount(&self) -> NSInteger;

        #[method(setInstanceCount:)]
        pub unsafe fn setInstanceCount(&self, instanceCount: NSInteger);

        #[method(preservesDepth)]
        pub unsafe fn preservesDepth(&self) -> bool;

        #[method(setPreservesDepth:)]
        pub unsafe fn setPreservesDepth(&self, preservesDepth: bool);

        #[method(instanceDelay)]
        pub unsafe fn instanceDelay(&self) -> CFTimeInterval;

        #[method(setInstanceDelay:)]
        pub unsafe fn setInstanceDelay(&self, instanceDelay: CFTimeInterval);

        #[method(instanceTransform)]
        pub unsafe fn instanceTransform(&self) -> CATransform3D;

        #[method(setInstanceTransform:)]
        pub unsafe fn setInstanceTransform(&self, instanceTransform: CATransform3D);

        #[method(instanceRedOffset)]
        pub unsafe fn instanceRedOffset(&self) -> c_float;

        #[method(setInstanceRedOffset:)]
        pub unsafe fn setInstanceRedOffset(&self, instanceRedOffset: c_float);

        #[method(instanceGreenOffset)]
        pub unsafe fn instanceGreenOffset(&self) -> c_float;

        #[method(setInstanceGreenOffset:)]
        pub unsafe fn setInstanceGreenOffset(&self, instanceGreenOffset: c_float);

        #[method(instanceBlueOffset)]
        pub unsafe fn instanceBlueOffset(&self) -> c_float;

        #[method(setInstanceBlueOffset:)]
        pub unsafe fn setInstanceBlueOffset(&self, instanceBlueOffset: c_float);

        #[method(instanceAlphaOffset)]
        pub unsafe fn instanceAlphaOffset(&self) -> c_float;

        #[method(setInstanceAlphaOffset:)]
        pub unsafe fn setInstanceAlphaOffset(&self, instanceAlphaOffset: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
    unsafe impl CAReplicatorLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(
            this: Option<Allocated<Self>>,
            layer: &Object,
        ) -> Id<Self, Shared>;
    }
);
