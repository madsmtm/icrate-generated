//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSURLConnection;

    unsafe impl ClassType for NSURLConnection {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSURLConnection {
        #[method_id(@__retain_semantics Init initWithRequest:delegate:startImmediately:)]
        pub unsafe fn initWithRequest_delegate_startImmediately(
            this: Option<Allocated<Self>>,
            request: &NSURLRequest,
            delegate: Option<&Object>,
            startImmediately: bool,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithRequest:delegate:)]
        pub unsafe fn initWithRequest_delegate(
            this: Option<Allocated<Self>>,
            request: &NSURLRequest,
            delegate: Option<&Object>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other connectionWithRequest:delegate:)]
        pub unsafe fn connectionWithRequest_delegate(
            request: &NSURLRequest,
            delegate: Option<&Object>,
        ) -> Option<Id<NSURLConnection, Shared>>;

        #[method_id(@__retain_semantics Other originalRequest)]
        pub unsafe fn originalRequest(&self) -> Id<NSURLRequest, Shared>;

        #[method_id(@__retain_semantics Other currentRequest)]
        pub unsafe fn currentRequest(&self) -> Id<NSURLRequest, Shared>;

        #[method(start)]
        pub unsafe fn start(&self);

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(unscheduleFromRunLoop:forMode:)]
        pub unsafe fn unscheduleFromRunLoop_forMode(
            &self,
            aRunLoop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[method(setDelegateQueue:)]
        pub unsafe fn setDelegateQueue(&self, queue: Option<&NSOperationQueue>);

        #[method(canHandleRequest:)]
        pub unsafe fn canHandleRequest(request: &NSURLRequest) -> bool;
    }
);

extern_protocol!(
    pub struct NSURLConnectionDelegate;

    unsafe impl ProtocolType for NSURLConnectionDelegate {
        #[optional]
        #[method(connection:didFailWithError:)]
        pub unsafe fn connection_didFailWithError(
            &self,
            connection: &NSURLConnection,
            error: &NSError,
        );

        #[optional]
        #[method(connectionShouldUseCredentialStorage:)]
        pub unsafe fn connectionShouldUseCredentialStorage(
            &self,
            connection: &NSURLConnection,
        ) -> bool;

        #[optional]
        #[method(connection:willSendRequestForAuthenticationChallenge:)]
        pub unsafe fn connection_willSendRequestForAuthenticationChallenge(
            &self,
            connection: &NSURLConnection,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[optional]
        #[method(connection:canAuthenticateAgainstProtectionSpace:)]
        pub unsafe fn connection_canAuthenticateAgainstProtectionSpace(
            &self,
            connection: &NSURLConnection,
            protectionSpace: &NSURLProtectionSpace,
        ) -> bool;

        #[optional]
        #[method(connection:didReceiveAuthenticationChallenge:)]
        pub unsafe fn connection_didReceiveAuthenticationChallenge(
            &self,
            connection: &NSURLConnection,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[optional]
        #[method(connection:didCancelAuthenticationChallenge:)]
        pub unsafe fn connection_didCancelAuthenticationChallenge(
            &self,
            connection: &NSURLConnection,
            challenge: &NSURLAuthenticationChallenge,
        );
    }
);

extern_protocol!(
    pub struct NSURLConnectionDataDelegate;

    unsafe impl ProtocolType for NSURLConnectionDataDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other connection:willSendRequest:redirectResponse:)]
        pub unsafe fn connection_willSendRequest_redirectResponse(
            &self,
            connection: &NSURLConnection,
            request: &NSURLRequest,
            response: Option<&NSURLResponse>,
        ) -> Option<Id<NSURLRequest, Shared>>;

        #[optional]
        #[method(connection:didReceiveResponse:)]
        pub unsafe fn connection_didReceiveResponse(
            &self,
            connection: &NSURLConnection,
            response: &NSURLResponse,
        );

        #[optional]
        #[method(connection:didReceiveData:)]
        pub unsafe fn connection_didReceiveData(&self, connection: &NSURLConnection, data: &NSData);

        #[optional]
        #[method_id(@__retain_semantics Other connection:needNewBodyStream:)]
        pub unsafe fn connection_needNewBodyStream(
            &self,
            connection: &NSURLConnection,
            request: &NSURLRequest,
        ) -> Option<Id<NSInputStream, Shared>>;

        #[optional]
        #[method(connection:didSendBodyData:totalBytesWritten:totalBytesExpectedToWrite:)]
        pub unsafe fn connection_didSendBodyData_totalBytesWritten_totalBytesExpectedToWrite(
            &self,
            connection: &NSURLConnection,
            bytesWritten: NSInteger,
            totalBytesWritten: NSInteger,
            totalBytesExpectedToWrite: NSInteger,
        );

        #[optional]
        #[method_id(@__retain_semantics Other connection:willCacheResponse:)]
        pub unsafe fn connection_willCacheResponse(
            &self,
            connection: &NSURLConnection,
            cachedResponse: &NSCachedURLResponse,
        ) -> Option<Id<NSCachedURLResponse, Shared>>;

        #[optional]
        #[method(connectionDidFinishLoading:)]
        pub unsafe fn connectionDidFinishLoading(&self, connection: &NSURLConnection);
    }
);

extern_protocol!(
    pub struct NSURLConnectionDownloadDelegate;

    unsafe impl ProtocolType for NSURLConnectionDownloadDelegate {
        #[optional]
        #[method(connection:didWriteData:totalBytesWritten:expectedTotalBytes:)]
        pub unsafe fn connection_didWriteData_totalBytesWritten_expectedTotalBytes(
            &self,
            connection: &NSURLConnection,
            bytesWritten: c_longlong,
            totalBytesWritten: c_longlong,
            expectedTotalBytes: c_longlong,
        );

        #[optional]
        #[method(connectionDidResumeDownloading:totalBytesWritten:expectedTotalBytes:)]
        pub unsafe fn connectionDidResumeDownloading_totalBytesWritten_expectedTotalBytes(
            &self,
            connection: &NSURLConnection,
            totalBytesWritten: c_longlong,
            expectedTotalBytes: c_longlong,
        );

        #[method(connectionDidFinishDownloading:destinationURL:)]
        pub unsafe fn connectionDidFinishDownloading_destinationURL(
            &self,
            connection: &NSURLConnection,
            destinationURL: &NSURL,
        );
    }
);

extern_methods!(
    /// NSURLConnectionSynchronousLoading
    unsafe impl NSURLConnection {
        #[method_id(@__retain_semantics Other sendSynchronousRequest:returningResponse:error:_)]
        pub unsafe fn sendSynchronousRequest_returningResponse_error(
            request: &NSURLRequest,
            response: *mut *mut NSURLResponse,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// NSURLConnectionQueuedLoading
    unsafe impl NSURLConnection {
        #[method(sendAsynchronousRequest:queue:completionHandler:)]
        pub unsafe fn sendAsynchronousRequest_queue_completionHandler(
            request: &NSURLRequest,
            queue: &NSOperationQueue,
            handler: &Block<(*mut NSURLResponse, *mut NSData, *mut NSError), ()>,
        );
    }
);
