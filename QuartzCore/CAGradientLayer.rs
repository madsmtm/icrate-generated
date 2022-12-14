//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CAGradientLayerType = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAGradientLayer;

    unsafe impl ClassType for CAGradientLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
    }
);

extern_methods!(
    unsafe impl CAGradientLayer {
        #[method_id(@__retain_semantics Other colors)]
        pub unsafe fn colors(&self) -> Option<Id<NSArray, Shared>>;

        #[method(setColors:)]
        pub unsafe fn setColors(&self, colors: Option<&NSArray>);

        #[method_id(@__retain_semantics Other locations)]
        pub unsafe fn locations(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;

        #[method(setLocations:)]
        pub unsafe fn setLocations(&self, locations: Option<&NSArray<NSNumber>>);

        #[method(startPoint)]
        pub unsafe fn startPoint(&self) -> CGPoint;

        #[method(setStartPoint:)]
        pub unsafe fn setStartPoint(&self, startPoint: CGPoint);

        #[method(endPoint)]
        pub unsafe fn endPoint(&self) -> CGPoint;

        #[method(setEndPoint:)]
        pub unsafe fn setEndPoint(&self, endPoint: CGPoint);

        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn type_(&self) -> Id<CAGradientLayerType, Shared>;

        #[method(setType:)]
        pub unsafe fn setType(&self, type_: &CAGradientLayerType);
    }
);

extern_static!(kCAGradientLayerAxial: &'static CAGradientLayerType);

extern_static!(kCAGradientLayerRadial: &'static CAGradientLayerType);

extern_static!(kCAGradientLayerConic: &'static CAGradientLayerType);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    unsafe impl CAGradientLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(
            this: Option<Allocated<Self>>,
            layer: &Object,
        ) -> Id<Self, Shared>;
    }
);
