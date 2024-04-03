//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MapKit_MKMultiPoint", feature = "MapKit_MKShape"))]
    pub struct MKPolygon;

    #[cfg(all(feature = "MapKit_MKMultiPoint", feature = "MapKit_MKShape"))]
    unsafe impl ClassType for MKPolygon {
        #[inherits(MKShape, NSObject)]
        type Super = MKMultiPoint;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "MapKit_MKAnnotation",
    feature = "MapKit_MKMultiPoint",
    feature = "MapKit_MKShape"
))]
unsafe impl MKAnnotation for MKPolygon {}

#[cfg(all(
    feature = "MapKit_MKAnnotation",
    feature = "MapKit_MKMultiPoint",
    feature = "MapKit_MKOverlay",
    feature = "MapKit_MKShape"
))]
unsafe impl MKOverlay for MKPolygon {}

#[cfg(all(feature = "MapKit_MKMultiPoint", feature = "MapKit_MKShape"))]
unsafe impl NSObjectProtocol for MKPolygon {}

extern_methods!(
    #[cfg(all(feature = "MapKit_MKMultiPoint", feature = "MapKit_MKShape"))]
    unsafe impl MKPolygon {
        #[cfg(feature = "MapKit_MKGeometry")]
        #[method_id(@__retain_semantics Other polygonWithPoints:count:)]
        pub unsafe fn polygonWithPoints_count(
            points: NonNull<MKMapPoint>,
            count: NSUInteger,
        ) -> Id<Self>;

        #[cfg(feature = "MapKit_MKGeometry")]
        #[method_id(@__retain_semantics Other polygonWithPoints:count:interiorPolygons:)]
        pub unsafe fn polygonWithPoints_count_interiorPolygons(
            points: NonNull<MKMapPoint>,
            count: NSUInteger,
            interior_polygons: Option<&NSArray<MKPolygon>>,
        ) -> Id<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other polygonWithCoordinates:count:)]
        pub unsafe fn polygonWithCoordinates_count(
            coords: NonNull<CLLocationCoordinate2D>,
            count: NSUInteger,
        ) -> Id<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other polygonWithCoordinates:count:interiorPolygons:)]
        pub unsafe fn polygonWithCoordinates_count_interiorPolygons(
            coords: NonNull<CLLocationCoordinate2D>,
            count: NSUInteger,
            interior_polygons: Option<&NSArray<MKPolygon>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other interiorPolygons)]
        pub unsafe fn interiorPolygons(&self) -> Option<Id<NSArray<MKPolygon>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MapKit_MKMultiPoint", feature = "MapKit_MKShape"))]
    unsafe impl MKPolygon {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
