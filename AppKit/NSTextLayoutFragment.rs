//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextLayoutFragmentEnumerationOptions(pub NSUInteger);
impl NSTextLayoutFragmentEnumerationOptions {
    #[doc(alias = "NSTextLayoutFragmentEnumerationOptionsNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSTextLayoutFragmentEnumerationOptionsReverse")]
    pub const Reverse: Self = Self(1 << 0);
    #[doc(alias = "NSTextLayoutFragmentEnumerationOptionsEstimatesSize")]
    pub const EstimatesSize: Self = Self(1 << 1);
    #[doc(alias = "NSTextLayoutFragmentEnumerationOptionsEnsuresLayout")]
    pub const EnsuresLayout: Self = Self(1 << 2);
    #[doc(alias = "NSTextLayoutFragmentEnumerationOptionsEnsuresExtraLineFragment")]
    pub const EnsuresExtraLineFragment: Self = Self(1 << 3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSTextLayoutFragmentEnumerationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSTextLayoutFragmentEnumerationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextLayoutFragmentState(pub NSUInteger);
impl NSTextLayoutFragmentState {
    #[doc(alias = "NSTextLayoutFragmentStateNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSTextLayoutFragmentStateEstimatedUsageBounds")]
    pub const EstimatedUsageBounds: Self = Self(1);
    #[doc(alias = "NSTextLayoutFragmentStateCalculatedUsageBounds")]
    pub const CalculatedUsageBounds: Self = Self(2);
    #[doc(alias = "NSTextLayoutFragmentStateLayoutAvailable")]
    pub const LayoutAvailable: Self = Self(3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSTextLayoutFragmentState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSTextLayoutFragmentState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextLayoutFragment;

    unsafe impl ClassType for NSTextLayoutFragment {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSTextLayoutFragment {}

unsafe impl NSObjectProtocol for NSTextLayoutFragment {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSTextLayoutFragment {}

extern_methods!(
    unsafe impl NSTextLayoutFragment {
        #[cfg(all(feature = "AppKit_NSTextElement", feature = "AppKit_NSTextRange"))]
        #[method_id(@__retain_semantics Init initWithTextElement:range:)]
        pub unsafe fn initWithTextElement_range(
            this: Allocated<Self>,
            text_element: &NSTextElement,
            range_in_element: Option<&NSTextRange>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager>>;

        #[cfg(feature = "AppKit_NSTextElement")]
        #[method_id(@__retain_semantics Other textElement)]
        pub unsafe fn textElement(&self) -> Option<Id<NSTextElement>>;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other rangeInElement)]
        pub unsafe fn rangeInElement(&self) -> Id<NSTextRange>;

        #[cfg(all(feature = "AppKit_NSTextLineFragment", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textLineFragments)]
        pub unsafe fn textLineFragments(&self) -> Id<NSArray<NSTextLineFragment>>;

        #[cfg(all(
            feature = "AppKit_NSTextLineFragment",
            feature = "Foundation_NSGeometry"
        ))]
        #[method_id(@__retain_semantics Other textLineFragmentForVerticalOffset:requiresExactMatch:)]
        pub unsafe fn textLineFragmentForVerticalOffset_requiresExactMatch(
            &self,
            vertical_offset: CGFloat,
            requires_exact_match: bool,
        ) -> Option<Id<NSTextLineFragment>>;

        #[cfg(all(feature = "AppKit_NSTextLineFragment", feature = "AppKit_NSTextRange"))]
        #[method_id(@__retain_semantics Other textLineFragmentForTextLocation:isUpstreamAffinity:)]
        pub unsafe fn textLineFragmentForTextLocation_isUpstreamAffinity(
            &self,
            text_location: &ProtocolObject<dyn NSTextLocation>,
            is_upstream_affinity: bool,
        ) -> Option<Id<NSTextLineFragment>>;

        #[cfg(feature = "Foundation_NSOperation")]
        #[method_id(@__retain_semantics Other layoutQueue)]
        pub unsafe fn layoutQueue(&self) -> Option<Id<NSOperationQueue>>;

        #[cfg(feature = "Foundation_NSOperation")]
        #[method(setLayoutQueue:)]
        pub unsafe fn setLayoutQueue(&self, layout_queue: Option<&NSOperationQueue>);

        #[method(state)]
        pub unsafe fn state(&self) -> NSTextLayoutFragmentState;

        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(layoutFragmentFrame)]
        pub unsafe fn layoutFragmentFrame(&self) -> CGRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(renderingSurfaceBounds)]
        pub unsafe fn renderingSurfaceBounds(&self) -> CGRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(leadingPadding)]
        pub unsafe fn leadingPadding(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(trailingPadding)]
        pub unsafe fn trailingPadding(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(topMargin)]
        pub unsafe fn topMargin(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(bottomMargin)]
        pub unsafe fn bottomMargin(&self) -> CGFloat;

        #[cfg(all(feature = "AppKit_NSTextAttachment", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textAttachmentViewProviders)]
        pub unsafe fn textAttachmentViewProviders(
            &self,
        ) -> Id<NSArray<NSTextAttachmentViewProvider>>;

        #[cfg(all(feature = "AppKit_NSTextRange", feature = "Foundation_NSGeometry"))]
        #[method(frameForTextAttachmentAtLocation:)]
        pub unsafe fn frameForTextAttachmentAtLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextLayoutFragment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
