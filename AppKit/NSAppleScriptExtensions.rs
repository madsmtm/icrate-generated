//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// NSExtensions
    unsafe impl NSAppleScript {
        #[method_id(@__retain_semantics Other richTextSource)]
        pub unsafe fn richTextSource(&self) -> Option<Id<NSAttributedString, Shared>>;
    }
);
