//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_struct!(
    pub struct GCAcceleration {
        pub x: c_double,
        pub y: c_double,
        pub z: c_double,
    }
);

extern_struct!(
    pub struct GCRotationRate {
        pub x: c_double,
        pub y: c_double,
        pub z: c_double,
    }
);

extern_struct!(
    pub struct GCEulerAngles {
        pub pitch: c_double,
        pub yaw: c_double,
        pub roll: c_double,
    }
);

extern_struct!(
    pub struct GCQuaternion {
        pub x: c_double,
        pub y: c_double,
        pub z: c_double,
        pub w: c_double,
    }
);

pub type GCMotionValueChangedHandler = *mut Block<(NonNull<GCMotion>,), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCMotion")]
    pub struct GCMotion;

    #[cfg(feature = "GameController_GCMotion")]
    unsafe impl ClassType for GCMotion {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "GameController_GCMotion")]
    unsafe impl GCMotion {
        #[cfg(feature = "GameController_GCController")]
        #[method_id(@__retain_semantics Other controller)]
        pub unsafe fn controller(&self) -> Option<Id<GCController, Shared>>;

        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCMotionValueChangedHandler;

        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCMotionValueChangedHandler,
        );

        #[method(sensorsRequireManualActivation)]
        pub unsafe fn sensorsRequireManualActivation(&self) -> bool;

        #[method(sensorsActive)]
        pub unsafe fn sensorsActive(&self) -> bool;

        #[method(setSensorsActive:)]
        pub unsafe fn setSensorsActive(&self, sensors_active: bool);

        #[method(hasGravityAndUserAcceleration)]
        pub unsafe fn hasGravityAndUserAcceleration(&self) -> bool;

        #[method(gravity)]
        pub unsafe fn gravity(&self) -> GCAcceleration;

        #[method(userAcceleration)]
        pub unsafe fn userAcceleration(&self) -> GCAcceleration;

        #[method(acceleration)]
        pub unsafe fn acceleration(&self) -> GCAcceleration;

        #[deprecated = "hasAttitudeAndRotationRate has been deprecated, use -hasAttitude and -hasRotationRate instead"]
        #[method(hasAttitudeAndRotationRate)]
        pub unsafe fn hasAttitudeAndRotationRate(&self) -> bool;

        #[method(hasAttitude)]
        pub unsafe fn hasAttitude(&self) -> bool;

        #[method(hasRotationRate)]
        pub unsafe fn hasRotationRate(&self) -> bool;

        #[method(attitude)]
        pub unsafe fn attitude(&self) -> GCQuaternion;

        #[method(rotationRate)]
        pub unsafe fn rotationRate(&self) -> GCRotationRate;

        #[method(setGravity:)]
        pub unsafe fn setGravity(&self, gravity: GCAcceleration);

        #[method(setUserAcceleration:)]
        pub unsafe fn setUserAcceleration(&self, user_acceleration: GCAcceleration);

        #[method(setAcceleration:)]
        pub unsafe fn setAcceleration(&self, acceleration: GCAcceleration);

        #[method(setAttitude:)]
        pub unsafe fn setAttitude(&self, attitude: GCQuaternion);

        #[method(setRotationRate:)]
        pub unsafe fn setRotationRate(&self, rotation_rate: GCRotationRate);

        #[method(setStateFromMotion:)]
        pub unsafe fn setStateFromMotion(&self, motion: &GCMotion);
    }
);
