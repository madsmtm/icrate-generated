//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

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
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
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
