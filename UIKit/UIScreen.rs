//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
use objc2_quartz_core::*;

use crate::*;

extern "C" {
    pub static UIScreenDidConnectNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIScreenDidDisconnectNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIScreenModeDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIScreenBrightnessDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIScreenCapturedDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIScreenReferenceDisplayModeStatusDidChangeNotification: &'static NSNotificationName;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIScreenOverscanCompensation(pub NSInteger);
impl UIScreenOverscanCompensation {
    #[doc(alias = "UIScreenOverscanCompensationScale")]
    pub const Scale: Self = Self(0);
    #[doc(alias = "UIScreenOverscanCompensationInsetBounds")]
    pub const InsetBounds: Self = Self(1);
    #[doc(alias = "UIScreenOverscanCompensationNone")]
    pub const None: Self = Self(2);
    #[deprecated]
    #[doc(alias = "UIScreenOverscanCompensationInsetApplicationFrame")]
    pub const InsetApplicationFrame: Self = Self(2);
}

unsafe impl Encode for UIScreenOverscanCompensation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIScreenOverscanCompensation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIScreenReferenceDisplayModeStatus(pub NSInteger);
impl UIScreenReferenceDisplayModeStatus {
    #[doc(alias = "UIScreenReferenceDisplayModeStatusNotSupported")]
    pub const NotSupported: Self = Self(0);
    #[doc(alias = "UIScreenReferenceDisplayModeStatusNotEnabled")]
    pub const NotEnabled: Self = Self(1);
    #[doc(alias = "UIScreenReferenceDisplayModeStatusLimited")]
    pub const Limited: Self = Self(2);
    #[doc(alias = "UIScreenReferenceDisplayModeStatusEnabled")]
    pub const Enabled: Self = Self(3);
}

unsafe impl Encode for UIScreenReferenceDisplayModeStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIScreenReferenceDisplayModeStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIScreen;

    unsafe impl ClassType for UIScreen {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIScreen {}

#[cfg(feature = "UITraitCollection")]
unsafe impl UITraitEnvironment for UIScreen {}

extern_methods!(
    unsafe impl UIScreen {
        #[deprecated = "Use UIApplication.shared.openSessions to find open sessions with scenes from other screens"]
        #[method_id(@__retain_semantics Other screens)]
        pub unsafe fn screens(mtm: MainThreadMarker) -> Id<NSArray<UIScreen>>;

        #[deprecated = "Use a UIScreen instance found through context instead: i.e, view.window.windowScene.screen"]
        #[method_id(@__retain_semantics Other mainScreen)]
        pub unsafe fn mainScreen(mtm: MainThreadMarker) -> Id<UIScreen>;

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[method(scale)]
        pub unsafe fn scale(&self) -> CGFloat;

        #[cfg(feature = "UIScreenMode")]
        #[method_id(@__retain_semantics Other availableModes)]
        pub unsafe fn availableModes(&self) -> Id<NSArray<UIScreenMode>>;

        #[cfg(feature = "UIScreenMode")]
        #[method_id(@__retain_semantics Other preferredMode)]
        pub unsafe fn preferredMode(&self) -> Option<Id<UIScreenMode>>;

        #[cfg(feature = "UIScreenMode")]
        #[method_id(@__retain_semantics Other currentMode)]
        pub unsafe fn currentMode(&self) -> Option<Id<UIScreenMode>>;

        #[cfg(feature = "UIScreenMode")]
        #[method(setCurrentMode:)]
        pub unsafe fn setCurrentMode(&self, current_mode: Option<&UIScreenMode>);

        #[method(overscanCompensation)]
        pub unsafe fn overscanCompensation(&self) -> UIScreenOverscanCompensation;

        #[method(setOverscanCompensation:)]
        pub unsafe fn setOverscanCompensation(
            &self,
            overscan_compensation: UIScreenOverscanCompensation,
        );

        #[cfg(feature = "UIGeometry")]
        #[method(overscanCompensationInsets)]
        pub unsafe fn overscanCompensationInsets(&self) -> UIEdgeInsets;

        #[method_id(@__retain_semantics Other mirroredScreen)]
        pub unsafe fn mirroredScreen(&self) -> Option<Id<UIScreen>>;

        #[deprecated = "Use the sceneCaptureState in UITraitCollection instead."]
        #[method(isCaptured)]
        pub unsafe fn isCaptured(&self) -> bool;

        #[method(brightness)]
        pub unsafe fn brightness(&self) -> CGFloat;

        #[method(setBrightness:)]
        pub unsafe fn setBrightness(&self, brightness: CGFloat);

        #[method(wantsSoftwareDimming)]
        pub unsafe fn wantsSoftwareDimming(&self) -> bool;

        #[method(setWantsSoftwareDimming:)]
        pub unsafe fn setWantsSoftwareDimming(&self, wants_software_dimming: bool);

        #[cfg(feature = "UIView")]
        #[method_id(@__retain_semantics Other coordinateSpace)]
        pub unsafe fn coordinateSpace(&self) -> Id<ProtocolObject<dyn UICoordinateSpace>>;

        #[cfg(feature = "UIView")]
        #[method_id(@__retain_semantics Other fixedCoordinateSpace)]
        pub unsafe fn fixedCoordinateSpace(&self) -> Id<ProtocolObject<dyn UICoordinateSpace>>;

        #[method(nativeBounds)]
        pub unsafe fn nativeBounds(&self) -> CGRect;

        #[method(nativeScale)]
        pub unsafe fn nativeScale(&self) -> CGFloat;

        #[cfg(feature = "objc2-quartz-core")]
        #[method_id(@__retain_semantics Other displayLinkWithTarget:selector:)]
        pub unsafe fn displayLinkWithTarget_selector(
            &self,
            target: &AnyObject,
            sel: Sel,
        ) -> Option<Id<CADisplayLink>>;

        #[method(maximumFramesPerSecond)]
        pub unsafe fn maximumFramesPerSecond(&self) -> NSInteger;

        #[method(referenceDisplayModeStatus)]
        pub unsafe fn referenceDisplayModeStatus(&self) -> UIScreenReferenceDisplayModeStatus;

        #[method(currentEDRHeadroom)]
        pub unsafe fn currentEDRHeadroom(&self) -> CGFloat;

        #[method(potentialEDRHeadroom)]
        pub unsafe fn potentialEDRHeadroom(&self) -> CGFloat;

        #[cfg(feature = "UIFocus")]
        #[deprecated = "Use -[UIWindowScene focusSystem].focusedItem instead"]
        #[method_id(@__retain_semantics Other focusedItem)]
        pub unsafe fn focusedItem(&self) -> Option<Id<ProtocolObject<dyn UIFocusItem>>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "Use -[UIWindowScene focusSystem].focusedItem instead"]
        #[method_id(@__retain_semantics Other focusedView)]
        pub unsafe fn focusedView(&self) -> Option<Id<UIView>>;

        #[deprecated = "Use -[UIWindowScene focusSystem] != nil instead"]
        #[method(supportsFocus)]
        pub unsafe fn supportsFocus(&self) -> bool;

        #[deprecated]
        #[method(applicationFrame)]
        pub unsafe fn applicationFrame(&self) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIScreen {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// UISnapshotting
    unsafe impl UIScreen {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other snapshotViewAfterScreenUpdates:)]
        pub unsafe fn snapshotViewAfterScreenUpdates(&self, after_updates: bool) -> Id<UIView>;
    }
);