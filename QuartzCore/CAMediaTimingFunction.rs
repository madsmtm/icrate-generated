//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type CAMediaTimingFunctionName = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAMediaTimingFunction;

    unsafe impl ClassType for CAMediaTimingFunction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CAMediaTimingFunction {}

unsafe impl NSObjectProtocol for CAMediaTimingFunction {}

unsafe impl NSSecureCoding for CAMediaTimingFunction {}

extern_methods!(
    unsafe impl CAMediaTimingFunction {
        #[method_id(@__retain_semantics Other functionWithName:)]
        pub unsafe fn functionWithName(name: &CAMediaTimingFunctionName) -> Id<Self>;

        #[method_id(@__retain_semantics Other functionWithControlPoints::::)]
        pub unsafe fn functionWithControlPoints(
            c1x: c_float,
            c1y: c_float,
            c2x: c_float,
            c2y: c_float,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithControlPoints::::)]
        pub unsafe fn initWithControlPoints(
            this: Allocated<Self>,
            c1x: c_float,
            c1y: c_float,
            c2x: c_float,
            c2y: c_float,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAMediaTimingFunction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    pub static kCAMediaTimingFunctionLinear: &'static CAMediaTimingFunctionName;
}

extern "C" {
    pub static kCAMediaTimingFunctionEaseIn: &'static CAMediaTimingFunctionName;
}

extern "C" {
    pub static kCAMediaTimingFunctionEaseOut: &'static CAMediaTimingFunctionName;
}

extern "C" {
    pub static kCAMediaTimingFunctionEaseInEaseOut: &'static CAMediaTimingFunctionName;
}

extern "C" {
    pub static kCAMediaTimingFunctionDefault: &'static CAMediaTimingFunctionName;
}
