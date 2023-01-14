//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLTextureType {
        MTLTextureType1D = 0,
        MTLTextureType1DArray = 1,
        MTLTextureType2D = 2,
        MTLTextureType2DArray = 3,
        MTLTextureType2DMultisample = 4,
        MTLTextureTypeCube = 5,
        MTLTextureTypeCubeArray = 6,
        MTLTextureType3D = 7,
        MTLTextureType2DMultisampleArray = 8,
        MTLTextureTypeTextureBuffer = 9,
    }
);

ns_enum!(
    #[underlying(u8)]
    pub enum MTLTextureSwizzle {
        MTLTextureSwizzleZero = 0,
        MTLTextureSwizzleOne = 1,
        MTLTextureSwizzleRed = 2,
        MTLTextureSwizzleGreen = 3,
        MTLTextureSwizzleBlue = 4,
        MTLTextureSwizzleAlpha = 5,
    }
);

extern_struct!(
    pub struct MTLTextureSwizzleChannels {
        pub red: MTLTextureSwizzle,
        pub green: MTLTextureSwizzle,
        pub blue: MTLTextureSwizzle,
        pub alpha: MTLTextureSwizzle,
    }
);

inline_fn!(
    pub unsafe fn MTLTextureSwizzleChannelsMake(
        r: MTLTextureSwizzle,
        g: MTLTextureSwizzle,
        b: MTLTextureSwizzle,
        a: MTLTextureSwizzle,
    ) -> MTLTextureSwizzleChannels {
        todo!()
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLSharedTextureHandle")]
    pub struct MTLSharedTextureHandle;

    #[cfg(feature = "Metal_MTLSharedTextureHandle")]
    unsafe impl ClassType for MTLSharedTextureHandle {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLSharedTextureHandle")]
    unsafe impl MTLSharedTextureHandle {
        #[method_id(@__retain_semantics Other device)]
        pub fn device(&self) -> Id<MTLDevice, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString, Shared>>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLTextureUsage {
        MTLTextureUsageUnknown = 0x0000,
        MTLTextureUsageShaderRead = 0x0001,
        MTLTextureUsageShaderWrite = 0x0002,
        MTLTextureUsageRenderTarget = 0x0004,
        MTLTextureUsagePixelFormatView = 0x0010,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLTextureCompressionType {
        MTLTextureCompressionTypeLossless = 0,
        MTLTextureCompressionTypeLossy = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLTextureDescriptor")]
    pub struct MTLTextureDescriptor;

    #[cfg(feature = "Metal_MTLTextureDescriptor")]
    unsafe impl ClassType for MTLTextureDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLTextureDescriptor")]
    unsafe impl MTLTextureDescriptor {
        #[method_id(@__retain_semantics Other texture2DDescriptorWithPixelFormat:width:height:mipmapped:)]
        pub unsafe fn texture2DDescriptorWithPixelFormat_width_height_mipmapped(
            pixel_format: MTLPixelFormat,
            width: NSUInteger,
            height: NSUInteger,
            mipmapped: bool,
        ) -> Id<MTLTextureDescriptor, Shared>;

        #[method_id(@__retain_semantics Other textureCubeDescriptorWithPixelFormat:size:mipmapped:)]
        pub unsafe fn textureCubeDescriptorWithPixelFormat_size_mipmapped(
            pixel_format: MTLPixelFormat,
            size: NSUInteger,
            mipmapped: bool,
        ) -> Id<MTLTextureDescriptor, Shared>;

        #[method_id(@__retain_semantics Other textureBufferDescriptorWithPixelFormat:width:resourceOptions:usage:)]
        pub unsafe fn textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage(
            pixel_format: MTLPixelFormat,
            width: NSUInteger,
            resource_options: MTLResourceOptions,
            usage: MTLTextureUsage,
        ) -> Id<MTLTextureDescriptor, Shared>;

        #[method(textureType)]
        pub fn textureType(&self) -> MTLTextureType;

        #[method(setTextureType:)]
        pub fn setTextureType(&self, texture_type: MTLTextureType);

        #[method(pixelFormat)]
        pub fn pixelFormat(&self) -> MTLPixelFormat;

        #[method(setPixelFormat:)]
        pub fn setPixelFormat(&self, pixel_format: MTLPixelFormat);

        #[method(width)]
        pub fn width(&self) -> NSUInteger;

        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: NSUInteger);

        #[method(height)]
        pub fn height(&self) -> NSUInteger;

        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: NSUInteger);

        #[method(depth)]
        pub fn depth(&self) -> NSUInteger;

        #[method(setDepth:)]
        pub unsafe fn setDepth(&self, depth: NSUInteger);

        #[method(mipmapLevelCount)]
        pub fn mipmapLevelCount(&self) -> NSUInteger;

        #[method(setMipmapLevelCount:)]
        pub unsafe fn setMipmapLevelCount(&self, mipmap_level_count: NSUInteger);

        #[method(sampleCount)]
        pub fn sampleCount(&self) -> NSUInteger;

        #[method(setSampleCount:)]
        pub unsafe fn setSampleCount(&self, sample_count: NSUInteger);

        #[method(arrayLength)]
        pub fn arrayLength(&self) -> NSUInteger;

        #[method(setArrayLength:)]
        pub unsafe fn setArrayLength(&self, array_length: NSUInteger);

        #[method(resourceOptions)]
        pub fn resourceOptions(&self) -> MTLResourceOptions;

        #[method(setResourceOptions:)]
        pub fn setResourceOptions(&self, resource_options: MTLResourceOptions);

        #[method(cpuCacheMode)]
        pub fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[method(setCpuCacheMode:)]
        pub fn setCpuCacheMode(&self, cpu_cache_mode: MTLCPUCacheMode);

        #[method(storageMode)]
        pub fn storageMode(&self) -> MTLStorageMode;

        #[method(setStorageMode:)]
        pub fn setStorageMode(&self, storage_mode: MTLStorageMode);

        #[method(hazardTrackingMode)]
        pub fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[method(setHazardTrackingMode:)]
        pub fn setHazardTrackingMode(&self, hazard_tracking_mode: MTLHazardTrackingMode);

        #[method(usage)]
        pub fn usage(&self) -> MTLTextureUsage;

        #[method(setUsage:)]
        pub fn setUsage(&self, usage: MTLTextureUsage);

        #[method(allowGPUOptimizedContents)]
        pub fn allowGPUOptimizedContents(&self) -> bool;

        #[method(setAllowGPUOptimizedContents:)]
        pub fn setAllowGPUOptimizedContents(&self, allow_gpu_optimized_contents: bool);

        #[method(compressionType)]
        pub unsafe fn compressionType(&self) -> MTLTextureCompressionType;

        #[method(setCompressionType:)]
        pub unsafe fn setCompressionType(&self, compression_type: MTLTextureCompressionType);

        #[method(swizzle)]
        pub fn swizzle(&self) -> MTLTextureSwizzleChannels;

        #[method(setSwizzle:)]
        pub fn setSwizzle(&self, swizzle: MTLTextureSwizzleChannels);
    }
);

extern_protocol!(
    pub struct MTLTexture;

    unsafe impl ProtocolType for MTLTexture {
        #[method_id(@__retain_semantics Other rootResource)]
        pub fn rootResource(&self) -> Option<Id<MTLResource, Shared>>;

        #[method_id(@__retain_semantics Other parentTexture)]
        pub fn parentTexture(&self) -> Option<Id<MTLTexture, Shared>>;

        #[method(parentRelativeLevel)]
        pub fn parentRelativeLevel(&self) -> NSUInteger;

        #[method(parentRelativeSlice)]
        pub fn parentRelativeSlice(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other buffer)]
        pub fn buffer(&self) -> Option<Id<MTLBuffer, Shared>>;

        #[method(bufferOffset)]
        pub fn bufferOffset(&self) -> NSUInteger;

        #[method(bufferBytesPerRow)]
        pub fn bufferBytesPerRow(&self) -> NSUInteger;

        #[method(iosurfacePlane)]
        pub fn iosurfacePlane(&self) -> NSUInteger;

        #[method(textureType)]
        pub fn textureType(&self) -> MTLTextureType;

        #[method(pixelFormat)]
        pub fn pixelFormat(&self) -> MTLPixelFormat;

        #[method(width)]
        pub fn width(&self) -> NSUInteger;

        #[method(height)]
        pub fn height(&self) -> NSUInteger;

        #[method(depth)]
        pub fn depth(&self) -> NSUInteger;

        #[method(mipmapLevelCount)]
        pub fn mipmapLevelCount(&self) -> NSUInteger;

        #[method(sampleCount)]
        pub fn sampleCount(&self) -> NSUInteger;

        #[method(arrayLength)]
        pub fn arrayLength(&self) -> NSUInteger;

        #[method(usage)]
        pub fn usage(&self) -> MTLTextureUsage;

        #[method(isShareable)]
        pub fn isShareable(&self) -> bool;

        #[method(isFramebufferOnly)]
        pub fn isFramebufferOnly(&self) -> bool;

        #[optional]
        #[method(firstMipmapInTail)]
        pub fn firstMipmapInTail(&self) -> NSUInteger;

        #[optional]
        #[method(tailSizeInBytes)]
        pub fn tailSizeInBytes(&self) -> NSUInteger;

        #[optional]
        #[method(isSparse)]
        pub fn isSparse(&self) -> bool;

        #[method(allowGPUOptimizedContents)]
        pub fn allowGPUOptimizedContents(&self) -> bool;

        #[method(compressionType)]
        pub unsafe fn compressionType(&self) -> MTLTextureCompressionType;

        #[method(gpuResourceID)]
        pub unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[method(getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:)]
        pub unsafe fn getBytes_bytesPerRow_bytesPerImage_fromRegion_mipmapLevel_slice(
            &self,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            bytes_per_image: NSUInteger,
            region: MTLRegion,
            level: NSUInteger,
            slice: NSUInteger,
        );

        #[method(replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:)]
        pub unsafe fn replaceRegion_mipmapLevel_slice_withBytes_bytesPerRow_bytesPerImage(
            &self,
            region: MTLRegion,
            level: NSUInteger,
            slice: NSUInteger,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            bytes_per_image: NSUInteger,
        );

        #[method(getBytes:bytesPerRow:fromRegion:mipmapLevel:)]
        pub unsafe fn getBytes_bytesPerRow_fromRegion_mipmapLevel(
            &self,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            region: MTLRegion,
            level: NSUInteger,
        );

        #[method(replaceRegion:mipmapLevel:withBytes:bytesPerRow:)]
        pub unsafe fn replaceRegion_mipmapLevel_withBytes_bytesPerRow(
            &self,
            region: MTLRegion,
            level: NSUInteger,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
        );

        #[method_id(@__retain_semantics New newTextureViewWithPixelFormat:)]
        pub fn newTextureViewWithPixelFormat(
            &self,
            pixel_format: MTLPixelFormat,
        ) -> Option<Id<MTLTexture, Shared>>;

        #[method_id(@__retain_semantics New newTextureViewWithPixelFormat:textureType:levels:slices:)]
        pub unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices(
            &self,
            pixel_format: MTLPixelFormat,
            texture_type: MTLTextureType,
            level_range: NSRange,
            slice_range: NSRange,
        ) -> Option<Id<MTLTexture, Shared>>;

        #[cfg(feature = "Metal_MTLSharedTextureHandle")]
        #[method_id(@__retain_semantics New newSharedTextureHandle)]
        pub fn newSharedTextureHandle(&self) -> Option<Id<MTLSharedTextureHandle, Shared>>;

        #[method_id(@__retain_semantics Other remoteStorageTexture)]
        pub fn remoteStorageTexture(&self) -> Option<Id<MTLTexture, Shared>>;

        #[method_id(@__retain_semantics New newRemoteTextureViewForDevice:)]
        pub unsafe fn newRemoteTextureViewForDevice(
            &self,
            device: &MTLDevice,
        ) -> Option<Id<MTLTexture, Shared>>;

        #[method(swizzle)]
        pub fn swizzle(&self) -> MTLTextureSwizzleChannels;

        #[method_id(@__retain_semantics New newTextureViewWithPixelFormat:textureType:levels:slices:swizzle:)]
        pub unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices_swizzle(
            &self,
            pixel_format: MTLPixelFormat,
            texture_type: MTLTextureType,
            level_range: NSRange,
            slice_range: NSRange,
            swizzle: MTLTextureSwizzleChannels,
        ) -> Option<Id<MTLTexture, Shared>>;
    }
);
