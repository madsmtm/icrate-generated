//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKWebsiteDataStore;

    unsafe impl ClassType for WKWebsiteDataStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for WKWebsiteDataStore {}

unsafe impl NSObjectProtocol for WKWebsiteDataStore {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for WKWebsiteDataStore {}

extern_methods!(
    unsafe impl WKWebsiteDataStore {
        #[method_id(@__retain_semantics Other defaultDataStore)]
        pub unsafe fn defaultDataStore() -> Id<WKWebsiteDataStore>;

        #[method_id(@__retain_semantics Other nonPersistentDataStore)]
        pub unsafe fn nonPersistentDataStore() -> Id<WKWebsiteDataStore>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(&self) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method(isPersistent)]
        pub unsafe fn isPersistent(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other allWebsiteDataTypes)]
        pub unsafe fn allWebsiteDataTypes() -> Id<NSSet<NSString>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString",
            feature = "WebKit_WKWebsiteDataRecord"
        ))]
        #[method(fetchDataRecordsOfTypes:completionHandler:)]
        pub unsafe fn fetchDataRecordsOfTypes_completionHandler(
            &self,
            data_types: &NSSet<NSString>,
            completion_handler: &Block<dyn Fn(NonNull<NSArray<WKWebsiteDataRecord>>)>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString",
            feature = "WebKit_WKWebsiteDataRecord"
        ))]
        #[method(removeDataOfTypes:forDataRecords:completionHandler:)]
        pub unsafe fn removeDataOfTypes_forDataRecords_completionHandler(
            &self,
            data_types: &NSSet<NSString>,
            data_records: &NSArray<WKWebsiteDataRecord>,
            completion_handler: &Block<dyn Fn()>,
        );

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString"
        ))]
        #[method(removeDataOfTypes:modifiedSince:completionHandler:)]
        pub unsafe fn removeDataOfTypes_modifiedSince_completionHandler(
            &self,
            data_types: &NSSet<NSString>,
            date: &NSDate,
            completion_handler: &Block<dyn Fn()>,
        );

        #[cfg(feature = "WebKit_WKHTTPCookieStore")]
        #[method_id(@__retain_semantics Other httpCookieStore)]
        pub unsafe fn httpCookieStore(&self) -> Id<WKHTTPCookieStore>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSUUID>>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other dataStoreForIdentifier:)]
        pub unsafe fn dataStoreForIdentifier(identifier: &NSUUID) -> Id<WKWebsiteDataStore>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSUUID"))]
        #[method(removeDataStoreForIdentifier:completionHandler:)]
        pub unsafe fn removeDataStoreForIdentifier_completionHandler(
            identifier: &NSUUID,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSUUID"))]
        #[method(fetchAllDataStoreIdentifiers:)]
        pub unsafe fn fetchAllDataStoreIdentifiers(
            completion_handler: &Block<dyn Fn(NonNull<NSArray<NSUUID>>)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKWebsiteDataStore {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new_class() -> Id<Self>;
    }
);
