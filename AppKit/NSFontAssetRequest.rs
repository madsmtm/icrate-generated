//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFontAssetRequestOptions {
        NSFontAssetRequestOptionUsesStandardUI = 1 << 0,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSFontAssetRequest")]
    pub struct NSFontAssetRequest;

    #[cfg(feature = "AppKit_NSFontAssetRequest")]
    unsafe impl ClassType for NSFontAssetRequest {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSFontAssetRequest")]
    unsafe impl NSFontAssetRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Init initWithFontDescriptors:options:)]
        pub unsafe fn initWithFontDescriptors_options(
            this: Option<Allocated<Self>>,
            fontDescriptors: &NSArray<NSFontDescriptor>,
            options: NSFontAssetRequestOptions,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other downloadedFontDescriptors)]
        pub unsafe fn downloadedFontDescriptors(&self) -> Id<NSArray<NSFontDescriptor>, Shared>;

        #[cfg(feature = "Foundation_NSProgress")]
        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Id<NSProgress, Shared>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(downloadFontAssetsWithCompletionHandler:)]
        pub unsafe fn downloadFontAssetsWithCompletionHandler(
            &self,
            completionHandler: &Block<(*mut NSError,), Bool>,
        );
    }
);
