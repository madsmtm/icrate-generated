//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKPointOfInterestFilter;

    unsafe impl ClassType for MKPointOfInterestFilter {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for MKPointOfInterestFilter {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MKPointOfInterestFilter {}

unsafe impl NSObjectProtocol for MKPointOfInterestFilter {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for MKPointOfInterestFilter {}

extern_methods!(
    unsafe impl MKPointOfInterestFilter {
        #[method_id(@__retain_semantics Other filterIncludingAllCategories)]
        pub unsafe fn filterIncludingAllCategories() -> Id<MKPointOfInterestFilter>;

        #[method_id(@__retain_semantics Other filterExcludingAllCategories)]
        pub unsafe fn filterExcludingAllCategories() -> Id<MKPointOfInterestFilter>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "MapKit_MKPointOfInterestCategory"
        ))]
        #[method_id(@__retain_semantics Init initIncludingCategories:)]
        pub unsafe fn initIncludingCategories(
            this: Allocated<Self>,
            categories: &NSArray<MKPointOfInterestCategory>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "MapKit_MKPointOfInterestCategory"
        ))]
        #[method_id(@__retain_semantics Init initExcludingCategories:)]
        pub unsafe fn initExcludingCategories(
            this: Allocated<Self>,
            categories: &NSArray<MKPointOfInterestCategory>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "MapKit_MKPointOfInterestCategory"
        ))]
        #[method(includesCategory:)]
        pub unsafe fn includesCategory(&self, category: &MKPointOfInterestCategory) -> bool;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "MapKit_MKPointOfInterestCategory"
        ))]
        #[method(excludesCategory:)]
        pub unsafe fn excludesCategory(&self, category: &MKPointOfInterestCategory) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKPointOfInterestFilter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
