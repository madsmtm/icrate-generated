//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::BackgroundAssets::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundAssets_BAURLDownload")]
    pub struct BAURLDownload;

    #[cfg(feature = "BackgroundAssets_BAURLDownload")]
    unsafe impl ClassType for BAURLDownload {
        #[inherits(NSObject)]
        type Super = BADownload;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "BackgroundAssets_BAURLDownload")]
unsafe impl NSCoding for BAURLDownload {}

#[cfg(feature = "BackgroundAssets_BAURLDownload")]
unsafe impl NSCopying for BAURLDownload {}

#[cfg(feature = "BackgroundAssets_BAURLDownload")]
unsafe impl NSObjectProtocol for BAURLDownload {}

#[cfg(feature = "BackgroundAssets_BAURLDownload")]
unsafe impl NSSecureCoding for BAURLDownload {}

extern_methods!(
    #[cfg(feature = "BackgroundAssets_BAURLDownload")]
    unsafe impl BAURLDownload {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURLRequest"))]
        #[method_id(@__retain_semantics Init initWithIdentifier:request:fileSize:applicationGroupIdentifier:)]
        pub unsafe fn initWithIdentifier_request_fileSize_applicationGroupIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            request: &NSURLRequest,
            file_size: NSUInteger,
            application_group_identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURLRequest"))]
        #[method_id(@__retain_semantics Init initWithIdentifier:request:essential:fileSize:applicationGroupIdentifier:priority:)]
        pub unsafe fn initWithIdentifier_request_essential_fileSize_applicationGroupIdentifier_priority(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            request: &NSURLRequest,
            essential: bool,
            file_size: NSUInteger,
            application_group_identifier: &NSString,
            priority: BADownloaderPriority,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURLRequest"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithIdentifier:request:applicationGroupIdentifier:)]
        pub unsafe fn initWithIdentifier_request_applicationGroupIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            request: &NSURLRequest,
            application_group_identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURLRequest"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithIdentifier:request:applicationGroupIdentifier:priority:)]
        pub unsafe fn initWithIdentifier_request_applicationGroupIdentifier_priority(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            request: &NSURLRequest,
            application_group_identifier: &NSString,
            priority: BADownloaderPriority,
        ) -> Id<Self>;
    }
);
