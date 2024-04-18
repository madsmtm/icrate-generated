//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type CALayerContentsGravity = NSString;

// NS_TYPED_ENUM
pub type CALayerContentsFormat = NSString;

// NS_TYPED_ENUM
pub type CALayerContentsFilter = NSString;

// NS_TYPED_ENUM
pub type CALayerCornerCurve = NSString;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CAAutoresizingMask(pub c_uint);
bitflags::bitflags! {
    impl CAAutoresizingMask: c_uint {
        const kCALayerNotSizable = 0;
        const kCALayerMinXMargin = 1<<0;
        const kCALayerWidthSizable = 1<<1;
        const kCALayerMaxXMargin = 1<<2;
        const kCALayerMinYMargin = 1<<3;
        const kCALayerHeightSizable = 1<<4;
        const kCALayerMaxYMargin = 1<<5;
    }
}

unsafe impl Encode for CAAutoresizingMask {
    const ENCODING: Encoding = c_uint::ENCODING;
}

unsafe impl RefEncode for CAAutoresizingMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CAEdgeAntialiasingMask(pub c_uint);
bitflags::bitflags! {
    impl CAEdgeAntialiasingMask: c_uint {
        const kCALayerLeftEdge = 1<<0;
        const kCALayerRightEdge = 1<<1;
        const kCALayerBottomEdge = 1<<2;
        const kCALayerTopEdge = 1<<3;
    }
}

unsafe impl Encode for CAEdgeAntialiasingMask {
    const ENCODING: Encoding = c_uint::ENCODING;
}

unsafe impl RefEncode for CAEdgeAntialiasingMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CACornerMask(pub NSUInteger);
bitflags::bitflags! {
    impl CACornerMask: NSUInteger {
        const kCALayerMinXMinYCorner = 1<<0;
        const kCALayerMaxXMinYCorner = 1<<1;
        const kCALayerMinXMaxYCorner = 1<<2;
        const kCALayerMaxXMaxYCorner = 1<<3;
    }
}

unsafe impl Encode for CACornerMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CACornerMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CALayer;

    unsafe impl ClassType for CALayer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CAMediaTiming")]
unsafe impl CAMediaTiming for CALayer {}

unsafe impl NSCoding for CALayer {}

unsafe impl NSObjectProtocol for CALayer {}

unsafe impl NSSecureCoding for CALayer {}

extern_methods!(
    unsafe impl CALayer {
        #[method_id(@__retain_semantics Other layer)]
        pub fn layer() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn init_with_layer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;

        #[method_id(@__retain_semantics Other presentationLayer)]
        pub unsafe fn presentation_layer(&self) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other modelLayer)]
        pub unsafe fn model_layer(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other defaultValueForKey:)]
        pub unsafe fn default_value_for_key(key: &NSString) -> Option<Retained<AnyObject>>;

        #[method(needsDisplayForKey:)]
        pub unsafe fn needs_display_for_key(key: &NSString) -> bool;

        #[method(shouldArchiveValueForKey:)]
        pub unsafe fn should_archive_value_for_key(&self, key: &NSString) -> bool;

        #[method(bounds)]
        pub fn bounds(&self) -> CGRect;

        #[method(setBounds:)]
        pub fn set_bounds(&self, bounds: CGRect);

        #[method(position)]
        pub fn position(&self) -> CGPoint;

        #[method(setPosition:)]
        pub fn set_position(&self, position: CGPoint);

        #[method(zPosition)]
        pub fn z_position(&self) -> CGFloat;

        #[method(setZPosition:)]
        pub fn set_z_position(&self, z_position: CGFloat);

        #[method(anchorPoint)]
        pub fn anchor_point(&self) -> CGPoint;

        #[method(setAnchorPoint:)]
        pub fn set_anchor_point(&self, anchor_point: CGPoint);

        #[method(anchorPointZ)]
        pub fn anchor_point_z(&self) -> CGFloat;

        #[method(setAnchorPointZ:)]
        pub fn set_anchor_point_z(&self, anchor_point_z: CGFloat);

        #[cfg(feature = "CATransform3D")]
        #[method(transform)]
        pub fn transform(&self) -> CATransform3D;

        #[cfg(feature = "CATransform3D")]
        #[method(setTransform:)]
        pub fn set_transform(&self, transform: CATransform3D);

        #[method(frame)]
        pub fn frame(&self) -> CGRect;

        #[method(setFrame:)]
        pub fn set_frame(&self, frame: CGRect);

        #[method(isHidden)]
        pub fn is_hidden(&self) -> bool;

        #[method(setHidden:)]
        pub fn set_hidden(&self, hidden: bool);

        #[method(isDoubleSided)]
        pub fn is_double_sided(&self) -> bool;

        #[method(setDoubleSided:)]
        pub fn set_double_sided(&self, double_sided: bool);

        #[method(isGeometryFlipped)]
        pub fn is_geometry_flipped(&self) -> bool;

        #[method(setGeometryFlipped:)]
        pub fn set_geometry_flipped(&self, geometry_flipped: bool);

        #[method(contentsAreFlipped)]
        pub fn contents_are_flipped(&self) -> bool;

        #[method_id(@__retain_semantics Other superlayer)]
        pub fn superlayer(&self) -> Option<Retained<CALayer>>;

        #[method(removeFromSuperlayer)]
        pub fn remove_from_superlayer(&self);

        #[method_id(@__retain_semantics Other sublayers)]
        pub unsafe fn sublayers(&self) -> Option<Retained<NSArray<CALayer>>>;

        #[method(setSublayers:)]
        pub unsafe fn set_sublayers(&self, sublayers: Option<&NSArray<CALayer>>);

        #[method(addSublayer:)]
        pub fn add_sublayer(&self, layer: &CALayer);

        #[method(insertSublayer:atIndex:)]
        pub fn insert_sublayer_at_index(&self, layer: &CALayer, idx: c_uint);

        #[method(insertSublayer:below:)]
        pub fn insert_sublayer_below(&self, layer: &CALayer, sibling: Option<&CALayer>);

        #[method(insertSublayer:above:)]
        pub fn insert_sublayer_above(&self, layer: &CALayer, sibling: Option<&CALayer>);

        #[method(replaceSublayer:with:)]
        pub unsafe fn replace_sublayer_with(&self, old_layer: &CALayer, new_layer: &CALayer);

        #[cfg(feature = "CATransform3D")]
        #[method(sublayerTransform)]
        pub fn sublayer_transform(&self) -> CATransform3D;

        #[cfg(feature = "CATransform3D")]
        #[method(setSublayerTransform:)]
        pub fn set_sublayer_transform(&self, sublayer_transform: CATransform3D);

        #[method_id(@__retain_semantics Other mask)]
        pub fn mask(&self) -> Option<Retained<CALayer>>;

        #[method(setMask:)]
        pub unsafe fn set_mask(&self, mask: Option<&CALayer>);

        #[method(masksToBounds)]
        pub fn masks_to_bounds(&self) -> bool;

        #[method(setMasksToBounds:)]
        pub fn set_masks_to_bounds(&self, masks_to_bounds: bool);

        #[method(convertPoint:fromLayer:)]
        pub fn convert_point_from_layer(&self, p: CGPoint, l: Option<&CALayer>) -> CGPoint;

        #[method(convertPoint:toLayer:)]
        pub fn convert_point_to_layer(&self, p: CGPoint, l: Option<&CALayer>) -> CGPoint;

        #[method(convertRect:fromLayer:)]
        pub fn convert_rect_from_layer(&self, r: CGRect, l: Option<&CALayer>) -> CGRect;

        #[method(convertRect:toLayer:)]
        pub fn convert_rect_to_layer(&self, r: CGRect, l: Option<&CALayer>) -> CGRect;

        #[method(convertTime:fromLayer:)]
        pub fn convert_time_from_layer(
            &self,
            t: CFTimeInterval,
            l: Option<&CALayer>,
        ) -> CFTimeInterval;

        #[method(convertTime:toLayer:)]
        pub fn convert_time_to_layer(
            &self,
            t: CFTimeInterval,
            l: Option<&CALayer>,
        ) -> CFTimeInterval;

        #[method_id(@__retain_semantics Other hitTest:)]
        pub fn hit_test(&self, p: CGPoint) -> Option<Retained<CALayer>>;

        #[method(containsPoint:)]
        pub fn contains_point(&self, p: CGPoint) -> bool;

        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Retained<AnyObject>>;

        #[method(setContents:)]
        pub unsafe fn set_contents(&self, contents: Option<&AnyObject>);

        #[method(contentsRect)]
        pub fn contents_rect(&self) -> CGRect;

        #[method(setContentsRect:)]
        pub fn set_contents_rect(&self, contents_rect: CGRect);

        #[method_id(@__retain_semantics Other contentsGravity)]
        pub fn contents_gravity(&self) -> Retained<CALayerContentsGravity>;

        #[method(setContentsGravity:)]
        pub fn set_contents_gravity(&self, contents_gravity: &CALayerContentsGravity);

        #[method(contentsScale)]
        pub fn contents_scale(&self) -> CGFloat;

        #[method(setContentsScale:)]
        pub fn set_contents_scale(&self, contents_scale: CGFloat);

        #[method(contentsCenter)]
        pub fn contents_center(&self) -> CGRect;

        #[method(setContentsCenter:)]
        pub fn set_contents_center(&self, contents_center: CGRect);

        #[method_id(@__retain_semantics Other contentsFormat)]
        pub fn contents_format(&self) -> Retained<CALayerContentsFormat>;

        #[method(setContentsFormat:)]
        pub fn set_contents_format(&self, contents_format: &CALayerContentsFormat);

        #[method(wantsExtendedDynamicRangeContent)]
        pub unsafe fn wants_extended_dynamic_range_content(&self) -> bool;

        #[method(setWantsExtendedDynamicRangeContent:)]
        pub unsafe fn set_wants_extended_dynamic_range_content(
            &self,
            wants_extended_dynamic_range_content: bool,
        );

        #[method_id(@__retain_semantics Other minificationFilter)]
        pub fn minification_filter(&self) -> Retained<CALayerContentsFilter>;

        #[method(setMinificationFilter:)]
        pub fn set_minification_filter(&self, minification_filter: &CALayerContentsFilter);

        #[method_id(@__retain_semantics Other magnificationFilter)]
        pub fn magnification_filter(&self) -> Retained<CALayerContentsFilter>;

        #[method(setMagnificationFilter:)]
        pub fn set_magnification_filter(&self, magnification_filter: &CALayerContentsFilter);

        #[method(minificationFilterBias)]
        pub fn minification_filter_bias(&self) -> c_float;

        #[method(setMinificationFilterBias:)]
        pub fn set_minification_filter_bias(&self, minification_filter_bias: c_float);

        #[method(isOpaque)]
        pub fn is_opaque(&self) -> bool;

        #[method(setOpaque:)]
        pub fn set_opaque(&self, opaque: bool);

        #[method(display)]
        pub fn display(&self);

        #[method(setNeedsDisplay)]
        pub fn set_needs_display(&self);

        #[method(setNeedsDisplayInRect:)]
        pub fn set_needs_display_in_rect(&self, r: CGRect);

        #[method(needsDisplay)]
        pub fn needs_display(&self) -> bool;

        #[method(displayIfNeeded)]
        pub fn display_if_needed(&self);

        #[method(needsDisplayOnBoundsChange)]
        pub fn needs_display_on_bounds_change(&self) -> bool;

        #[method(setNeedsDisplayOnBoundsChange:)]
        pub fn set_needs_display_on_bounds_change(&self, needs_display_on_bounds_change: bool);

        #[method(drawsAsynchronously)]
        pub fn draws_asynchronously(&self) -> bool;

        #[method(setDrawsAsynchronously:)]
        pub fn set_draws_asynchronously(&self, draws_asynchronously: bool);

        #[method(edgeAntialiasingMask)]
        pub fn edge_antialiasing_mask(&self) -> CAEdgeAntialiasingMask;

        #[method(setEdgeAntialiasingMask:)]
        pub fn set_edge_antialiasing_mask(&self, edge_antialiasing_mask: CAEdgeAntialiasingMask);

        #[method(allowsEdgeAntialiasing)]
        pub fn allows_edge_antialiasing(&self) -> bool;

        #[method(setAllowsEdgeAntialiasing:)]
        pub fn set_allows_edge_antialiasing(&self, allows_edge_antialiasing: bool);

        #[method(cornerRadius)]
        pub fn corner_radius(&self) -> CGFloat;

        #[method(setCornerRadius:)]
        pub fn set_corner_radius(&self, corner_radius: CGFloat);

        #[method(maskedCorners)]
        pub fn masked_corners(&self) -> CACornerMask;

        #[method(setMaskedCorners:)]
        pub fn set_masked_corners(&self, masked_corners: CACornerMask);

        #[method_id(@__retain_semantics Other cornerCurve)]
        pub fn corner_curve(&self) -> Retained<CALayerCornerCurve>;

        #[method(setCornerCurve:)]
        pub fn set_corner_curve(&self, corner_curve: &CALayerCornerCurve);

        #[method(cornerCurveExpansionFactor:)]
        pub fn corner_curve_expansion_factor(curve: &CALayerCornerCurve) -> CGFloat;

        #[method(borderWidth)]
        pub fn border_width(&self) -> CGFloat;

        #[method(setBorderWidth:)]
        pub fn set_border_width(&self, border_width: CGFloat);

        #[method(opacity)]
        pub fn opacity(&self) -> c_float;

        #[method(setOpacity:)]
        pub fn set_opacity(&self, opacity: c_float);

        #[method(allowsGroupOpacity)]
        pub fn allows_group_opacity(&self) -> bool;

        #[method(setAllowsGroupOpacity:)]
        pub fn set_allows_group_opacity(&self, allows_group_opacity: bool);

        #[method_id(@__retain_semantics Other compositingFilter)]
        pub unsafe fn compositing_filter(&self) -> Option<Retained<AnyObject>>;

        #[method(setCompositingFilter:)]
        pub unsafe fn set_compositing_filter(&self, compositing_filter: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other filters)]
        pub unsafe fn filters(&self) -> Option<Retained<NSArray>>;

        #[method(setFilters:)]
        pub unsafe fn set_filters(&self, filters: Option<&NSArray>);

        #[method_id(@__retain_semantics Other backgroundFilters)]
        pub unsafe fn background_filters(&self) -> Option<Retained<NSArray>>;

        #[method(setBackgroundFilters:)]
        pub unsafe fn set_background_filters(&self, background_filters: Option<&NSArray>);

        #[method(shouldRasterize)]
        pub fn should_rasterize(&self) -> bool;

        #[method(setShouldRasterize:)]
        pub fn set_should_rasterize(&self, should_rasterize: bool);

        #[method(rasterizationScale)]
        pub fn rasterization_scale(&self) -> CGFloat;

        #[method(setRasterizationScale:)]
        pub fn set_rasterization_scale(&self, rasterization_scale: CGFloat);

        #[method(shadowOpacity)]
        pub fn shadow_opacity(&self) -> c_float;

        #[method(setShadowOpacity:)]
        pub fn set_shadow_opacity(&self, shadow_opacity: c_float);

        #[method(shadowOffset)]
        pub fn shadow_offset(&self) -> CGSize;

        #[method(setShadowOffset:)]
        pub fn set_shadow_offset(&self, shadow_offset: CGSize);

        #[method(shadowRadius)]
        pub fn shadow_radius(&self) -> CGFloat;

        #[method(setShadowRadius:)]
        pub fn set_shadow_radius(&self, shadow_radius: CGFloat);

        #[method(autoresizingMask)]
        pub fn autoresizing_mask(&self) -> CAAutoresizingMask;

        #[method(setAutoresizingMask:)]
        pub fn set_autoresizing_mask(&self, autoresizing_mask: CAAutoresizingMask);

        #[method_id(@__retain_semantics Other layoutManager)]
        pub fn layout_manager(&self) -> Option<Retained<ProtocolObject<dyn CALayoutManager>>>;

        #[method(setLayoutManager:)]
        pub fn set_layout_manager(
            &self,
            layout_manager: Option<&ProtocolObject<dyn CALayoutManager>>,
        );

        #[method(preferredFrameSize)]
        pub fn preferred_frame_size(&self) -> CGSize;

        #[method(setNeedsLayout)]
        pub fn set_needs_layout(&self);

        #[method(needsLayout)]
        pub fn needs_layout(&self) -> bool;

        #[method(layoutIfNeeded)]
        pub fn layout_if_needed(&self);

        #[method(layoutSublayers)]
        pub fn layout_sublayers(&self);

        #[method(resizeSublayersWithOldSize:)]
        pub fn resize_sublayers_with_old_size(&self, size: CGSize);

        #[method(resizeWithOldSuperlayerSize:)]
        pub fn resize_with_old_superlayer_size(&self, size: CGSize);

        #[method_id(@__retain_semantics Other defaultActionForKey:)]
        pub fn default_action_for_key(
            event: &NSString,
        ) -> Option<Retained<ProtocolObject<dyn CAAction>>>;

        #[method_id(@__retain_semantics Other actionForKey:)]
        pub fn action_for_key(
            &self,
            event: &NSString,
        ) -> Option<Retained<ProtocolObject<dyn CAAction>>>;

        #[method_id(@__retain_semantics Other actions)]
        pub fn actions(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, ProtocolObject<dyn CAAction>>>>;

        #[method(setActions:)]
        pub fn set_actions(
            &self,
            actions: Option<&NSDictionary<NSString, ProtocolObject<dyn CAAction>>>,
        );

        #[cfg(feature = "CAAnimation")]
        #[method(addAnimation:forKey:)]
        pub fn add_animation_for_key(&self, anim: &CAAnimation, key: Option<&NSString>);

        #[method(removeAllAnimations)]
        pub fn remove_all_animations(&self);

        #[method(removeAnimationForKey:)]
        pub fn remove_animation_for_key(&self, key: &NSString);

        #[method_id(@__retain_semantics Other animationKeys)]
        pub fn animation_keys(&self) -> Option<Retained<NSArray<NSString>>>;

        #[cfg(feature = "CAAnimation")]
        #[method_id(@__retain_semantics Other animationForKey:)]
        pub unsafe fn animation_for_key(&self, key: &NSString) -> Option<Retained<CAAnimation>>;

        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Option<Retained<NSString>>;

        #[method(setName:)]
        pub fn set_name(&self, name: Option<&NSString>);

        #[method_id(@__retain_semantics Other delegate)]
        pub fn delegate(&self) -> Option<Retained<ProtocolObject<dyn CALayerDelegate>>>;

        #[method(setDelegate:)]
        pub fn set_delegate(&self, delegate: Option<&ProtocolObject<dyn CALayerDelegate>>);

        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Retained<NSDictionary>>;

        #[method(setStyle:)]
        pub unsafe fn set_style(&self, style: Option<&NSDictionary>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CALayer {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for CALayer {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

extern_protocol!(
    pub unsafe trait CALayoutManager: NSObjectProtocol {
        #[optional]
        #[method(preferredSizeOfLayer:)]
        unsafe fn preferred_size_of_layer(&self, layer: &CALayer) -> CGSize;

        #[optional]
        #[method(invalidateLayoutOfLayer:)]
        unsafe fn invalidate_layout_of_layer(&self, layer: &CALayer);

        #[optional]
        #[method(layoutSublayersOfLayer:)]
        unsafe fn layout_sublayers_of_layer(&self, layer: &CALayer);
    }

    unsafe impl ProtocolType for dyn CALayoutManager {}
);

extern_protocol!(
    pub unsafe trait CAAction {
        #[method(runActionForKey:object:arguments:)]
        unsafe fn run_action_for_key_object_arguments(
            &self,
            event: &NSString,
            an_object: &AnyObject,
            dict: Option<&NSDictionary>,
        );
    }

    unsafe impl ProtocolType for dyn CAAction {}
);

unsafe impl CAAction for NSNull {}

extern_protocol!(
    pub unsafe trait CALayerDelegate: NSObjectProtocol {
        #[optional]
        #[method(displayLayer:)]
        unsafe fn display_layer(&self, layer: &CALayer);

        #[optional]
        #[method(layerWillDraw:)]
        unsafe fn layer_will_draw(&self, layer: &CALayer);

        #[optional]
        #[method(layoutSublayersOfLayer:)]
        unsafe fn layout_sublayers_of_layer(&self, layer: &CALayer);

        #[optional]
        #[method_id(@__retain_semantics Other actionForLayer:forKey:)]
        unsafe fn action_for_layer_for_key(
            &self,
            layer: &CALayer,
            event: &NSString,
        ) -> Option<Retained<ProtocolObject<dyn CAAction>>>;
    }

    unsafe impl ProtocolType for dyn CALayerDelegate {}
);

extern "C" {
    pub static kCAGravityCenter: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAGravityTop: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAGravityBottom: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAGravityLeft: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAGravityRight: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAGravityTopLeft: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAGravityTopRight: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAGravityBottomLeft: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAGravityBottomRight: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAGravityResize: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAGravityResizeAspect: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAGravityResizeAspectFill: &'static CALayerContentsGravity;
}

extern "C" {
    pub static kCAContentsFormatRGBA8Uint: &'static CALayerContentsFormat;
}

extern "C" {
    pub static kCAContentsFormatRGBA16Float: &'static CALayerContentsFormat;
}

extern "C" {
    pub static kCAContentsFormatGray8Uint: &'static CALayerContentsFormat;
}

extern "C" {
    pub static kCAFilterNearest: &'static CALayerContentsFilter;
}

extern "C" {
    pub static kCAFilterLinear: &'static CALayerContentsFilter;
}

extern "C" {
    pub static kCAFilterTrilinear: &'static CALayerContentsFilter;
}

extern "C" {
    pub static kCACornerCurveCircular: &'static CALayerCornerCurve;
}

extern "C" {
    pub static kCACornerCurveContinuous: &'static CALayerCornerCurve;
}

extern "C" {
    pub static kCAOnOrderIn: &'static NSString;
}

extern "C" {
    pub static kCAOnOrderOut: &'static NSString;
}

extern "C" {
    pub static kCATransition: &'static NSString;
}
