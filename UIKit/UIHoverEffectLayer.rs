//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    pub struct UIHoverEffectLayer;

    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl ClassType for UIHoverEffectLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl CAMediaTiming for UIHoverEffectLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSCoding for UIHoverEffectLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSObjectProtocol for UIHoverEffectLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSSecureCoding for UIHoverEffectLayer {}

extern_methods!(
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl UIHoverEffectLayer {
        #[cfg(feature = "UIHoverStyle")]
        #[method_id(@__retain_semantics Other hoverStyle)]
        pub unsafe fn hoverStyle(&self) -> Retained<UIHoverStyle>;

        #[cfg(feature = "UIHoverStyle")]
        #[method(setHoverStyle:)]
        pub unsafe fn setHoverStyle(&self, hover_style: &UIHoverStyle);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other containerView)]
        pub unsafe fn containerView(&self) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(setContainerView:)]
        pub unsafe fn setContainerView(&self, container_view: Option<&UIView>);

        #[cfg(all(feature = "UIHoverStyle", feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Init initWithContainerView:style:)]
        pub unsafe fn initWithContainerView_style(
            this: Allocated<Self>,
            container_view: &UIView,
            style: Option<&UIHoverStyle>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl UIHoverEffectLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn init_with_layer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl UIHoverEffectLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
