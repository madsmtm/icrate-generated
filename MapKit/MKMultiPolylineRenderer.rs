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
    #[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
    pub struct MKMultiPolylineRenderer;

    #[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
    unsafe impl ClassType for MKMultiPolylineRenderer {
        #[inherits(MKOverlayRenderer, NSObject)]
        type Super = MKOverlayPathRenderer;
    }
);

#[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
unsafe impl NSObjectProtocol for MKMultiPolylineRenderer {}

extern_methods!(
    #[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
    unsafe impl MKMultiPolylineRenderer {
        #[cfg(feature = "MapKit_MKMultiPolyline")]
        #[method_id(@__retain_semantics Init initWithMultiPolyline:)]
        pub unsafe fn initWithMultiPolyline(
            this: Option<Allocated<Self>>,
            multi_polyline: &MKMultiPolyline,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "MapKit_MKMultiPolyline")]
        #[method_id(@__retain_semantics Other multiPolyline)]
        pub unsafe fn multiPolyline(&self) -> Id<MKMultiPolyline, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
    unsafe impl MKMultiPolylineRenderer {
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Option<Allocated<Self>>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Id<Self, Shared>;
    }
);