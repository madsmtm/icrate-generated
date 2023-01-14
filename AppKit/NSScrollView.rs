//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSScrollElasticity {
        NSScrollElasticityAutomatic = 0,
        NSScrollElasticityNone = 1,
        NSScrollElasticityAllowed = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrollView")]
    pub struct NSScrollView;

    #[cfg(feature = "AppKit_NSScrollView")]
    unsafe impl ClassType for NSScrollView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSScrollView")]
    unsafe impl NSScrollView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method(frameSizeForContentSize:horizontalScrollerClass:verticalScrollerClass:borderType:controlSize:scrollerStyle:)]
        pub unsafe fn frameSizeForContentSize_horizontalScrollerClass_verticalScrollerClass_borderType_controlSize_scrollerStyle(
            c_size: NSSize,
            horizontal_scroller_class: Option<&Class>,
            vertical_scroller_class: Option<&Class>,
            r#type: NSBorderType,
            control_size: NSControlSize,
            scroller_style: NSScrollerStyle,
        ) -> NSSize;

        #[method(contentSizeForFrameSize:horizontalScrollerClass:verticalScrollerClass:borderType:controlSize:scrollerStyle:)]
        pub unsafe fn contentSizeForFrameSize_horizontalScrollerClass_verticalScrollerClass_borderType_controlSize_scrollerStyle(
            f_size: NSSize,
            horizontal_scroller_class: Option<&Class>,
            vertical_scroller_class: Option<&Class>,
            r#type: NSBorderType,
            control_size: NSControlSize,
            scroller_style: NSScrollerStyle,
        ) -> NSSize;

        #[method(frameSizeForContentSize:hasHorizontalScroller:hasVerticalScroller:borderType:)]
        pub unsafe fn frameSizeForContentSize_hasHorizontalScroller_hasVerticalScroller_borderType(
            c_size: NSSize,
            h_flag: bool,
            v_flag: bool,
            r#type: NSBorderType,
        ) -> NSSize;

        #[method(contentSizeForFrameSize:hasHorizontalScroller:hasVerticalScroller:borderType:)]
        pub unsafe fn contentSizeForFrameSize_hasHorizontalScroller_hasVerticalScroller_borderType(
            f_size: NSSize,
            h_flag: bool,
            v_flag: bool,
            r#type: NSBorderType,
        ) -> NSSize;

        #[method(documentVisibleRect)]
        pub unsafe fn documentVisibleRect(&self) -> NSRect;

        #[method(contentSize)]
        pub unsafe fn contentSize(&self) -> NSSize;

        #[method_id(@__retain_semantics Other documentView)]
        pub unsafe fn documentView(&self) -> Option<Id<NSView, Shared>>;

        #[method(setDocumentView:)]
        pub unsafe fn setDocumentView(&self, document_view: Option<&NSView>);

        #[cfg(feature = "AppKit_NSClipView")]
        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Id<NSClipView, Shared>;

        #[cfg(feature = "AppKit_NSClipView")]
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, content_view: &NSClipView);

        #[cfg(feature = "AppKit_NSCursor")]
        #[method_id(@__retain_semantics Other documentCursor)]
        pub unsafe fn documentCursor(&self) -> Option<Id<NSCursor, Shared>>;

        #[cfg(feature = "AppKit_NSCursor")]
        #[method(setDocumentCursor:)]
        pub unsafe fn setDocumentCursor(&self, document_cursor: Option<&NSCursor>);

        #[method(borderType)]
        pub unsafe fn borderType(&self) -> NSBorderType;

        #[method(setBorderType:)]
        pub unsafe fn setBorderType(&self, border_type: NSBorderType);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[method(hasVerticalScroller)]
        pub unsafe fn hasVerticalScroller(&self) -> bool;

        #[method(setHasVerticalScroller:)]
        pub unsafe fn setHasVerticalScroller(&self, has_vertical_scroller: bool);

        #[method(hasHorizontalScroller)]
        pub unsafe fn hasHorizontalScroller(&self) -> bool;

        #[method(setHasHorizontalScroller:)]
        pub unsafe fn setHasHorizontalScroller(&self, has_horizontal_scroller: bool);

        #[cfg(feature = "AppKit_NSScroller")]
        #[method_id(@__retain_semantics Other verticalScroller)]
        pub unsafe fn verticalScroller(&self) -> Option<Id<NSScroller, Shared>>;

        #[cfg(feature = "AppKit_NSScroller")]
        #[method(setVerticalScroller:)]
        pub unsafe fn setVerticalScroller(&self, vertical_scroller: Option<&NSScroller>);

        #[cfg(feature = "AppKit_NSScroller")]
        #[method_id(@__retain_semantics Other horizontalScroller)]
        pub unsafe fn horizontalScroller(&self) -> Option<Id<NSScroller, Shared>>;

        #[cfg(feature = "AppKit_NSScroller")]
        #[method(setHorizontalScroller:)]
        pub unsafe fn setHorizontalScroller(&self, horizontal_scroller: Option<&NSScroller>);

        #[method(autohidesScrollers)]
        pub unsafe fn autohidesScrollers(&self) -> bool;

        #[method(setAutohidesScrollers:)]
        pub unsafe fn setAutohidesScrollers(&self, autohides_scrollers: bool);

        #[method(horizontalLineScroll)]
        pub unsafe fn horizontalLineScroll(&self) -> CGFloat;

        #[method(setHorizontalLineScroll:)]
        pub unsafe fn setHorizontalLineScroll(&self, horizontal_line_scroll: CGFloat);

        #[method(verticalLineScroll)]
        pub unsafe fn verticalLineScroll(&self) -> CGFloat;

        #[method(setVerticalLineScroll:)]
        pub unsafe fn setVerticalLineScroll(&self, vertical_line_scroll: CGFloat);

        #[method(lineScroll)]
        pub unsafe fn lineScroll(&self) -> CGFloat;

        #[method(setLineScroll:)]
        pub unsafe fn setLineScroll(&self, line_scroll: CGFloat);

        #[method(horizontalPageScroll)]
        pub unsafe fn horizontalPageScroll(&self) -> CGFloat;

        #[method(setHorizontalPageScroll:)]
        pub unsafe fn setHorizontalPageScroll(&self, horizontal_page_scroll: CGFloat);

        #[method(verticalPageScroll)]
        pub unsafe fn verticalPageScroll(&self) -> CGFloat;

        #[method(setVerticalPageScroll:)]
        pub unsafe fn setVerticalPageScroll(&self, vertical_page_scroll: CGFloat);

        #[method(pageScroll)]
        pub unsafe fn pageScroll(&self) -> CGFloat;

        #[method(setPageScroll:)]
        pub unsafe fn setPageScroll(&self, page_scroll: CGFloat);

        #[method(scrollsDynamically)]
        pub unsafe fn scrollsDynamically(&self) -> bool;

        #[method(setScrollsDynamically:)]
        pub unsafe fn setScrollsDynamically(&self, scrolls_dynamically: bool);

        #[method(tile)]
        pub unsafe fn tile(&self);

        #[cfg(feature = "AppKit_NSClipView")]
        #[method(reflectScrolledClipView:)]
        pub unsafe fn reflectScrolledClipView(&self, c_view: &NSClipView);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(scrollWheel:)]
        pub unsafe fn scrollWheel(&self, event: &NSEvent);

        #[method(scrollerStyle)]
        pub unsafe fn scrollerStyle(&self) -> NSScrollerStyle;

        #[method(setScrollerStyle:)]
        pub unsafe fn setScrollerStyle(&self, scroller_style: NSScrollerStyle);

        #[method(scrollerKnobStyle)]
        pub unsafe fn scrollerKnobStyle(&self) -> NSScrollerKnobStyle;

        #[method(setScrollerKnobStyle:)]
        pub unsafe fn setScrollerKnobStyle(&self, scroller_knob_style: NSScrollerKnobStyle);

        #[method(flashScrollers)]
        pub unsafe fn flashScrollers(&self);

        #[method(horizontalScrollElasticity)]
        pub unsafe fn horizontalScrollElasticity(&self) -> NSScrollElasticity;

        #[method(setHorizontalScrollElasticity:)]
        pub unsafe fn setHorizontalScrollElasticity(
            &self,
            horizontal_scroll_elasticity: NSScrollElasticity,
        );

        #[method(verticalScrollElasticity)]
        pub unsafe fn verticalScrollElasticity(&self) -> NSScrollElasticity;

        #[method(setVerticalScrollElasticity:)]
        pub unsafe fn setVerticalScrollElasticity(
            &self,
            vertical_scroll_elasticity: NSScrollElasticity,
        );

        #[method(usesPredominantAxisScrolling)]
        pub unsafe fn usesPredominantAxisScrolling(&self) -> bool;

        #[method(setUsesPredominantAxisScrolling:)]
        pub unsafe fn setUsesPredominantAxisScrolling(&self, uses_predominant_axis_scrolling: bool);

        #[method(allowsMagnification)]
        pub unsafe fn allowsMagnification(&self) -> bool;

        #[method(setAllowsMagnification:)]
        pub unsafe fn setAllowsMagnification(&self, allows_magnification: bool);

        #[method(magnification)]
        pub unsafe fn magnification(&self) -> CGFloat;

        #[method(setMagnification:)]
        pub unsafe fn setMagnification(&self, magnification: CGFloat);

        #[method(maxMagnification)]
        pub unsafe fn maxMagnification(&self) -> CGFloat;

        #[method(setMaxMagnification:)]
        pub unsafe fn setMaxMagnification(&self, max_magnification: CGFloat);

        #[method(minMagnification)]
        pub unsafe fn minMagnification(&self) -> CGFloat;

        #[method(setMinMagnification:)]
        pub unsafe fn setMinMagnification(&self, min_magnification: CGFloat);

        #[method(magnifyToFitRect:)]
        pub unsafe fn magnifyToFitRect(&self, rect: NSRect);

        #[method(setMagnification:centeredAtPoint:)]
        pub unsafe fn setMagnification_centeredAtPoint(
            &self,
            magnification: CGFloat,
            point: NSPoint,
        );

        #[method(addFloatingSubview:forAxis:)]
        pub unsafe fn addFloatingSubview_forAxis(&self, view: &NSView, axis: NSEventGestureAxis);

        #[method(automaticallyAdjustsContentInsets)]
        pub unsafe fn automaticallyAdjustsContentInsets(&self) -> bool;

        #[method(setAutomaticallyAdjustsContentInsets:)]
        pub unsafe fn setAutomaticallyAdjustsContentInsets(
            &self,
            automatically_adjusts_content_insets: bool,
        );

        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSEdgeInsets;

        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, content_insets: NSEdgeInsets);

        #[method(scrollerInsets)]
        pub unsafe fn scrollerInsets(&self) -> NSEdgeInsets;

        #[method(setScrollerInsets:)]
        pub unsafe fn setScrollerInsets(&self, scroller_insets: NSEdgeInsets);
    }
);

extern_static!(NSScrollViewWillStartLiveMagnifyNotification: &'static NSNotificationName);

extern_static!(NSScrollViewDidEndLiveMagnifyNotification: &'static NSNotificationName);

extern_static!(NSScrollViewWillStartLiveScrollNotification: &'static NSNotificationName);

extern_static!(NSScrollViewDidLiveScrollNotification: &'static NSNotificationName);

extern_static!(NSScrollViewDidEndLiveScrollNotification: &'static NSNotificationName);

extern_methods!(
    /// NSRulerSupport
    #[cfg(feature = "AppKit_NSScrollView")]
    unsafe impl NSScrollView {
        #[method(rulerViewClass)]
        pub unsafe fn rulerViewClass() -> Option<&'static Class>;

        #[method(setRulerViewClass:)]
        pub unsafe fn setRulerViewClass(ruler_view_class: Option<&Class>);

        #[method(rulersVisible)]
        pub unsafe fn rulersVisible(&self) -> bool;

        #[method(setRulersVisible:)]
        pub unsafe fn setRulersVisible(&self, rulers_visible: bool);

        #[method(hasHorizontalRuler)]
        pub unsafe fn hasHorizontalRuler(&self) -> bool;

        #[method(setHasHorizontalRuler:)]
        pub unsafe fn setHasHorizontalRuler(&self, has_horizontal_ruler: bool);

        #[method(hasVerticalRuler)]
        pub unsafe fn hasVerticalRuler(&self) -> bool;

        #[method(setHasVerticalRuler:)]
        pub unsafe fn setHasVerticalRuler(&self, has_vertical_ruler: bool);

        #[cfg(feature = "AppKit_NSRulerView")]
        #[method_id(@__retain_semantics Other horizontalRulerView)]
        pub unsafe fn horizontalRulerView(&self) -> Option<Id<NSRulerView, Shared>>;

        #[cfg(feature = "AppKit_NSRulerView")]
        #[method(setHorizontalRulerView:)]
        pub unsafe fn setHorizontalRulerView(&self, horizontal_ruler_view: Option<&NSRulerView>);

        #[cfg(feature = "AppKit_NSRulerView")]
        #[method_id(@__retain_semantics Other verticalRulerView)]
        pub unsafe fn verticalRulerView(&self) -> Option<Id<NSRulerView, Shared>>;

        #[cfg(feature = "AppKit_NSRulerView")]
        #[method(setVerticalRulerView:)]
        pub unsafe fn setVerticalRulerView(&self, vertical_ruler_view: Option<&NSRulerView>);
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSScrollViewFindBarPosition {
        NSScrollViewFindBarPositionAboveHorizontalRuler = 0,
        NSScrollViewFindBarPositionAboveContent = 1,
        NSScrollViewFindBarPositionBelowContent = 2,
    }
);

extern_methods!(
    /// NSFindBarSupport
    #[cfg(feature = "AppKit_NSScrollView")]
    unsafe impl NSScrollView {
        #[method(findBarPosition)]
        pub unsafe fn findBarPosition(&self) -> NSScrollViewFindBarPosition;

        #[method(setFindBarPosition:)]
        pub unsafe fn setFindBarPosition(&self, find_bar_position: NSScrollViewFindBarPosition);
    }
);
