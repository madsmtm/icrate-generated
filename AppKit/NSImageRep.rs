//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_enum!(
    pub type NSImageHintKey = NSString;
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSImageRepMatchesDevice = 0,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSImageLayoutDirection {
        NSImageLayoutDirectionUnspecified = -1,
        NSImageLayoutDirectionLeftToRight = 2,
        NSImageLayoutDirectionRightToLeft = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSImageRep")]
    pub struct NSImageRep;

    #[cfg(feature = "AppKit_NSImageRep")]
    unsafe impl ClassType for NSImageRep {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSImageRep")]
    unsafe impl NSImageRep {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method(draw)]
        pub unsafe fn draw(&self) -> bool;

        #[method(drawAtPoint:)]
        pub unsafe fn drawAtPoint(&self, point: NSPoint) -> bool;

        #[method(drawInRect:)]
        pub unsafe fn drawInRect(&self, rect: NSRect) -> bool;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(drawInRect:fromRect:operation:fraction:respectFlipped:hints:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction_respectFlipped_hints(
            &self,
            dstSpacePortionRect: NSRect,
            srcSpacePortionRect: NSRect,
            op: NSCompositingOperation,
            requestedAlpha: CGFloat,
            respectContextIsFlipped: bool,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
        ) -> bool;

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[method(hasAlpha)]
        pub unsafe fn hasAlpha(&self) -> bool;

        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: bool);

        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;

        #[method(setOpaque:)]
        pub unsafe fn setOpaque(&self, opaque: bool);

        #[method_id(@__retain_semantics Other colorSpaceName)]
        pub unsafe fn colorSpaceName(&self) -> Id<NSColorSpaceName, Shared>;

        #[method(setColorSpaceName:)]
        pub unsafe fn setColorSpaceName(&self, colorSpaceName: &NSColorSpaceName);

        #[method(bitsPerSample)]
        pub unsafe fn bitsPerSample(&self) -> NSInteger;

        #[method(setBitsPerSample:)]
        pub unsafe fn setBitsPerSample(&self, bitsPerSample: NSInteger);

        #[method(pixelsWide)]
        pub unsafe fn pixelsWide(&self) -> NSInteger;

        #[method(setPixelsWide:)]
        pub unsafe fn setPixelsWide(&self, pixelsWide: NSInteger);

        #[method(pixelsHigh)]
        pub unsafe fn pixelsHigh(&self) -> NSInteger;

        #[method(setPixelsHigh:)]
        pub unsafe fn setPixelsHigh(&self, pixelsHigh: NSInteger);

        #[method(layoutDirection)]
        pub unsafe fn layoutDirection(&self) -> NSImageLayoutDirection;

        #[method(setLayoutDirection:)]
        pub unsafe fn setLayoutDirection(&self, layoutDirection: NSImageLayoutDirection);

        #[method(registerImageRepClass:)]
        pub unsafe fn registerImageRepClass(imageRepClass: &Class);

        #[method(unregisterImageRepClass:)]
        pub unsafe fn unregisterImageRepClass(imageRepClass: &Class);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other registeredImageRepClasses)]
        pub unsafe fn registeredImageRepClasses() -> Id<NSArray<TodoClass>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(imageRepClassForFileType:)]
        pub unsafe fn imageRepClassForFileType(type_: &NSString) -> Option<&'static Class>;

        #[method(imageRepClassForPasteboardType:)]
        pub unsafe fn imageRepClassForPasteboardType(
            type_: &NSPasteboardType,
        ) -> Option<&'static Class>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(imageRepClassForType:)]
        pub unsafe fn imageRepClassForType(type_: &NSString) -> Option<&'static Class>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(imageRepClassForData:)]
        pub unsafe fn imageRepClassForData(data: &NSData) -> Option<&'static Class>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(canInitWithData:)]
        pub unsafe fn canInitWithData(data: &NSData) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageUnfilteredFileTypes)]
        pub unsafe fn imageUnfilteredFileTypes() -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other imageUnfilteredPasteboardTypes)]
        pub unsafe fn imageUnfilteredPasteboardTypes() -> Id<NSArray<NSPasteboardType>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageFileTypes)]
        pub unsafe fn imageFileTypes() -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other imagePasteboardTypes)]
        pub unsafe fn imagePasteboardTypes() -> Id<NSArray<NSPasteboardType>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageUnfilteredTypes)]
        pub unsafe fn imageUnfilteredTypes() -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageTypes)]
        pub unsafe fn imageTypes() -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &NSPasteboard) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageRepsWithContentsOfFile:)]
        pub unsafe fn imageRepsWithContentsOfFile(
            filename: &NSString,
        ) -> Option<Id<NSArray<NSImageRep>, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other imageRepWithContentsOfFile:)]
        pub unsafe fn imageRepWithContentsOfFile(
            filename: &NSString,
        ) -> Option<Id<NSImageRep, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other imageRepsWithContentsOfURL:)]
        pub unsafe fn imageRepsWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Id<NSArray<NSImageRep>, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other imageRepWithContentsOfURL:)]
        pub unsafe fn imageRepWithContentsOfURL(url: &NSURL) -> Option<Id<NSImageRep, Shared>>;

        #[cfg(all(feature = "AppKit_NSPasteboard", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other imageRepsWithPasteboard:)]
        pub unsafe fn imageRepsWithPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Option<Id<NSArray<NSImageRep>, Shared>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Other imageRepWithPasteboard:)]
        pub unsafe fn imageRepWithPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Option<Id<NSImageRep, Shared>>;
    }
);

extern_static!(NSImageRepRegistryDidChangeNotification: &'static NSNotificationName);
