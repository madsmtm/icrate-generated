//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// NSAppKitAdditions
    unsafe impl NSAffineTransform {
        #[method_id(@__retain_semantics Other transformBezierPath:)]
        pub unsafe fn transformBezierPath(&self, path: &NSBezierPath) -> Id<NSBezierPath, Shared>;

        #[method(set)]
        pub unsafe fn set(&self);

        #[method(concat)]
        pub unsafe fn concat(&self);
    }
);
