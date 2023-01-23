//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        GCDualSenseAdaptiveTriggerDiscretePositionCount = 10,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GCDualSenseAdaptiveTriggerMode {
        GCDualSenseAdaptiveTriggerModeOff = 0,
        GCDualSenseAdaptiveTriggerModeFeedback = 1,
        GCDualSenseAdaptiveTriggerModeWeapon = 2,
        GCDualSenseAdaptiveTriggerModeVibration = 3,
        GCDualSenseAdaptiveTriggerModeSlopeFeedback = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GCDualSenseAdaptiveTriggerStatus {
        GCDualSenseAdaptiveTriggerStatusUnknown = -1,
        GCDualSenseAdaptiveTriggerStatusFeedbackNoLoad = 0,
        GCDualSenseAdaptiveTriggerStatusFeedbackLoadApplied = 1,
        GCDualSenseAdaptiveTriggerStatusWeaponReady = 2,
        GCDualSenseAdaptiveTriggerStatusWeaponFiring = 3,
        GCDualSenseAdaptiveTriggerStatusWeaponFired = 4,
        GCDualSenseAdaptiveTriggerStatusVibrationNotVibrating = 5,
        GCDualSenseAdaptiveTriggerStatusVibrationIsVibrating = 6,
        GCDualSenseAdaptiveTriggerStatusSlopeFeedbackReady = 7,
        GCDualSenseAdaptiveTriggerStatusSlopeFeedbackApplyingLoad = 8,
        GCDualSenseAdaptiveTriggerStatusSlopeFeedbackFinished = 9,
    }
);

extern_struct!(
    pub struct GCDualSenseAdaptiveTriggerPositionalAmplitudes {
        pub values: [c_float; 10],
    }
);

extern_struct!(
    pub struct GCDualSenseAdaptiveTriggerPositionalResistiveStrengths {
        pub values: [c_float; 10],
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCDualSenseAdaptiveTrigger")]
    pub struct GCDualSenseAdaptiveTrigger;

    #[cfg(feature = "GameController_GCDualSenseAdaptiveTrigger")]
    unsafe impl ClassType for GCDualSenseAdaptiveTrigger {
        #[inherits(GCControllerElement, NSObject)]
        type Super = GCControllerButtonInput;
    }
);

extern_methods!(
    #[cfg(feature = "GameController_GCDualSenseAdaptiveTrigger")]
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
