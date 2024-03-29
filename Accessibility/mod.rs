// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `Accessibility` framework
#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "Accessibility", kind = "framework")]
extern "C" {}

#[cfg(feature = "Accessibility_AXAudiograph")]
#[path = "AXAudiograph.rs"]
mod __AXAudiograph;
#[cfg(feature = "Accessibility_AXBrailleMap")]
#[path = "AXBrailleMap.rs"]
mod __AXBrailleMap;
#[cfg(feature = "Accessibility_AXColorUtilities")]
#[path = "AXColorUtilities.rs"]
mod __AXColorUtilities;
#[cfg(feature = "Accessibility_AXCustomContent")]
#[path = "AXCustomContent.rs"]
mod __AXCustomContent;
#[cfg(feature = "Accessibility_AXFoundation")]
#[path = "AXFoundation.rs"]
mod __AXFoundation;
#[cfg(feature = "Accessibility_AXHearingUtilities")]
#[path = "AXHearingUtilities.rs"]
mod __AXHearingUtilities;
#[cfg(feature = "Accessibility_AXSettings")]
#[path = "AXSettings.rs"]
mod __AXSettings;

#[cfg(feature = "Accessibility_AXAudiograph")]
pub use self::__AXAudiograph::AXCategoricalDataAxisDescriptor;
#[cfg(feature = "Accessibility_AXAudiograph")]
pub use self::__AXAudiograph::AXChart;
#[cfg(feature = "Accessibility_AXAudiograph")]
pub use self::__AXAudiograph::AXChartDescriptor;
#[cfg(feature = "Accessibility_AXAudiograph")]
pub use self::__AXAudiograph::AXChartDescriptorContentDirection;
#[cfg(all(
    feature = "Accessibility_AXAudiograph",
    feature = "Foundation_NSObject"
))]
pub use self::__AXAudiograph::AXDataAxisDescriptor;
#[cfg(feature = "Accessibility_AXAudiograph")]
pub use self::__AXAudiograph::AXDataPoint;
#[cfg(feature = "Accessibility_AXAudiograph")]
pub use self::__AXAudiograph::AXDataPointValue;
#[cfg(feature = "Accessibility_AXAudiograph")]
pub use self::__AXAudiograph::AXDataSeriesDescriptor;
#[cfg(feature = "Accessibility_AXAudiograph")]
pub use self::__AXAudiograph::AXLiveAudioGraph;
#[cfg(feature = "Accessibility_AXAudiograph")]
pub use self::__AXAudiograph::AXNumericDataAxisDescriptor;
#[cfg(feature = "Accessibility_AXAudiograph")]
pub use self::__AXAudiograph::AXNumericDataAxisDescriptorScale;
#[cfg(feature = "Accessibility_AXBrailleMap")]
pub use self::__AXBrailleMap::AXBrailleMap;
#[cfg(feature = "Accessibility_AXBrailleMap")]
pub use self::__AXBrailleMap::AXBrailleMapRenderer;
#[cfg(feature = "Accessibility_AXCustomContent")]
pub use self::__AXCustomContent::AXCustomContent;
#[cfg(feature = "Accessibility_AXCustomContent")]
pub use self::__AXCustomContent::AXCustomContentImportance;
#[cfg(feature = "Accessibility_AXCustomContent")]
pub use self::__AXCustomContent::AXCustomContentProvider;
#[cfg(all(
    feature = "Accessibility_AXCustomContent",
    feature = "Foundation_NSArray"
))]
pub use self::__AXCustomContent::AXCustomContentReturnBlock;
#[cfg(feature = "Accessibility_AXHearingUtilities")]
pub use self::__AXHearingUtilities::AXHearingDeviceEar;
#[cfg(all(
    feature = "Accessibility_AXHearingUtilities",
    feature = "Foundation_NSArray",
    feature = "Foundation_NSUUID"
))]
pub use self::__AXHearingUtilities::AXMFiHearingDevicePairedUUIDs;
#[cfg(all(
    feature = "Accessibility_AXHearingUtilities",
    feature = "Foundation_NSNotification",
    feature = "Foundation_NSString"
))]
pub use self::__AXHearingUtilities::AXMFiHearingDevicePairedUUIDsDidChangeNotification;
#[cfg(feature = "Accessibility_AXHearingUtilities")]
pub use self::__AXHearingUtilities::AXMFiHearingDeviceStreamingEar;
#[cfg(all(
    feature = "Accessibility_AXHearingUtilities",
    feature = "Foundation_NSNotification",
    feature = "Foundation_NSString"
))]
pub use self::__AXHearingUtilities::AXMFiHearingDeviceStreamingEarDidChangeNotification;
#[cfg(feature = "Accessibility_AXHearingUtilities")]
pub use self::__AXHearingUtilities::AXSupportsBidirectionalAXMFiHearingDeviceStreaming;
#[cfg(feature = "Accessibility_AXSettings")]
pub use self::__AXSettings::AXAnimatedImagesEnabled;
#[cfg(all(
    feature = "Accessibility_AXSettings",
    feature = "Foundation_NSNotification",
    feature = "Foundation_NSString"
))]
pub use self::__AXSettings::AXAnimatedImagesEnabledDidChangeNotification;
#[cfg(feature = "Accessibility_AXSettings")]
pub use self::__AXSettings::AXPrefersHorizontalTextLayout;
#[cfg(all(
    feature = "Accessibility_AXSettings",
    feature = "Foundation_NSNotification",
    feature = "Foundation_NSString"
))]
pub use self::__AXSettings::AXPrefersHorizontalTextLayoutDidChangeNotification;
