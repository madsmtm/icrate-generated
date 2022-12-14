//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionConstantValues;

    unsafe impl ClassType for MTLFunctionConstantValues {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MTLFunctionConstantValues {
        #[method(setConstantValue:type:atIndex:)]
        pub unsafe fn setConstantValue_type_atIndex(
            &self,
            value: NonNull<c_void>,
            type_: MTLDataType,
            index: NSUInteger,
        );

        #[method(setConstantValues:type:withRange:)]
        pub unsafe fn setConstantValues_type_withRange(
            &self,
            values: NonNull<c_void>,
            type_: MTLDataType,
            range: NSRange,
        );

        #[method(setConstantValue:type:withName:)]
        pub unsafe fn setConstantValue_type_withName(
            &self,
            value: NonNull<c_void>,
            type_: MTLDataType,
            name: &NSString,
        );

        #[method(reset)]
        pub fn reset(&self);
    }
);
