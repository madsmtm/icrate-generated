//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    #[cfg(feature = "Metal_MTLResource")]
    pub unsafe trait MTLBuffer: MTLResource {
        #[method(length)]
        fn length(&self) -> NSUInteger;

        #[method(contents)]
        fn contents(&self) -> NonNull<c_void>;

        #[method(didModifyRange:)]
        fn didModifyRange(&self, range: NSRange);

        #[cfg(feature = "Metal_MTLTexture")]
        #[method_id(@__retain_semantics New newTextureWithDescriptor:offset:bytesPerRow:)]
        fn newTextureWithDescriptor_offset_bytesPerRow(
            &self,
            descriptor: &MTLTextureDescriptor,
            offset: NSUInteger,
            bytes_per_row: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method(addDebugMarker:range:)]
        fn addDebugMarker_range(&self, marker: &NSString, range: NSRange);

        #[method(removeAllDebugMarkers)]
        fn removeAllDebugMarkers(&self);

        #[method_id(@__retain_semantics Other remoteStorageBuffer)]
        fn remoteStorageBuffer(&self) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[cfg(feature = "Metal_MTLDevice")]
        #[method_id(@__retain_semantics New newRemoteBufferViewForDevice:)]
        fn newRemoteBufferViewForDevice(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[method(gpuAddress)]
        fn gpuAddress(&self) -> u64;
    }

    #[cfg(feature = "Metal_MTLResource")]
    unsafe impl ProtocolType for dyn MTLBuffer {}
);
