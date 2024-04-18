//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLResourceUsage(pub NSUInteger);
bitflags::bitflags! {
    impl MTLResourceUsage: NSUInteger {
        #[doc(alias = "MTLResourceUsageRead")]
        const Read = 1<<0;
        #[doc(alias = "MTLResourceUsageWrite")]
        const Write = 1<<1;
#[deprecated]
        #[doc(alias = "MTLResourceUsageSample")]
        const Sample = 1<<2;
    }
}

unsafe impl Encode for MTLResourceUsage {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLResourceUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLBarrierScope(pub NSUInteger);
bitflags::bitflags! {
    impl MTLBarrierScope: NSUInteger {
        #[doc(alias = "MTLBarrierScopeBuffers")]
        const Buffers = 1<<0;
        #[doc(alias = "MTLBarrierScopeTextures")]
        const Textures = 1<<1;
        #[doc(alias = "MTLBarrierScopeRenderTargets")]
        const RenderTargets = 1<<2;
    }
}

unsafe impl Encode for MTLBarrierScope {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLBarrierScope {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait MTLCommandEncoder: NSObjectProtocol + IsRetainable {
        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        fn set_label(&self, label: Option<&NSString>);

        #[method(endEncoding)]
        fn end_encoding(&self);

        #[method(insertDebugSignpost:)]
        fn insert_debug_signpost(&self, string: &NSString);

        #[method(pushDebugGroup:)]
        fn push_debug_group(&self, string: &NSString);

        #[method(popDebugGroup)]
        fn pop_debug_group(&self);
    }

    unsafe impl ProtocolType for dyn MTLCommandEncoder {}
);
