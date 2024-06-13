//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLMutability(pub NSUInteger);
impl MTLMutability {
    #[doc(alias = "MTLMutabilityDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "MTLMutabilityMutable")]
    pub const Mutable: Self = Self(1);
    #[doc(alias = "MTLMutabilityImmutable")]
    pub const Immutable: Self = Self(2);
}

unsafe impl Encode for MTLMutability {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLMutability {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLShaderValidation(pub NSInteger);
impl MTLShaderValidation {
    #[doc(alias = "MTLShaderValidationDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "MTLShaderValidationEnabled")]
    pub const Enabled: Self = Self(1);
    #[doc(alias = "MTLShaderValidationDisabled")]
    pub const Disabled: Self = Self(2);
}

unsafe impl Encode for MTLShaderValidation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLShaderValidation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLPipelineBufferDescriptor;

    unsafe impl ClassType for MTLPipelineBufferDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MTLPipelineBufferDescriptor {}

unsafe impl NSObjectProtocol for MTLPipelineBufferDescriptor {}

extern_methods!(
    unsafe impl MTLPipelineBufferDescriptor {
        #[method(mutability)]
        pub fn mutability(&self) -> MTLMutability;

        #[method(setMutability:)]
        pub fn setMutability(&self, mutability: MTLMutability);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLPipelineBufferDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLPipelineBufferDescriptorArray;

    unsafe impl ClassType for MTLPipelineBufferDescriptorArray {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MTLPipelineBufferDescriptorArray {}

extern_methods!(
    unsafe impl MTLPipelineBufferDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            buffer_index: NSUInteger,
        ) -> Retained<MTLPipelineBufferDescriptor>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            buffer: Option<&MTLPipelineBufferDescriptor>,
            buffer_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLPipelineBufferDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
