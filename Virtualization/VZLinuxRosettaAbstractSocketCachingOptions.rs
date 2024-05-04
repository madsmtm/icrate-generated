//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZLinuxRosettaCachingOptions")]
    pub struct VZLinuxRosettaAbstractSocketCachingOptions;

    #[cfg(feature = "VZLinuxRosettaCachingOptions")]
    unsafe impl ClassType for VZLinuxRosettaAbstractSocketCachingOptions {
        #[inherits(NSObject)]
        type Super = VZLinuxRosettaCachingOptions;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZLinuxRosettaCachingOptions")]
unsafe impl NSObjectProtocol for VZLinuxRosettaAbstractSocketCachingOptions {}

extern_methods!(
    #[cfg(feature = "VZLinuxRosettaCachingOptions")]
    unsafe impl VZLinuxRosettaAbstractSocketCachingOptions {
        #[method_id(@__retain_semantics Init initWithName:error:_)]
        pub unsafe fn initWithName_error(
            this: Allocated<Self>,
            name: &NSString,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[method(maximumNameLength)]
        pub unsafe fn maximumNameLength() -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZLinuxRosettaCachingOptions`
    #[cfg(feature = "VZLinuxRosettaCachingOptions")]
    unsafe impl VZLinuxRosettaAbstractSocketCachingOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
