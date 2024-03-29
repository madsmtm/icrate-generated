//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLDownload")]
    #[deprecated]
    pub struct WebDownload;

    #[cfg(feature = "Foundation_NSURLDownload")]
    unsafe impl ClassType for WebDownload {
        #[inherits(NSObject)]
        type Super = NSURLDownload;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSURLDownload")]
unsafe impl NSObjectProtocol for WebDownload {}

extern_methods!(
    #[cfg(feature = "Foundation_NSURLDownload")]
    unsafe impl WebDownload {}
);

extern_methods!(
    /// Methods declared on superclass `NSURLDownload`
    #[cfg(feature = "Foundation_NSURLDownload")]
    unsafe impl WebDownload {
        #[cfg(feature = "Foundation_NSURLRequest")]
        #[deprecated = "Use NSURLSession downloadTask (see NSURLSession.h)"]
        #[method_id(@__retain_semantics Init initWithRequest:delegate:)]
        pub unsafe fn initWithRequest_delegate(
            this: Allocated<Self>,
            request: &NSURLRequest,
            delegate: Option<&ProtocolObject<dyn NSURLDownloadDelegate>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSURLSession downloadTask (see NSURLSession.h)"]
        #[method_id(@__retain_semantics Init initWithResumeData:delegate:path:)]
        pub unsafe fn initWithResumeData_delegate_path(
            this: Allocated<Self>,
            resume_data: &NSData,
            delegate: Option<&ProtocolObject<dyn NSURLDownloadDelegate>>,
            path: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSURLDownload")]
    unsafe impl WebDownload {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    #[cfg(feature = "Foundation_NSURLDownload")]
    #[deprecated]
    pub unsafe trait WebDownloadDelegate: NSURLDownloadDelegate {
        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSWindow"))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other downloadWindowForAuthenticationSheet:)]
        unsafe fn downloadWindowForAuthenticationSheet(
            &self,
            download: Option<&WebDownload>,
            mtm: MainThreadMarker,
        ) -> Option<Id<NSWindow>>;
    }

    #[cfg(feature = "Foundation_NSURLDownload")]
    unsafe impl ProtocolType for dyn WebDownloadDelegate {}
);
