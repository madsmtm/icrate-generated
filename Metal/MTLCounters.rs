//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type MTLCommonCounter = NSString;

extern "C" {
    pub static MTLCommonCounterTimestamp: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterTessellationInputPatches: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterVertexInvocations: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterPostTessellationVertexInvocations: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterClipperInvocations: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterClipperPrimitivesOut: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterFragmentInvocations: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterFragmentsPassed: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterComputeKernelInvocations: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterTotalCycles: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterVertexCycles: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterTessellationCycles: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterPostTessellationVertexCycles: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterFragmentCycles: &'static MTLCommonCounter;
}

extern "C" {
    pub static MTLCommonCounterRenderTargetWriteCycles: &'static MTLCommonCounter;
}

// NS_TYPED_ENUM
pub type MTLCommonCounterSet = NSString;

extern "C" {
    pub static MTLCommonCounterSetTimestamp: &'static MTLCommonCounterSet;
}

extern "C" {
    pub static MTLCommonCounterSetStageUtilization: &'static MTLCommonCounterSet;
}

extern "C" {
    pub static MTLCommonCounterSetStatistic: &'static MTLCommonCounterSet;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultTimestamp {
    pub timestamp: u64,
}

unsafe impl Encode for MTLCounterResultTimestamp {
    const ENCODING: Encoding = Encoding::Struct("?", &[<u64>::ENCODING]);
}

unsafe impl RefEncode for MTLCounterResultTimestamp {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultStageUtilization {
    pub totalCycles: u64,
    pub vertexCycles: u64,
    pub tessellationCycles: u64,
    pub postTessellationVertexCycles: u64,
    pub fragmentCycles: u64,
    pub renderTargetCycles: u64,
}

unsafe impl Encode for MTLCounterResultStageUtilization {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLCounterResultStageUtilization {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultStatistic {
    pub tessellationInputPatches: u64,
    pub vertexInvocations: u64,
    pub postTessellationVertexInvocations: u64,
    pub clipperInvocations: u64,
    pub clipperPrimitivesOut: u64,
    pub fragmentInvocations: u64,
    pub fragmentsPassed: u64,
    pub computeKernelInvocations: u64,
}

unsafe impl Encode for MTLCounterResultStatistic {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLCounterResultStatistic {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait MTLCounter: NSObjectProtocol + IsRetainable {
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Retained<NSString>;
    }

    unsafe impl ProtocolType for dyn MTLCounter {}
);

extern_protocol!(
    pub unsafe trait MTLCounterSet: NSObjectProtocol + IsRetainable {
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other counters)]
        unsafe fn counters(&self) -> Retained<NSArray<ProtocolObject<dyn MTLCounter>>>;
    }

    unsafe impl ProtocolType for dyn MTLCounterSet {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCounterSampleBufferDescriptor;

    unsafe impl ClassType for MTLCounterSampleBufferDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MTLCounterSampleBufferDescriptor {}

unsafe impl NSObjectProtocol for MTLCounterSampleBufferDescriptor {}

extern_methods!(
    unsafe impl MTLCounterSampleBufferDescriptor {
        #[method_id(@__retain_semantics Other counterSet)]
        pub unsafe fn counter_set(&self) -> Option<Retained<ProtocolObject<dyn MTLCounterSet>>>;

        #[method(setCounterSet:)]
        pub unsafe fn set_counter_set(
            &self,
            counter_set: Option<&ProtocolObject<dyn MTLCounterSet>>,
        );

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        #[method(setLabel:)]
        pub unsafe fn set_label(&self, label: &NSString);

        #[cfg(feature = "MTLResource")]
        #[method(storageMode)]
        pub unsafe fn storage_mode(&self) -> MTLStorageMode;

        #[cfg(feature = "MTLResource")]
        #[method(setStorageMode:)]
        pub unsafe fn set_storage_mode(&self, storage_mode: MTLStorageMode);

        #[method(sampleCount)]
        pub unsafe fn sample_count(&self) -> NSUInteger;

        #[method(setSampleCount:)]
        pub unsafe fn set_sample_count(&self, sample_count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLCounterSampleBufferDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLCounterSampleBuffer: NSObjectProtocol + IsRetainable {
        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Retained<NSString>;

        #[method(sampleCount)]
        unsafe fn sample_count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other resolveCounterRange:)]
        unsafe fn resolve_counter_range(&self, range: NSRange) -> Option<Retained<NSData>>;
    }

    unsafe impl ProtocolType for dyn MTLCounterSampleBuffer {}
);

extern "C" {
    pub static MTLCounterErrorDomain: &'static NSErrorDomain;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCounterSampleBufferError(pub NSInteger);
impl MTLCounterSampleBufferError {
    #[doc(alias = "MTLCounterSampleBufferErrorOutOfMemory")]
    pub const OutOfMemory: Self = Self(0);
    #[doc(alias = "MTLCounterSampleBufferErrorInvalid")]
    pub const Invalid: Self = Self(1);
    #[doc(alias = "MTLCounterSampleBufferErrorInternal")]
    pub const Internal: Self = Self(2);
}

unsafe impl Encode for MTLCounterSampleBufferError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLCounterSampleBufferError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
