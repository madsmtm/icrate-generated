//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSAppleScriptErrorMessage: &'static NSString);

extern_static!(NSAppleScriptErrorNumber: &'static NSString);

extern_static!(NSAppleScriptErrorAppName: &'static NSString);

extern_static!(NSAppleScriptErrorBriefMessage: &'static NSString);

extern_static!(NSAppleScriptErrorRange: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAppleScript;

    unsafe impl ClassType for NSAppleScript {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAppleScript {
        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            errorInfo: *mut *mut NSDictionary<NSString, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithSource:)]
        pub unsafe fn initWithSource(
            this: Option<Allocated<Self>>,
            source: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Option<Id<NSString, Shared>>;

        #[method(isCompiled)]
        pub unsafe fn isCompiled(&self) -> bool;

        #[method(compileAndReturnError:)]
        pub unsafe fn compileAndReturnError(
            &self,
            errorInfo: *mut *mut NSDictionary<NSString, Object>,
        ) -> bool;

        #[method_id(@__retain_semantics Other executeAndReturnError:)]
        pub unsafe fn executeAndReturnError(
            &self,
            errorInfo: *mut *mut NSDictionary<NSString, Object>,
        ) -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other executeAppleEvent:error:)]
        pub unsafe fn executeAppleEvent_error(
            &self,
            event: &NSAppleEventDescriptor,
            errorInfo: *mut *mut NSDictionary<NSString, Object>,
        ) -> Id<NSAppleEventDescriptor, Shared>;
    }
);
