//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "HealthKit_HKDocumentSample",
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample"
    ))]
    pub struct HKCDADocumentSample;

    #[cfg(all(
        feature = "HealthKit_HKDocumentSample",
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample"
    ))]
    unsafe impl ClassType for HKCDADocumentSample {
        #[inherits(HKSample, HKObject, NSObject)]
        type Super = HKDocumentSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKDocumentSample",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample"
))]
unsafe impl NSCoding for HKCDADocumentSample {}

#[cfg(all(
    feature = "HealthKit_HKDocumentSample",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample"
))]
unsafe impl NSObjectProtocol for HKCDADocumentSample {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKDocumentSample",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample"
))]
unsafe impl NSSecureCoding for HKCDADocumentSample {}

extern_methods!(
    #[cfg(all(
        feature = "HealthKit_HKDocumentSample",
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample"
    ))]
    unsafe impl HKCDADocumentSample {
        #[method_id(@__retain_semantics Other document)]
        pub unsafe fn document(&self) -> Option<Id<HKCDADocument>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other CDADocumentSampleWithData:startDate:endDate:metadata:validationError:_)]
        pub unsafe fn CDADocumentSampleWithData_startDate_endDate_metadata_validationError(
            document_data: &NSData,
            start_date: &NSDate,
            end_date: &NSDate,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Id<Self>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(all(
        feature = "HealthKit_HKDocumentSample",
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample"
    ))]
    unsafe impl HKCDADocumentSample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "HealthKit_HKDocumentSample",
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample"
    ))]
    unsafe impl HKCDADocumentSample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKCDADocument;

    unsafe impl ClassType for HKCDADocument {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for HKCDADocument {}

extern_methods!(
    unsafe impl HKCDADocument {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other documentData)]
        pub unsafe fn documentData(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other patientName)]
        pub unsafe fn patientName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other authorName)]
        pub unsafe fn authorName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other custodianName)]
        pub unsafe fn custodianName(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKCDADocument {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathCDATitle: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathCDAPatientName: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathCDAAuthorName: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKPredicateKeyPathCDACustodianName: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKDetailedCDAValidationErrorKey: &'static NSString;
}
