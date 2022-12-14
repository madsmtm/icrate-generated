//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_static!(MTLCaptureErrorDomain: &'static NSErrorDomain);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLCaptureError {
        MTLCaptureErrorNotSupported = 1,
        MTLCaptureErrorAlreadyCapturing = 2,
        MTLCaptureErrorInvalidDescriptor = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLCaptureDestination {
        MTLCaptureDestinationDeveloperTools = 1,
        MTLCaptureDestinationGPUTraceDocument = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCaptureDescriptor;

    unsafe impl ClassType for MTLCaptureDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MTLCaptureDescriptor {
        #[method_id(@__retain_semantics Other captureObject)]
        pub unsafe fn captureObject(&self) -> Option<Id<Object, Shared>>;

        #[method(setCaptureObject:)]
        pub unsafe fn setCaptureObject(&self, captureObject: Option<&Object>);

        #[method(destination)]
        pub fn destination(&self) -> MTLCaptureDestination;

        #[method(setDestination:)]
        pub fn setDestination(&self, destination: MTLCaptureDestination);

        #[method_id(@__retain_semantics Other outputURL)]
        pub fn outputURL(&self) -> Option<Id<NSURL, Shared>>;

        #[method(setOutputURL:)]
        pub fn setOutputURL(&self, outputURL: Option<&NSURL>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCaptureManager;

    unsafe impl ClassType for MTLCaptureManager {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MTLCaptureManager {
        #[method_id(@__retain_semantics Other sharedCaptureManager)]
        pub unsafe fn sharedCaptureManager() -> Id<MTLCaptureManager, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New newCaptureScopeWithDevice:)]
        pub fn newCaptureScopeWithDevice(&self, device: &MTLDevice) -> Id<MTLCaptureScope, Shared>;

        #[method_id(@__retain_semantics New newCaptureScopeWithCommandQueue:)]
        pub fn newCaptureScopeWithCommandQueue(
            &self,
            commandQueue: &MTLCommandQueue,
        ) -> Id<MTLCaptureScope, Shared>;

        #[method(supportsDestination:)]
        pub fn supportsDestination(&self, destination: MTLCaptureDestination) -> bool;

        #[method(startCaptureWithDescriptor:error:_)]
        pub fn startCaptureWithDescriptor_error(
            &self,
            descriptor: &MTLCaptureDescriptor,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(startCaptureWithDevice:)]
        pub fn startCaptureWithDevice(&self, device: &MTLDevice);

        #[method(startCaptureWithCommandQueue:)]
        pub fn startCaptureWithCommandQueue(&self, commandQueue: &MTLCommandQueue);

        #[method(startCaptureWithScope:)]
        pub fn startCaptureWithScope(&self, captureScope: &MTLCaptureScope);

        #[method(stopCapture)]
        pub fn stopCapture(&self);

        #[method_id(@__retain_semantics Other defaultCaptureScope)]
        pub fn defaultCaptureScope(&self) -> Option<Id<MTLCaptureScope, Shared>>;

        #[method(setDefaultCaptureScope:)]
        pub fn setDefaultCaptureScope(&self, defaultCaptureScope: Option<&MTLCaptureScope>);

        #[method(isCapturing)]
        pub fn isCapturing(&self) -> bool;
    }
);
