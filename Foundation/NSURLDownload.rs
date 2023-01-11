//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLDownload")]
    pub struct NSURLDownload;

    #[cfg(feature = "Foundation_NSURLDownload")]
    unsafe impl ClassType for NSURLDownload {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSURLDownload")]
    unsafe impl NSURLDownload {
        #[cfg(feature = "Foundation_NSString")]
        #[method(canResumeDownloadDecodedWithEncodingMIMEType:)]
        pub unsafe fn canResumeDownloadDecodedWithEncodingMIMEType(MIMEType: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Init initWithRequest:delegate:)]
        pub unsafe fn initWithRequest_delegate(
            this: Option<Allocated<Self>>,
            request: &NSURLRequest,
            delegate: Option<&NSURLDownloadDelegate>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithResumeData:delegate:path:)]
        pub unsafe fn initWithResumeData_delegate_path(
            this: Option<Allocated<Self>>,
            resumeData: &NSData,
            delegate: Option<&NSURLDownloadDelegate>,
            path: &NSString,
        ) -> Id<Self, Shared>;

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDestination:allowOverwrite:)]
        pub unsafe fn setDestination_allowOverwrite(&self, path: &NSString, allowOverwrite: bool);

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Id<NSURLRequest, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other resumeData)]
        pub unsafe fn resumeData(&self) -> Option<Id<NSData, Shared>>;

        #[method(deletesFileUponFailure)]
        pub unsafe fn deletesFileUponFailure(&self) -> bool;

        #[method(setDeletesFileUponFailure:)]
        pub unsafe fn setDeletesFileUponFailure(&self, deletesFileUponFailure: bool);
    }
);

extern_protocol!(
    pub struct NSURLDownloadDelegate;

    unsafe impl ProtocolType for NSURLDownloadDelegate {
        #[optional]
        #[method(downloadDidBegin:)]
        pub unsafe fn downloadDidBegin(&self, download: &NSURLDownload);

        #[optional]
        #[method_id(@__retain_semantics Other download:willSendRequest:redirectResponse:)]
        pub unsafe fn download_willSendRequest_redirectResponse(
            &self,
            download: &NSURLDownload,
            request: &NSURLRequest,
            redirectResponse: Option<&NSURLResponse>,
        ) -> Option<Id<NSURLRequest, Shared>>;

        #[optional]
        #[method(download:canAuthenticateAgainstProtectionSpace:)]
        pub unsafe fn download_canAuthenticateAgainstProtectionSpace(
            &self,
            connection: &NSURLDownload,
            protectionSpace: &NSURLProtectionSpace,
        ) -> bool;

        #[optional]
        #[method(download:didReceiveAuthenticationChallenge:)]
        pub unsafe fn download_didReceiveAuthenticationChallenge(
            &self,
            download: &NSURLDownload,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[optional]
        #[method(download:didCancelAuthenticationChallenge:)]
        pub unsafe fn download_didCancelAuthenticationChallenge(
            &self,
            download: &NSURLDownload,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[optional]
        #[method(downloadShouldUseCredentialStorage:)]
        pub unsafe fn downloadShouldUseCredentialStorage(&self, download: &NSURLDownload) -> bool;

        #[optional]
        #[method(download:didReceiveResponse:)]
        pub unsafe fn download_didReceiveResponse(
            &self,
            download: &NSURLDownload,
            response: &NSURLResponse,
        );

        #[optional]
        #[method(download:willResumeWithResponse:fromByte:)]
        pub unsafe fn download_willResumeWithResponse_fromByte(
            &self,
            download: &NSURLDownload,
            response: &NSURLResponse,
            startingByte: c_longlong,
        );

        #[optional]
        #[method(download:didReceiveDataOfLength:)]
        pub unsafe fn download_didReceiveDataOfLength(
            &self,
            download: &NSURLDownload,
            length: NSUInteger,
        );

        #[optional]
        #[method(download:shouldDecodeSourceDataOfMIMEType:)]
        pub unsafe fn download_shouldDecodeSourceDataOfMIMEType(
            &self,
            download: &NSURLDownload,
            encodingType: &NSString,
        ) -> bool;

        #[optional]
        #[method(download:decideDestinationWithSuggestedFilename:)]
        pub unsafe fn download_decideDestinationWithSuggestedFilename(
            &self,
            download: &NSURLDownload,
            filename: &NSString,
        );

        #[optional]
        #[method(download:didCreateDestination:)]
        pub unsafe fn download_didCreateDestination(
            &self,
            download: &NSURLDownload,
            path: &NSString,
        );

        #[optional]
        #[method(downloadDidFinish:)]
        pub unsafe fn downloadDidFinish(&self, download: &NSURLDownload);

        #[optional]
        #[method(download:didFailWithError:)]
        pub unsafe fn download_didFailWithError(&self, download: &NSURLDownload, error: &NSError);
    }
);
