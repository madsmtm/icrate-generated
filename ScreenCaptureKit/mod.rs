// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

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

#[link(name = "ScreenCaptureKit", kind = "framework")]
extern "C" {}

#[cfg(feature = "SCContentSharingPicker")]
#[path = "SCContentSharingPicker.rs"]
mod __SCContentSharingPicker;
#[cfg(feature = "SCError")]
#[path = "SCError.rs"]
mod __SCError;
#[cfg(feature = "SCScreenshotManager")]
#[path = "SCScreenshotManager.rs"]
mod __SCScreenshotManager;
#[cfg(feature = "SCShareableContent")]
#[path = "SCShareableContent.rs"]
mod __SCShareableContent;
#[cfg(feature = "SCStream")]
#[path = "SCStream.rs"]
mod __SCStream;

#[cfg(feature = "SCContentSharingPicker")]
pub use self::__SCContentSharingPicker::SCContentSharingPicker;
#[cfg(feature = "SCContentSharingPicker")]
pub use self::__SCContentSharingPicker::SCContentSharingPickerConfiguration;
#[cfg(feature = "SCContentSharingPicker")]
pub use self::__SCContentSharingPicker::SCContentSharingPickerMode;
#[cfg(feature = "SCContentSharingPicker")]
pub use self::__SCContentSharingPicker::SCContentSharingPickerObserver;
#[cfg(feature = "SCError")]
pub use self::__SCError::SCStreamErrorCode;
#[cfg(feature = "SCError")]
pub use self::__SCError::SCStreamErrorDomain;
#[cfg(feature = "SCScreenshotManager")]
pub use self::__SCScreenshotManager::SCScreenshotManager;
#[cfg(feature = "SCShareableContent")]
pub use self::__SCShareableContent::SCDisplay;
#[cfg(feature = "SCShareableContent")]
pub use self::__SCShareableContent::SCRunningApplication;
#[cfg(feature = "SCShareableContent")]
pub use self::__SCShareableContent::SCShareableContent;
#[cfg(feature = "SCShareableContent")]
pub use self::__SCShareableContent::SCShareableContentInfo;
#[cfg(feature = "SCShareableContent")]
pub use self::__SCShareableContent::SCShareableContentStyle;
#[cfg(feature = "SCShareableContent")]
pub use self::__SCShareableContent::SCWindow;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCCaptureResolutionType;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCContentFilter;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCFrameStatus;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCPresenterOverlayAlertSetting;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStream;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamConfiguration;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamDelegate;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamFrameInfo;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamFrameInfoBoundingRect;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamFrameInfoContentRect;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamFrameInfoContentScale;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamFrameInfoDirtyRects;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamFrameInfoDisplayTime;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamFrameInfoPresenterOverlayContentRect;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamFrameInfoScaleFactor;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamFrameInfoScreenRect;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamFrameInfoStatus;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamOutput;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamOutputType;
#[cfg(feature = "SCStream")]
pub use self::__SCStream::SCStreamType;