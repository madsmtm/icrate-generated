//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCollectionViewScrollDirection(pub NSInteger);
impl NSCollectionViewScrollDirection {
    #[doc(alias = "NSCollectionViewScrollDirectionVertical")]
    pub const Vertical: Self = Self(0);
    #[doc(alias = "NSCollectionViewScrollDirectionHorizontal")]
    pub const Horizontal: Self = Self(1);
}

unsafe impl Encode for NSCollectionViewScrollDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSCollectionViewScrollDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    #[cfg(feature = "AppKit_NSCollectionView")]
    pub static NSCollectionElementKindSectionHeader:
        &'static NSCollectionViewSupplementaryElementKind;
}

extern "C" {
    #[cfg(feature = "AppKit_NSCollectionView")]
    pub static NSCollectionElementKindSectionFooter:
        &'static NSCollectionViewSupplementaryElementKind;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    pub struct NSCollectionViewFlowLayoutInvalidationContext;

    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    unsafe impl ClassType for NSCollectionViewFlowLayoutInvalidationContext {
        #[inherits(NSObject)]
        type Super = NSCollectionViewLayoutInvalidationContext;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSCollectionViewLayout")]
unsafe impl NSObjectProtocol for NSCollectionViewFlowLayoutInvalidationContext {}

extern_methods!(
    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    unsafe impl NSCollectionViewFlowLayoutInvalidationContext {
        #[method(invalidateFlowLayoutDelegateMetrics)]
        pub unsafe fn invalidateFlowLayoutDelegateMetrics(&self) -> bool;

        #[method(setInvalidateFlowLayoutDelegateMetrics:)]
        pub unsafe fn setInvalidateFlowLayoutDelegateMetrics(
            &self,
            invalidate_flow_layout_delegate_metrics: bool,
        );

        #[method(invalidateFlowLayoutAttributes)]
        pub unsafe fn invalidateFlowLayoutAttributes(&self) -> bool;

        #[method(setInvalidateFlowLayoutAttributes:)]
        pub unsafe fn setInvalidateFlowLayoutAttributes(
            &self,
            invalidate_flow_layout_attributes: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    unsafe impl NSCollectionViewFlowLayoutInvalidationContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    #[cfg(feature = "AppKit_NSCollectionView")]
    pub unsafe trait NSCollectionViewDelegateFlowLayout:
        NSCollectionViewDelegate + IsMainThreadOnly
    {
        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayout",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(collectionView:layout:sizeForItemAtIndexPath:)]
        unsafe fn collectionView_layout_sizeForItemAtIndexPath(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            index_path: &NSIndexPath,
        ) -> NSSize;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayout",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(collectionView:layout:insetForSectionAtIndex:)]
        unsafe fn collectionView_layout_insetForSectionAtIndex(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> NSEdgeInsets;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayout",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(collectionView:layout:minimumLineSpacingForSectionAtIndex:)]
        unsafe fn collectionView_layout_minimumLineSpacingForSectionAtIndex(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> CGFloat;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayout",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(collectionView:layout:minimumInteritemSpacingForSectionAtIndex:)]
        unsafe fn collectionView_layout_minimumInteritemSpacingForSectionAtIndex(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> CGFloat;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayout",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(collectionView:layout:referenceSizeForHeaderInSection:)]
        unsafe fn collectionView_layout_referenceSizeForHeaderInSection(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> NSSize;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayout",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(collectionView:layout:referenceSizeForFooterInSection:)]
        unsafe fn collectionView_layout_referenceSizeForFooterInSection(
            &self,
            collection_view: &NSCollectionView,
            collection_view_layout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> NSSize;
    }

    #[cfg(feature = "AppKit_NSCollectionView")]
    unsafe impl ProtocolType for dyn NSCollectionViewDelegateFlowLayout {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    pub struct NSCollectionViewFlowLayout;

    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    unsafe impl ClassType for NSCollectionViewFlowLayout {
        #[inherits(NSObject)]
        type Super = NSCollectionViewLayout;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSCollectionViewLayout")]
unsafe impl NSCoding for NSCollectionViewFlowLayout {}

#[cfg(feature = "AppKit_NSCollectionViewLayout")]
unsafe impl NSObjectProtocol for NSCollectionViewFlowLayout {}

extern_methods!(
    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    unsafe impl NSCollectionViewFlowLayout {
        #[method(minimumLineSpacing)]
        pub unsafe fn minimumLineSpacing(&self) -> CGFloat;

        #[method(setMinimumLineSpacing:)]
        pub unsafe fn setMinimumLineSpacing(&self, minimum_line_spacing: CGFloat);

        #[method(minimumInteritemSpacing)]
        pub unsafe fn minimumInteritemSpacing(&self) -> CGFloat;

        #[method(setMinimumInteritemSpacing:)]
        pub unsafe fn setMinimumInteritemSpacing(&self, minimum_interitem_spacing: CGFloat);

        #[method(itemSize)]
        pub unsafe fn itemSize(&self) -> NSSize;

        #[method(setItemSize:)]
        pub unsafe fn setItemSize(&self, item_size: NSSize);

        #[method(estimatedItemSize)]
        pub unsafe fn estimatedItemSize(&self) -> NSSize;

        #[method(setEstimatedItemSize:)]
        pub unsafe fn setEstimatedItemSize(&self, estimated_item_size: NSSize);

        #[method(scrollDirection)]
        pub unsafe fn scrollDirection(&self) -> NSCollectionViewScrollDirection;

        #[method(setScrollDirection:)]
        pub unsafe fn setScrollDirection(&self, scroll_direction: NSCollectionViewScrollDirection);

        #[method(headerReferenceSize)]
        pub unsafe fn headerReferenceSize(&self) -> NSSize;

        #[method(setHeaderReferenceSize:)]
        pub unsafe fn setHeaderReferenceSize(&self, header_reference_size: NSSize);

        #[method(footerReferenceSize)]
        pub unsafe fn footerReferenceSize(&self) -> NSSize;

        #[method(setFooterReferenceSize:)]
        pub unsafe fn setFooterReferenceSize(&self, footer_reference_size: NSSize);

        #[method(sectionInset)]
        pub unsafe fn sectionInset(&self) -> NSEdgeInsets;

        #[method(setSectionInset:)]
        pub unsafe fn setSectionInset(&self, section_inset: NSEdgeInsets);

        #[method(sectionHeadersPinToVisibleBounds)]
        pub unsafe fn sectionHeadersPinToVisibleBounds(&self) -> bool;

        #[method(setSectionHeadersPinToVisibleBounds:)]
        pub unsafe fn setSectionHeadersPinToVisibleBounds(
            &self,
            section_headers_pin_to_visible_bounds: bool,
        );

        #[method(sectionFootersPinToVisibleBounds)]
        pub unsafe fn sectionFootersPinToVisibleBounds(&self) -> bool;

        #[method(setSectionFootersPinToVisibleBounds:)]
        pub unsafe fn setSectionFootersPinToVisibleBounds(
            &self,
            section_footers_pin_to_visible_bounds: bool,
        );

        #[method(sectionAtIndexIsCollapsed:)]
        pub unsafe fn sectionAtIndexIsCollapsed(&self, section_index: NSUInteger) -> bool;

        #[method(collapseSectionAtIndex:)]
        pub unsafe fn collapseSectionAtIndex(&self, section_index: NSUInteger);

        #[method(expandSectionAtIndex:)]
        pub unsafe fn expandSectionAtIndex(&self, section_index: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    unsafe impl NSCollectionViewFlowLayout {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
