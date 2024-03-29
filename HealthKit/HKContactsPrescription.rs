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
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample",
        feature = "HealthKit_HKVisionPrescription"
    ))]
    pub struct HKContactsPrescription;

    #[cfg(all(
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample",
        feature = "HealthKit_HKVisionPrescription"
    ))]
    unsafe impl ClassType for HKContactsPrescription {
        #[inherits(HKSample, HKObject, NSObject)]
        type Super = HKVisionPrescription;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample",
    feature = "HealthKit_HKVisionPrescription"
))]
unsafe impl NSCoding for HKContactsPrescription {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample",
    feature = "HealthKit_HKVisionPrescription"
))]
unsafe impl NSCopying for HKContactsPrescription {}

#[cfg(all(
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample",
    feature = "HealthKit_HKVisionPrescription"
))]
unsafe impl NSObjectProtocol for HKContactsPrescription {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample",
    feature = "HealthKit_HKVisionPrescription"
))]
unsafe impl NSSecureCoding for HKContactsPrescription {}

extern_methods!(
    #[cfg(all(
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample",
        feature = "HealthKit_HKVisionPrescription"
    ))]
    unsafe impl HKContactsPrescription {
        #[cfg(all(
            feature = "HealthKit_HKContactsLensSpecification",
            feature = "HealthKit_HKLensSpecification"
        ))]
        #[method_id(@__retain_semantics Other rightEye)]
        pub unsafe fn rightEye(&self) -> Option<Id<HKContactsLensSpecification>>;

        #[cfg(all(
            feature = "HealthKit_HKContactsLensSpecification",
            feature = "HealthKit_HKLensSpecification"
        ))]
        #[method_id(@__retain_semantics Other leftEye)]
        pub unsafe fn leftEye(&self) -> Option<Id<HKContactsLensSpecification>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other brand)]
        pub unsafe fn brand(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKContactsLensSpecification",
            feature = "HealthKit_HKDevice",
            feature = "HealthKit_HKLensSpecification"
        ))]
        #[method_id(@__retain_semantics Other prescriptionWithRightEyeSpecification:leftEyeSpecification:brand:dateIssued:expirationDate:device:metadata:)]
        pub unsafe fn prescriptionWithRightEyeSpecification_leftEyeSpecification_brand_dateIssued_expirationDate_device_metadata(
            right_eye_specification: Option<&HKContactsLensSpecification>,
            left_eye_specification: Option<&HKContactsLensSpecification>,
            brand: &NSString,
            date_issued: &NSDate,
            expiration_date: Option<&NSDate>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKDevice"
        ))]
        #[method_id(@__retain_semantics Other prescriptionWithType:dateIssued:expirationDate:device:metadata:)]
        pub unsafe fn prescriptionWithType_dateIssued_expirationDate_device_metadata(
            r#type: HKVisionPrescriptionType,
            date_issued: &NSDate,
            expiration_date: Option<&NSDate>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;
    }
);
