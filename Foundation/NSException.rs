//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSGenericException: &'static NSExceptionName);

extern_static!(NSRangeException: &'static NSExceptionName);

extern_static!(NSInvalidArgumentException: &'static NSExceptionName);

extern_static!(NSInternalInconsistencyException: &'static NSExceptionName);

extern_static!(NSMallocException: &'static NSExceptionName);

extern_static!(NSObjectInaccessibleException: &'static NSExceptionName);

extern_static!(NSObjectNotAvailableException: &'static NSExceptionName);

extern_static!(NSDestinationInvalidException: &'static NSExceptionName);

extern_static!(NSPortTimeoutException: &'static NSExceptionName);

extern_static!(NSInvalidSendPortException: &'static NSExceptionName);

extern_static!(NSInvalidReceivePortException: &'static NSExceptionName);

extern_static!(NSPortSendException: &'static NSExceptionName);

extern_static!(NSPortReceiveException: &'static NSExceptionName);

extern_static!(NSOldStyleException: &'static NSExceptionName);

extern_static!(NSInconsistentArchiveException: &'static NSExceptionName);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSException")]
    pub struct NSException;

    #[cfg(feature = "Foundation_NSException")]
    unsafe impl ClassType for NSException {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSException")]
    unsafe impl NSException {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other exceptionWithName:reason:userInfo:)]
        pub unsafe fn exceptionWithName_reason_userInfo(
            name: &NSExceptionName,
            reason: Option<&NSString>,
            userInfo: Option<&NSDictionary>,
        ) -> Id<NSException, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithName:reason:userInfo:)]
        pub unsafe fn initWithName_reason_userInfo(
            this: Option<Allocated<Self>>,
            aName: &NSExceptionName,
            aReason: Option<&NSString>,
            aUserInfo: Option<&NSDictionary>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<NSExceptionName, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other reason)]
        pub fn reason(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other callStackReturnAddresses)]
        pub unsafe fn callStackReturnAddresses(&self) -> Id<NSArray<NSNumber>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other callStackSymbols)]
        pub unsafe fn callStackSymbols(&self) -> Id<NSArray<NSString>, Shared>;
    }
);

extern_methods!(
    /// NSExceptionRaisingConveniences
    #[cfg(feature = "Foundation_NSException")]
    unsafe impl NSException {}
);

pub type NSUncaughtExceptionHandler = TodoFunction;

extern_fn!(
    pub unsafe fn NSGetUncaughtExceptionHandler() -> *mut NSUncaughtExceptionHandler;
);

extern_fn!(
    pub unsafe fn NSSetUncaughtExceptionHandler(_: *mut NSUncaughtExceptionHandler);
);

extern_static!(NSAssertionHandlerKey: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSAssertionHandler")]
    pub struct NSAssertionHandler;

    #[cfg(feature = "Foundation_NSAssertionHandler")]
    unsafe impl ClassType for NSAssertionHandler {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSAssertionHandler")]
    unsafe impl NSAssertionHandler {
        #[method_id(@__retain_semantics Other currentHandler)]
        pub unsafe fn currentHandler() -> Id<NSAssertionHandler, Shared>;
    }
);
