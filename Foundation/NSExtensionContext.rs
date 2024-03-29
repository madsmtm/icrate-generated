//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSExtensionContext;

    unsafe impl ClassType for NSExtensionContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSExtensionContext {}

extern_methods!(
    unsafe impl NSExtensionContext {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other inputItems)]
        pub unsafe fn inputItems(&self) -> Id<NSArray>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(completeRequestReturningItems:completionHandler:)]
        pub unsafe fn completeRequestReturningItems_completionHandler(
            &self,
            items: Option<&NSArray>,
            completion_handler: Option<&Block<dyn Fn(Bool)>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(cancelRequestWithError:)]
        pub unsafe fn cancelRequestWithError(&self, error: &NSError);

        #[cfg(feature = "Foundation_NSURL")]
        #[method(openURL:completionHandler:)]
        pub unsafe fn openURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: Option<&Block<dyn Fn(Bool)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSExtensionContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSExtensionItemsAndErrorsKey: Option<&'static NSString>;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSExtensionHostWillEnterForegroundNotification: Option<&'static NSString>;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSExtensionHostDidEnterBackgroundNotification: Option<&'static NSString>;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSExtensionHostWillResignActiveNotification: Option<&'static NSString>;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSExtensionHostDidBecomeActiveNotification: Option<&'static NSString>;
}
