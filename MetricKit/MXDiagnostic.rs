//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXDiagnostic;

    unsafe impl ClassType for MXDiagnostic {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for MXDiagnostic {}

unsafe impl NSObjectProtocol for MXDiagnostic {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for MXDiagnostic {}

extern_methods!(
    unsafe impl MXDiagnostic {
        #[cfg(feature = "MetricKit_MXMetaData")]
        #[method_id(@__retain_semantics Other metaData)]
        pub unsafe fn metaData(&self) -> Id<MXMetaData>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other applicationVersion)]
        pub unsafe fn applicationVersion(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MetricKit_MXSignpostRecord"))]
        #[method_id(@__retain_semantics Other signpostData)]
        pub unsafe fn signpostData(&self) -> Option<Id<NSArray<MXSignpostRecord>>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Id<NSDictionary>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MXDiagnostic {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
