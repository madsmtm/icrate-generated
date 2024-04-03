//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    pub struct NSValue;

    unsafe impl ClassType for NSValue {
        type Super = NSObject;
        type Mutability = Immutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSValue {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSValue {}

unsafe impl NSObjectProtocol for NSValue {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSValue {}

extern_methods!(
    unsafe impl NSValue {
        #[method(getValue:size:)]
        pub unsafe fn getValue_size(&self, value: NonNull<c_void>, size: NSUInteger);

        #[method(objCType)]
        pub fn objCType(&self) -> NonNull<c_char>;

        #[method_id(@__retain_semantics Init initWithBytes:objCType:)]
        pub unsafe fn initWithBytes_objCType(
            this: Allocated<Self>,
            value: NonNull<c_void>,
            r#type: NonNull<c_char>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// NSValueCreation
    unsafe impl NSValue {
        #[method_id(@__retain_semantics Other valueWithBytes:objCType:)]
        pub unsafe fn valueWithBytes_objCType(
            value: NonNull<c_void>,
            r#type: NonNull<c_char>,
        ) -> Id<NSValue>;

        #[method_id(@__retain_semantics Other value:withObjCType:)]
        pub unsafe fn value_withObjCType(
            value: NonNull<c_void>,
            r#type: NonNull<c_char>,
        ) -> Id<NSValue>;
    }
);

extern_methods!(
    /// NSValueExtensionMethods
    unsafe impl NSValue {
        #[method_id(@__retain_semantics Other valueWithNonretainedObject:)]
        pub unsafe fn valueWithNonretainedObject(an_object: Option<&AnyObject>) -> Id<NSValue>;

        #[method_id(@__retain_semantics Other nonretainedObjectValue)]
        pub unsafe fn nonretainedObjectValue(&self) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other valueWithPointer:)]
        pub unsafe fn valueWithPointer(pointer: *mut c_void) -> Id<NSValue>;

        #[method(pointerValue)]
        pub unsafe fn pointerValue(&self) -> *mut c_void;

        #[method(isEqualToValue:)]
        pub fn isEqualToValue(&self, value: &NSValue) -> bool;
    }
);

extern_class!(
    pub struct NSNumber;

    unsafe impl ClassType for NSNumber {
        #[inherits(NSObject)]
        type Super = NSValue;
        type Mutability = Immutable;
    }
);

unsafe impl Send for NSNumber {}

unsafe impl Sync for NSNumber {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSNumber {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSNumber {}

unsafe impl NSObjectProtocol for NSNumber {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSNumber {}

extern_methods!(
    unsafe impl NSNumber {
        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithChar:)]
        pub fn initWithChar(this: Allocated<Self>, value: c_char) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithUnsignedChar:)]
        pub fn initWithUnsignedChar(this: Allocated<Self>, value: c_uchar) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithShort:)]
        pub fn initWithShort(this: Allocated<Self>, value: c_short) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithUnsignedShort:)]
        pub fn initWithUnsignedShort(this: Allocated<Self>, value: c_ushort) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithInt:)]
        pub fn initWithInt(this: Allocated<Self>, value: c_int) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithUnsignedInt:)]
        pub fn initWithUnsignedInt(this: Allocated<Self>, value: c_uint) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithLong:)]
        pub fn initWithLong(this: Allocated<Self>, value: c_long) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithUnsignedLong:)]
        pub fn initWithUnsignedLong(this: Allocated<Self>, value: c_ulong) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithLongLong:)]
        pub fn initWithLongLong(this: Allocated<Self>, value: c_longlong) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithUnsignedLongLong:)]
        pub fn initWithUnsignedLongLong(this: Allocated<Self>, value: c_ulonglong) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithFloat:)]
        pub fn initWithFloat(this: Allocated<Self>, value: c_float) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithDouble:)]
        pub fn initWithDouble(this: Allocated<Self>, value: c_double) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithBool:)]
        pub fn initWithBool(this: Allocated<Self>, value: bool) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithInteger:)]
        pub fn initWithInteger(this: Allocated<Self>, value: NSInteger) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Init initWithUnsignedInteger:)]
        pub fn initWithUnsignedInteger(this: Allocated<Self>, value: NSUInteger) -> Id<NSNumber>;

        #[method(charValue)]
        pub fn charValue(&self) -> c_char;

        #[method(unsignedCharValue)]
        pub fn unsignedCharValue(&self) -> c_uchar;

        #[method(shortValue)]
        pub fn shortValue(&self) -> c_short;

        #[method(unsignedShortValue)]
        pub fn unsignedShortValue(&self) -> c_ushort;

        #[method(intValue)]
        pub fn intValue(&self) -> c_int;

        #[method(unsignedIntValue)]
        pub fn unsignedIntValue(&self) -> c_uint;

        #[method(longValue)]
        pub fn longValue(&self) -> c_long;

        #[method(unsignedLongValue)]
        pub fn unsignedLongValue(&self) -> c_ulong;

        #[method(longLongValue)]
        pub fn longLongValue(&self) -> c_longlong;

        #[method(unsignedLongLongValue)]
        pub fn unsignedLongLongValue(&self) -> c_ulonglong;

        #[method(floatValue)]
        pub fn floatValue(&self) -> c_float;

        #[method(doubleValue)]
        pub fn doubleValue(&self) -> c_double;

        #[method(boolValue)]
        pub fn boolValue(&self) -> bool;

        #[method(integerValue)]
        pub fn integerValue(&self) -> NSInteger;

        #[method(unsignedIntegerValue)]
        pub fn unsignedIntegerValue(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringValue)]
        pub fn stringValue(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSObjCRuntime")]
        #[method(compare:)]
        pub fn compare(&self, other_number: &NSNumber) -> NSComparisonResult;

        #[method(isEqualToNumber:)]
        pub fn isEqualToNumber(&self, number: &NSNumber) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&AnyObject>) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSValue`
    unsafe impl NSNumber {
        #[method_id(@__retain_semantics Init initWithBytes:objCType:)]
        pub unsafe fn initWithBytes_objCType(
            this: Allocated<Self>,
            value: NonNull<c_void>,
            r#type: NonNull<c_char>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// NSNumberCreation
    unsafe impl NSNumber {
        #[method_id(@__retain_semantics Other numberWithChar:)]
        pub fn numberWithChar(value: c_char) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithUnsignedChar:)]
        pub fn numberWithUnsignedChar(value: c_uchar) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithShort:)]
        pub fn numberWithShort(value: c_short) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithUnsignedShort:)]
        pub fn numberWithUnsignedShort(value: c_ushort) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithInt:)]
        pub fn numberWithInt(value: c_int) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithUnsignedInt:)]
        pub fn numberWithUnsignedInt(value: c_uint) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithLong:)]
        pub fn numberWithLong(value: c_long) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithUnsignedLong:)]
        pub fn numberWithUnsignedLong(value: c_ulong) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithLongLong:)]
        pub fn numberWithLongLong(value: c_longlong) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithUnsignedLongLong:)]
        pub fn numberWithUnsignedLongLong(value: c_ulonglong) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithFloat:)]
        pub fn numberWithFloat(value: c_float) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithDouble:)]
        pub fn numberWithDouble(value: c_double) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithBool:)]
        pub fn numberWithBool(value: bool) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithInteger:)]
        pub fn numberWithInteger(value: NSInteger) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other numberWithUnsignedInteger:)]
        pub fn numberWithUnsignedInteger(value: NSUInteger) -> Id<NSNumber>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSValue {
        #[deprecated]
        #[method(getValue:)]
        pub unsafe fn getValue(&self, value: NonNull<c_void>);
    }
);
