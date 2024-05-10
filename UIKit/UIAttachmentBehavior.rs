//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAttachmentBehaviorType(pub NSInteger);
impl UIAttachmentBehaviorType {
    #[doc(alias = "UIAttachmentBehaviorTypeItems")]
    pub const Items: Self = Self(0);
    #[doc(alias = "UIAttachmentBehaviorTypeAnchor")]
    pub const Anchor: Self = Self(1);
}

unsafe impl Encode for UIAttachmentBehaviorType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAttachmentBehaviorType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UIFloatRange {
    pub minimum: CGFloat,
    pub maximum: CGFloat,
}

unsafe impl Encode for UIFloatRange {
    const ENCODING: Encoding = Encoding::Struct("?", &[<CGFloat>::ENCODING, <CGFloat>::ENCODING]);
}

unsafe impl RefEncode for UIFloatRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl Send for UIFloatRange {}

unsafe impl Sync for UIFloatRange {}

extern "C" {
    pub static UIFloatRangeZero: UIFloatRange;
}

extern "C" {
    pub static UIFloatRangeInfinite: UIFloatRange;
}

extern "C" {
    pub fn UIFloatRangeIsInfinite(range: UIFloatRange) -> Bool;
}

// TODO: pub fn UIFloatRangeMake(minimum: CGFloat,maximum: CGFloat,) -> UIFloatRange;

// TODO: pub fn UIFloatRangeIsEqualToRange(range: UIFloatRange,other_range: UIFloatRange,) -> Bool;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIDynamicBehavior")]
    pub struct UIAttachmentBehavior;

    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl ClassType for UIAttachmentBehavior {
        #[inherits(NSObject)]
        type Super = UIDynamicBehavior;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIDynamicBehavior")]
unsafe impl NSObjectProtocol for UIAttachmentBehavior {}

extern_methods!(
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIAttachmentBehavior {
        #[method_id(@__retain_semantics Init initWithItem:attachedToAnchor:)]
        pub unsafe fn initWithItem_attachedToAnchor(
            this: Allocated<Self>,
            item: &ProtocolObject<dyn UIDynamicItem>,
            point: CGPoint,
        ) -> Id<Self>;

        #[cfg(feature = "UIGeometry")]
        #[method_id(@__retain_semantics Init initWithItem:offsetFromCenter:attachedToAnchor:)]
        pub unsafe fn initWithItem_offsetFromCenter_attachedToAnchor(
            this: Allocated<Self>,
            item: &ProtocolObject<dyn UIDynamicItem>,
            offset: UIOffset,
            point: CGPoint,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithItem:attachedToItem:)]
        pub unsafe fn initWithItem_attachedToItem(
            this: Allocated<Self>,
            item1: &ProtocolObject<dyn UIDynamicItem>,
            item2: &ProtocolObject<dyn UIDynamicItem>,
        ) -> Id<Self>;

        #[cfg(feature = "UIGeometry")]
        #[method_id(@__retain_semantics Init initWithItem:offsetFromCenter:attachedToItem:offsetFromCenter:)]
        pub unsafe fn initWithItem_offsetFromCenter_attachedToItem_offsetFromCenter(
            this: Allocated<Self>,
            item1: &ProtocolObject<dyn UIDynamicItem>,
            offset1: UIOffset,
            item2: &ProtocolObject<dyn UIDynamicItem>,
            offset2: UIOffset,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other slidingAttachmentWithItem:attachedToItem:attachmentAnchor:axisOfTranslation:)]
        pub unsafe fn slidingAttachmentWithItem_attachedToItem_attachmentAnchor_axisOfTranslation(
            item1: &ProtocolObject<dyn UIDynamicItem>,
            item2: &ProtocolObject<dyn UIDynamicItem>,
            point: CGPoint,
            axis: CGVector,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other slidingAttachmentWithItem:attachmentAnchor:axisOfTranslation:)]
        pub unsafe fn slidingAttachmentWithItem_attachmentAnchor_axisOfTranslation(
            item: &ProtocolObject<dyn UIDynamicItem>,
            point: CGPoint,
            axis: CGVector,
        ) -> Id<Self>;

        #[cfg(feature = "UIGeometry")]
        #[method_id(@__retain_semantics Other limitAttachmentWithItem:offsetFromCenter:attachedToItem:offsetFromCenter:)]
        pub unsafe fn limitAttachmentWithItem_offsetFromCenter_attachedToItem_offsetFromCenter(
            item1: &ProtocolObject<dyn UIDynamicItem>,
            offset1: UIOffset,
            item2: &ProtocolObject<dyn UIDynamicItem>,
            offset2: UIOffset,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other fixedAttachmentWithItem:attachedToItem:attachmentAnchor:)]
        pub unsafe fn fixedAttachmentWithItem_attachedToItem_attachmentAnchor(
            item1: &ProtocolObject<dyn UIDynamicItem>,
            item2: &ProtocolObject<dyn UIDynamicItem>,
            point: CGPoint,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other pinAttachmentWithItem:attachedToItem:attachmentAnchor:)]
        pub unsafe fn pinAttachmentWithItem_attachedToItem_attachmentAnchor(
            item1: &ProtocolObject<dyn UIDynamicItem>,
            item2: &ProtocolObject<dyn UIDynamicItem>,
            point: CGPoint,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Id<NSArray<ProtocolObject<dyn UIDynamicItem>>>;

        #[method(attachedBehaviorType)]
        pub unsafe fn attachedBehaviorType(&self) -> UIAttachmentBehaviorType;

        #[method(anchorPoint)]
        pub unsafe fn anchorPoint(&self) -> CGPoint;

        #[method(setAnchorPoint:)]
        pub unsafe fn setAnchorPoint(&self, anchor_point: CGPoint);

        #[method(length)]
        pub unsafe fn length(&self) -> CGFloat;

        #[method(setLength:)]
        pub unsafe fn setLength(&self, length: CGFloat);

        #[method(damping)]
        pub unsafe fn damping(&self) -> CGFloat;

        #[method(setDamping:)]
        pub unsafe fn setDamping(&self, damping: CGFloat);

        #[method(frequency)]
        pub unsafe fn frequency(&self) -> CGFloat;

        #[method(setFrequency:)]
        pub unsafe fn setFrequency(&self, frequency: CGFloat);

        #[method(frictionTorque)]
        pub unsafe fn frictionTorque(&self) -> CGFloat;

        #[method(setFrictionTorque:)]
        pub unsafe fn setFrictionTorque(&self, friction_torque: CGFloat);

        #[method(attachmentRange)]
        pub unsafe fn attachmentRange(&self) -> UIFloatRange;

        #[method(setAttachmentRange:)]
        pub unsafe fn setAttachmentRange(&self, attachment_range: UIFloatRange);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIAttachmentBehavior {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);