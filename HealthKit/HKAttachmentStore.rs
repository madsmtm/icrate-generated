//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKAttachmentStore;

    unsafe impl ClassType for HKAttachmentStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for HKAttachmentStore {}

extern_methods!(
    unsafe impl HKAttachmentStore {
        #[cfg(feature = "HealthKit_HKHealthStore")]
        #[method_id(@__retain_semantics Init initWithHealthStore:)]
        pub unsafe fn initWithHealthStore(
            this: Allocated<Self>,
            health_store: &HKHealthStore,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL",
            feature = "HealthKit_HKAttachment",
            feature = "HealthKit_HKObject",
            feature = "UniformTypeIdentifiers_UTType"
        ))]
        #[method(addAttachmentToObject:name:contentType:URL:metadata:completion:)]
        pub unsafe fn addAttachmentToObject_name_contentType_URL_metadata_completion(
            &self,
            object: &HKObject,
            name: &NSString,
            content_type: &UTType,
            url: &NSURL,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
            completion: &Block<dyn Fn(*mut HKAttachment, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKAttachment",
            feature = "HealthKit_HKObject"
        ))]
        #[method(removeAttachment:fromObject:completion:)]
        pub unsafe fn removeAttachment_fromObject_completion(
            &self,
            attachment: &HKAttachment,
            object: &HKObject,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKAttachment",
            feature = "HealthKit_HKObject"
        ))]
        #[method(getAttachmentsForObject:completion:)]
        pub unsafe fn getAttachmentsForObject_completion(
            &self,
            object: &HKObject,
            completion: &Block<dyn Fn(*mut NSArray<HKAttachment>, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "HealthKit_HKAttachment"
        ))]
        #[method_id(@__retain_semantics Other getDataForAttachment:completion:)]
        pub unsafe fn getDataForAttachment_completion(
            &self,
            attachment: &HKAttachment,
            completion: &Block<dyn Fn(*mut NSData, *mut NSError)>,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "HealthKit_HKAttachment"
        ))]
        #[method_id(@__retain_semantics Other streamDataForAttachment:dataHandler:)]
        pub unsafe fn streamDataForAttachment_dataHandler(
            &self,
            attachment: &HKAttachment,
            data_handler: &Block<dyn Fn(*mut NSData, *mut NSError, Bool)>,
        ) -> Id<NSProgress>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKAttachmentStore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
