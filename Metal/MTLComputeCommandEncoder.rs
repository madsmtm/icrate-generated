//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLDispatchThreadgroupsIndirectArguments {
    pub threadgroupsPerGrid: [u32; 3],
}

unsafe impl Encode for MTLDispatchThreadgroupsIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct("?", &[<[u32; 3]>::ENCODING]);
}

unsafe impl RefEncode for MTLDispatchThreadgroupsIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLStageInRegionIndirectArguments {
    pub stageInOrigin: [u32; 3],
    pub stageInSize: [u32; 3],
}

unsafe impl Encode for MTLStageInRegionIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct("?", &[<[u32; 3]>::ENCODING, <[u32; 3]>::ENCODING]);
}

unsafe impl RefEncode for MTLStageInRegionIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    #[cfg(feature = "MTLCommandEncoder")]
    pub unsafe trait MTLComputeCommandEncoder: MTLCommandEncoder + IsRetainable {
        #[cfg(feature = "MTLCommandBuffer")]
        #[method(dispatchType)]
        unsafe fn dispatch_type(&self) -> MTLDispatchType;

        #[cfg(feature = "MTLComputePipeline")]
        #[method(setComputePipelineState:)]
        fn set_compute_pipeline_state(&self, state: &ProtocolObject<dyn MTLComputePipelineState>);

        #[method(setBytes:length:atIndex:)]
        unsafe fn set_bytes_length_at_index(
            &self,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setBuffer:offset:atIndex:)]
        unsafe fn setBuffer_offset_atIndex_(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[method(setBufferOffset:atIndex:)]
        unsafe fn setBufferOffset_atIndex_(&self, offset: NSUInteger, index: NSUInteger);

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setBuffers:offsets:withRange:)]
        unsafe fn set_buffers_offsets_with_range(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<NSUInteger>,
            range: NSRange,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setBuffer:offset:attributeStride:atIndex:)]
        unsafe fn setBuffer_offset_attributeStride_atIndex_(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            stride: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setBuffers:offsets:attributeStrides:withRange:)]
        unsafe fn set_buffers_offsets_attribute_strides_with_range(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<NSUInteger>,
            strides: NonNull<NSUInteger>,
            range: NSRange,
        );

        #[method(setBufferOffset:attributeStride:atIndex:)]
        unsafe fn setBufferOffset_attributeStride_atIndex_(
            &self,
            offset: NSUInteger,
            stride: NSUInteger,
            index: NSUInteger,
        );

        #[method(setBytes:length:attributeStride:atIndex:)]
        unsafe fn set_bytes_length_attribute_stride_at_index(
            &self,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            stride: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLResource", feature = "MTLVisibleFunctionTable"))]
        #[method(setVisibleFunctionTable:atBufferIndex:)]
        unsafe fn set_visible_function_table_at_buffer_index(
            &self,
            visible_function_table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: NSUInteger,
        );

        #[cfg(all(feature = "MTLResource", feature = "MTLVisibleFunctionTable"))]
        #[method(setVisibleFunctionTables:withBufferRange:)]
        unsafe fn set_visible_function_tables_with_buffer_range(
            &self,
            visible_function_tables: NonNull<*const ProtocolObject<dyn MTLVisibleFunctionTable>>,
            range: NSRange,
        );

        #[cfg(all(feature = "MTLIntersectionFunctionTable", feature = "MTLResource"))]
        #[method(setIntersectionFunctionTable:atBufferIndex:)]
        unsafe fn set_intersection_function_table_at_buffer_index(
            &self,
            intersection_function_table: Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>,
            buffer_index: NSUInteger,
        );

        #[cfg(all(feature = "MTLIntersectionFunctionTable", feature = "MTLResource"))]
        #[method(setIntersectionFunctionTables:withBufferRange:)]
        unsafe fn set_intersection_function_tables_with_buffer_range(
            &self,
            intersection_function_tables: NonNull<
                *const ProtocolObject<dyn MTLIntersectionFunctionTable>,
            >,
            range: NSRange,
        );

        #[cfg(all(feature = "MTLAccelerationStructure", feature = "MTLResource"))]
        #[method(setAccelerationStructure:atBufferIndex:)]
        unsafe fn set_acceleration_structure_at_buffer_index(
            &self,
            acceleration_structure: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
            buffer_index: NSUInteger,
        );

        #[cfg(all(feature = "MTLResource", feature = "MTLTexture"))]
        #[method(setTexture:atIndex:)]
        unsafe fn set_texture_at_index(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLResource", feature = "MTLTexture"))]
        #[method(setTextures:withRange:)]
        unsafe fn set_textures_with_range(
            &self,
            textures: NonNull<*const ProtocolObject<dyn MTLTexture>>,
            range: NSRange,
        );

        #[cfg(feature = "MTLSampler")]
        #[method(setSamplerState:atIndex:)]
        unsafe fn set_sampler_state_at_index(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLSampler")]
        #[method(setSamplerStates:withRange:)]
        unsafe fn set_sampler_states_with_range(
            &self,
            samplers: NonNull<*const ProtocolObject<dyn MTLSamplerState>>,
            range: NSRange,
        );

        #[cfg(feature = "MTLSampler")]
        #[method(setSamplerState:lodMinClamp:lodMaxClamp:atIndex:)]
        unsafe fn set_sampler_state_lod_min_clamp_lod_max_clamp_at_index(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamp: c_float,
            lod_max_clamp: c_float,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLSampler")]
        #[method(setSamplerStates:lodMinClamps:lodMaxClamps:withRange:)]
        unsafe fn set_sampler_states_lod_min_clamps_lod_max_clamps_with_range(
            &self,
            samplers: NonNull<*const ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamps: NonNull<c_float>,
            lod_max_clamps: NonNull<c_float>,
            range: NSRange,
        );

        #[method(setThreadgroupMemoryLength:atIndex:)]
        unsafe fn set_threadgroup_memory_length_at_index(
            &self,
            length: NSUInteger,
            index: NSUInteger,
        );

        #[method(setImageblockWidth:height:)]
        unsafe fn set_imageblock_width_height(&self, width: NSUInteger, height: NSUInteger);

        #[cfg(feature = "MTLTypes")]
        #[method(setStageInRegion:)]
        unsafe fn set_stage_in_region(&self, region: MTLRegion);

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setStageInRegionWithIndirectBuffer:indirectBufferOffset:)]
        unsafe fn set_stage_in_region_with_indirect_buffer_indirect_buffer_offset(
            &self,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: NSUInteger,
        );

        #[cfg(feature = "MTLTypes")]
        #[method(dispatchThreadgroups:threadsPerThreadgroup:)]
        fn dispatch_threadgroups_threads_per_threadgroup(
            &self,
            threadgroups_per_grid: MTLSize,
            threads_per_threadgroup: MTLSize,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource", feature = "MTLTypes"))]
        #[method(dispatchThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerThreadgroup:)]
        unsafe fn dispatch_threadgroups_with_indirect_buffer_indirect_buffer_offset_threads_per_threadgroup(
            &self,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: NSUInteger,
            threads_per_threadgroup: MTLSize,
        );

        #[cfg(feature = "MTLTypes")]
        #[method(dispatchThreads:threadsPerThreadgroup:)]
        fn dispatch_threads_threads_per_threadgroup(
            &self,
            threads_per_grid: MTLSize,
            threads_per_threadgroup: MTLSize,
        );

        #[cfg(feature = "MTLFence")]
        #[method(updateFence:)]
        fn update_fence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[cfg(feature = "MTLFence")]
        #[method(waitForFence:)]
        fn wait_for_fence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[cfg(feature = "MTLResource")]
        #[method(useResource:usage:)]
        fn use_resource_usage(
            &self,
            resource: &ProtocolObject<dyn MTLResource>,
            usage: MTLResourceUsage,
        );

        #[cfg(feature = "MTLResource")]
        #[method(useResources:count:usage:)]
        unsafe fn use_resources_count_usage(
            &self,
            resources: NonNull<NonNull<ProtocolObject<dyn MTLResource>>>,
            count: NSUInteger,
            usage: MTLResourceUsage,
        );

        #[cfg(feature = "MTLHeap")]
        #[method(useHeap:)]
        fn use_heap(&self, heap: &ProtocolObject<dyn MTLHeap>);

        #[cfg(feature = "MTLHeap")]
        #[method(useHeaps:count:)]
        unsafe fn use_heaps_count(
            &self,
            heaps: NonNull<NonNull<ProtocolObject<dyn MTLHeap>>>,
            count: NSUInteger,
        );

        #[cfg(all(feature = "MTLIndirectCommandBuffer", feature = "MTLResource"))]
        #[method(executeCommandsInBuffer:withRange:)]
        unsafe fn execute_commands_in_buffer_with_range(
            &self,
            indirect_command_buffer: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            execution_range: NSRange,
        );

        #[cfg(all(
            feature = "MTLBuffer",
            feature = "MTLIndirectCommandBuffer",
            feature = "MTLResource"
        ))]
        #[method(executeCommandsInBuffer:indirectBuffer:indirectBufferOffset:)]
        unsafe fn execute_commands_in_buffer_indirect_buffer_indirect_buffer_offset(
            &self,
            indirect_commandbuffer: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            indirect_range_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: NSUInteger,
        );

        #[method(memoryBarrierWithScope:)]
        unsafe fn memory_barrier_with_scope(&self, scope: MTLBarrierScope);

        #[cfg(feature = "MTLResource")]
        #[method(memoryBarrierWithResources:count:)]
        unsafe fn memory_barrier_with_resources_count(
            &self,
            resources: NonNull<NonNull<ProtocolObject<dyn MTLResource>>>,
            count: NSUInteger,
        );

        #[cfg(feature = "MTLCounters")]
        #[method(sampleCountersInBuffer:atSampleIndex:withBarrier:)]
        unsafe fn sample_counters_in_buffer_at_sample_index_with_barrier(
            &self,
            sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
            sample_index: NSUInteger,
            barrier: bool,
        );
    }

    #[cfg(feature = "MTLCommandEncoder")]
    unsafe impl ProtocolType for dyn MTLComputeCommandEncoder {}
);
