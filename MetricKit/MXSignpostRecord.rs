//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXSignpostRecord;

    unsafe impl ClassType for MXSignpostRecord {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for MXSignpostRecord {}

unsafe impl NSObjectProtocol for MXSignpostRecord {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for MXSignpostRecord {}

extern_methods!(
    unsafe impl MXSignpostRecord {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subsystem)]
        pub unsafe fn subsystem(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other beginTimeStamp)]
        pub unsafe fn beginTimeStamp(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other endTimeStamp)]
        pub unsafe fn endTimeStamp(&self) -> Option<Id<NSDate>>;

        #[cfg(all(feature = "Foundation_NSMeasurement", feature = "Foundation_NSUnit"))]
        #[method_id(@__retain_semantics Other duration)]
        pub unsafe fn duration(&self) -> Option<Id<NSMeasurement<NSUnitDuration>>>;

        #[method(isInterval)]
        pub unsafe fn isInterval(&self) -> bool;

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
    unsafe impl MXSignpostRecord {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
