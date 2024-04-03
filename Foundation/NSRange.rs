//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

pub type NSRangePointer = *mut NSRange;

// TODO: pub fn NSMakeRange(loc: NSUInteger,len: NSUInteger,) -> NSRange;

// TODO: pub fn NSMaxRange(range: NSRange,) -> NSUInteger;

// TODO: pub fn NSLocationInRange(loc: NSUInteger,range: NSRange,) -> Bool;

// TODO: pub fn NSEqualRanges(range1: NSRange,range2: NSRange,) -> Bool;

extern "C" {
    pub fn NSUnionRange(range1: NSRange, range2: NSRange) -> NSRange;
}

extern "C" {
    pub fn NSIntersectionRange(range1: NSRange, range2: NSRange) -> NSRange;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub fn NSStringFromRange(range: NSRange) -> NonNull<NSString>;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub fn NSRangeFromString(a_string: &NSString) -> NSRange;
}

extern_methods!(
    /// NSValueRangeExtensions
    #[cfg(feature = "Foundation_NSValue")]
    unsafe impl NSValue {
        #[method_id(@__retain_semantics Other valueWithRange:)]
        pub unsafe fn valueWithRange(range: NSRange) -> Id<NSValue>;

        #[method(rangeValue)]
        pub unsafe fn rangeValue(&self) -> NSRange;
    }
);
