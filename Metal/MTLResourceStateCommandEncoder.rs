//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLSparseTextureMappingMode {
        MTLSparseTextureMappingModeMap = 0,
        MTLSparseTextureMappingModeUnmap = 1,
    }
);

extern_struct!(
    pub struct MTLMapIndirectArguments {
        pub regionOriginX: u32,
        pub regionOriginY: u32,
        pub regionOriginZ: u32,
        pub regionSizeWidth: u32,
        pub regionSizeHeight: u32,
        pub regionSizeDepth: u32,
        pub mipMapLevel: u32,
        pub sliceId: u32,
    }
);

extern_protocol!(
    pub struct MTLResourceStateCommandEncoder;

    unsafe impl ProtocolType for MTLResourceStateCommandEncoder {
        #[optional]
        #[method(updateTextureMappings:mode:regions:mipLevels:slices:numRegions:)]
        pub unsafe fn updateTextureMappings_mode_regions_mipLevels_slices_numRegions(
            &self,
            texture: &MTLTexture,
            mode: MTLSparseTextureMappingMode,
            regions: NonNull<MTLRegion>,
            mip_levels: NonNull<NSUInteger>,
            slices: NonNull<NSUInteger>,
            num_regions: NSUInteger,
        );

        #[optional]
        #[method(updateTextureMapping:mode:region:mipLevel:slice:)]
        pub unsafe fn updateTextureMapping_mode_region_mipLevel_slice(
            &self,
            texture: &MTLTexture,
            mode: MTLSparseTextureMappingMode,
            region: MTLRegion,
            mip_level: NSUInteger,
            slice: NSUInteger,
        );

        #[optional]
        #[method(updateTextureMapping:mode:indirectBuffer:indirectBufferOffset:)]
        pub unsafe fn updateTextureMapping_mode_indirectBuffer_indirectBufferOffset(
            &self,
            texture: &MTLTexture,
            mode: MTLSparseTextureMappingMode,
            indirect_buffer: &MTLBuffer,
            indirect_buffer_offset: NSUInteger,
        );

        #[optional]
        #[method(updateFence:)]
        pub unsafe fn updateFence(&self, fence: &MTLFence);

        #[optional]
        #[method(waitForFence:)]
        pub unsafe fn waitForFence(&self, fence: &MTLFence);

        #[optional]
        #[method(moveTextureMappingsFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:)]
        pub unsafe fn moveTextureMappingsFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
            &self,
            source_texture: &MTLTexture,
            source_slice: NSUInteger,
            source_level: NSUInteger,
            source_origin: MTLOrigin,
            source_size: MTLSize,
            destination_texture: &MTLTexture,
            destination_slice: NSUInteger,
            destination_level: NSUInteger,
            destination_origin: MTLOrigin,
        );
    }
);
