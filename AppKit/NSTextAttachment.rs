//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        NSAttachmentCharacter = 0xFFFC,
    }
);

extern_protocol!(
    pub unsafe trait NSTextAttachmentContainer: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSTextContainer"))]
        #[method_id(@__retain_semantics Other imageForBounds:textContainer:characterIndex:)]
        unsafe fn imageForBounds_textContainer_characterIndex(
            &self,
            image_bounds: CGRect,
            text_container: Option<&NSTextContainer>,
            char_index: NSUInteger,
        ) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSTextContainer")]
        #[method(attachmentBoundsForTextContainer:proposedLineFragment:glyphPosition:characterIndex:)]
        unsafe fn attachmentBoundsForTextContainer_proposedLineFragment_glyphPosition_characterIndex(
            &self,
            text_container: Option<&NSTextContainer>,
            line_frag: CGRect,
            position: CGPoint,
            char_index: NSUInteger,
        ) -> CGRect;
    }

    unsafe impl ProtocolType for dyn NSTextAttachmentContainer {}
);

extern_protocol!(
    pub unsafe trait NSTextAttachmentLayout: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSImage",
            feature = "AppKit_NSTextContainer",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other imageForBounds:attributes:location:textContainer:)]
        unsafe fn imageForBounds_attributes_location_textContainer(
            &self,
            bounds: CGRect,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            location: &(impl NSTextLocation + Message),
            text_container: Option<&NSTextContainer>,
        ) -> Option<Id<NSImage>>;

        #[cfg(all(
            feature = "AppKit_NSTextContainer",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(attachmentBoundsForAttributes:location:textContainer:proposedLineFragment:position:)]
        unsafe fn attachmentBoundsForAttributes_location_textContainer_proposedLineFragment_position(
            &self,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            location: &(impl NSTextLocation + Message),
            text_container: Option<&NSTextContainer>,
            proposed_line_fragment: CGRect,
            position: CGPoint,
        ) -> CGRect;

        #[cfg(all(
            feature = "AppKit_NSTextAttachmentViewProvider",
            feature = "AppKit_NSTextContainer",
            feature = "AppKit_NSView"
        ))]
        #[method_id(@__retain_semantics Other viewProviderForParentView:location:textContainer:)]
        unsafe fn viewProviderForParentView_location_textContainer(
            &self,
            parent_view: Option<&NSView>,
            location: &(impl NSTextLocation + Message),
            text_container: Option<&NSTextContainer>,
        ) -> Option<Id<NSTextAttachmentViewProvider>>;
    }

    unsafe impl ProtocolType for dyn NSTextAttachmentLayout {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextAttachment")]
    pub struct NSTextAttachment;

    #[cfg(feature = "AppKit_NSTextAttachment")]
    unsafe impl ClassType for NSTextAttachment {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSTextAttachment")]
unsafe impl NSCoding for NSTextAttachment {}

#[cfg(feature = "AppKit_NSTextAttachment")]
unsafe impl NSObjectProtocol for NSTextAttachment {}

#[cfg(feature = "AppKit_NSTextAttachment")]
unsafe impl NSSecureCoding for NSTextAttachment {}

#[cfg(feature = "AppKit_NSTextAttachment")]
unsafe impl NSTextAttachmentContainer for NSTextAttachment {}

#[cfg(feature = "AppKit_NSTextAttachment")]
unsafe impl NSTextAttachmentLayout for NSTextAttachment {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextAttachment")]
    unsafe impl NSTextAttachment {
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithData:ofType:)]
        pub unsafe fn initWithData_ofType(
            this: Option<Allocated<Self>>,
            content_data: Option<&NSData>,
            uti: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method_id(@__retain_semantics Init initWithFileWrapper:)]
        pub unsafe fn initWithFileWrapper(
            this: Option<Allocated<Self>>,
            file_wrapper: Option<&NSFileWrapper>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&NSData>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, file_type: Option<&NSString>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[method(setBounds:)]
        pub unsafe fn setBounds(&self, bounds: CGRect);

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method_id(@__retain_semantics Other fileWrapper)]
        pub unsafe fn fileWrapper(&self) -> Option<Id<NSFileWrapper>>;

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method(setFileWrapper:)]
        pub unsafe fn setFileWrapper(&self, file_wrapper: Option<&NSFileWrapper>);

        #[method_id(@__retain_semantics Other attachmentCell)]
        pub unsafe fn attachmentCell(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTextAttachmentCellProtocol>>>;

        #[method(setAttachmentCell:)]
        pub unsafe fn setAttachmentCell(
            &self,
            attachment_cell: Option<&(impl NSTextAttachmentCellProtocol + Message)>,
        );

        #[method(lineLayoutPadding)]
        pub unsafe fn lineLayoutPadding(&self) -> CGFloat;

        #[method(setLineLayoutPadding:)]
        pub unsafe fn setLineLayoutPadding(&self, line_layout_padding: CGFloat);

        #[cfg(feature = "Foundation_NSString")]
        #[method(textAttachmentViewProviderClassForFileType:)]
        pub unsafe fn textAttachmentViewProviderClassForFileType(
            file_type: &NSString,
        ) -> Option<&'static AnyClass>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(registerTextAttachmentViewProviderClass:forFileType:)]
        pub unsafe fn registerTextAttachmentViewProviderClass_forFileType(
            text_attachment_view_provider_class: &AnyClass,
            file_type: &NSString,
        );

        #[method(allowsTextAttachmentView)]
        pub unsafe fn allowsTextAttachmentView(&self) -> bool;

        #[method(setAllowsTextAttachmentView:)]
        pub unsafe fn setAllowsTextAttachmentView(&self, allows_text_attachment_view: bool);

        #[method(usesTextAttachmentView)]
        pub unsafe fn usesTextAttachmentView(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTextAttachment")]
    unsafe impl NSTextAttachment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSAttributedStringAttachmentConveniences
    #[cfg(feature = "Foundation_NSAttributedString")]
    unsafe impl NSAttributedString {
        #[cfg(feature = "AppKit_NSTextAttachment")]
        #[method_id(@__retain_semantics Other attributedStringWithAttachment:)]
        pub unsafe fn attributedStringWithAttachment(
            attachment: &NSTextAttachment,
        ) -> Id<NSAttributedString>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextAttachmentViewProvider")]
    pub struct NSTextAttachmentViewProvider;

    #[cfg(feature = "AppKit_NSTextAttachmentViewProvider")]
    unsafe impl ClassType for NSTextAttachmentViewProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSTextAttachmentViewProvider")]
unsafe impl NSObjectProtocol for NSTextAttachmentViewProvider {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextAttachmentViewProvider")]
    unsafe impl NSTextAttachmentViewProvider {
        #[cfg(all(
            feature = "AppKit_NSTextAttachment",
            feature = "AppKit_NSTextLayoutManager",
            feature = "AppKit_NSView"
        ))]
        #[method_id(@__retain_semantics Init initWithTextAttachment:parentView:textLayoutManager:location:)]
        pub unsafe fn initWithTextAttachment_parentView_textLayoutManager_location(
            this: Option<Allocated<Self>>,
            text_attachment: &NSTextAttachment,
            parent_view: Option<&NSView>,
            text_layout_manager: Option<&NSTextLayoutManager>,
            location: &(impl NSTextLocation + Message),
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "AppKit_NSTextAttachment")]
        #[method_id(@__retain_semantics Other textAttachment)]
        pub unsafe fn textAttachment(&self) -> Option<Id<NSTextAttachment>>;

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager>>;

        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Id<ProtocolObject<dyn NSTextLocation>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self, mtm: MainThreadMarker) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[method(loadView)]
        pub unsafe fn loadView(&self);

        #[method(tracksTextAttachmentViewBounds)]
        pub unsafe fn tracksTextAttachmentViewBounds(&self) -> bool;

        #[method(setTracksTextAttachmentViewBounds:)]
        pub unsafe fn setTracksTextAttachmentViewBounds(
            &self,
            tracks_text_attachment_view_bounds: bool,
        );

        #[cfg(all(
            feature = "AppKit_NSTextContainer",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(attachmentBoundsForAttributes:location:textContainer:proposedLineFragment:position:)]
        pub unsafe fn attachmentBoundsForAttributes_location_textContainer_proposedLineFragment_position(
            &self,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            location: &(impl NSTextLocation + Message),
            text_container: Option<&NSTextContainer>,
            proposed_line_fragment: CGRect,
            position: CGPoint,
        ) -> CGRect;
    }
);

extern_methods!(
    /// NSMutableAttributedStringAttachmentConveniences
    #[cfg(feature = "Foundation_NSMutableAttributedString")]
    unsafe impl NSMutableAttributedString {
        #[cfg(feature = "Foundation_NSString")]
        #[method(updateAttachmentsFromPath:)]
        pub unsafe fn updateAttachmentsFromPath(&mut self, path: &NSString);
    }
);
