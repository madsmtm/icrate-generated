//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

#[cfg(feature = "CoreLocation_CLLocation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MKCoordinateSpan {
    pub latitudeDelta: CLLocationDegrees,
    pub longitudeDelta: CLLocationDegrees,
}

#[cfg(feature = "CoreLocation_CLLocation")]
#[cfg(feature = "objc2")]
unsafe impl Encode for MKCoordinateSpan {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[<CLLocationDegrees>::ENCODING, <CLLocationDegrees>::ENCODING],
    );
}

#[cfg(feature = "CoreLocation_CLLocation")]
#[cfg(feature = "objc2")]
unsafe impl RefEncode for MKCoordinateSpan {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "CoreLocation_CLLocation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MKCoordinateRegion {
    pub center: CLLocationCoordinate2D,
    pub span: MKCoordinateSpan,
}

#[cfg(feature = "CoreLocation_CLLocation")]
#[cfg(feature = "objc2")]
unsafe impl Encode for MKCoordinateRegion {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <CLLocationCoordinate2D>::ENCODING,
            <MKCoordinateSpan>::ENCODING,
        ],
    );
}

#[cfg(feature = "CoreLocation_CLLocation")]
#[cfg(feature = "objc2")]
unsafe impl RefEncode for MKCoordinateRegion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn MKCoordinateSpanMake(latitude_delta: CLLocationDegrees,longitude_delta: CLLocationDegrees,) -> MKCoordinateSpan;

// TODO: pub fn MKCoordinateRegionMake(center_coordinate: CLLocationCoordinate2D,span: MKCoordinateSpan,) -> MKCoordinateRegion;

extern "C" {
    #[cfg(feature = "CoreLocation_CLLocation")]
    pub fn MKCoordinateRegionMakeWithDistance(
        center_coordinate: CLLocationCoordinate2D,
        latitudinal_meters: CLLocationDistance,
        longitudinal_meters: CLLocationDistance,
    ) -> MKCoordinateRegion;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MKMapPoint {
    pub x: c_double,
    pub y: c_double,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MKMapPoint {
    const ENCODING: Encoding = Encoding::Struct("?", &[<c_double>::ENCODING, <c_double>::ENCODING]);
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MKMapPoint {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MKMapSize {
    pub width: c_double,
    pub height: c_double,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MKMapSize {
    const ENCODING: Encoding = Encoding::Struct("?", &[<c_double>::ENCODING, <c_double>::ENCODING]);
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MKMapSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MKMapRect {
    pub origin: MKMapPoint,
    pub size: MKMapSize,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MKMapRect {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[<MKMapPoint>::ENCODING, <MKMapSize>::ENCODING]);
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MKMapRect {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "Foundation_NSGeometry")]
pub type MKZoomScale = CGFloat;

extern "C" {
    pub static MKMapSizeWorld: MKMapSize;
}

extern "C" {
    pub static MKMapRectWorld: MKMapRect;
}

extern "C" {
    #[cfg(feature = "CoreLocation_CLLocation")]
    pub fn MKMapPointForCoordinate(coordinate: CLLocationCoordinate2D) -> MKMapPoint;
}

extern "C" {
    #[cfg(feature = "CoreLocation_CLLocation")]
    pub fn MKCoordinateForMapPoint(map_point: MKMapPoint) -> CLLocationCoordinate2D;
}

extern "C" {
    #[cfg(feature = "CoreLocation_CLLocation")]
    pub fn MKMetersPerMapPointAtLatitude(latitude: CLLocationDegrees) -> CLLocationDistance;
}

extern "C" {
    #[cfg(feature = "CoreLocation_CLLocation")]
    pub fn MKMapPointsPerMeterAtLatitude(latitude: CLLocationDegrees) -> c_double;
}

extern "C" {
    #[cfg(feature = "CoreLocation_CLLocation")]
    pub fn MKMetersBetweenMapPoints(a: MKMapPoint, b: MKMapPoint) -> CLLocationDistance;
}

extern "C" {
    pub static MKMapRectNull: MKMapRect;
}

// TODO: pub fn MKMapPointMake(x: c_double,y: c_double,) -> MKMapPoint;

// TODO: pub fn MKMapSizeMake(width: c_double,height: c_double,) -> MKMapSize;

// TODO: pub fn MKMapRectMake(x: c_double,y: c_double,width: c_double,height: c_double,) -> MKMapRect;

// TODO: pub fn MKMapRectGetMinX(rect: MKMapRect,) -> c_double;

// TODO: pub fn MKMapRectGetMinY(rect: MKMapRect,) -> c_double;

// TODO: pub fn MKMapRectGetMidX(rect: MKMapRect,) -> c_double;

// TODO: pub fn MKMapRectGetMidY(rect: MKMapRect,) -> c_double;

// TODO: pub fn MKMapRectGetMaxX(rect: MKMapRect,) -> c_double;

// TODO: pub fn MKMapRectGetMaxY(rect: MKMapRect,) -> c_double;

// TODO: pub fn MKMapRectGetWidth(rect: MKMapRect,) -> c_double;

// TODO: pub fn MKMapRectGetHeight(rect: MKMapRect,) -> c_double;

// TODO: pub fn MKMapPointEqualToPoint(point1: MKMapPoint,point2: MKMapPoint,) -> Bool;

// TODO: pub fn MKMapSizeEqualToSize(size1: MKMapSize,size2: MKMapSize,) -> Bool;

// TODO: pub fn MKMapRectEqualToRect(rect1: MKMapRect,rect2: MKMapRect,) -> Bool;

// TODO: pub fn MKMapRectIsNull(rect: MKMapRect,) -> Bool;

// TODO: pub fn MKMapRectIsEmpty(rect: MKMapRect,) -> Bool;

// TODO: pub fn MKStringFromMapPoint(point: MKMapPoint,) -> NonNull<NSString>;

// TODO: pub fn MKStringFromMapSize(size: MKMapSize,) -> NonNull<NSString>;

// TODO: pub fn MKStringFromMapRect(rect: MKMapRect,) -> NonNull<NSString>;

extern "C" {
    pub fn MKMapRectUnion(rect1: MKMapRect, rect2: MKMapRect) -> MKMapRect;
}

extern "C" {
    pub fn MKMapRectIntersection(rect1: MKMapRect, rect2: MKMapRect) -> MKMapRect;
}

extern "C" {
    pub fn MKMapRectInset(rect: MKMapRect, dx: c_double, dy: c_double) -> MKMapRect;
}

extern "C" {
    pub fn MKMapRectOffset(rect: MKMapRect, dx: c_double, dy: c_double) -> MKMapRect;
}

extern "C" {
    pub fn MKMapRectContainsPoint(rect: MKMapRect, point: MKMapPoint) -> Bool;
}

extern "C" {
    pub fn MKMapRectContainsRect(rect1: MKMapRect, rect2: MKMapRect) -> Bool;
}

extern "C" {
    pub fn MKMapRectIntersectsRect(rect1: MKMapRect, rect2: MKMapRect) -> Bool;
}

extern "C" {
    #[cfg(feature = "CoreLocation_CLLocation")]
    pub fn MKCoordinateRegionForMapRect(rect: MKMapRect) -> MKCoordinateRegion;
}

extern "C" {
    pub fn MKMapRectSpans180thMeridian(rect: MKMapRect) -> Bool;
}

extern "C" {
    pub fn MKMapRectRemainder(rect: MKMapRect) -> MKMapRect;
}

extern_category!(
    /// Category on [`NSValue`].
    pub unsafe trait NSValueMapKitGeometryExtensions {
        #[cfg(all(feature = "CoreLocation_CLLocation", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other valueWithMKCoordinate:)]
        unsafe fn valueWithMKCoordinate(coordinate: CLLocationCoordinate2D) -> Id<NSValue>;

        #[cfg(all(
            feature = "CoreLocation_CLLocation",
            feature = "Foundation_NSValue",
            feature = "MapKit_MKGeometry"
        ))]
        #[method_id(@__retain_semantics Other valueWithMKCoordinateSpan:)]
        unsafe fn valueWithMKCoordinateSpan(span: MKCoordinateSpan) -> Id<NSValue>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method(MKCoordinateValue)]
        unsafe fn MKCoordinateValue(&self) -> CLLocationCoordinate2D;

        #[cfg(all(feature = "CoreLocation_CLLocation", feature = "MapKit_MKGeometry"))]
        #[method(MKCoordinateSpanValue)]
        unsafe fn MKCoordinateSpanValue(&self) -> MKCoordinateSpan;
    }

    #[cfg(feature = "Foundation_NSValue")]
    unsafe impl NSValueMapKitGeometryExtensions for NSValue {}
);
