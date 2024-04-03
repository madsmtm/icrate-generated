//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSConnection;

    unsafe impl ClassType for NSConnection {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSConnection {}

extern_methods!(
    unsafe impl NSConnection {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSValue"
        ))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other statistics)]
        pub unsafe fn statistics(&self) -> Id<NSDictionary<NSString, NSNumber>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other allConnections)]
        pub unsafe fn allConnections() -> Id<NSArray<NSConnection>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other defaultConnection)]
        pub unsafe fn defaultConnection() -> Id<NSConnection>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other connectionWithRegisteredName:host:)]
        pub unsafe fn connectionWithRegisteredName_host(
            name: &NSString,
            host_name: Option<&NSString>,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other connectionWithRegisteredName:host:usingNameServer:)]
        pub unsafe fn connectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            host_name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSDistantObject",
            feature = "Foundation_NSProxy",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other rootProxyForConnectionWithRegisteredName:host:)]
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host(
            name: &NSString,
            host_name: Option<&NSString>,
        ) -> Option<Id<NSDistantObject>>;

        #[cfg(all(
            feature = "Foundation_NSDistantObject",
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSProxy",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other rootProxyForConnectionWithRegisteredName:host:usingNameServer:)]
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            host_name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Id<NSDistantObject>>;

        #[cfg(all(
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other serviceConnectionWithName:rootObject:usingNameServer:)]
        pub unsafe fn serviceConnectionWithName_rootObject_usingNameServer(
            name: &NSString,
            root: &AnyObject,
            server: &NSPortNameServer,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other serviceConnectionWithName:rootObject:)]
        pub unsafe fn serviceConnectionWithName_rootObject(
            name: &NSString,
            root: &AnyObject,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(requestTimeout)]
        pub unsafe fn requestTimeout(&self) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSDate")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setRequestTimeout:)]
        pub unsafe fn setRequestTimeout(&self, request_timeout: NSTimeInterval);

        #[cfg(feature = "Foundation_NSDate")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(replyTimeout)]
        pub unsafe fn replyTimeout(&self) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSDate")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setReplyTimeout:)]
        pub unsafe fn setReplyTimeout(&self, reply_timeout: NSTimeInterval);

        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other rootObject)]
        pub unsafe fn rootObject(&self) -> Option<Id<AnyObject>>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setRootObject:)]
        pub unsafe fn setRootObject(&self, root_object: Option<&AnyObject>);

        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSConnectionDelegate>>>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSConnectionDelegate>>,
        );

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(independentConversationQueueing)]
        pub unsafe fn independentConversationQueueing(&self) -> bool;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setIndependentConversationQueueing:)]
        pub unsafe fn setIndependentConversationQueueing(
            &self,
            independent_conversation_queueing: bool,
        );

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSDistantObject", feature = "Foundation_NSProxy"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other rootProxy)]
        pub unsafe fn rootProxy(&self) -> Id<NSDistantObject>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(addRequestMode:)]
        pub unsafe fn addRequestMode(&self, rmode: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(removeRequestMode:)]
        pub unsafe fn removeRequestMode(&self, rmode: &NSString);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other requestModes)]
        pub unsafe fn requestModes(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerName:)]
        pub unsafe fn registerName(&self, name: Option<&NSString>) -> bool;

        #[cfg(all(
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerName:withNameServer:)]
        pub unsafe fn registerName_withNameServer(
            &self,
            name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> bool;

        #[cfg(feature = "Foundation_NSPort")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other connectionWithReceivePort:sendPort:)]
        pub unsafe fn connectionWithReceivePort_sendPort(
            receive_port: Option<&NSPort>,
            send_port: Option<&NSPort>,
        ) -> Option<Id<Self>>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other currentConversation)]
        pub unsafe fn currentConversation() -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSPort")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Init initWithReceivePort:sendPort:)]
        pub unsafe fn initWithReceivePort_sendPort(
            this: Allocated<Self>,
            receive_port: Option<&NSPort>,
            send_port: Option<&NSPort>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSPort")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other sendPort)]
        pub unsafe fn sendPort(&self) -> Id<NSPort>;

        #[cfg(feature = "Foundation_NSPort")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other receivePort)]
        pub unsafe fn receivePort(&self) -> Id<NSPort>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(enableMultipleThreads)]
        pub unsafe fn enableMultipleThreads(&self);

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(multipleThreadsEnabled)]
        pub unsafe fn multipleThreadsEnabled(&self) -> bool;

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(addRunLoop:)]
        pub unsafe fn addRunLoop(&self, runloop: &NSRunLoop);

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(removeRunLoop:)]
        pub unsafe fn removeRunLoop(&self, runloop: &NSRunLoop);

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(runInNewThread)]
        pub unsafe fn runInNewThread(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other remoteObjects)]
        pub unsafe fn remoteObjects(&self) -> Id<NSArray>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other localObjects)]
        pub unsafe fn localObjects(&self) -> Id<NSArray>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(dispatchWithComponents:)]
        pub unsafe fn dispatchWithComponents(&self, components: &NSArray);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSConnection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSConnectionReplyMode: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSConnectionDidDieNotification: &'static NSString;
}

extern_protocol!(
    #[deprecated = "Use NSXPCConnection instead"]
    pub unsafe trait NSConnectionDelegate: NSObjectProtocol {
        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method(makeNewConnection:sender:)]
        unsafe fn makeNewConnection_sender(
            &self,
            conn: &NSConnection,
            ancestor: &NSConnection,
        ) -> bool;

        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method(connection:shouldMakeNewConnection:)]
        unsafe fn connection_shouldMakeNewConnection(
            &self,
            ancestor: &NSConnection,
            conn: &NSConnection,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method_id(@__retain_semantics Other authenticationDataForComponents:)]
        unsafe fn authenticationDataForComponents(&self, components: &NSArray) -> Id<NSData>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method(authenticateComponents:withData:)]
        unsafe fn authenticateComponents_withData(
            &self,
            components: &NSArray,
            signature: &NSData,
        ) -> bool;

        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method_id(@__retain_semantics Other createConversationForConnection:)]
        unsafe fn createConversationForConnection(&self, conn: &NSConnection) -> Id<AnyObject>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method(connection:handleRequest:)]
        unsafe fn connection_handleRequest(
            &self,
            connection: &NSConnection,
            doreq: &NSDistantObjectRequest,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSConnectionDelegate {}
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSFailedAuthenticationException: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSConnectionDidInitializeNotification: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSDistantObjectRequest;

    unsafe impl ClassType for NSDistantObjectRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSDistantObjectRequest {}

extern_methods!(
    unsafe impl NSDistantObjectRequest {
        #[cfg(feature = "Foundation_NSInvocation")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other invocation)]
        pub unsafe fn invocation(&self) -> Id<NSInvocation>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other connection)]
        pub unsafe fn connection(&self) -> Id<NSConnection>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other conversation)]
        pub unsafe fn conversation(&self) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSException")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(replyWithException:)]
        pub unsafe fn replyWithException(&self, exception: Option<&NSException>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDistantObjectRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
