//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#[path = "MTLAccelerationStructure.rs"]
mod __MTLAccelerationStructure;
#[path = "MTLAccelerationStructureCommandEncoder.rs"]
mod __MTLAccelerationStructureCommandEncoder;
#[path = "MTLAccelerationStructureTypes.rs"]
mod __MTLAccelerationStructureTypes;
#[path = "MTLArgument.rs"]
mod __MTLArgument;
#[path = "MTLArgumentEncoder.rs"]
mod __MTLArgumentEncoder;
#[path = "MTLBinaryArchive.rs"]
mod __MTLBinaryArchive;
#[path = "MTLBlitCommandEncoder.rs"]
mod __MTLBlitCommandEncoder;
#[path = "MTLBlitPass.rs"]
mod __MTLBlitPass;
#[path = "MTLBuffer.rs"]
mod __MTLBuffer;
#[path = "MTLCaptureManager.rs"]
mod __MTLCaptureManager;
#[path = "MTLCaptureScope.rs"]
mod __MTLCaptureScope;
#[path = "MTLCommandBuffer.rs"]
mod __MTLCommandBuffer;
#[path = "MTLCommandEncoder.rs"]
mod __MTLCommandEncoder;
#[path = "MTLCommandQueue.rs"]
mod __MTLCommandQueue;
#[path = "MTLComputeCommandEncoder.rs"]
mod __MTLComputeCommandEncoder;
#[path = "MTLComputePass.rs"]
mod __MTLComputePass;
#[path = "MTLComputePipeline.rs"]
mod __MTLComputePipeline;
#[path = "MTLCounters.rs"]
mod __MTLCounters;
#[path = "MTLDefines.rs"]
mod __MTLDefines;
#[path = "MTLDepthStencil.rs"]
mod __MTLDepthStencil;
#[path = "MTLDevice.rs"]
mod __MTLDevice;
#[path = "MTLDrawable.rs"]
mod __MTLDrawable;
#[path = "MTLDynamicLibrary.rs"]
mod __MTLDynamicLibrary;
#[path = "MTLEvent.rs"]
mod __MTLEvent;
#[path = "MTLFence.rs"]
mod __MTLFence;
#[path = "MTLFunctionConstantValues.rs"]
mod __MTLFunctionConstantValues;
#[path = "MTLFunctionDescriptor.rs"]
mod __MTLFunctionDescriptor;
#[path = "MTLFunctionHandle.rs"]
mod __MTLFunctionHandle;
#[path = "MTLFunctionLog.rs"]
mod __MTLFunctionLog;
#[path = "MTLFunctionStitching.rs"]
mod __MTLFunctionStitching;
#[path = "MTLHeap.rs"]
mod __MTLHeap;
#[path = "MTLIndirectCommandBuffer.rs"]
mod __MTLIndirectCommandBuffer;
#[path = "MTLIndirectCommandEncoder.rs"]
mod __MTLIndirectCommandEncoder;
#[path = "MTLIntersectionFunctionTable.rs"]
mod __MTLIntersectionFunctionTable;
#[path = "MTLLibrary.rs"]
mod __MTLLibrary;
#[path = "MTLLinkedFunctions.rs"]
mod __MTLLinkedFunctions;
#[path = "MTLParallelRenderCommandEncoder.rs"]
mod __MTLParallelRenderCommandEncoder;
#[path = "MTLPipeline.rs"]
mod __MTLPipeline;
#[path = "MTLPixelFormat.rs"]
mod __MTLPixelFormat;
#[path = "MTLRasterizationRate.rs"]
mod __MTLRasterizationRate;
#[path = "MTLRenderCommandEncoder.rs"]
mod __MTLRenderCommandEncoder;
#[path = "MTLRenderPass.rs"]
mod __MTLRenderPass;
#[path = "MTLRenderPipeline.rs"]
mod __MTLRenderPipeline;
#[path = "MTLResource.rs"]
mod __MTLResource;
#[path = "MTLResourceStateCommandEncoder.rs"]
mod __MTLResourceStateCommandEncoder;
#[path = "MTLResourceStatePass.rs"]
mod __MTLResourceStatePass;
#[path = "MTLSampler.rs"]
mod __MTLSampler;
#[path = "MTLStageInputOutputDescriptor.rs"]
mod __MTLStageInputOutputDescriptor;
#[path = "MTLTexture.rs"]
mod __MTLTexture;
#[path = "MTLTypes.rs"]
mod __MTLTypes;
#[path = "MTLVertexDescriptor.rs"]
mod __MTLVertexDescriptor;
#[path = "MTLVisibleFunctionTable.rs"]
mod __MTLVisibleFunctionTable;

pub use self::__MTLAccelerationStructure::{
    MTLAccelerationStructure, MTLAccelerationStructureBoundingBoxGeometryDescriptor,
    MTLAccelerationStructureDescriptor, MTLAccelerationStructureGeometryDescriptor,
    MTLAccelerationStructureInstanceDescriptor, MTLAccelerationStructureInstanceDescriptorType,
    MTLAccelerationStructureInstanceDescriptorTypeDefault,
    MTLAccelerationStructureInstanceDescriptorTypeMotion,
    MTLAccelerationStructureInstanceDescriptorTypeUserID,
    MTLAccelerationStructureInstanceOptionDisableTriangleCulling,
    MTLAccelerationStructureInstanceOptionNonOpaque, MTLAccelerationStructureInstanceOptionNone,
    MTLAccelerationStructureInstanceOptionOpaque,
    MTLAccelerationStructureInstanceOptionTriangleFrontFacingWindingCounterClockwise,
    MTLAccelerationStructureInstanceOptions,
    MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor,
    MTLAccelerationStructureMotionInstanceDescriptor,
    MTLAccelerationStructureMotionTriangleGeometryDescriptor,
    MTLAccelerationStructureTriangleGeometryDescriptor, MTLAccelerationStructureUsage,
    MTLAccelerationStructureUsageExtendedLimits, MTLAccelerationStructureUsageNone,
    MTLAccelerationStructureUsagePreferFastBuild, MTLAccelerationStructureUsageRefit,
    MTLAccelerationStructureUserIDInstanceDescriptor, MTLInstanceAccelerationStructureDescriptor,
    MTLMotionBorderMode, MTLMotionBorderModeClamp, MTLMotionBorderModeVanish,
    MTLMotionKeyframeData, MTLPrimitiveAccelerationStructureDescriptor,
};
pub use self::__MTLAccelerationStructureCommandEncoder::MTLAccelerationStructureCommandEncoder;
pub use self::__MTLAccelerationStructureTypes::{MTLAxisAlignedBoundingBox, MTLPackedFloat4x3};
pub use self::__MTLArgument::{
    MTLArgument, MTLArgumentAccess, MTLArgumentAccessReadOnly, MTLArgumentAccessReadWrite,
    MTLArgumentAccessWriteOnly, MTLArgumentType, MTLArgumentTypeBuffer, MTLArgumentTypeImageblock,
    MTLArgumentTypeImageblockData, MTLArgumentTypeInstanceAccelerationStructure,
    MTLArgumentTypeIntersectionFunctionTable, MTLArgumentTypePrimitiveAccelerationStructure,
    MTLArgumentTypeSampler, MTLArgumentTypeTexture, MTLArgumentTypeThreadgroupMemory,
    MTLArgumentTypeVisibleFunctionTable, MTLArrayType, MTLDataType, MTLDataTypeArray,
    MTLDataTypeBool, MTLDataTypeBool2, MTLDataTypeBool3, MTLDataTypeBool4, MTLDataTypeChar,
    MTLDataTypeChar2, MTLDataTypeChar3, MTLDataTypeChar4, MTLDataTypeComputePipeline,
    MTLDataTypeFloat, MTLDataTypeFloat2, MTLDataTypeFloat2x2, MTLDataTypeFloat2x3,
    MTLDataTypeFloat2x4, MTLDataTypeFloat3, MTLDataTypeFloat3x2, MTLDataTypeFloat3x3,
    MTLDataTypeFloat3x4, MTLDataTypeFloat4, MTLDataTypeFloat4x2, MTLDataTypeFloat4x3,
    MTLDataTypeFloat4x4, MTLDataTypeHalf, MTLDataTypeHalf2, MTLDataTypeHalf2x2, MTLDataTypeHalf2x3,
    MTLDataTypeHalf2x4, MTLDataTypeHalf3, MTLDataTypeHalf3x2, MTLDataTypeHalf3x3,
    MTLDataTypeHalf3x4, MTLDataTypeHalf4, MTLDataTypeHalf4x2, MTLDataTypeHalf4x3,
    MTLDataTypeHalf4x4, MTLDataTypeIndirectCommandBuffer, MTLDataTypeInstanceAccelerationStructure,
    MTLDataTypeInt, MTLDataTypeInt2, MTLDataTypeInt3, MTLDataTypeInt4,
    MTLDataTypeIntersectionFunctionTable, MTLDataTypeLong, MTLDataTypeLong2, MTLDataTypeLong3,
    MTLDataTypeLong4, MTLDataTypeNone, MTLDataTypePointer,
    MTLDataTypePrimitiveAccelerationStructure, MTLDataTypeR16Snorm, MTLDataTypeR16Unorm,
    MTLDataTypeR8Snorm, MTLDataTypeR8Unorm, MTLDataTypeRG11B10Float, MTLDataTypeRG16Snorm,
    MTLDataTypeRG16Unorm, MTLDataTypeRG8Snorm, MTLDataTypeRG8Unorm, MTLDataTypeRGB10A2Unorm,
    MTLDataTypeRGB9E5Float, MTLDataTypeRGBA16Snorm, MTLDataTypeRGBA16Unorm, MTLDataTypeRGBA8Snorm,
    MTLDataTypeRGBA8Unorm, MTLDataTypeRGBA8Unorm_sRGB, MTLDataTypeRenderPipeline,
    MTLDataTypeSampler, MTLDataTypeShort, MTLDataTypeShort2, MTLDataTypeShort3, MTLDataTypeShort4,
    MTLDataTypeStruct, MTLDataTypeTexture, MTLDataTypeUChar, MTLDataTypeUChar2, MTLDataTypeUChar3,
    MTLDataTypeUChar4, MTLDataTypeUInt, MTLDataTypeUInt2, MTLDataTypeUInt3, MTLDataTypeUInt4,
    MTLDataTypeULong, MTLDataTypeULong2, MTLDataTypeULong3, MTLDataTypeULong4, MTLDataTypeUShort,
    MTLDataTypeUShort2, MTLDataTypeUShort3, MTLDataTypeUShort4, MTLDataTypeVisibleFunctionTable,
    MTLPointerType, MTLStructMember, MTLStructType, MTLTextureReferenceType, MTLType,
};
pub use self::__MTLArgumentEncoder::MTLArgumentEncoder;
pub use self::__MTLBinaryArchive::{
    MTLBinaryArchive, MTLBinaryArchiveDescriptor, MTLBinaryArchiveDomain, MTLBinaryArchiveError,
    MTLBinaryArchiveErrorCompilationFailure, MTLBinaryArchiveErrorInvalidFile,
    MTLBinaryArchiveErrorNone, MTLBinaryArchiveErrorUnexpectedElement,
};
pub use self::__MTLBlitCommandEncoder::{
    MTLBlitCommandEncoder, MTLBlitOption, MTLBlitOptionDepthFromDepthStencil, MTLBlitOptionNone,
    MTLBlitOptionRowLinearPVRTC, MTLBlitOptionStencilFromDepthStencil,
};
pub use self::__MTLBlitPass::{
    MTLBlitPassDescriptor, MTLBlitPassSampleBufferAttachmentDescriptor,
    MTLBlitPassSampleBufferAttachmentDescriptorArray,
};
pub use self::__MTLBuffer::MTLBuffer;
pub use self::__MTLCaptureManager::{
    MTLCaptureDescriptor, MTLCaptureDestination, MTLCaptureDestinationDeveloperTools,
    MTLCaptureDestinationGPUTraceDocument, MTLCaptureError, MTLCaptureErrorAlreadyCapturing,
    MTLCaptureErrorDomain, MTLCaptureErrorInvalidDescriptor, MTLCaptureErrorNotSupported,
    MTLCaptureManager,
};
pub use self::__MTLCaptureScope::MTLCaptureScope;
pub use self::__MTLCommandBuffer::{
    MTLCommandBuffer, MTLCommandBufferDescriptor, MTLCommandBufferEncoderInfo,
    MTLCommandBufferEncoderInfoErrorKey, MTLCommandBufferError, MTLCommandBufferErrorAccessRevoked,
    MTLCommandBufferErrorBlacklisted, MTLCommandBufferErrorDeviceRemoved,
    MTLCommandBufferErrorDomain, MTLCommandBufferErrorInternal,
    MTLCommandBufferErrorInvalidResource, MTLCommandBufferErrorMemoryless,
    MTLCommandBufferErrorNone, MTLCommandBufferErrorNotPermitted, MTLCommandBufferErrorOption,
    MTLCommandBufferErrorOptionEncoderExecutionStatus, MTLCommandBufferErrorOptionNone,
    MTLCommandBufferErrorOutOfMemory, MTLCommandBufferErrorPageFault,
    MTLCommandBufferErrorStackOverflow, MTLCommandBufferErrorTimeout, MTLCommandBufferHandler,
    MTLCommandBufferStatus, MTLCommandBufferStatusCommitted, MTLCommandBufferStatusCompleted,
    MTLCommandBufferStatusEnqueued, MTLCommandBufferStatusError, MTLCommandBufferStatusNotEnqueued,
    MTLCommandBufferStatusScheduled, MTLCommandEncoderErrorState,
    MTLCommandEncoderErrorStateAffected, MTLCommandEncoderErrorStateCompleted,
    MTLCommandEncoderErrorStateFaulted, MTLCommandEncoderErrorStatePending,
    MTLCommandEncoderErrorStateUnknown, MTLDispatchType, MTLDispatchTypeConcurrent,
    MTLDispatchTypeSerial,
};
pub use self::__MTLCommandEncoder::{
    MTLBarrierScope, MTLBarrierScopeBuffers, MTLBarrierScopeRenderTargets, MTLBarrierScopeTextures,
    MTLCommandEncoder, MTLResourceUsage, MTLResourceUsageRead, MTLResourceUsageSample,
    MTLResourceUsageWrite,
};
pub use self::__MTLCommandQueue::MTLCommandQueue;
pub use self::__MTLComputeCommandEncoder::{
    MTLComputeCommandEncoder, MTLDispatchThreadgroupsIndirectArguments,
    MTLStageInRegionIndirectArguments,
};
pub use self::__MTLComputePass::{
    MTLComputePassDescriptor, MTLComputePassSampleBufferAttachmentDescriptor,
    MTLComputePassSampleBufferAttachmentDescriptorArray,
};
pub use self::__MTLComputePipeline::{
    MTLComputePipelineDescriptor, MTLComputePipelineReflection, MTLComputePipelineState,
};
pub use self::__MTLCounters::{
    MTLCommonCounter, MTLCommonCounterClipperInvocations, MTLCommonCounterClipperPrimitivesOut,
    MTLCommonCounterComputeKernelInvocations, MTLCommonCounterFragmentCycles,
    MTLCommonCounterFragmentInvocations, MTLCommonCounterFragmentsPassed,
    MTLCommonCounterPostTessellationVertexCycles,
    MTLCommonCounterPostTessellationVertexInvocations, MTLCommonCounterRenderTargetWriteCycles,
    MTLCommonCounterSet, MTLCommonCounterSetStageUtilization, MTLCommonCounterSetStatistic,
    MTLCommonCounterSetTimestamp, MTLCommonCounterTessellationCycles,
    MTLCommonCounterTessellationInputPatches, MTLCommonCounterTimestamp,
    MTLCommonCounterTotalCycles, MTLCommonCounterVertexCycles, MTLCommonCounterVertexInvocations,
    MTLCounter, MTLCounterErrorDomain, MTLCounterResultStageUtilization, MTLCounterResultStatistic,
    MTLCounterResultTimestamp, MTLCounterSampleBuffer, MTLCounterSampleBufferDescriptor,
    MTLCounterSampleBufferError, MTLCounterSampleBufferErrorInternal,
    MTLCounterSampleBufferErrorInvalid, MTLCounterSampleBufferErrorOutOfMemory, MTLCounterSet,
};
pub use self::__MTLDepthStencil::{
    MTLCompareFunction, MTLCompareFunctionAlways, MTLCompareFunctionEqual,
    MTLCompareFunctionGreater, MTLCompareFunctionGreaterEqual, MTLCompareFunctionLess,
    MTLCompareFunctionLessEqual, MTLCompareFunctionNever, MTLCompareFunctionNotEqual,
    MTLDepthStencilDescriptor, MTLDepthStencilState, MTLStencilDescriptor, MTLStencilOperation,
    MTLStencilOperationDecrementClamp, MTLStencilOperationDecrementWrap,
    MTLStencilOperationIncrementClamp, MTLStencilOperationIncrementWrap, MTLStencilOperationInvert,
    MTLStencilOperationKeep, MTLStencilOperationReplace, MTLStencilOperationZero,
};
pub use self::__MTLDevice::{
    MTLAccelerationStructureSizes, MTLArgumentBuffersTier, MTLArgumentBuffersTier1,
    MTLArgumentBuffersTier2, MTLArgumentDescriptor, MTLCopyAllDevices, MTLCounterSamplingPoint,
    MTLCounterSamplingPointAtBlitBoundary, MTLCounterSamplingPointAtDispatchBoundary,
    MTLCounterSamplingPointAtDrawBoundary, MTLCounterSamplingPointAtStageBoundary,
    MTLCounterSamplingPointAtTileDispatchBoundary, MTLCreateSystemDefaultDevice, MTLDevice,
    MTLDeviceLocation, MTLDeviceLocationBuiltIn, MTLDeviceLocationExternal, MTLDeviceLocationSlot,
    MTLDeviceLocationUnspecified, MTLDeviceNotificationHandler, MTLDeviceNotificationName,
    MTLDeviceRemovalRequestedNotification, MTLDeviceWasAddedNotification,
    MTLDeviceWasRemovedNotification, MTLFeatureSet, MTLFeatureSet_OSX_GPUFamily1_v1,
    MTLFeatureSet_OSX_GPUFamily1_v2, MTLFeatureSet_OSX_ReadWriteTextureTier2,
    MTLFeatureSet_TVOS_GPUFamily1_v1, MTLFeatureSet_iOS_GPUFamily1_v1,
    MTLFeatureSet_iOS_GPUFamily1_v2, MTLFeatureSet_iOS_GPUFamily1_v3,
    MTLFeatureSet_iOS_GPUFamily1_v4, MTLFeatureSet_iOS_GPUFamily1_v5,
    MTLFeatureSet_iOS_GPUFamily2_v1, MTLFeatureSet_iOS_GPUFamily2_v2,
    MTLFeatureSet_iOS_GPUFamily2_v3, MTLFeatureSet_iOS_GPUFamily2_v4,
    MTLFeatureSet_iOS_GPUFamily2_v5, MTLFeatureSet_iOS_GPUFamily3_v1,
    MTLFeatureSet_iOS_GPUFamily3_v2, MTLFeatureSet_iOS_GPUFamily3_v3,
    MTLFeatureSet_iOS_GPUFamily3_v4, MTLFeatureSet_iOS_GPUFamily4_v1,
    MTLFeatureSet_iOS_GPUFamily4_v2, MTLFeatureSet_iOS_GPUFamily5_v1,
    MTLFeatureSet_macOS_GPUFamily1_v1, MTLFeatureSet_macOS_GPUFamily1_v2,
    MTLFeatureSet_macOS_GPUFamily1_v3, MTLFeatureSet_macOS_GPUFamily1_v4,
    MTLFeatureSet_macOS_GPUFamily2_v1, MTLFeatureSet_macOS_ReadWriteTextureTier2,
    MTLFeatureSet_tvOS_GPUFamily1_v1, MTLFeatureSet_tvOS_GPUFamily1_v2,
    MTLFeatureSet_tvOS_GPUFamily1_v3, MTLFeatureSet_tvOS_GPUFamily1_v4,
    MTLFeatureSet_tvOS_GPUFamily2_v1, MTLFeatureSet_tvOS_GPUFamily2_v2, MTLGPUFamily,
    MTLGPUFamilyApple1, MTLGPUFamilyApple2, MTLGPUFamilyApple3, MTLGPUFamilyApple4,
    MTLGPUFamilyApple5, MTLGPUFamilyApple6, MTLGPUFamilyApple7, MTLGPUFamilyCommon1,
    MTLGPUFamilyCommon2, MTLGPUFamilyCommon3, MTLGPUFamilyMac1, MTLGPUFamilyMac2,
    MTLGPUFamilyMacCatalyst1, MTLGPUFamilyMacCatalyst2,
    MTLNewComputePipelineStateCompletionHandler,
    MTLNewComputePipelineStateWithReflectionCompletionHandler, MTLNewLibraryCompletionHandler,
    MTLNewRenderPipelineStateCompletionHandler,
    MTLNewRenderPipelineStateWithReflectionCompletionHandler, MTLPipelineOption,
    MTLPipelineOptionArgumentInfo, MTLPipelineOptionBufferTypeInfo,
    MTLPipelineOptionFailOnBinaryArchiveMiss, MTLPipelineOptionNone, MTLReadWriteTextureTier,
    MTLReadWriteTextureTier1, MTLReadWriteTextureTier2, MTLReadWriteTextureTierNone,
    MTLRemoveDeviceObserver, MTLSizeAndAlign, MTLSparseTextureRegionAlignmentMode,
    MTLSparseTextureRegionAlignmentModeInward, MTLSparseTextureRegionAlignmentModeOutward,
    MTLTimestamp,
};
pub use self::__MTLDrawable::{MTLDrawable, MTLDrawablePresentedHandler};
pub use self::__MTLDynamicLibrary::{
    MTLDynamicLibrary, MTLDynamicLibraryDomain, MTLDynamicLibraryError,
    MTLDynamicLibraryErrorCompilationFailure, MTLDynamicLibraryErrorDependencyLoadFailure,
    MTLDynamicLibraryErrorInvalidFile, MTLDynamicLibraryErrorNone,
    MTLDynamicLibraryErrorUnresolvedInstallName, MTLDynamicLibraryErrorUnsupported,
};
pub use self::__MTLEvent::{
    MTLEvent, MTLSharedEvent, MTLSharedEventHandle, MTLSharedEventListener,
    MTLSharedEventNotificationBlock,
};
pub use self::__MTLFence::MTLFence;
pub use self::__MTLFunctionConstantValues::MTLFunctionConstantValues;
pub use self::__MTLFunctionDescriptor::{
    MTLFunctionDescriptor, MTLFunctionOptionCompileToBinary, MTLFunctionOptionNone,
    MTLFunctionOptions, MTLIntersectionFunctionDescriptor,
};
pub use self::__MTLFunctionHandle::MTLFunctionHandle;
pub use self::__MTLFunctionLog::{
    MTLFunctionLog, MTLFunctionLogDebugLocation, MTLFunctionLogType, MTLFunctionLogTypeValidation,
    MTLLogContainer,
};
pub use self::__MTLFunctionStitching::{
    MTLFunctionStitchingAttribute, MTLFunctionStitchingAttributeAlwaysInline,
    MTLFunctionStitchingFunctionNode, MTLFunctionStitchingGraph, MTLFunctionStitchingInputNode,
    MTLFunctionStitchingNode, MTLStitchedLibraryDescriptor,
};
pub use self::__MTLHeap::{
    MTLHeap, MTLHeapDescriptor, MTLHeapType, MTLHeapTypeAutomatic, MTLHeapTypePlacement,
    MTLHeapTypeSparse,
};
pub use self::__MTLIndirectCommandBuffer::{
    MTLIndirectCommandBuffer, MTLIndirectCommandBufferDescriptor,
    MTLIndirectCommandBufferExecutionRange, MTLIndirectCommandType,
    MTLIndirectCommandTypeConcurrentDispatch, MTLIndirectCommandTypeConcurrentDispatchThreads,
    MTLIndirectCommandTypeDraw, MTLIndirectCommandTypeDrawIndexed,
    MTLIndirectCommandTypeDrawIndexedPatches, MTLIndirectCommandTypeDrawPatches,
};
pub use self::__MTLIndirectCommandEncoder::{MTLIndirectComputeCommand, MTLIndirectRenderCommand};
pub use self::__MTLIntersectionFunctionTable::{
    MTLIntersectionFunctionSignature, MTLIntersectionFunctionSignatureExtendedLimits,
    MTLIntersectionFunctionSignatureInstanceMotion, MTLIntersectionFunctionSignatureInstancing,
    MTLIntersectionFunctionSignatureNone, MTLIntersectionFunctionSignaturePrimitiveMotion,
    MTLIntersectionFunctionSignatureTriangleData, MTLIntersectionFunctionSignatureWorldSpaceData,
    MTLIntersectionFunctionTable, MTLIntersectionFunctionTableDescriptor,
};
pub use self::__MTLLibrary::{
    MTLAttribute, MTLCompileOptions, MTLFunction, MTLFunctionConstant, MTLFunctionType,
    MTLFunctionTypeFragment, MTLFunctionTypeIntersection, MTLFunctionTypeKernel,
    MTLFunctionTypeVertex, MTLFunctionTypeVisible, MTLLanguageVersion, MTLLanguageVersion1_0,
    MTLLanguageVersion1_1, MTLLanguageVersion1_2, MTLLanguageVersion2_0, MTLLanguageVersion2_1,
    MTLLanguageVersion2_2, MTLLanguageVersion2_3, MTLLanguageVersion2_4, MTLLibrary,
    MTLLibraryError, MTLLibraryErrorCompileFailure, MTLLibraryErrorCompileWarning,
    MTLLibraryErrorDomain, MTLLibraryErrorFileNotFound, MTLLibraryErrorFunctionNotFound,
    MTLLibraryErrorInternal, MTLLibraryErrorUnsupported, MTLLibraryType, MTLLibraryTypeDynamic,
    MTLLibraryTypeExecutable, MTLPatchType, MTLPatchTypeNone, MTLPatchTypeQuad,
    MTLPatchTypeTriangle, MTLVertexAttribute,
};
pub use self::__MTLLinkedFunctions::MTLLinkedFunctions;
pub use self::__MTLParallelRenderCommandEncoder::MTLParallelRenderCommandEncoder;
pub use self::__MTLPipeline::{
    MTLMutability, MTLMutabilityDefault, MTLMutabilityImmutable, MTLMutabilityMutable,
    MTLPipelineBufferDescriptor, MTLPipelineBufferDescriptorArray,
};
pub use self::__MTLPixelFormat::{
    MTLPixelFormat, MTLPixelFormatA1BGR5Unorm, MTLPixelFormatA8Unorm, MTLPixelFormatABGR4Unorm,
    MTLPixelFormatASTC_10x10_HDR, MTLPixelFormatASTC_10x10_LDR, MTLPixelFormatASTC_10x10_sRGB,
    MTLPixelFormatASTC_10x5_HDR, MTLPixelFormatASTC_10x5_LDR, MTLPixelFormatASTC_10x5_sRGB,
    MTLPixelFormatASTC_10x6_HDR, MTLPixelFormatASTC_10x6_LDR, MTLPixelFormatASTC_10x6_sRGB,
    MTLPixelFormatASTC_10x8_HDR, MTLPixelFormatASTC_10x8_LDR, MTLPixelFormatASTC_10x8_sRGB,
    MTLPixelFormatASTC_12x10_HDR, MTLPixelFormatASTC_12x10_LDR, MTLPixelFormatASTC_12x10_sRGB,
    MTLPixelFormatASTC_12x12_HDR, MTLPixelFormatASTC_12x12_LDR, MTLPixelFormatASTC_12x12_sRGB,
    MTLPixelFormatASTC_4x4_HDR, MTLPixelFormatASTC_4x4_LDR, MTLPixelFormatASTC_4x4_sRGB,
    MTLPixelFormatASTC_5x4_HDR, MTLPixelFormatASTC_5x4_LDR, MTLPixelFormatASTC_5x4_sRGB,
    MTLPixelFormatASTC_5x5_HDR, MTLPixelFormatASTC_5x5_LDR, MTLPixelFormatASTC_5x5_sRGB,
    MTLPixelFormatASTC_6x5_HDR, MTLPixelFormatASTC_6x5_LDR, MTLPixelFormatASTC_6x5_sRGB,
    MTLPixelFormatASTC_6x6_HDR, MTLPixelFormatASTC_6x6_LDR, MTLPixelFormatASTC_6x6_sRGB,
    MTLPixelFormatASTC_8x5_HDR, MTLPixelFormatASTC_8x5_LDR, MTLPixelFormatASTC_8x5_sRGB,
    MTLPixelFormatASTC_8x6_HDR, MTLPixelFormatASTC_8x6_LDR, MTLPixelFormatASTC_8x6_sRGB,
    MTLPixelFormatASTC_8x8_HDR, MTLPixelFormatASTC_8x8_LDR, MTLPixelFormatASTC_8x8_sRGB,
    MTLPixelFormatB5G6R5Unorm, MTLPixelFormatBC1_RGBA, MTLPixelFormatBC1_RGBA_sRGB,
    MTLPixelFormatBC2_RGBA, MTLPixelFormatBC2_RGBA_sRGB, MTLPixelFormatBC3_RGBA,
    MTLPixelFormatBC3_RGBA_sRGB, MTLPixelFormatBC4_RSnorm, MTLPixelFormatBC4_RUnorm,
    MTLPixelFormatBC5_RGSnorm, MTLPixelFormatBC5_RGUnorm, MTLPixelFormatBC6H_RGBFloat,
    MTLPixelFormatBC6H_RGBUfloat, MTLPixelFormatBC7_RGBAUnorm, MTLPixelFormatBC7_RGBAUnorm_sRGB,
    MTLPixelFormatBGR10A2Unorm, MTLPixelFormatBGR10_XR, MTLPixelFormatBGR10_XR_sRGB,
    MTLPixelFormatBGR5A1Unorm, MTLPixelFormatBGRA10_XR, MTLPixelFormatBGRA10_XR_sRGB,
    MTLPixelFormatBGRA8Unorm, MTLPixelFormatBGRA8Unorm_sRGB, MTLPixelFormatBGRG422,
    MTLPixelFormatDepth16Unorm, MTLPixelFormatDepth24Unorm_Stencil8, MTLPixelFormatDepth32Float,
    MTLPixelFormatDepth32Float_Stencil8, MTLPixelFormatEAC_R11Snorm, MTLPixelFormatEAC_R11Unorm,
    MTLPixelFormatEAC_RG11Snorm, MTLPixelFormatEAC_RG11Unorm, MTLPixelFormatEAC_RGBA8,
    MTLPixelFormatEAC_RGBA8_sRGB, MTLPixelFormatETC2_RGB8, MTLPixelFormatETC2_RGB8A1,
    MTLPixelFormatETC2_RGB8A1_sRGB, MTLPixelFormatETC2_RGB8_sRGB, MTLPixelFormatGBGR422,
    MTLPixelFormatInvalid, MTLPixelFormatPVRTC_RGBA_2BPP, MTLPixelFormatPVRTC_RGBA_2BPP_sRGB,
    MTLPixelFormatPVRTC_RGBA_4BPP, MTLPixelFormatPVRTC_RGBA_4BPP_sRGB,
    MTLPixelFormatPVRTC_RGB_2BPP, MTLPixelFormatPVRTC_RGB_2BPP_sRGB, MTLPixelFormatPVRTC_RGB_4BPP,
    MTLPixelFormatPVRTC_RGB_4BPP_sRGB, MTLPixelFormatR16Float, MTLPixelFormatR16Sint,
    MTLPixelFormatR16Snorm, MTLPixelFormatR16Uint, MTLPixelFormatR16Unorm, MTLPixelFormatR32Float,
    MTLPixelFormatR32Sint, MTLPixelFormatR32Uint, MTLPixelFormatR8Sint, MTLPixelFormatR8Snorm,
    MTLPixelFormatR8Uint, MTLPixelFormatR8Unorm, MTLPixelFormatR8Unorm_sRGB,
    MTLPixelFormatRG11B10Float, MTLPixelFormatRG16Float, MTLPixelFormatRG16Sint,
    MTLPixelFormatRG16Snorm, MTLPixelFormatRG16Uint, MTLPixelFormatRG16Unorm,
    MTLPixelFormatRG32Float, MTLPixelFormatRG32Sint, MTLPixelFormatRG32Uint, MTLPixelFormatRG8Sint,
    MTLPixelFormatRG8Snorm, MTLPixelFormatRG8Uint, MTLPixelFormatRG8Unorm,
    MTLPixelFormatRG8Unorm_sRGB, MTLPixelFormatRGB10A2Uint, MTLPixelFormatRGB10A2Unorm,
    MTLPixelFormatRGB9E5Float, MTLPixelFormatRGBA16Float, MTLPixelFormatRGBA16Sint,
    MTLPixelFormatRGBA16Snorm, MTLPixelFormatRGBA16Uint, MTLPixelFormatRGBA16Unorm,
    MTLPixelFormatRGBA32Float, MTLPixelFormatRGBA32Sint, MTLPixelFormatRGBA32Uint,
    MTLPixelFormatRGBA8Sint, MTLPixelFormatRGBA8Snorm, MTLPixelFormatRGBA8Uint,
    MTLPixelFormatRGBA8Unorm, MTLPixelFormatRGBA8Unorm_sRGB, MTLPixelFormatStencil8,
    MTLPixelFormatX24_Stencil8, MTLPixelFormatX32_Stencil8,
};
pub use self::__MTLRasterizationRate::{
    MTLRasterizationRateLayerArray, MTLRasterizationRateLayerDescriptor, MTLRasterizationRateMap,
    MTLRasterizationRateMapDescriptor, MTLRasterizationRateSampleArray,
};
pub use self::__MTLRenderCommandEncoder::{
    MTLCullMode, MTLCullModeBack, MTLCullModeFront, MTLCullModeNone, MTLDepthClipMode,
    MTLDepthClipModeClamp, MTLDepthClipModeClip, MTLDrawIndexedPrimitivesIndirectArguments,
    MTLDrawPatchIndirectArguments, MTLDrawPrimitivesIndirectArguments, MTLPrimitiveType,
    MTLPrimitiveTypeLine, MTLPrimitiveTypeLineStrip, MTLPrimitiveTypePoint,
    MTLPrimitiveTypeTriangle, MTLPrimitiveTypeTriangleStrip, MTLQuadTessellationFactorsHalf,
    MTLRenderCommandEncoder, MTLRenderStageFragment, MTLRenderStageTile, MTLRenderStageVertex,
    MTLRenderStages, MTLScissorRect, MTLTriangleFillMode, MTLTriangleFillModeFill,
    MTLTriangleFillModeLines, MTLTriangleTessellationFactorsHalf,
    MTLVertexAmplificationViewMapping, MTLViewport, MTLVisibilityResultMode,
    MTLVisibilityResultModeBoolean, MTLVisibilityResultModeCounting,
    MTLVisibilityResultModeDisabled, MTLWinding, MTLWindingClockwise, MTLWindingCounterClockwise,
};
pub use self::__MTLRenderPass::{
    MTLClearColor, MTLLoadAction, MTLLoadActionClear, MTLLoadActionDontCare, MTLLoadActionLoad,
    MTLMultisampleDepthResolveFilter, MTLMultisampleDepthResolveFilterMax,
    MTLMultisampleDepthResolveFilterMin, MTLMultisampleDepthResolveFilterSample0,
    MTLMultisampleStencilResolveFilter, MTLMultisampleStencilResolveFilterDepthResolvedSample,
    MTLMultisampleStencilResolveFilterSample0, MTLRenderPassAttachmentDescriptor,
    MTLRenderPassColorAttachmentDescriptor, MTLRenderPassColorAttachmentDescriptorArray,
    MTLRenderPassDepthAttachmentDescriptor, MTLRenderPassDescriptor,
    MTLRenderPassSampleBufferAttachmentDescriptor,
    MTLRenderPassSampleBufferAttachmentDescriptorArray, MTLRenderPassStencilAttachmentDescriptor,
    MTLStoreAction, MTLStoreActionCustomSampleDepthStore, MTLStoreActionDontCare,
    MTLStoreActionMultisampleResolve, MTLStoreActionOptionCustomSamplePositions,
    MTLStoreActionOptionNone, MTLStoreActionOptions, MTLStoreActionStore,
    MTLStoreActionStoreAndMultisampleResolve, MTLStoreActionUnknown,
};
pub use self::__MTLRenderPipeline::{
    MTLBlendFactor, MTLBlendFactorBlendAlpha, MTLBlendFactorBlendColor,
    MTLBlendFactorDestinationAlpha, MTLBlendFactorDestinationColor, MTLBlendFactorOne,
    MTLBlendFactorOneMinusBlendAlpha, MTLBlendFactorOneMinusBlendColor,
    MTLBlendFactorOneMinusDestinationAlpha, MTLBlendFactorOneMinusDestinationColor,
    MTLBlendFactorOneMinusSource1Alpha, MTLBlendFactorOneMinusSource1Color,
    MTLBlendFactorOneMinusSourceAlpha, MTLBlendFactorOneMinusSourceColor,
    MTLBlendFactorSource1Alpha, MTLBlendFactorSource1Color, MTLBlendFactorSourceAlpha,
    MTLBlendFactorSourceAlphaSaturated, MTLBlendFactorSourceColor, MTLBlendFactorZero,
    MTLBlendOperation, MTLBlendOperationAdd, MTLBlendOperationMax, MTLBlendOperationMin,
    MTLBlendOperationReverseSubtract, MTLBlendOperationSubtract, MTLColorWriteMask,
    MTLColorWriteMaskAll, MTLColorWriteMaskAlpha, MTLColorWriteMaskBlue, MTLColorWriteMaskGreen,
    MTLColorWriteMaskNone, MTLColorWriteMaskRed, MTLPrimitiveTopologyClass,
    MTLPrimitiveTopologyClassLine, MTLPrimitiveTopologyClassPoint,
    MTLPrimitiveTopologyClassTriangle, MTLPrimitiveTopologyClassUnspecified,
    MTLRenderPipelineColorAttachmentDescriptor, MTLRenderPipelineColorAttachmentDescriptorArray,
    MTLRenderPipelineDescriptor, MTLRenderPipelineFunctionsDescriptor, MTLRenderPipelineReflection,
    MTLRenderPipelineState, MTLTessellationControlPointIndexType,
    MTLTessellationControlPointIndexTypeNone, MTLTessellationControlPointIndexTypeUInt16,
    MTLTessellationControlPointIndexTypeUInt32, MTLTessellationFactorFormat,
    MTLTessellationFactorFormatHalf, MTLTessellationFactorStepFunction,
    MTLTessellationFactorStepFunctionConstant, MTLTessellationFactorStepFunctionPerInstance,
    MTLTessellationFactorStepFunctionPerPatch,
    MTLTessellationFactorStepFunctionPerPatchAndPerInstance, MTLTessellationPartitionMode,
    MTLTessellationPartitionModeFractionalEven, MTLTessellationPartitionModeFractionalOdd,
    MTLTessellationPartitionModeInteger, MTLTessellationPartitionModePow2,
    MTLTileRenderPipelineColorAttachmentDescriptor,
    MTLTileRenderPipelineColorAttachmentDescriptorArray, MTLTileRenderPipelineDescriptor,
};
pub use self::__MTLResource::{
    MTLCPUCacheMode, MTLCPUCacheModeDefaultCache, MTLCPUCacheModeWriteCombined,
    MTLHazardTrackingMode, MTLHazardTrackingModeDefault, MTLHazardTrackingModeTracked,
    MTLHazardTrackingModeUntracked, MTLPurgeableState, MTLPurgeableStateEmpty,
    MTLPurgeableStateKeepCurrent, MTLPurgeableStateNonVolatile, MTLPurgeableStateVolatile,
    MTLResource, MTLResourceCPUCacheModeDefaultCache, MTLResourceCPUCacheModeWriteCombined,
    MTLResourceHazardTrackingModeDefault, MTLResourceHazardTrackingModeTracked,
    MTLResourceHazardTrackingModeUntracked, MTLResourceOptionCPUCacheModeDefault,
    MTLResourceOptionCPUCacheModeWriteCombined, MTLResourceOptions, MTLResourceStorageModeManaged,
    MTLResourceStorageModeMemoryless, MTLResourceStorageModePrivate, MTLResourceStorageModeShared,
    MTLStorageMode, MTLStorageModeManaged, MTLStorageModeMemoryless, MTLStorageModePrivate,
    MTLStorageModeShared,
};
pub use self::__MTLResourceStateCommandEncoder::{
    MTLMapIndirectArguments, MTLResourceStateCommandEncoder, MTLSparseTextureMappingMode,
    MTLSparseTextureMappingModeMap, MTLSparseTextureMappingModeUnmap,
};
pub use self::__MTLResourceStatePass::{
    MTLResourceStatePassDescriptor, MTLResourceStatePassSampleBufferAttachmentDescriptor,
    MTLResourceStatePassSampleBufferAttachmentDescriptorArray,
};
pub use self::__MTLSampler::{
    MTLSamplerAddressMode, MTLSamplerAddressModeClampToBorderColor,
    MTLSamplerAddressModeClampToEdge, MTLSamplerAddressModeClampToZero,
    MTLSamplerAddressModeMirrorClampToEdge, MTLSamplerAddressModeMirrorRepeat,
    MTLSamplerAddressModeRepeat, MTLSamplerBorderColor, MTLSamplerBorderColorOpaqueBlack,
    MTLSamplerBorderColorOpaqueWhite, MTLSamplerBorderColorTransparentBlack, MTLSamplerDescriptor,
    MTLSamplerMinMagFilter, MTLSamplerMinMagFilterLinear, MTLSamplerMinMagFilterNearest,
    MTLSamplerMipFilter, MTLSamplerMipFilterLinear, MTLSamplerMipFilterNearest,
    MTLSamplerMipFilterNotMipmapped, MTLSamplerState,
};
pub use self::__MTLStageInputOutputDescriptor::{
    MTLAttributeDescriptor, MTLAttributeDescriptorArray, MTLAttributeFormat,
    MTLAttributeFormatChar, MTLAttributeFormatChar2, MTLAttributeFormatChar2Normalized,
    MTLAttributeFormatChar3, MTLAttributeFormatChar3Normalized, MTLAttributeFormatChar4,
    MTLAttributeFormatChar4Normalized, MTLAttributeFormatCharNormalized, MTLAttributeFormatFloat,
    MTLAttributeFormatFloat2, MTLAttributeFormatFloat3, MTLAttributeFormatFloat4,
    MTLAttributeFormatHalf, MTLAttributeFormatHalf2, MTLAttributeFormatHalf3,
    MTLAttributeFormatHalf4, MTLAttributeFormatInt, MTLAttributeFormatInt1010102Normalized,
    MTLAttributeFormatInt2, MTLAttributeFormatInt3, MTLAttributeFormatInt4,
    MTLAttributeFormatInvalid, MTLAttributeFormatShort, MTLAttributeFormatShort2,
    MTLAttributeFormatShort2Normalized, MTLAttributeFormatShort3,
    MTLAttributeFormatShort3Normalized, MTLAttributeFormatShort4,
    MTLAttributeFormatShort4Normalized, MTLAttributeFormatShortNormalized, MTLAttributeFormatUChar,
    MTLAttributeFormatUChar2, MTLAttributeFormatUChar2Normalized, MTLAttributeFormatUChar3,
    MTLAttributeFormatUChar3Normalized, MTLAttributeFormatUChar4,
    MTLAttributeFormatUChar4Normalized, MTLAttributeFormatUChar4Normalized_BGRA,
    MTLAttributeFormatUCharNormalized, MTLAttributeFormatUInt,
    MTLAttributeFormatUInt1010102Normalized, MTLAttributeFormatUInt2, MTLAttributeFormatUInt3,
    MTLAttributeFormatUInt4, MTLAttributeFormatUShort, MTLAttributeFormatUShort2,
    MTLAttributeFormatUShort2Normalized, MTLAttributeFormatUShort3,
    MTLAttributeFormatUShort3Normalized, MTLAttributeFormatUShort4,
    MTLAttributeFormatUShort4Normalized, MTLAttributeFormatUShortNormalized,
    MTLBufferLayoutDescriptor, MTLBufferLayoutDescriptorArray, MTLIndexType, MTLIndexTypeUInt16,
    MTLIndexTypeUInt32, MTLStageInputOutputDescriptor, MTLStepFunction, MTLStepFunctionConstant,
    MTLStepFunctionPerInstance, MTLStepFunctionPerPatch, MTLStepFunctionPerPatchControlPoint,
    MTLStepFunctionPerVertex, MTLStepFunctionThreadPositionInGridX,
    MTLStepFunctionThreadPositionInGridXIndexed, MTLStepFunctionThreadPositionInGridY,
    MTLStepFunctionThreadPositionInGridYIndexed,
};
pub use self::__MTLTexture::{
    MTLSharedTextureHandle, MTLTexture, MTLTextureDescriptor, MTLTextureSwizzle,
    MTLTextureSwizzleAlpha, MTLTextureSwizzleBlue, MTLTextureSwizzleChannels,
    MTLTextureSwizzleGreen, MTLTextureSwizzleOne, MTLTextureSwizzleRed, MTLTextureSwizzleZero,
    MTLTextureType, MTLTextureType1D, MTLTextureType1DArray, MTLTextureType2D,
    MTLTextureType2DArray, MTLTextureType2DMultisample, MTLTextureType2DMultisampleArray,
    MTLTextureType3D, MTLTextureTypeCube, MTLTextureTypeCubeArray, MTLTextureTypeTextureBuffer,
    MTLTextureUsage, MTLTextureUsagePixelFormatView, MTLTextureUsageRenderTarget,
    MTLTextureUsageShaderRead, MTLTextureUsageShaderWrite, MTLTextureUsageUnknown,
};
pub use self::__MTLTypes::{MTLCoordinate2D, MTLOrigin, MTLRegion, MTLSamplePosition, MTLSize};
pub use self::__MTLVertexDescriptor::{
    MTLVertexAttributeDescriptor, MTLVertexAttributeDescriptorArray,
    MTLVertexBufferLayoutDescriptor, MTLVertexBufferLayoutDescriptorArray, MTLVertexDescriptor,
    MTLVertexFormat, MTLVertexFormatChar, MTLVertexFormatChar2, MTLVertexFormatChar2Normalized,
    MTLVertexFormatChar3, MTLVertexFormatChar3Normalized, MTLVertexFormatChar4,
    MTLVertexFormatChar4Normalized, MTLVertexFormatCharNormalized, MTLVertexFormatFloat,
    MTLVertexFormatFloat2, MTLVertexFormatFloat3, MTLVertexFormatFloat4, MTLVertexFormatHalf,
    MTLVertexFormatHalf2, MTLVertexFormatHalf3, MTLVertexFormatHalf4, MTLVertexFormatInt,
    MTLVertexFormatInt1010102Normalized, MTLVertexFormatInt2, MTLVertexFormatInt3,
    MTLVertexFormatInt4, MTLVertexFormatInvalid, MTLVertexFormatShort, MTLVertexFormatShort2,
    MTLVertexFormatShort2Normalized, MTLVertexFormatShort3, MTLVertexFormatShort3Normalized,
    MTLVertexFormatShort4, MTLVertexFormatShort4Normalized, MTLVertexFormatShortNormalized,
    MTLVertexFormatUChar, MTLVertexFormatUChar2, MTLVertexFormatUChar2Normalized,
    MTLVertexFormatUChar3, MTLVertexFormatUChar3Normalized, MTLVertexFormatUChar4,
    MTLVertexFormatUChar4Normalized, MTLVertexFormatUChar4Normalized_BGRA,
    MTLVertexFormatUCharNormalized, MTLVertexFormatUInt, MTLVertexFormatUInt1010102Normalized,
    MTLVertexFormatUInt2, MTLVertexFormatUInt3, MTLVertexFormatUInt4, MTLVertexFormatUShort,
    MTLVertexFormatUShort2, MTLVertexFormatUShort2Normalized, MTLVertexFormatUShort3,
    MTLVertexFormatUShort3Normalized, MTLVertexFormatUShort4, MTLVertexFormatUShort4Normalized,
    MTLVertexFormatUShortNormalized, MTLVertexStepFunction, MTLVertexStepFunctionConstant,
    MTLVertexStepFunctionPerInstance, MTLVertexStepFunctionPerPatch,
    MTLVertexStepFunctionPerPatchControlPoint, MTLVertexStepFunctionPerVertex,
};
pub use self::__MTLVisibleFunctionTable::{
    MTLVisibleFunctionTable, MTLVisibleFunctionTableDescriptor,
};