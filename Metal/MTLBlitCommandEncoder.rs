//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLBlitOption(pub NSUInteger);
bitflags::bitflags! {
    impl MTLBlitOption: NSUInteger {
        #[doc(alias = "MTLBlitOptionNone")]
        const None = 0;
        #[doc(alias = "MTLBlitOptionDepthFromDepthStencil")]
        const DepthFromDepthStencil = 1<<0;
        #[doc(alias = "MTLBlitOptionStencilFromDepthStencil")]
        const StencilFromDepthStencil = 1<<1;
        #[doc(alias = "MTLBlitOptionRowLinearPVRTC")]
        const RowLinearPVRTC = 1<<2;
    }
}

unsafe impl Encode for MTLBlitOption {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLBlitOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    #[cfg(feature = "MTLCommandEncoder")]
    pub unsafe trait MTLBlitCommandEncoder: MTLCommandEncoder + IsRetainable {
        #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
        #[method(synchronizeResource:)]
        fn synchronizeResource(&self, resource: &ProtocolObject<dyn MTLResource>);

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(synchronizeTexture:slice:level:)]
        unsafe fn synchronizeTexture_slice_level(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            slice: NSUInteger,
            level: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture",
            feature = "MTLTypes"
        ))]
        #[method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:)]
        unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            source_slice: NSUInteger,
            source_level: NSUInteger,
            source_origin: MTLOrigin,
            source_size: MTLSize,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            destination_slice: NSUInteger,
            destination_level: NSUInteger,
            destination_origin: MTLOrigin,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource",
            feature = "MTLTexture",
            feature = "MTLTypes"
        ))]
        #[method(copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:)]
        unsafe fn copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
            &self,
            source_buffer: &ProtocolObject<dyn MTLBuffer>,
            source_offset: NSUInteger,
            source_bytes_per_row: NSUInteger,
            source_bytes_per_image: NSUInteger,
            source_size: MTLSize,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            destination_slice: NSUInteger,
            destination_level: NSUInteger,
            destination_origin: MTLOrigin,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource",
            feature = "MTLTexture",
            feature = "MTLTypes"
        ))]
        #[method(copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:options:)]
        unsafe fn copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_options(
            &self,
            source_buffer: &ProtocolObject<dyn MTLBuffer>,
            source_offset: NSUInteger,
            source_bytes_per_row: NSUInteger,
            source_bytes_per_image: NSUInteger,
            source_size: MTLSize,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            destination_slice: NSUInteger,
            destination_level: NSUInteger,
            destination_origin: MTLOrigin,
            options: MTLBlitOption,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource",
            feature = "MTLTexture",
            feature = "MTLTypes"
        ))]
        #[method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:)]
        unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            source_slice: NSUInteger,
            source_level: NSUInteger,
            source_origin: MTLOrigin,
            source_size: MTLSize,
            destination_buffer: &ProtocolObject<dyn MTLBuffer>,
            destination_offset: NSUInteger,
            destination_bytes_per_row: NSUInteger,
            destination_bytes_per_image: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource",
            feature = "MTLTexture",
            feature = "MTLTypes"
        ))]
        #[method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)]
        unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_options(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            source_slice: NSUInteger,
            source_level: NSUInteger,
            source_origin: MTLOrigin,
            source_size: MTLSize,
            destination_buffer: &ProtocolObject<dyn MTLBuffer>,
            destination_offset: NSUInteger,
            destination_bytes_per_row: NSUInteger,
            destination_bytes_per_image: NSUInteger,
            options: MTLBlitOption,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(generateMipmapsForTexture:)]
        fn generateMipmapsForTexture(&self, texture: &ProtocolObject<dyn MTLTexture>);

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(fillBuffer:range:value:)]
        fn fillBuffer_range_value(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            range: NSRange,
            value: u8,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:)]
        unsafe fn copyFromTexture_sourceSlice_sourceLevel_toTexture_destinationSlice_destinationLevel_sliceCount_levelCount(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            source_slice: NSUInteger,
            source_level: NSUInteger,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            destination_slice: NSUInteger,
            destination_level: NSUInteger,
            slice_count: NSUInteger,
            level_count: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(copyFromTexture:toTexture:)]
        unsafe fn copyFromTexture_toTexture(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:)]
        unsafe fn copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size(
            &self,
            source_buffer: &ProtocolObject<dyn MTLBuffer>,
            source_offset: NSUInteger,
            destination_buffer: &ProtocolObject<dyn MTLBuffer>,
            destination_offset: NSUInteger,
            size: NSUInteger,
        );

        #[cfg(feature = "MTLFence")]
        #[method(updateFence:)]
        fn updateFence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[cfg(feature = "MTLFence")]
        #[method(waitForFence:)]
        fn waitForFence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource",
            feature = "MTLTexture",
            feature = "MTLTypes"
        ))]
        #[optional]
        #[method(getTextureAccessCounters:region:mipLevel:slice:resetCounters:countersBuffer:countersBufferOffset:)]
        unsafe fn getTextureAccessCounters_region_mipLevel_slice_resetCounters_countersBuffer_countersBufferOffset(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            region: MTLRegion,
            mip_level: NSUInteger,
            slice: NSUInteger,
            reset_counters: bool,
            counters_buffer: &ProtocolObject<dyn MTLBuffer>,
            counters_buffer_offset: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture",
            feature = "MTLTypes"
        ))]
        #[optional]
        #[method(resetTextureAccessCounters:region:mipLevel:slice:)]
        unsafe fn resetTextureAccessCounters_region_mipLevel_slice(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            region: MTLRegion,
            mip_level: NSUInteger,
            slice: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(optimizeContentsForGPUAccess:)]
        fn optimizeContentsForGPUAccess(&self, texture: &ProtocolObject<dyn MTLTexture>);

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(optimizeContentsForGPUAccess:slice:level:)]
        unsafe fn optimizeContentsForGPUAccess_slice_level(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            slice: NSUInteger,
            level: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(optimizeContentsForCPUAccess:)]
        unsafe fn optimizeContentsForCPUAccess(&self, texture: &ProtocolObject<dyn MTLTexture>);

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(optimizeContentsForCPUAccess:slice:level:)]
        unsafe fn optimizeContentsForCPUAccess_slice_level(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            slice: NSUInteger,
            level: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLIndirectCommandBuffer",
            feature = "MTLResource"
        ))]
        #[method(resetCommandsInBuffer:withRange:)]
        unsafe fn resetCommandsInBuffer_withRange(
            &self,
            buffer: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            range: NSRange,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLIndirectCommandBuffer",
            feature = "MTLResource"
        ))]
        #[method(copyIndirectCommandBuffer:sourceRange:destination:destinationIndex:)]
        unsafe fn copyIndirectCommandBuffer_sourceRange_destination_destinationIndex(
            &self,
            source: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            source_range: NSRange,
            destination: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            destination_index: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLIndirectCommandBuffer",
            feature = "MTLResource"
        ))]
        #[method(optimizeIndirectCommandBuffer:withRange:)]
        unsafe fn optimizeIndirectCommandBuffer_withRange(
            &self,
            indirect_command_buffer: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            range: NSRange,
        );

        #[cfg(feature = "MTLCounters")]
        #[method(sampleCountersInBuffer:atSampleIndex:withBarrier:)]
        unsafe fn sampleCountersInBuffer_atSampleIndex_withBarrier(
            &self,
            sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
            sample_index: NSUInteger,
            barrier: bool,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLCounters",
            feature = "MTLResource"
        ))]
        #[method(resolveCounters:inRange:destinationBuffer:destinationOffset:)]
        unsafe fn resolveCounters_inRange_destinationBuffer_destinationOffset(
            &self,
            sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
            range: NSRange,
            destination_buffer: &ProtocolObject<dyn MTLBuffer>,
            destination_offset: NSUInteger,
        );
    }

    #[cfg(feature = "MTLCommandEncoder")]
    unsafe impl ProtocolType for dyn MTLBlitCommandEncoder {}
);
