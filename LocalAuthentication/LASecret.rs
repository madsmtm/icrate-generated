//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LASecret;

    unsafe impl ClassType for LASecret {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for LASecret {}

extern_methods!(
    unsafe impl LASecret {
        #[cfg(feature = "block2")]
        #[method(loadDataWithCompletion:)]
        pub unsafe fn loadDataWithCompletion(
            &self,
            handler: &Block<dyn Fn(*mut NSData, *mut NSError)>,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
