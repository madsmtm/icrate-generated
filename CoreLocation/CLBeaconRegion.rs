//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLRegion")]
    #[deprecated]
    pub struct CLBeaconRegion;

    #[cfg(feature = "CoreLocation_CLRegion")]
    unsafe impl ClassType for CLBeaconRegion {
        #[inherits(NSObject)]
        type Super = CLRegion;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreLocation_CLRegion")]
unsafe impl NSCoding for CLBeaconRegion {}

#[cfg(feature = "CoreLocation_CLRegion")]
unsafe impl NSCopying for CLBeaconRegion {}

#[cfg(feature = "CoreLocation_CLRegion")]
unsafe impl NSObjectProtocol for CLBeaconRegion {}

#[cfg(feature = "CoreLocation_CLRegion")]
unsafe impl NSSecureCoding for CLBeaconRegion {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLRegion")]
    unsafe impl CLBeaconRegion {
        #[method_id(@__retain_semantics Init initWithUUID:identifier:)]
        pub unsafe fn initWithUUID_identifier(
            this: Allocated<Self>,
            uuid: &NSUUID,
            identifier: &NSString,
        ) -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithProximityUUID:identifier:)]
        pub unsafe fn initWithProximityUUID_identifier(
            this: Allocated<Self>,
            proximity_uuid: &NSUUID,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "CoreLocation_CLBeaconIdentityCondition")]
        #[method_id(@__retain_semantics Init initWithUUID:major:identifier:)]
        pub unsafe fn initWithUUID_major_identifier(
            this: Allocated<Self>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "CoreLocation_CLBeaconIdentityCondition")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithProximityUUID:major:identifier:)]
        pub unsafe fn initWithProximityUUID_major_identifier(
            this: Allocated<Self>,
            proximity_uuid: &NSUUID,
            major: CLBeaconMajorValue,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "CoreLocation_CLBeaconIdentityCondition")]
        #[method_id(@__retain_semantics Init initWithUUID:major:minor:identifier:)]
        pub unsafe fn initWithUUID_major_minor_identifier(
            this: Allocated<Self>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
            minor: CLBeaconMinorValue,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "CoreLocation_CLBeaconIdentityCondition")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithProximityUUID:major:minor:identifier:)]
        pub unsafe fn initWithProximityUUID_major_minor_identifier(
            this: Allocated<Self>,
            proximity_uuid: &NSUUID,
            major: CLBeaconMajorValue,
            minor: CLBeaconMinorValue,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CoreLocation_CLBeaconIdentityCondition",
            feature = "CoreLocation_CLBeaconIdentityConstraint",
            feature = "CoreLocation_CLCondition"
        ))]
        #[method_id(@__retain_semantics Init initWithBeaconIdentityConstraint:identifier:)]
        pub unsafe fn initWithBeaconIdentityConstraint_identifier(
            this: Allocated<Self>,
            beacon_identity_constraint: &CLBeaconIdentityConstraint,
            identifier: &NSString,
        ) -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other peripheralDataWithMeasuredPower:)]
        pub unsafe fn peripheralDataWithMeasuredPower(
            &self,
            measured_power: Option<&NSNumber>,
        ) -> Id<NSMutableDictionary<NSString, AnyObject>>;

        #[cfg(all(
            feature = "CoreLocation_CLBeaconIdentityCondition",
            feature = "CoreLocation_CLBeaconIdentityConstraint",
            feature = "CoreLocation_CLCondition"
        ))]
        #[method_id(@__retain_semantics Other beaconIdentityConstraint)]
        pub unsafe fn beaconIdentityConstraint(&self) -> Id<CLBeaconIdentityConstraint>;

        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID>;

        #[deprecated]
        #[method_id(@__retain_semantics Other proximityUUID)]
        pub unsafe fn proximityUUID(&self) -> Id<NSUUID>;

        #[deprecated]
        #[method_id(@__retain_semantics Other major)]
        pub unsafe fn major(&self) -> Option<Id<NSNumber>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other minor)]
        pub unsafe fn minor(&self) -> Option<Id<NSNumber>>;

        #[deprecated]
        #[method(notifyEntryStateOnDisplay)]
        pub unsafe fn notifyEntryStateOnDisplay(&self) -> bool;

        #[deprecated]
        #[method(setNotifyEntryStateOnDisplay:)]
        pub unsafe fn setNotifyEntryStateOnDisplay(&self, notify_entry_state_on_display: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `CLRegion`
    #[cfg(feature = "CoreLocation_CLRegion")]
    unsafe impl CLBeaconRegion {
        #[cfg(feature = "CoreLocation_CLLocation")]
        #[deprecated = "Please see CLCircularRegion"]
        #[method_id(@__retain_semantics Init initCircularRegionWithCenter:radius:identifier:)]
        pub unsafe fn initCircularRegionWithCenter_radius_identifier(
            this: Allocated<Self>,
            center: CLLocationCoordinate2D,
            radius: CLLocationDistance,
            identifier: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreLocation_CLRegion")]
    unsafe impl CLBeaconRegion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLBeacon;

    unsafe impl ClassType for CLBeacon {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CLBeacon {}

unsafe impl NSCopying for CLBeacon {}

unsafe impl NSObjectProtocol for CLBeacon {}

unsafe impl NSSecureCoding for CLBeacon {}

extern_methods!(
    unsafe impl CLBeacon {
        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Id<NSDate>;

        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID>;

        #[deprecated]
        #[method_id(@__retain_semantics Other proximityUUID)]
        pub unsafe fn proximityUUID(&self) -> Id<NSUUID>;

        #[method_id(@__retain_semantics Other major)]
        pub unsafe fn major(&self) -> Id<NSNumber>;

        #[method_id(@__retain_semantics Other minor)]
        pub unsafe fn minor(&self) -> Id<NSNumber>;

        #[cfg(feature = "CoreLocation_CLRegion")]
        #[method(proximity)]
        pub unsafe fn proximity(&self) -> CLProximity;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method(accuracy)]
        pub unsafe fn accuracy(&self) -> CLLocationAccuracy;

        #[method(rssi)]
        pub unsafe fn rssi(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLBeacon {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
