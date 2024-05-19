//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline", feature = "MKShape"))]
    pub struct MKGeodesicPolyline;

    #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline", feature = "MKShape"))]
    unsafe impl ClassType for MKGeodesicPolyline {
        #[inherits(MKMultiPoint, MKShape, NSObject)]
        type Super = MKPolyline;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "MKAnnotation",
    feature = "MKMultiPoint",
    feature = "MKPolyline",
    feature = "MKShape"
))]
unsafe impl MKAnnotation for MKGeodesicPolyline {}

#[cfg(all(
    feature = "MKAnnotation",
    feature = "MKMultiPoint",
    feature = "MKOverlay",
    feature = "MKPolyline",
    feature = "MKShape"
))]
unsafe impl MKOverlay for MKGeodesicPolyline {}

#[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline", feature = "MKShape"))]
unsafe impl NSObjectProtocol for MKGeodesicPolyline {}

extern_methods!(
    #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline", feature = "MKShape"))]
    unsafe impl MKGeodesicPolyline {
        #[cfg(feature = "MKGeometry")]
        #[method_id(@__retain_semantics Other polylineWithPoints:count:)]
        pub unsafe fn polylineWithPoints_count(
            points: NonNull<MKMapPoint>,
            count: NSUInteger,
        ) -> Id<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other polylineWithCoordinates:count:)]
        pub unsafe fn polylineWithCoordinates_count(
            coords: NonNull<CLLocationCoordinate2D>,
            count: NSUInteger,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline", feature = "MKShape"))]
    unsafe impl MKGeodesicPolyline {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
