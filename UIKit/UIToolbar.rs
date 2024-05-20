//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UIToolbar;

    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl ClassType for UIToolbar {
        #[inherits(UIResponder, NSObject)]
        type Super = UIView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
unsafe impl CALayerDelegate for UIToolbar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIToolbar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIToolbar {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UIToolbar {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UIToolbar {}

#[cfg(all(feature = "UIBarCommon", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIBarPositioning for UIToolbar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIToolbar {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIToolbar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UIToolbar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UIToolbar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UIToolbar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIToolbar {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIToolbar {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIToolbar {
        #[cfg(feature = "UIInterface")]
        #[method(barStyle)]
        pub unsafe fn barStyle(&self) -> UIBarStyle;

        #[cfg(feature = "UIInterface")]
        #[method(setBarStyle:)]
        pub unsafe fn setBarStyle(&self, bar_style: UIBarStyle);

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Option<Id<NSArray<UIBarButtonItem>>>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method(setItems:)]
        pub unsafe fn setItems(&self, items: Option<&NSArray<UIBarButtonItem>>);

        #[method(isTranslucent)]
        pub unsafe fn isTranslucent(&self) -> bool;

        #[method(setTranslucent:)]
        pub unsafe fn setTranslucent(&self, translucent: bool);

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method(setItems:animated:)]
        pub unsafe fn setItems_animated(
            &self,
            items: Option<&NSArray<UIBarButtonItem>>,
            animated: bool,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other tintColor)]
        pub unsafe fn tintColor(&self) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setTintColor:)]
        pub unsafe fn setTintColor(&self, tint_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other barTintColor)]
        pub unsafe fn barTintColor(&self) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBarTintColor:)]
        pub unsafe fn setBarTintColor(&self, bar_tint_color: Option<&UIColor>);

        #[cfg(all(feature = "UIBarCommon", feature = "UIImage"))]
        #[method(setBackgroundImage:forToolbarPosition:barMetrics:)]
        pub unsafe fn setBackgroundImage_forToolbarPosition_barMetrics(
            &self,
            background_image: Option<&UIImage>,
            top_or_bottom: UIBarPosition,
            bar_metrics: UIBarMetrics,
        );

        #[cfg(all(feature = "UIBarCommon", feature = "UIImage"))]
        #[method_id(@__retain_semantics Other backgroundImageForToolbarPosition:barMetrics:)]
        pub unsafe fn backgroundImageForToolbarPosition_barMetrics(
            &self,
            top_or_bottom: UIBarPosition,
            bar_metrics: UIBarMetrics,
        ) -> Option<Id<UIImage>>;

        #[cfg(all(feature = "UIBarCommon", feature = "UIImage"))]
        #[method(setShadowImage:forToolbarPosition:)]
        pub unsafe fn setShadowImage_forToolbarPosition(
            &self,
            shadow_image: Option<&UIImage>,
            top_or_bottom: UIBarPosition,
        );

        #[cfg(all(feature = "UIBarCommon", feature = "UIImage"))]
        #[method_id(@__retain_semantics Other shadowImageForToolbarPosition:)]
        pub unsafe fn shadowImageForToolbarPosition(
            &self,
            top_or_bottom: UIBarPosition,
        ) -> Option<Id<UIImage>>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UIToolbarAppearance"))]
        #[method_id(@__retain_semantics Other standardAppearance)]
        pub unsafe fn standardAppearance(&self) -> Id<UIToolbarAppearance>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UIToolbarAppearance"))]
        #[method(setStandardAppearance:)]
        pub unsafe fn setStandardAppearance(&self, standard_appearance: &UIToolbarAppearance);

        #[cfg(all(feature = "UIBarAppearance", feature = "UIToolbarAppearance"))]
        #[method_id(@__retain_semantics Other compactAppearance)]
        pub unsafe fn compactAppearance(&self) -> Option<Id<UIToolbarAppearance>>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UIToolbarAppearance"))]
        #[method(setCompactAppearance:)]
        pub unsafe fn setCompactAppearance(&self, compact_appearance: Option<&UIToolbarAppearance>);

        #[cfg(all(feature = "UIBarAppearance", feature = "UIToolbarAppearance"))]
        #[method_id(@__retain_semantics Other scrollEdgeAppearance)]
        pub unsafe fn scrollEdgeAppearance(&self) -> Option<Id<UIToolbarAppearance>>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UIToolbarAppearance"))]
        #[method(setScrollEdgeAppearance:)]
        pub unsafe fn setScrollEdgeAppearance(
            &self,
            scroll_edge_appearance: Option<&UIToolbarAppearance>,
        );

        #[cfg(all(feature = "UIBarAppearance", feature = "UIToolbarAppearance"))]
        #[method_id(@__retain_semantics Other compactScrollEdgeAppearance)]
        pub unsafe fn compactScrollEdgeAppearance(&self) -> Option<Id<UIToolbarAppearance>>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UIToolbarAppearance"))]
        #[method(setCompactScrollEdgeAppearance:)]
        pub unsafe fn setCompactScrollEdgeAppearance(
            &self,
            compact_scroll_edge_appearance: Option<&UIToolbarAppearance>,
        );

        #[cfg(feature = "UIBarCommon")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn UIToolbarDelegate>>>;

        #[cfg(feature = "UIBarCommon")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn UIToolbarDelegate>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIToolbar {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIToolbar {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    #[cfg(feature = "UIBarCommon")]
    pub unsafe trait UIToolbarDelegate: UIBarPositioningDelegate + IsMainThreadOnly {}

    #[cfg(feature = "UIBarCommon")]
    unsafe impl ProtocolType for dyn UIToolbarDelegate {}
);
