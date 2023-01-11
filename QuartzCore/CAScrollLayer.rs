//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CAScrollLayerScrollMode = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CAScrollLayer")]
    pub struct CAScrollLayer;

    #[cfg(feature = "CoreAnimation_CAScrollLayer")]
    unsafe impl ClassType for CAScrollLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
    }
);

extern_methods!(
    #[cfg(feature = "CoreAnimation_CAScrollLayer")]
    unsafe impl CAScrollLayer {
        #[method(scrollToPoint:)]
        pub unsafe fn scrollToPoint(&self, p: CGPoint);

        #[method(scrollToRect:)]
        pub unsafe fn scrollToRect(&self, r: CGRect);

        #[method_id(@__retain_semantics Other scrollMode)]
        pub unsafe fn scrollMode(&self) -> Id<CAScrollLayerScrollMode, Shared>;

        #[method(setScrollMode:)]
        pub unsafe fn setScrollMode(&self, scrollMode: &CAScrollLayerScrollMode);
    }
);

extern_methods!(
    /// CALayerScrolling
    #[cfg(feature = "CoreAnimation_CALayer")]
    unsafe impl CALayer {
        #[method(scrollPoint:)]
        pub unsafe fn scrollPoint(&self, p: CGPoint);

        #[method(scrollRectToVisible:)]
        pub unsafe fn scrollRectToVisible(&self, r: CGRect);

        #[method(visibleRect)]
        pub unsafe fn visibleRect(&self) -> CGRect;
    }
);

extern_static!(kCAScrollNone: &'static CAScrollLayerScrollMode);

extern_static!(kCAScrollVertically: &'static CAScrollLayerScrollMode);

extern_static!(kCAScrollHorizontally: &'static CAScrollLayerScrollMode);

extern_static!(kCAScrollBoth: &'static CAScrollLayerScrollMode);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CoreAnimation_CAScrollLayer")]
    unsafe impl CAScrollLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(
            this: Option<Allocated<Self>>,
            layer: &Object,
        ) -> Id<Self, Shared>;
    }
);
