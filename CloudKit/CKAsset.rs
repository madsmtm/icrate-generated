//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKAsset;

    unsafe impl ClassType for CKAsset {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CKAsset {}

extern_methods!(
    unsafe impl CKAsset {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithFileURL:)]
        pub unsafe fn initWithFileURL(this: Allocated<Self>, file_url: &NSURL) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other fileURL)]
        pub unsafe fn fileURL(&self) -> Option<Id<NSURL>>;
    }
);
