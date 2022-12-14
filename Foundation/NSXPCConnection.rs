//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct NSXPCProxyCreating;

    unsafe impl ProtocolType for NSXPCProxyCreating {
        #[method_id(@__retain_semantics Other remoteObjectProxy)]
        pub unsafe fn remoteObjectProxy(&self) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other remoteObjectProxyWithErrorHandler:)]
        pub unsafe fn remoteObjectProxyWithErrorHandler(
            &self,
            handler: &Block<(NonNull<NSError>,), ()>,
        ) -> Id<Object, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other synchronousRemoteObjectProxyWithErrorHandler:)]
        pub unsafe fn synchronousRemoteObjectProxyWithErrorHandler(
            &self,
            handler: &Block<(NonNull<NSError>,), ()>,
        ) -> Id<Object, Shared>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSXPCConnectionOptions {
        NSXPCConnectionPrivileged = 1 << 12,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSXPCConnection;

    unsafe impl ClassType for NSXPCConnection {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSXPCConnection {
        #[method_id(@__retain_semantics Init initWithServiceName:)]
        pub unsafe fn initWithServiceName(
            this: Option<Allocated<Self>>,
            serviceName: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other serviceName)]
        pub unsafe fn serviceName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Init initWithMachServiceName:options:)]
        pub unsafe fn initWithMachServiceName_options(
            this: Option<Allocated<Self>>,
            name: &NSString,
            options: NSXPCConnectionOptions,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithListenerEndpoint:)]
        pub unsafe fn initWithListenerEndpoint(
            this: Option<Allocated<Self>>,
            endpoint: &NSXPCListenerEndpoint,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other endpoint)]
        pub unsafe fn endpoint(&self) -> Id<NSXPCListenerEndpoint, Shared>;

        #[method_id(@__retain_semantics Other exportedInterface)]
        pub unsafe fn exportedInterface(&self) -> Option<Id<NSXPCInterface, Shared>>;

        #[method(setExportedInterface:)]
        pub unsafe fn setExportedInterface(&self, exportedInterface: Option<&NSXPCInterface>);

        #[method_id(@__retain_semantics Other exportedObject)]
        pub unsafe fn exportedObject(&self) -> Option<Id<Object, Shared>>;

        #[method(setExportedObject:)]
        pub unsafe fn setExportedObject(&self, exportedObject: Option<&Object>);

        #[method_id(@__retain_semantics Other remoteObjectInterface)]
        pub unsafe fn remoteObjectInterface(&self) -> Option<Id<NSXPCInterface, Shared>>;

        #[method(setRemoteObjectInterface:)]
        pub unsafe fn setRemoteObjectInterface(
            &self,
            remoteObjectInterface: Option<&NSXPCInterface>,
        );

        #[method_id(@__retain_semantics Other remoteObjectProxy)]
        pub unsafe fn remoteObjectProxy(&self) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other remoteObjectProxyWithErrorHandler:)]
        pub unsafe fn remoteObjectProxyWithErrorHandler(
            &self,
            handler: &Block<(NonNull<NSError>,), ()>,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other synchronousRemoteObjectProxyWithErrorHandler:)]
        pub unsafe fn synchronousRemoteObjectProxyWithErrorHandler(
            &self,
            handler: &Block<(NonNull<NSError>,), ()>,
        ) -> Id<Object, Shared>;

        #[method(interruptionHandler)]
        pub unsafe fn interruptionHandler(&self) -> *mut Block<(), ()>;

        #[method(setInterruptionHandler:)]
        pub unsafe fn setInterruptionHandler(&self, interruptionHandler: Option<&Block<(), ()>>);

        #[method(invalidationHandler)]
        pub unsafe fn invalidationHandler(&self) -> *mut Block<(), ()>;

        #[method(setInvalidationHandler:)]
        pub unsafe fn setInvalidationHandler(&self, invalidationHandler: Option<&Block<(), ()>>);

        #[method(resume)]
        pub unsafe fn resume(&self);

        #[method(suspend)]
        pub unsafe fn suspend(&self);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method_id(@__retain_semantics Other currentConnection)]
        pub unsafe fn currentConnection() -> Option<Id<NSXPCConnection, Shared>>;

        #[method(scheduleSendBarrierBlock:)]
        pub unsafe fn scheduleSendBarrierBlock(&self, block: &Block<(), ()>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSXPCListener;

    unsafe impl ClassType for NSXPCListener {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSXPCListener {
        #[method_id(@__retain_semantics Other serviceListener)]
        pub unsafe fn serviceListener() -> Id<NSXPCListener, Shared>;

        #[method_id(@__retain_semantics Other anonymousListener)]
        pub unsafe fn anonymousListener() -> Id<NSXPCListener, Shared>;

        #[method_id(@__retain_semantics Init initWithMachServiceName:)]
        pub unsafe fn initWithMachServiceName(
            this: Option<Allocated<Self>>,
            name: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSXPCListenerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSXPCListenerDelegate>);

        #[method_id(@__retain_semantics Other endpoint)]
        pub unsafe fn endpoint(&self) -> Id<NSXPCListenerEndpoint, Shared>;

        #[method(resume)]
        pub unsafe fn resume(&self);

        #[method(suspend)]
        pub unsafe fn suspend(&self);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
    }
);

extern_protocol!(
    pub struct NSXPCListenerDelegate;

    unsafe impl ProtocolType for NSXPCListenerDelegate {
        #[optional]
        #[method(listener:shouldAcceptNewConnection:)]
        pub unsafe fn listener_shouldAcceptNewConnection(
            &self,
            listener: &NSXPCListener,
            newConnection: &NSXPCConnection,
        ) -> bool;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSXPCInterface;

    unsafe impl ClassType for NSXPCInterface {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSXPCInterface {
        #[method_id(@__retain_semantics Other interfaceWithProtocol:)]
        pub unsafe fn interfaceWithProtocol(protocol: &Protocol) -> Id<NSXPCInterface, Shared>;

        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Id<Protocol, Shared>;

        #[method(setProtocol:)]
        pub unsafe fn setProtocol(&self, protocol: &Protocol);

        #[method(setClasses:forSelector:argumentIndex:ofReply:)]
        pub unsafe fn setClasses_forSelector_argumentIndex_ofReply(
            &self,
            classes: &NSSet<TodoClass>,
            sel: Sel,
            arg: NSUInteger,
            ofReply: bool,
        );

        #[method_id(@__retain_semantics Other classesForSelector:argumentIndex:ofReply:)]
        pub unsafe fn classesForSelector_argumentIndex_ofReply(
            &self,
            sel: Sel,
            arg: NSUInteger,
            ofReply: bool,
        ) -> Id<NSSet<TodoClass>, Shared>;

        #[method(setInterface:forSelector:argumentIndex:ofReply:)]
        pub unsafe fn setInterface_forSelector_argumentIndex_ofReply(
            &self,
            ifc: &NSXPCInterface,
            sel: Sel,
            arg: NSUInteger,
            ofReply: bool,
        );

        #[method_id(@__retain_semantics Other interfaceForSelector:argumentIndex:ofReply:)]
        pub unsafe fn interfaceForSelector_argumentIndex_ofReply(
            &self,
            sel: Sel,
            arg: NSUInteger,
            ofReply: bool,
        ) -> Option<Id<NSXPCInterface, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSXPCListenerEndpoint;

    unsafe impl ClassType for NSXPCListenerEndpoint {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSXPCListenerEndpoint {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSXPCCoder;

    unsafe impl ClassType for NSXPCCoder {
        #[inherits(NSObject)]
        type Super = NSCoder;
    }
);

extern_methods!(
    unsafe impl NSXPCCoder {
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSObject, Shared>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&NSObject>);

        #[method_id(@__retain_semantics Other connection)]
        pub unsafe fn connection(&self) -> Option<Id<NSXPCConnection, Shared>>;
    }
);
