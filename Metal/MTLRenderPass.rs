//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLClearColor {
        pub red: c_double,
        pub green: c_double,
        pub blue: c_double,
        pub alpha: c_double,
    }
);

inline_fn!(
    pub unsafe fn MTLClearColorMake(
        red: c_double,
        green: c_double,
        blue: c_double,
        alpha: c_double,
    ) -> MTLClearColor {
        todo!()
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLLoadAction {
        MTLLoadActionDontCare = 0,
        MTLLoadActionLoad = 1,
        MTLLoadActionClear = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLStoreAction {
        MTLStoreActionDontCare = 0,
        MTLStoreActionStore = 1,
        MTLStoreActionMultisampleResolve = 2,
        MTLStoreActionStoreAndMultisampleResolve = 3,
        MTLStoreActionUnknown = 4,
        MTLStoreActionCustomSampleDepthStore = 5,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLStoreActionOptions {
        MTLStoreActionOptionNone = 0,
        MTLStoreActionOptionCustomSamplePositions = 1 << 0,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPassAttachmentDescriptor")]
    pub struct MTLRenderPassAttachmentDescriptor;

    #[cfg(feature = "Metal_MTLRenderPassAttachmentDescriptor")]
    unsafe impl ClassType for MTLRenderPassAttachmentDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLRenderPassAttachmentDescriptor")]
unsafe impl NSCopying for MTLRenderPassAttachmentDescriptor {}

#[cfg(feature = "Metal_MTLRenderPassAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLRenderPassAttachmentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPassAttachmentDescriptor")]
    unsafe impl MTLRenderPassAttachmentDescriptor {
        #[method_id(@__retain_semantics Other texture)]
        pub fn texture(&self) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method(setTexture:)]
        pub fn setTexture(&self, texture: Option<&(impl MTLTexture + Message)>);

        #[method(level)]
        pub fn level(&self) -> NSUInteger;

        #[method(setLevel:)]
        pub fn setLevel(&self, level: NSUInteger);

        #[method(slice)]
        pub fn slice(&self) -> NSUInteger;

        #[method(setSlice:)]
        pub fn setSlice(&self, slice: NSUInteger);

        #[method(depthPlane)]
        pub fn depthPlane(&self) -> NSUInteger;

        #[method(setDepthPlane:)]
        pub fn setDepthPlane(&self, depth_plane: NSUInteger);

        #[method_id(@__retain_semantics Other resolveTexture)]
        pub fn resolveTexture(&self) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method(setResolveTexture:)]
        pub fn setResolveTexture(&self, resolve_texture: Option<&(impl MTLTexture + Message)>);

        #[method(resolveLevel)]
        pub fn resolveLevel(&self) -> NSUInteger;

        #[method(setResolveLevel:)]
        pub fn setResolveLevel(&self, resolve_level: NSUInteger);

        #[method(resolveSlice)]
        pub fn resolveSlice(&self) -> NSUInteger;

        #[method(setResolveSlice:)]
        pub fn setResolveSlice(&self, resolve_slice: NSUInteger);

        #[method(resolveDepthPlane)]
        pub fn resolveDepthPlane(&self) -> NSUInteger;

        #[method(setResolveDepthPlane:)]
        pub fn setResolveDepthPlane(&self, resolve_depth_plane: NSUInteger);

        #[method(loadAction)]
        pub fn loadAction(&self) -> MTLLoadAction;

        #[method(setLoadAction:)]
        pub fn setLoadAction(&self, load_action: MTLLoadAction);

        #[method(storeAction)]
        pub fn storeAction(&self) -> MTLStoreAction;

        #[method(setStoreAction:)]
        pub fn setStoreAction(&self, store_action: MTLStoreAction);

        #[method(storeActionOptions)]
        pub fn storeActionOptions(&self) -> MTLStoreActionOptions;

        #[method(setStoreActionOptions:)]
        pub fn setStoreActionOptions(&self, store_action_options: MTLStoreActionOptions);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLRenderPassAttachmentDescriptor")]
    unsafe impl MTLRenderPassAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptor")]
    pub struct MTLRenderPassColorAttachmentDescriptor;

    #[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptor")]
    unsafe impl ClassType for MTLRenderPassColorAttachmentDescriptor {
        #[inherits(NSObject)]
        type Super = MTLRenderPassAttachmentDescriptor;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptor")]
unsafe impl NSCopying for MTLRenderPassColorAttachmentDescriptor {}

#[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLRenderPassColorAttachmentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptor")]
    unsafe impl MTLRenderPassColorAttachmentDescriptor {
        #[method(clearColor)]
        pub fn clearColor(&self) -> MTLClearColor;

        #[method(setClearColor:)]
        pub fn setClearColor(&self, clear_color: MTLClearColor);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptor")]
    unsafe impl MTLRenderPassColorAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptor")]
impl DefaultId for MTLRenderPassColorAttachmentDescriptor {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLMultisampleDepthResolveFilter {
        MTLMultisampleDepthResolveFilterSample0 = 0,
        MTLMultisampleDepthResolveFilterMin = 1,
        MTLMultisampleDepthResolveFilterMax = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPassDepthAttachmentDescriptor")]
    pub struct MTLRenderPassDepthAttachmentDescriptor;

    #[cfg(feature = "Metal_MTLRenderPassDepthAttachmentDescriptor")]
    unsafe impl ClassType for MTLRenderPassDepthAttachmentDescriptor {
        #[inherits(NSObject)]
        type Super = MTLRenderPassAttachmentDescriptor;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLRenderPassDepthAttachmentDescriptor")]
unsafe impl NSCopying for MTLRenderPassDepthAttachmentDescriptor {}

#[cfg(feature = "Metal_MTLRenderPassDepthAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLRenderPassDepthAttachmentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPassDepthAttachmentDescriptor")]
    unsafe impl MTLRenderPassDepthAttachmentDescriptor {
        #[method(clearDepth)]
        pub fn clearDepth(&self) -> c_double;

        #[method(setClearDepth:)]
        pub fn setClearDepth(&self, clear_depth: c_double);

        #[method(depthResolveFilter)]
        pub fn depthResolveFilter(&self) -> MTLMultisampleDepthResolveFilter;

        #[method(setDepthResolveFilter:)]
        pub fn setDepthResolveFilter(&self, depth_resolve_filter: MTLMultisampleDepthResolveFilter);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLRenderPassDepthAttachmentDescriptor")]
    unsafe impl MTLRenderPassDepthAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLMultisampleStencilResolveFilter {
        MTLMultisampleStencilResolveFilterSample0 = 0,
        MTLMultisampleStencilResolveFilterDepthResolvedSample = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPassStencilAttachmentDescriptor")]
    pub struct MTLRenderPassStencilAttachmentDescriptor;

    #[cfg(feature = "Metal_MTLRenderPassStencilAttachmentDescriptor")]
    unsafe impl ClassType for MTLRenderPassStencilAttachmentDescriptor {
        #[inherits(NSObject)]
        type Super = MTLRenderPassAttachmentDescriptor;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLRenderPassStencilAttachmentDescriptor")]
unsafe impl NSCopying for MTLRenderPassStencilAttachmentDescriptor {}

#[cfg(feature = "Metal_MTLRenderPassStencilAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLRenderPassStencilAttachmentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPassStencilAttachmentDescriptor")]
    unsafe impl MTLRenderPassStencilAttachmentDescriptor {
        #[method(clearStencil)]
        pub fn clearStencil(&self) -> u32;

        #[method(setClearStencil:)]
        pub fn setClearStencil(&self, clear_stencil: u32);

        #[method(stencilResolveFilter)]
        pub fn stencilResolveFilter(&self) -> MTLMultisampleStencilResolveFilter;

        #[method(setStencilResolveFilter:)]
        pub fn setStencilResolveFilter(
            &self,
            stencil_resolve_filter: MTLMultisampleStencilResolveFilter,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLRenderPassStencilAttachmentDescriptor")]
    unsafe impl MTLRenderPassStencilAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptorArray")]
    pub struct MTLRenderPassColorAttachmentDescriptorArray;

    #[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptorArray")]
    unsafe impl ClassType for MTLRenderPassColorAttachmentDescriptorArray {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptorArray")]
unsafe impl NSObjectProtocol for MTLRenderPassColorAttachmentDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptorArray")]
    unsafe impl MTLRenderPassColorAttachmentDescriptorArray {
        #[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Id<MTLRenderPassColorAttachmentDescriptor>;

        #[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLRenderPassColorAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptorArray")]
    unsafe impl MTLRenderPassColorAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptor")]
    pub struct MTLRenderPassSampleBufferAttachmentDescriptor;

    #[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptor")]
    unsafe impl ClassType for MTLRenderPassSampleBufferAttachmentDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptor")]
unsafe impl NSCopying for MTLRenderPassSampleBufferAttachmentDescriptor {}

#[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLRenderPassSampleBufferAttachmentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptor")]
    unsafe impl MTLRenderPassSampleBufferAttachmentDescriptor {
        #[method_id(@__retain_semantics Other sampleBuffer)]
        pub fn sampleBuffer(&self) -> Option<Id<ProtocolObject<dyn MTLCounterSampleBuffer>>>;

        #[method(setSampleBuffer:)]
        pub fn setSampleBuffer(
            &self,
            sample_buffer: Option<&(impl MTLCounterSampleBuffer + Message)>,
        );

        #[method(startOfVertexSampleIndex)]
        pub fn startOfVertexSampleIndex(&self) -> NSUInteger;

        #[method(setStartOfVertexSampleIndex:)]
        pub unsafe fn setStartOfVertexSampleIndex(&self, start_of_vertex_sample_index: NSUInteger);

        #[method(endOfVertexSampleIndex)]
        pub fn endOfVertexSampleIndex(&self) -> NSUInteger;

        #[method(setEndOfVertexSampleIndex:)]
        pub unsafe fn setEndOfVertexSampleIndex(&self, end_of_vertex_sample_index: NSUInteger);

        #[method(startOfFragmentSampleIndex)]
        pub fn startOfFragmentSampleIndex(&self) -> NSUInteger;

        #[method(setStartOfFragmentSampleIndex:)]
        pub unsafe fn setStartOfFragmentSampleIndex(
            &self,
            start_of_fragment_sample_index: NSUInteger,
        );

        #[method(endOfFragmentSampleIndex)]
        pub fn endOfFragmentSampleIndex(&self) -> NSUInteger;

        #[method(setEndOfFragmentSampleIndex:)]
        pub unsafe fn setEndOfFragmentSampleIndex(&self, end_of_fragment_sample_index: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptor")]
    unsafe impl MTLRenderPassSampleBufferAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptorArray")]
    pub struct MTLRenderPassSampleBufferAttachmentDescriptorArray;

    #[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptorArray")]
    unsafe impl ClassType for MTLRenderPassSampleBufferAttachmentDescriptorArray {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptorArray")]
unsafe impl NSObjectProtocol for MTLRenderPassSampleBufferAttachmentDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptorArray")]
    unsafe impl MTLRenderPassSampleBufferAttachmentDescriptorArray {
        #[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Id<MTLRenderPassSampleBufferAttachmentDescriptor>;

        #[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLRenderPassSampleBufferAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptorArray")]
    unsafe impl MTLRenderPassSampleBufferAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPassDescriptor")]
    pub struct MTLRenderPassDescriptor;

    #[cfg(feature = "Metal_MTLRenderPassDescriptor")]
    unsafe impl ClassType for MTLRenderPassDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLRenderPassDescriptor")]
unsafe impl NSCopying for MTLRenderPassDescriptor {}

#[cfg(feature = "Metal_MTLRenderPassDescriptor")]
unsafe impl NSObjectProtocol for MTLRenderPassDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPassDescriptor")]
    unsafe impl MTLRenderPassDescriptor {
        #[method_id(@__retain_semantics Other renderPassDescriptor)]
        pub fn renderPassDescriptor() -> Id<MTLRenderPassDescriptor>;

        #[cfg(feature = "Metal_MTLRenderPassColorAttachmentDescriptorArray")]
        #[method_id(@__retain_semantics Other colorAttachments)]
        pub fn colorAttachments(&self) -> Id<MTLRenderPassColorAttachmentDescriptorArray>;

        #[cfg(feature = "Metal_MTLRenderPassDepthAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other depthAttachment)]
        pub fn depthAttachment(&self) -> Id<MTLRenderPassDepthAttachmentDescriptor>;

        #[cfg(feature = "Metal_MTLRenderPassDepthAttachmentDescriptor")]
        #[method(setDepthAttachment:)]
        pub fn setDepthAttachment(
            &self,
            depth_attachment: Option<&MTLRenderPassDepthAttachmentDescriptor>,
        );

        #[cfg(feature = "Metal_MTLRenderPassStencilAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other stencilAttachment)]
        pub fn stencilAttachment(&self) -> Id<MTLRenderPassStencilAttachmentDescriptor>;

        #[cfg(feature = "Metal_MTLRenderPassStencilAttachmentDescriptor")]
        #[method(setStencilAttachment:)]
        pub fn setStencilAttachment(
            &self,
            stencil_attachment: Option<&MTLRenderPassStencilAttachmentDescriptor>,
        );

        #[method_id(@__retain_semantics Other visibilityResultBuffer)]
        pub fn visibilityResultBuffer(&self) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[method(setVisibilityResultBuffer:)]
        pub fn setVisibilityResultBuffer(
            &self,
            visibility_result_buffer: Option<&(impl MTLBuffer + Message)>,
        );

        #[method(renderTargetArrayLength)]
        pub fn renderTargetArrayLength(&self) -> NSUInteger;

        #[method(setRenderTargetArrayLength:)]
        pub unsafe fn setRenderTargetArrayLength(&self, render_target_array_length: NSUInteger);

        #[method(imageblockSampleLength)]
        pub fn imageblockSampleLength(&self) -> NSUInteger;

        #[method(setImageblockSampleLength:)]
        pub unsafe fn setImageblockSampleLength(&self, imageblock_sample_length: NSUInteger);

        #[method(threadgroupMemoryLength)]
        pub fn threadgroupMemoryLength(&self) -> NSUInteger;

        #[method(setThreadgroupMemoryLength:)]
        pub unsafe fn setThreadgroupMemoryLength(&self, threadgroup_memory_length: NSUInteger);

        #[method(tileWidth)]
        pub fn tileWidth(&self) -> NSUInteger;

        #[method(setTileWidth:)]
        pub fn setTileWidth(&self, tile_width: NSUInteger);

        #[method(tileHeight)]
        pub fn tileHeight(&self) -> NSUInteger;

        #[method(setTileHeight:)]
        pub fn setTileHeight(&self, tile_height: NSUInteger);

        #[method(defaultRasterSampleCount)]
        pub fn defaultRasterSampleCount(&self) -> NSUInteger;

        #[method(setDefaultRasterSampleCount:)]
        pub fn setDefaultRasterSampleCount(&self, default_raster_sample_count: NSUInteger);

        #[method(renderTargetWidth)]
        pub fn renderTargetWidth(&self) -> NSUInteger;

        #[method(setRenderTargetWidth:)]
        pub fn setRenderTargetWidth(&self, render_target_width: NSUInteger);

        #[method(renderTargetHeight)]
        pub fn renderTargetHeight(&self) -> NSUInteger;

        #[method(setRenderTargetHeight:)]
        pub fn setRenderTargetHeight(&self, render_target_height: NSUInteger);

        #[method(setSamplePositions:count:)]
        pub unsafe fn setSamplePositions_count(
            &self,
            positions: *mut MTLSamplePosition,
            count: NSUInteger,
        );

        #[method(getSamplePositions:count:)]
        pub unsafe fn getSamplePositions_count(
            &self,
            positions: *mut MTLSamplePosition,
            count: NSUInteger,
        ) -> NSUInteger;

        #[method_id(@__retain_semantics Other rasterizationRateMap)]
        pub fn rasterizationRateMap(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLRasterizationRateMap>>>;

        #[method(setRasterizationRateMap:)]
        pub fn setRasterizationRateMap(
            &self,
            rasterization_rate_map: Option<&(impl MTLRasterizationRateMap + Message)>,
        );

        #[cfg(feature = "Metal_MTLRenderPassSampleBufferAttachmentDescriptorArray")]
        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub fn sampleBufferAttachments(
            &self,
        ) -> Id<MTLRenderPassSampleBufferAttachmentDescriptorArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLRenderPassDescriptor")]
    unsafe impl MTLRenderPassDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

inline_fn!(
    pub unsafe fn MTLClearColorMake(
        red: c_double,
        green: c_double,
        blue: c_double,
        alpha: c_double,
    ) -> MTLClearColor {
        todo!()
    }
);
