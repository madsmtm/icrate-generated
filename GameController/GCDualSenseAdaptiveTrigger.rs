//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub const GCDualSenseAdaptiveTriggerDiscretePositionCount: c_uint = 10;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GCDualSenseAdaptiveTriggerMode(pub NSInteger);
impl GCDualSenseAdaptiveTriggerMode {
    #[doc(alias = "GCDualSenseAdaptiveTriggerModeOff")]
    pub const Off: Self = Self(0);
    #[doc(alias = "GCDualSenseAdaptiveTriggerModeFeedback")]
    pub const Feedback: Self = Self(1);
    #[doc(alias = "GCDualSenseAdaptiveTriggerModeWeapon")]
    pub const Weapon: Self = Self(2);
    #[doc(alias = "GCDualSenseAdaptiveTriggerModeVibration")]
    pub const Vibration: Self = Self(3);
    #[doc(alias = "GCDualSenseAdaptiveTriggerModeSlopeFeedback")]
    pub const SlopeFeedback: Self = Self(4);
}

unsafe impl Encode for GCDualSenseAdaptiveTriggerMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GCDualSenseAdaptiveTriggerMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GCDualSenseAdaptiveTriggerStatus(pub NSInteger);
impl GCDualSenseAdaptiveTriggerStatus {
    #[doc(alias = "GCDualSenseAdaptiveTriggerStatusUnknown")]
    pub const Unknown: Self = Self(-1);
    #[doc(alias = "GCDualSenseAdaptiveTriggerStatusFeedbackNoLoad")]
    pub const FeedbackNoLoad: Self = Self(0);
    #[doc(alias = "GCDualSenseAdaptiveTriggerStatusFeedbackLoadApplied")]
    pub const FeedbackLoadApplied: Self = Self(1);
    #[doc(alias = "GCDualSenseAdaptiveTriggerStatusWeaponReady")]
    pub const WeaponReady: Self = Self(2);
    #[doc(alias = "GCDualSenseAdaptiveTriggerStatusWeaponFiring")]
    pub const WeaponFiring: Self = Self(3);
    #[doc(alias = "GCDualSenseAdaptiveTriggerStatusWeaponFired")]
    pub const WeaponFired: Self = Self(4);
    #[doc(alias = "GCDualSenseAdaptiveTriggerStatusVibrationNotVibrating")]
    pub const VibrationNotVibrating: Self = Self(5);
    #[doc(alias = "GCDualSenseAdaptiveTriggerStatusVibrationIsVibrating")]
    pub const VibrationIsVibrating: Self = Self(6);
    #[doc(alias = "GCDualSenseAdaptiveTriggerStatusSlopeFeedbackReady")]
    pub const SlopeFeedbackReady: Self = Self(7);
    #[doc(alias = "GCDualSenseAdaptiveTriggerStatusSlopeFeedbackApplyingLoad")]
    pub const SlopeFeedbackApplyingLoad: Self = Self(8);
    #[doc(alias = "GCDualSenseAdaptiveTriggerStatusSlopeFeedbackFinished")]
    pub const SlopeFeedbackFinished: Self = Self(9);
}

unsafe impl Encode for GCDualSenseAdaptiveTriggerStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GCDualSenseAdaptiveTriggerStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCDualSenseAdaptiveTriggerPositionalAmplitudes {
    pub values: [c_float; 10],
}

unsafe impl Encode for GCDualSenseAdaptiveTriggerPositionalAmplitudes {
    const ENCODING: Encoding = Encoding::Struct("?", &[<[c_float; 10]>::ENCODING]);
}

unsafe impl RefEncode for GCDualSenseAdaptiveTriggerPositionalAmplitudes {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCDualSenseAdaptiveTriggerPositionalResistiveStrengths {
    pub values: [c_float; 10],
}

unsafe impl Encode for GCDualSenseAdaptiveTriggerPositionalResistiveStrengths {
    const ENCODING: Encoding = Encoding::Struct("?", &[<[c_float; 10]>::ENCODING]);
}

unsafe impl RefEncode for GCDualSenseAdaptiveTriggerPositionalResistiveStrengths {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "GameController_GCControllerButtonInput",
        feature = "GameController_GCControllerElement"
    ))]
    pub struct GCDualSenseAdaptiveTrigger;

    #[cfg(all(
        feature = "GameController_GCControllerButtonInput",
        feature = "GameController_GCControllerElement"
    ))]
    unsafe impl ClassType for GCDualSenseAdaptiveTrigger {
        #[inherits(GCControllerElement, NSObject)]
        type Super = GCControllerButtonInput;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "GameController_GCControllerButtonInput",
    feature = "GameController_GCControllerElement"
))]
unsafe impl NSObjectProtocol for GCDualSenseAdaptiveTrigger {}

extern_methods!(
    #[cfg(all(
        feature = "GameController_GCControllerButtonInput",
        feature = "GameController_GCControllerElement"
    ))]
    unsafe impl GCDualSenseAdaptiveTrigger {
        #[method(mode)]
        pub unsafe fn mode(&self) -> GCDualSenseAdaptiveTriggerMode;

        #[method(status)]
        pub unsafe fn status(&self) -> GCDualSenseAdaptiveTriggerStatus;

        #[method(armPosition)]
        pub unsafe fn armPosition(&self) -> c_float;

        #[method(setModeSlopeFeedbackWithStartPosition:endPosition:startStrength:endStrength:)]
        pub unsafe fn setModeSlopeFeedbackWithStartPosition_endPosition_startStrength_endStrength(
            &self,
            start_position: c_float,
            end_position: c_float,
            start_strength: c_float,
            end_strength: c_float,
        );

        #[method(setModeFeedbackWithStartPosition:resistiveStrength:)]
        pub unsafe fn setModeFeedbackWithStartPosition_resistiveStrength(
            &self,
            start_position: c_float,
            resistive_strength: c_float,
        );

        #[method(setModeFeedbackWithResistiveStrengths:)]
        pub unsafe fn setModeFeedbackWithResistiveStrengths(
            &self,
            positional_resistive_strengths: GCDualSenseAdaptiveTriggerPositionalResistiveStrengths,
        );

        #[method(setModeWeaponWithStartPosition:endPosition:resistiveStrength:)]
        pub unsafe fn setModeWeaponWithStartPosition_endPosition_resistiveStrength(
            &self,
            start_position: c_float,
            end_position: c_float,
            resistive_strength: c_float,
        );

        #[method(setModeVibrationWithStartPosition:amplitude:frequency:)]
        pub unsafe fn setModeVibrationWithStartPosition_amplitude_frequency(
            &self,
            start_position: c_float,
            amplitude: c_float,
            frequency: c_float,
        );

        #[method(setModeVibrationWithAmplitudes:frequency:)]
        pub unsafe fn setModeVibrationWithAmplitudes_frequency(
            &self,
            positional_amplitudes: GCDualSenseAdaptiveTriggerPositionalAmplitudes,
            frequency: c_float,
        );

        #[method(setModeOff)]
        pub unsafe fn setModeOff(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "GameController_GCControllerButtonInput",
        feature = "GameController_GCControllerElement"
    ))]
    unsafe impl GCDualSenseAdaptiveTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
