//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[deprecated = "Use MKLocalSearchCompleterResultType"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKSearchCompletionFilterType(pub NSInteger);
impl MKSearchCompletionFilterType {
    #[deprecated = "Use MKLocalSearchCompleterResultType"]
    #[doc(alias = "MKSearchCompletionFilterTypeLocationsAndQueries")]
    pub const LocationsAndQueries: Self = Self(0);
    #[deprecated = "Use MKLocalSearchCompleterResultType"]
    #[doc(alias = "MKSearchCompletionFilterTypeLocationsOnly")]
    pub const LocationsOnly: Self = Self(1);
}

unsafe impl Encode for MKSearchCompletionFilterType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MKSearchCompletionFilterType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKLocalSearchCompleterResultType(pub NSUInteger);
bitflags::bitflags! {
    impl MKLocalSearchCompleterResultType: NSUInteger {
        #[doc(alias = "MKLocalSearchCompleterResultTypeAddress")]
        const Address = 1<<0;
        #[doc(alias = "MKLocalSearchCompleterResultTypePointOfInterest")]
        const PointOfInterest = 1<<1;
        #[doc(alias = "MKLocalSearchCompleterResultTypeQuery")]
        const Query = 1<<2;
        #[doc(alias = "MKLocalSearchCompleterResultTypePhysicalFeature")]
        const PhysicalFeature = 1<<3;
    }
}

unsafe impl Encode for MKLocalSearchCompleterResultType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MKLocalSearchCompleterResultType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLocalSearchCompleter;

    unsafe impl ClassType for MKLocalSearchCompleter {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MKLocalSearchCompleter {}

extern_methods!(
    unsafe impl MKLocalSearchCompleter {
        #[method_id(@__retain_semantics Other queryFragment)]
        pub unsafe fn queryFragment(&self) -> Retained<NSString>;

        #[method(setQueryFragment:)]
        pub unsafe fn setQueryFragment(&self, query_fragment: &NSString);

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

        #[cfg(feature = "MKTypes")]
        #[method(regionPriority)]
        pub unsafe fn regionPriority(&self) -> MKLocalSearchRegionPriority;

        #[cfg(feature = "MKTypes")]
        #[method(setRegionPriority:)]
        pub unsafe fn setRegionPriority(&self, region_priority: MKLocalSearchRegionPriority);

        #[deprecated = "Use resultTypes"]
        #[method(filterType)]
        pub unsafe fn filterType(&self) -> MKSearchCompletionFilterType;

        #[deprecated = "Use resultTypes"]
        #[method(setFilterType:)]
        pub unsafe fn setFilterType(&self, filter_type: MKSearchCompletionFilterType);

        #[method(resultTypes)]
        pub unsafe fn resultTypes(&self) -> MKLocalSearchCompleterResultType;

        #[method(setResultTypes:)]
        pub unsafe fn setResultTypes(&self, result_types: MKLocalSearchCompleterResultType);

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Retained<MKPointOfInterestFilter>>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[cfg(feature = "MKAddressFilter")]
        #[method_id(@__retain_semantics Other addressFilter)]
        pub unsafe fn addressFilter(&self) -> Option<Retained<MKAddressFilter>>;

        #[cfg(feature = "MKAddressFilter")]
        #[method(setAddressFilter:)]
        pub unsafe fn setAddressFilter(&self, address_filter: Option<&MKAddressFilter>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MKLocalSearchCompleterDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn MKLocalSearchCompleterDelegate>>,
        );

        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Retained<NSArray<MKLocalSearchCompletion>>;

        #[method(isSearching)]
        pub unsafe fn isSearching(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKLocalSearchCompleter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MKLocalSearchCompleterDelegate: NSObjectProtocol {
        #[optional]
        #[method(completerDidUpdateResults:)]
        unsafe fn completerDidUpdateResults(&self, completer: &MKLocalSearchCompleter);

        #[optional]
        #[method(completer:didFailWithError:)]
        unsafe fn completer_didFailWithError(
            &self,
            completer: &MKLocalSearchCompleter,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn MKLocalSearchCompleterDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLocalSearchCompletion;

    unsafe impl ClassType for MKLocalSearchCompletion {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MKLocalSearchCompletion {}

extern_methods!(
    unsafe impl MKLocalSearchCompletion {
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other titleHighlightRanges)]
        pub unsafe fn titleHighlightRanges(&self) -> Retained<NSArray<NSValue>>;

        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other subtitleHighlightRanges)]
        pub unsafe fn subtitleHighlightRanges(&self) -> Retained<NSArray<NSValue>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKLocalSearchCompletion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "MKLocalSearchRequest")]
    unsafe impl MKLocalSearchRequest {
        #[method_id(@__retain_semantics Init initWithCompletion:)]
        pub unsafe fn initWithCompletion(
            this: Allocated<Self>,
            completion: &MKLocalSearchCompletion,
        ) -> Retained<Self>;
    }
);
