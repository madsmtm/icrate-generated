//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLHeapType {
        MTLHeapTypeAutomatic = 0,
        MTLHeapTypePlacement = 1,
        MTLHeapTypeSparse = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLHeapDescriptor;

    unsafe impl ClassType for MTLHeapDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MTLHeapDescriptor {
        #[method(size)]
        pub fn size(&self) -> NSUInteger;

        #[method(setSize:)]
        pub fn setSize(&self, size: NSUInteger);

        #[method(storageMode)]
        pub fn storageMode(&self) -> MTLStorageMode;

        #[method(setStorageMode:)]
        pub fn setStorageMode(&self, storageMode: MTLStorageMode);

        #[method(cpuCacheMode)]
        pub fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[method(setCpuCacheMode:)]
        pub fn setCpuCacheMode(&self, cpuCacheMode: MTLCPUCacheMode);

        #[method(hazardTrackingMode)]
        pub fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[method(setHazardTrackingMode:)]
        pub fn setHazardTrackingMode(&self, hazardTrackingMode: MTLHazardTrackingMode);

        #[method(resourceOptions)]
        pub fn resourceOptions(&self) -> MTLResourceOptions;

        #[method(setResourceOptions:)]
        pub fn setResourceOptions(&self, resourceOptions: MTLResourceOptions);

        #[method(type)]
        pub unsafe fn type_(&self) -> MTLHeapType;

        #[method(setType:)]
        pub fn setType(&self, type_: MTLHeapType);
    }
);

extern_protocol!(
    pub struct MTLHeap;

    unsafe impl ProtocolType for MTLHeap {
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString, Shared>>;

        #[method(setLabel:)]
        pub fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other device)]
        pub fn device(&self) -> Id<MTLDevice, Shared>;

        #[method(storageMode)]
        pub fn storageMode(&self) -> MTLStorageMode;

        #[method(cpuCacheMode)]
        pub fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[method(hazardTrackingMode)]
        pub fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[method(resourceOptions)]
        pub fn resourceOptions(&self) -> MTLResourceOptions;

        #[method(size)]
        pub fn size(&self) -> NSUInteger;

        #[method(usedSize)]
        pub fn usedSize(&self) -> NSUInteger;

        #[method(currentAllocatedSize)]
        pub fn currentAllocatedSize(&self) -> NSUInteger;

        #[method(maxAvailableSizeWithAlignment:)]
        pub fn maxAvailableSizeWithAlignment(&self, alignment: NSUInteger) -> NSUInteger;

        #[method_id(@__retain_semantics New newBufferWithLength:options:)]
        pub fn newBufferWithLength_options(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
        ) -> Option<Id<MTLBuffer, Shared>>;

        #[method_id(@__retain_semantics New newTextureWithDescriptor:)]
        pub fn newTextureWithDescriptor(
            &self,
            desc: &MTLTextureDescriptor,
        ) -> Option<Id<MTLTexture, Shared>>;

        #[method(setPurgeableState:)]
        pub fn setPurgeableState(&self, state: MTLPurgeableState) -> MTLPurgeableState;

        #[method(type)]
        pub unsafe fn type_(&self) -> MTLHeapType;

        #[method_id(@__retain_semantics New newBufferWithLength:options:offset:)]
        pub unsafe fn newBufferWithLength_options_offset(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
            offset: NSUInteger,
        ) -> Option<Id<MTLBuffer, Shared>>;

        #[method_id(@__retain_semantics New newTextureWithDescriptor:offset:)]
        pub unsafe fn newTextureWithDescriptor_offset(
            &self,
            descriptor: &MTLTextureDescriptor,
            offset: NSUInteger,
        ) -> Option<Id<MTLTexture, Shared>>;
    }
);
