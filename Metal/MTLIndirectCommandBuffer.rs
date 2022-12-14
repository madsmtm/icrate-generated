//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLIndirectCommandType {
        MTLIndirectCommandTypeDraw = 1 << 0,
        MTLIndirectCommandTypeDrawIndexed = 1 << 1,
        MTLIndirectCommandTypeDrawPatches = 1 << 2,
        MTLIndirectCommandTypeDrawIndexedPatches = 1 << 3,
        MTLIndirectCommandTypeConcurrentDispatch = 1 << 5,
        MTLIndirectCommandTypeConcurrentDispatchThreads = 1 << 6,
    }
);

extern_struct!(
    pub struct MTLIndirectCommandBufferExecutionRange {
        pub location: u32,
        pub length: u32,
    }
);

inline_fn!(
    pub unsafe fn MTLIndirectCommandBufferExecutionRangeMake(
        location: u32,
        length: u32,
    ) -> MTLIndirectCommandBufferExecutionRange {
        todo!()
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIndirectCommandBufferDescriptor;

    unsafe impl ClassType for MTLIndirectCommandBufferDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MTLIndirectCommandBufferDescriptor {
        #[method(commandTypes)]
        pub fn commandTypes(&self) -> MTLIndirectCommandType;

        #[method(setCommandTypes:)]
        pub fn setCommandTypes(&self, commandTypes: MTLIndirectCommandType);

        #[method(inheritPipelineState)]
        pub fn inheritPipelineState(&self) -> bool;

        #[method(setInheritPipelineState:)]
        pub fn setInheritPipelineState(&self, inheritPipelineState: bool);

        #[method(inheritBuffers)]
        pub fn inheritBuffers(&self) -> bool;

        #[method(setInheritBuffers:)]
        pub fn setInheritBuffers(&self, inheritBuffers: bool);

        #[method(maxVertexBufferBindCount)]
        pub fn maxVertexBufferBindCount(&self) -> NSUInteger;

        #[method(setMaxVertexBufferBindCount:)]
        pub fn setMaxVertexBufferBindCount(&self, maxVertexBufferBindCount: NSUInteger);

        #[method(maxFragmentBufferBindCount)]
        pub fn maxFragmentBufferBindCount(&self) -> NSUInteger;

        #[method(setMaxFragmentBufferBindCount:)]
        pub fn setMaxFragmentBufferBindCount(&self, maxFragmentBufferBindCount: NSUInteger);

        #[method(maxKernelBufferBindCount)]
        pub fn maxKernelBufferBindCount(&self) -> NSUInteger;

        #[method(setMaxKernelBufferBindCount:)]
        pub fn setMaxKernelBufferBindCount(&self, maxKernelBufferBindCount: NSUInteger);
    }
);

extern_protocol!(
    pub struct MTLIndirectCommandBuffer;

    unsafe impl ProtocolType for MTLIndirectCommandBuffer {
        #[method(size)]
        pub fn size(&self) -> NSUInteger;

        #[method(resetWithRange:)]
        pub unsafe fn resetWithRange(&self, range: NSRange);

        #[method_id(@__retain_semantics Other indirectRenderCommandAtIndex:)]
        pub unsafe fn indirectRenderCommandAtIndex(
            &self,
            commandIndex: NSUInteger,
        ) -> Id<MTLIndirectRenderCommand, Shared>;

        #[method_id(@__retain_semantics Other indirectComputeCommandAtIndex:)]
        pub unsafe fn indirectComputeCommandAtIndex(
            &self,
            commandIndex: NSUInteger,
        ) -> Id<MTLIndirectComputeCommand, Shared>;
    }
);
