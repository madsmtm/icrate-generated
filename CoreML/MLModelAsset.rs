//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLModelAsset;

    unsafe impl ClassType for MLModelAsset {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MLModelAsset {}

extern_methods!(
    unsafe impl MLModelAsset {
        #[method_id(@__retain_semantics Other modelAssetWithSpecificationData:error:_)]
        pub unsafe fn modelAssetWithSpecificationData_error(
            specification_data: &NSData,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
