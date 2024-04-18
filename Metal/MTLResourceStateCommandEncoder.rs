//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLSparseTextureMappingMode(pub NSUInteger);
impl MTLSparseTextureMappingMode {
    #[doc(alias = "MTLSparseTextureMappingModeMap")]
    pub const Map: Self = Self(0);
    #[doc(alias = "MTLSparseTextureMappingModeUnmap")]
    pub const Unmap: Self = Self(1);
}

unsafe impl Encode for MTLSparseTextureMappingMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLSparseTextureMappingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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

unsafe impl Encode for MTLMapIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLMapIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    #[cfg(feature = "MTLCommandEncoder")]
    pub unsafe trait MTLResourceStateCommandEncoder:
        MTLCommandEncoder + IsRetainable
    {
        #[cfg(all(feature = "MTLResource", feature = "MTLTexture", feature = "MTLTypes"))]
        #[optional]
        #[method(updateTextureMappings:mode:regions:mipLevels:slices:numRegions:)]
        unsafe fn update_texture_mappings_mode_regions_mip_levels_slices_num_regions(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            mode: MTLSparseTextureMappingMode,
            regions: NonNull<MTLRegion>,
            mip_levels: NonNull<NSUInteger>,
            slices: NonNull<NSUInteger>,
            num_regions: NSUInteger,
        );

        #[cfg(all(feature = "MTLResource", feature = "MTLTexture", feature = "MTLTypes"))]
        #[optional]
        #[method(updateTextureMapping:mode:region:mipLevel:slice:)]
        unsafe fn update_texture_mapping_mode_region_mip_level_slice(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            mode: MTLSparseTextureMappingMode,
            region: MTLRegion,
            mip_level: NSUInteger,
            slice: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource", feature = "MTLTexture"))]
        #[optional]
        #[method(updateTextureMapping:mode:indirectBuffer:indirectBufferOffset:)]
        unsafe fn update_texture_mapping_mode_indirect_buffer_indirect_buffer_offset(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            mode: MTLSparseTextureMappingMode,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: NSUInteger,
        );

        #[cfg(feature = "MTLFence")]
        #[optional]
        #[method(updateFence:)]
        unsafe fn update_fence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[cfg(feature = "MTLFence")]
        #[optional]
        #[method(waitForFence:)]
        unsafe fn wait_for_fence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[cfg(all(feature = "MTLResource", feature = "MTLTexture", feature = "MTLTypes"))]
        #[optional]
        #[method(moveTextureMappingsFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:)]
        unsafe fn move_texture_mappings_from_texture_source_slice_source_level_source_origin_source_size_to_texture_destination_slice_destination_level_destination_origin(
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
    }

    #[cfg(feature = "MTLCommandEncoder")]
    unsafe impl ProtocolType for dyn MTLResourceStateCommandEncoder {}
);
