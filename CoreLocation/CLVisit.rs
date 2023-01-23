//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLVisit")]
    pub struct CLVisit;

    #[cfg(feature = "CoreLocation_CLVisit")]
    unsafe impl ClassType for CLVisit {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreLocation_CLVisit")]
    unsafe impl CLVisit {
        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other arrivalDate)]
        pub unsafe fn arrivalDate(&self) -> Id<NSDate, Shared>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other departureDate)]
        pub unsafe fn departureDate(&self) -> Id<NSDate, Shared>;

        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[method(horizontalAccuracy)]
        pub unsafe fn horizontalAccuracy(&self) -> CLLocationAccuracy;
    }
);
