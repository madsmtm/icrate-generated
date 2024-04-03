//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type CAValueFunctionName = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAValueFunction;

    unsafe impl ClassType for CAValueFunction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CAValueFunction {}

unsafe impl NSObjectProtocol for CAValueFunction {}

unsafe impl NSSecureCoding for CAValueFunction {}

extern_methods!(
    unsafe impl CAValueFunction {
        #[method_id(@__retain_semantics Other functionWithName:)]
        pub unsafe fn functionWithName(name: &CAValueFunctionName) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<CAValueFunctionName>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAValueFunction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    pub static kCAValueFunctionRotateX: &'static CAValueFunctionName;
}

extern "C" {
    pub static kCAValueFunctionRotateY: &'static CAValueFunctionName;
}

extern "C" {
    pub static kCAValueFunctionRotateZ: &'static CAValueFunctionName;
}

extern "C" {
    pub static kCAValueFunctionScale: &'static CAValueFunctionName;
}

extern "C" {
    pub static kCAValueFunctionScaleX: &'static CAValueFunctionName;
}

extern "C" {
    pub static kCAValueFunctionScaleY: &'static CAValueFunctionName;
}

extern "C" {
    pub static kCAValueFunctionScaleZ: &'static CAValueFunctionName;
}

extern "C" {
    pub static kCAValueFunctionTranslate: &'static CAValueFunctionName;
}

extern "C" {
    pub static kCAValueFunctionTranslateX: &'static CAValueFunctionName;
}

extern "C" {
    pub static kCAValueFunctionTranslateY: &'static CAValueFunctionName;
}

extern "C" {
    pub static kCAValueFunctionTranslateZ: &'static CAValueFunctionName;
}
