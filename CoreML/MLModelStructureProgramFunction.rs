//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLModelStructureProgramFunction;

    unsafe impl ClassType for MLModelStructureProgramFunction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for MLModelStructureProgramFunction {}

unsafe impl Sync for MLModelStructureProgramFunction {}

unsafe impl NSObjectProtocol for MLModelStructureProgramFunction {}

extern_methods!(
    unsafe impl MLModelStructureProgramFunction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "MLModelStructureProgramNamedValueType")]
        #[method_id(@__retain_semantics Other inputs)]
        pub unsafe fn inputs(&self) -> Id<NSArray<MLModelStructureProgramNamedValueType>>;

        #[cfg(feature = "MLModelStructureProgramBlock")]
        #[method_id(@__retain_semantics Other block)]
        pub unsafe fn block(&self) -> Id<MLModelStructureProgramBlock>;
    }
);
