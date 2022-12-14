//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#[path = "CAAnimation.rs"]
mod __CAAnimation;
#[path = "CABase.rs"]
mod __CABase;
#[path = "CAConstraintLayoutManager.rs"]
mod __CAConstraintLayoutManager;
#[path = "CADisplayLink.rs"]
mod __CADisplayLink;
#[path = "CAEDRMetadata.rs"]
mod __CAEDRMetadata;
#[path = "CAEmitterCell.rs"]
mod __CAEmitterCell;
#[path = "CAEmitterLayer.rs"]
mod __CAEmitterLayer;
#[path = "CAFrameRateRange.rs"]
mod __CAFrameRateRange;
#[path = "CAGradientLayer.rs"]
mod __CAGradientLayer;
#[path = "CALayer.rs"]
mod __CALayer;
#[path = "CAMediaTiming.rs"]
mod __CAMediaTiming;
#[path = "CAMediaTimingFunction.rs"]
mod __CAMediaTimingFunction;
#[path = "CAMetalLayer.rs"]
mod __CAMetalLayer;
#[path = "CAOpenGLLayer.rs"]
mod __CAOpenGLLayer;
#[path = "CARemoteLayerClient.rs"]
mod __CARemoteLayerClient;
#[path = "CARemoteLayerServer.rs"]
mod __CARemoteLayerServer;
#[path = "CARenderer.rs"]
mod __CARenderer;
#[path = "CAReplicatorLayer.rs"]
mod __CAReplicatorLayer;
#[path = "CAScrollLayer.rs"]
mod __CAScrollLayer;
#[path = "CAShapeLayer.rs"]
mod __CAShapeLayer;
#[path = "CATextLayer.rs"]
mod __CATextLayer;
#[path = "CATiledLayer.rs"]
mod __CATiledLayer;
#[path = "CATransaction.rs"]
mod __CATransaction;
#[path = "CATransform3D.rs"]
mod __CATransform3D;
#[path = "CATransformLayer.rs"]
mod __CATransformLayer;
#[path = "CAValueFunction.rs"]
mod __CAValueFunction;

pub use self::__CAAnimation::{
    kCAAnimationCubic, kCAAnimationCubicPaced, kCAAnimationDiscrete, kCAAnimationLinear,
    kCAAnimationPaced, kCAAnimationRotateAuto, kCAAnimationRotateAutoReverse, kCATransitionFade,
    kCATransitionFromBottom, kCATransitionFromLeft, kCATransitionFromRight, kCATransitionFromTop,
    kCATransitionMoveIn, kCATransitionPush, kCATransitionReveal, CAAnimation,
    CAAnimationCalculationMode, CAAnimationDelegate, CAAnimationGroup, CAAnimationRotationMode,
    CABasicAnimation, CAKeyframeAnimation, CAPropertyAnimation, CASpringAnimation, CATransition,
    CATransitionSubtype, CATransitionType,
};
pub use self::__CABase::CACurrentMediaTime;
pub use self::__CAConstraintLayoutManager::{
    kCAConstraintHeight, kCAConstraintMaxX, kCAConstraintMaxY, kCAConstraintMidX,
    kCAConstraintMidY, kCAConstraintMinX, kCAConstraintMinY, kCAConstraintWidth, CAConstraint,
    CAConstraintAttribute, CAConstraintLayoutManager,
};
pub use self::__CADisplayLink::CADisplayLink;
pub use self::__CAEDRMetadata::CAEDRMetadata;
pub use self::__CAEmitterCell::CAEmitterCell;
pub use self::__CAEmitterLayer::{
    kCAEmitterLayerAdditive, kCAEmitterLayerBackToFront, kCAEmitterLayerCircle,
    kCAEmitterLayerCuboid, kCAEmitterLayerLine, kCAEmitterLayerOldestFirst,
    kCAEmitterLayerOldestLast, kCAEmitterLayerOutline, kCAEmitterLayerPoint, kCAEmitterLayerPoints,
    kCAEmitterLayerRectangle, kCAEmitterLayerSphere, kCAEmitterLayerSurface,
    kCAEmitterLayerUnordered, kCAEmitterLayerVolume, CAEmitterLayer, CAEmitterLayerEmitterMode,
    CAEmitterLayerEmitterShape, CAEmitterLayerRenderMode,
};
pub use self::__CAFrameRateRange::{
    CAFrameRateRange, CAFrameRateRangeDefault, CAFrameRateRangeIsEqualToRange, CAFrameRateRangeMake,
};
pub use self::__CAGradientLayer::{
    kCAGradientLayerAxial, kCAGradientLayerConic, kCAGradientLayerRadial, CAGradientLayer,
    CAGradientLayerType,
};
pub use self::__CALayer::{
    kCAContentsFormatGray8Uint, kCAContentsFormatRGBA16Float, kCAContentsFormatRGBA8Uint,
    kCACornerCurveCircular, kCACornerCurveContinuous, kCAFilterLinear, kCAFilterNearest,
    kCAFilterTrilinear, kCAGravityBottom, kCAGravityBottomLeft, kCAGravityBottomRight,
    kCAGravityCenter, kCAGravityLeft, kCAGravityResize, kCAGravityResizeAspect,
    kCAGravityResizeAspectFill, kCAGravityRight, kCAGravityTop, kCAGravityTopLeft,
    kCAGravityTopRight, kCALayerBottomEdge, kCALayerHeightSizable, kCALayerLeftEdge,
    kCALayerMaxXMargin, kCALayerMaxXMaxYCorner, kCALayerMaxXMinYCorner, kCALayerMaxYMargin,
    kCALayerMinXMargin, kCALayerMinXMaxYCorner, kCALayerMinXMinYCorner, kCALayerMinYMargin,
    kCALayerNotSizable, kCALayerRightEdge, kCALayerTopEdge, kCALayerWidthSizable, kCAOnOrderIn,
    kCAOnOrderOut, kCATransition, CAAction, CAAutoresizingMask, CACornerMask,
    CAEdgeAntialiasingMask, CALayer, CALayerContentsFilter, CALayerContentsFormat,
    CALayerContentsGravity, CALayerCornerCurve, CALayerDelegate, CALayoutManager,
};
pub use self::__CAMediaTiming::{
    kCAFillModeBackwards, kCAFillModeBoth, kCAFillModeForwards, kCAFillModeRemoved, CAMediaTiming,
    CAMediaTimingFillMode,
};
pub use self::__CAMediaTimingFunction::{
    kCAMediaTimingFunctionDefault, kCAMediaTimingFunctionEaseIn,
    kCAMediaTimingFunctionEaseInEaseOut, kCAMediaTimingFunctionEaseOut,
    kCAMediaTimingFunctionLinear, CAMediaTimingFunction, CAMediaTimingFunctionName,
};
pub use self::__CARemoteLayerClient::CARemoteLayerClient;
pub use self::__CARemoteLayerServer::CARemoteLayerServer;
pub use self::__CARenderer::{kCARendererColorSpace, kCARendererMetalCommandQueue, CARenderer};
pub use self::__CAReplicatorLayer::CAReplicatorLayer;
pub use self::__CAScrollLayer::{
    kCAScrollBoth, kCAScrollHorizontally, kCAScrollNone, kCAScrollVertically, CAScrollLayer,
    CAScrollLayerScrollMode,
};
pub use self::__CAShapeLayer::{
    kCAFillRuleEvenOdd, kCAFillRuleNonZero, kCALineCapButt, kCALineCapRound, kCALineCapSquare,
    kCALineJoinBevel, kCALineJoinMiter, kCALineJoinRound, CAShapeLayer, CAShapeLayerFillRule,
    CAShapeLayerLineCap, CAShapeLayerLineJoin,
};
pub use self::__CATextLayer::{
    kCAAlignmentCenter, kCAAlignmentJustified, kCAAlignmentLeft, kCAAlignmentNatural,
    kCAAlignmentRight, kCATruncationEnd, kCATruncationMiddle, kCATruncationNone,
    kCATruncationStart, CATextLayer, CATextLayerAlignmentMode, CATextLayerTruncationMode,
};
pub use self::__CATiledLayer::CATiledLayer;
pub use self::__CATransaction::{
    kCATransactionAnimationDuration, kCATransactionAnimationTimingFunction,
    kCATransactionCompletionBlock, kCATransactionDisableActions, CATransaction,
};
pub use self::__CATransform3D::{
    CATransform3D, CATransform3DConcat, CATransform3DEqualToTransform, CATransform3DIdentity,
    CATransform3DInvert, CATransform3DIsAffine, CATransform3DIsIdentity, CATransform3DMakeRotation,
    CATransform3DMakeScale, CATransform3DMakeTranslation, CATransform3DRotate, CATransform3DScale,
    CATransform3DTranslate,
};
pub use self::__CATransformLayer::CATransformLayer;
pub use self::__CAValueFunction::{
    kCAValueFunctionRotateX, kCAValueFunctionRotateY, kCAValueFunctionRotateZ,
    kCAValueFunctionScale, kCAValueFunctionScaleX, kCAValueFunctionScaleY, kCAValueFunctionScaleZ,
    kCAValueFunctionTranslate, kCAValueFunctionTranslateX, kCAValueFunctionTranslateY,
    kCAValueFunctionTranslateZ, CAValueFunction, CAValueFunctionName,
};
