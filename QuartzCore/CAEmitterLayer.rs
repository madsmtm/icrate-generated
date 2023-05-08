//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CAEmitterLayerEmitterShape = NSString;
);

typed_enum!(
    pub type CAEmitterLayerEmitterMode = NSString;
);

typed_enum!(
    pub type CAEmitterLayerRenderMode = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CAEmitterLayer")]
    pub struct CAEmitterLayer;

    #[cfg(feature = "CoreAnimation_CAEmitterLayer")]
    unsafe impl ClassType for CAEmitterLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreAnimation_CAEmitterLayer")]
unsafe impl CAMediaTiming for CAEmitterLayer {}

#[cfg(feature = "CoreAnimation_CAEmitterLayer")]
unsafe impl NSCoding for CAEmitterLayer {}

#[cfg(feature = "CoreAnimation_CAEmitterLayer")]
unsafe impl NSObjectProtocol for CAEmitterLayer {}

#[cfg(feature = "CoreAnimation_CAEmitterLayer")]
unsafe impl NSSecureCoding for CAEmitterLayer {}

extern_methods!(
    #[cfg(feature = "CoreAnimation_CAEmitterLayer")]
    unsafe impl CAEmitterLayer {
        #[cfg(all(
            feature = "CoreAnimation_CAEmitterCell",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other emitterCells)]
        pub unsafe fn emitterCells(&self) -> Option<Id<NSArray<CAEmitterCell>>>;

        #[cfg(all(
            feature = "CoreAnimation_CAEmitterCell",
            feature = "Foundation_NSArray"
        ))]
        #[method(setEmitterCells:)]
        pub unsafe fn setEmitterCells(&self, emitter_cells: Option<&NSArray<CAEmitterCell>>);

        #[method(birthRate)]
        pub unsafe fn birthRate(&self) -> c_float;

        #[method(setBirthRate:)]
        pub unsafe fn setBirthRate(&self, birth_rate: c_float);

        #[method(lifetime)]
        pub unsafe fn lifetime(&self) -> c_float;

        #[method(setLifetime:)]
        pub unsafe fn setLifetime(&self, lifetime: c_float);

        #[method(emitterPosition)]
        pub unsafe fn emitterPosition(&self) -> CGPoint;

        #[method(setEmitterPosition:)]
        pub unsafe fn setEmitterPosition(&self, emitter_position: CGPoint);

        #[method(emitterZPosition)]
        pub unsafe fn emitterZPosition(&self) -> CGFloat;

        #[method(setEmitterZPosition:)]
        pub unsafe fn setEmitterZPosition(&self, emitter_z_position: CGFloat);

        #[method(emitterSize)]
        pub unsafe fn emitterSize(&self) -> CGSize;

        #[method(setEmitterSize:)]
        pub unsafe fn setEmitterSize(&self, emitter_size: CGSize);

        #[method(emitterDepth)]
        pub unsafe fn emitterDepth(&self) -> CGFloat;

        #[method(setEmitterDepth:)]
        pub unsafe fn setEmitterDepth(&self, emitter_depth: CGFloat);

        #[method_id(@__retain_semantics Other emitterShape)]
        pub unsafe fn emitterShape(&self) -> Id<CAEmitterLayerEmitterShape>;

        #[method(setEmitterShape:)]
        pub unsafe fn setEmitterShape(&self, emitter_shape: &CAEmitterLayerEmitterShape);

        #[method_id(@__retain_semantics Other emitterMode)]
        pub unsafe fn emitterMode(&self) -> Id<CAEmitterLayerEmitterMode>;

        #[method(setEmitterMode:)]
        pub unsafe fn setEmitterMode(&self, emitter_mode: &CAEmitterLayerEmitterMode);

        #[method_id(@__retain_semantics Other renderMode)]
        pub unsafe fn renderMode(&self) -> Id<CAEmitterLayerRenderMode>;

        #[method(setRenderMode:)]
        pub unsafe fn setRenderMode(&self, render_mode: &CAEmitterLayerRenderMode);

        #[method(preservesDepth)]
        pub unsafe fn preservesDepth(&self) -> bool;

        #[method(setPreservesDepth:)]
        pub unsafe fn setPreservesDepth(&self, preserves_depth: bool);

        #[method(velocity)]
        pub unsafe fn velocity(&self) -> c_float;

        #[method(setVelocity:)]
        pub unsafe fn setVelocity(&self, velocity: c_float);

        #[method(scale)]
        pub unsafe fn scale(&self) -> c_float;

        #[method(setScale:)]
        pub unsafe fn setScale(&self, scale: c_float);

        #[method(spin)]
        pub unsafe fn spin(&self) -> c_float;

        #[method(setSpin:)]
        pub unsafe fn setSpin(&self, spin: c_float);

        #[method(seed)]
        pub unsafe fn seed(&self) -> c_uint;

        #[method(setSeed:)]
        pub unsafe fn setSeed(&self, seed: c_uint);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CoreAnimation_CAEmitterLayer")]
    unsafe impl CAEmitterLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Option<Allocated<Self>>, layer: &Object) -> Id<Self>;
    }
);

extern_static!(kCAEmitterLayerPoint: &'static CAEmitterLayerEmitterShape);

extern_static!(kCAEmitterLayerLine: &'static CAEmitterLayerEmitterShape);

extern_static!(kCAEmitterLayerRectangle: &'static CAEmitterLayerEmitterShape);

extern_static!(kCAEmitterLayerCuboid: &'static CAEmitterLayerEmitterShape);

extern_static!(kCAEmitterLayerCircle: &'static CAEmitterLayerEmitterShape);

extern_static!(kCAEmitterLayerSphere: &'static CAEmitterLayerEmitterShape);

extern_static!(kCAEmitterLayerPoints: &'static CAEmitterLayerEmitterMode);

extern_static!(kCAEmitterLayerOutline: &'static CAEmitterLayerEmitterMode);

extern_static!(kCAEmitterLayerSurface: &'static CAEmitterLayerEmitterMode);

extern_static!(kCAEmitterLayerVolume: &'static CAEmitterLayerEmitterMode);

extern_static!(kCAEmitterLayerUnordered: &'static CAEmitterLayerRenderMode);

extern_static!(kCAEmitterLayerOldestFirst: &'static CAEmitterLayerRenderMode);

extern_static!(kCAEmitterLayerOldestLast: &'static CAEmitterLayerRenderMode);

extern_static!(kCAEmitterLayerBackToFront: &'static CAEmitterLayerRenderMode);

extern_static!(kCAEmitterLayerAdditive: &'static CAEmitterLayerRenderMode);
