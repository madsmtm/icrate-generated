//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSHTTPCookieAcceptPolicy(pub NSUInteger);
impl NSHTTPCookieAcceptPolicy {
    #[doc(alias = "NSHTTPCookieAcceptPolicyAlways")]
    pub const Always: Self = Self(0);
    #[doc(alias = "NSHTTPCookieAcceptPolicyNever")]
    pub const Never: Self = Self(1);
    #[doc(alias = "NSHTTPCookieAcceptPolicyOnlyFromMainDocumentDomain")]
    pub const OnlyFromMainDocumentDomain: Self = Self(2);
}

unsafe impl Encode for NSHTTPCookieAcceptPolicy {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSHTTPCookieAcceptPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSHTTPCookieStorage;

    unsafe impl ClassType for NSHTTPCookieStorage {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSHTTPCookieStorage {}

unsafe impl Sync for NSHTTPCookieStorage {}

unsafe impl NSObjectProtocol for NSHTTPCookieStorage {}

extern_methods!(
    unsafe impl NSHTTPCookieStorage {
        #[method_id(@__retain_semantics Other sharedHTTPCookieStorage)]
        pub unsafe fn sharedHTTPCookieStorage() -> Id<NSHTTPCookieStorage>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sharedCookieStorageForGroupContainerIdentifier:)]
        pub unsafe fn sharedCookieStorageForGroupContainerIdentifier(
            identifier: &NSString,
        ) -> Id<NSHTTPCookieStorage>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSHTTPCookie"))]
        #[method_id(@__retain_semantics Other cookies)]
        pub unsafe fn cookies(&self) -> Option<Id<NSArray<NSHTTPCookie>>>;

        #[cfg(feature = "Foundation_NSHTTPCookie")]
        #[method(setCookie:)]
        pub unsafe fn setCookie(&self, cookie: &NSHTTPCookie);

        #[cfg(feature = "Foundation_NSHTTPCookie")]
        #[method(deleteCookie:)]
        pub unsafe fn deleteCookie(&self, cookie: &NSHTTPCookie);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(removeCookiesSinceDate:)]
        pub unsafe fn removeCookiesSinceDate(&self, date: &NSDate);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other cookiesForURL:)]
        pub unsafe fn cookiesForURL(&self, url: &NSURL) -> Option<Id<NSArray<NSHTTPCookie>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURL"
        ))]
        #[method(setCookies:forURL:mainDocumentURL:)]
        pub unsafe fn setCookies_forURL_mainDocumentURL(
            &self,
            cookies: &NSArray<NSHTTPCookie>,
            url: Option<&NSURL>,
            main_document_url: Option<&NSURL>,
        );

        #[method(cookieAcceptPolicy)]
        pub unsafe fn cookieAcceptPolicy(&self) -> NSHTTPCookieAcceptPolicy;

        #[method(setCookieAcceptPolicy:)]
        pub unsafe fn setCookieAcceptPolicy(&self, cookie_accept_policy: NSHTTPCookieAcceptPolicy);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortedCookiesUsingDescriptors:)]
        pub unsafe fn sortedCookiesUsingDescriptors(
            &self,
            sort_order: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<NSHTTPCookie>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSHTTPCookieStorage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSURLSessionTaskAdditions
    unsafe impl NSHTTPCookieStorage {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURLSession"
        ))]
        #[method(storeCookies:forTask:)]
        pub unsafe fn storeCookies_forTask(
            &self,
            cookies: &NSArray<NSHTTPCookie>,
            task: &NSURLSessionTask,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURLSession",
            feature = "block2"
        ))]
        #[method(getCookiesForTask:completionHandler:)]
        pub unsafe fn getCookiesForTask_completionHandler(
            &self,
            task: &NSURLSessionTask,
            completion_handler: &Block<dyn Fn(*mut NSArray<NSHTTPCookie>)>,
        );
    }
);

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static NSHTTPCookieManagerAcceptPolicyChangedNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static NSHTTPCookieManagerCookiesChangedNotification: &'static NSNotificationName;
}
