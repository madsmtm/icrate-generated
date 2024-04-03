//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSPasteboardType = NSString;

extern "C" {
    pub static NSPasteboardTypeString: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypePDF: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeTIFF: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypePNG: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeRTF: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeRTFD: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeHTML: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeTabularText: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeFont: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeRuler: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeColor: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeSound: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeMultipleTextSelection: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeTextFinderOptions: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeURL: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeFileURL: &'static NSPasteboardType;
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSPasteboardName = NSString;

extern "C" {
    pub static NSPasteboardNameGeneral: &'static NSPasteboardName;
}

extern "C" {
    pub static NSPasteboardNameFont: &'static NSPasteboardName;
}

extern "C" {
    pub static NSPasteboardNameRuler: &'static NSPasteboardName;
}

extern "C" {
    pub static NSPasteboardNameFind: &'static NSPasteboardName;
}

extern "C" {
    pub static NSPasteboardNameDrag: &'static NSPasteboardName;
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPasteboardContentsOptions(pub NSUInteger);
impl NSPasteboardContentsOptions {
    pub const NSPasteboardContentsCurrentHostOnly: Self = Self(1 << 0);
}

unsafe impl Encode for NSPasteboardContentsOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPasteboardContentsOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_ENUM
pub type NSPasteboardReadingOptionKey = NSString;

extern "C" {
    pub static NSPasteboardURLReadingFileURLsOnlyKey: &'static NSPasteboardReadingOptionKey;
}

extern "C" {
    pub static NSPasteboardURLReadingContentsConformToTypesKey:
        &'static NSPasteboardReadingOptionKey;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPasteboard;

    unsafe impl ClassType for NSPasteboard {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSPasteboard {}

extern_methods!(
    unsafe impl NSPasteboard {
        #[method_id(@__retain_semantics Other generalPasteboard)]
        pub unsafe fn generalPasteboard() -> Id<NSPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardWithName:)]
        pub unsafe fn pasteboardWithName(name: &NSPasteboardName) -> Id<NSPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardWithUniqueName)]
        pub unsafe fn pasteboardWithUniqueName() -> Id<NSPasteboard>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSPasteboardName>;

        #[method(changeCount)]
        pub unsafe fn changeCount(&self) -> NSInteger;

        #[method(prepareForNewContentsWithOptions:)]
        pub unsafe fn prepareForNewContentsWithOptions(
            &self,
            options: NSPasteboardContentsOptions,
        ) -> NSInteger;

        #[method(clearContents)]
        pub unsafe fn clearContents(&self) -> NSInteger;

        #[method(writeObjects:)]
        pub unsafe fn writeObjects(
            &self,
            objects: &NSArray<ProtocolObject<dyn NSPasteboardWriting>>,
        ) -> bool;

        #[method_id(@__retain_semantics Other readObjectsForClasses:options:)]
        pub unsafe fn readObjectsForClasses_options(
            &self,
            class_array: &NSArray<TodoClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, AnyObject>>,
        ) -> Option<Id<NSArray>>;

        #[cfg(feature = "AppKit_NSPasteboardItem")]
        #[method_id(@__retain_semantics Other pasteboardItems)]
        pub unsafe fn pasteboardItems(&self) -> Option<Id<NSArray<NSPasteboardItem>>>;

        #[cfg(feature = "AppKit_NSPasteboardItem")]
        #[method(indexOfPasteboardItem:)]
        pub unsafe fn indexOfPasteboardItem(
            &self,
            pasteboard_item: &NSPasteboardItem,
        ) -> NSUInteger;

        #[method(canReadItemWithDataConformingToTypes:)]
        pub unsafe fn canReadItemWithDataConformingToTypes(
            &self,
            types: &NSArray<NSString>,
        ) -> bool;

        #[method(canReadObjectForClasses:options:)]
        pub unsafe fn canReadObjectForClasses_options(
            &self,
            class_array: &NSArray<TodoClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, AnyObject>>,
        ) -> bool;

        #[method(declareTypes:owner:)]
        pub unsafe fn declareTypes_owner(
            &self,
            new_types: &NSArray<NSPasteboardType>,
            new_owner: Option<&AnyObject>,
        ) -> NSInteger;

        #[method(addTypes:owner:)]
        pub unsafe fn addTypes_owner(
            &self,
            new_types: &NSArray<NSPasteboardType>,
            new_owner: Option<&AnyObject>,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other types)]
        pub unsafe fn types(&self) -> Option<Id<NSArray<NSPasteboardType>>>;

        #[method_id(@__retain_semantics Other availableTypeFromArray:)]
        pub unsafe fn availableTypeFromArray(
            &self,
            types: &NSArray<NSPasteboardType>,
        ) -> Option<Id<NSPasteboardType>>;

        #[method(setData:forType:)]
        pub unsafe fn setData_forType(
            &self,
            data: Option<&NSData>,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[method(setPropertyList:forType:)]
        pub unsafe fn setPropertyList_forType(
            &self,
            plist: &AnyObject,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[method(setString:forType:)]
        pub unsafe fn setString_forType(
            &self,
            string: &NSString,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[method_id(@__retain_semantics Other dataForType:)]
        pub unsafe fn dataForType(&self, data_type: &NSPasteboardType) -> Option<Id<NSData>>;

        #[method_id(@__retain_semantics Other propertyListForType:)]
        pub fn propertyListForType(&self, data_type: &NSPasteboardType) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other stringForType:)]
        pub unsafe fn stringForType(&self, data_type: &NSPasteboardType) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPasteboard {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// FilterServices
    unsafe impl NSPasteboard {
        #[method_id(@__retain_semantics Other typesFilterableTo:)]
        pub unsafe fn typesFilterableTo(r#type: &NSPasteboardType)
            -> Id<NSArray<NSPasteboardType>>;

        #[method_id(@__retain_semantics Other pasteboardByFilteringFile:)]
        pub unsafe fn pasteboardByFilteringFile(filename: &NSString) -> Id<NSPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardByFilteringData:ofType:)]
        pub unsafe fn pasteboardByFilteringData_ofType(
            data: &NSData,
            r#type: &NSPasteboardType,
        ) -> Id<NSPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardByFilteringTypesInPasteboard:)]
        pub unsafe fn pasteboardByFilteringTypesInPasteboard(
            pboard: &NSPasteboard,
        ) -> Id<NSPasteboard>;
    }
);

extern_protocol!(
    pub unsafe trait NSPasteboardTypeOwner: NSObjectProtocol {
        #[method(pasteboard:provideDataForType:)]
        unsafe fn pasteboard_provideDataForType(
            &self,
            sender: &NSPasteboard,
            r#type: &NSPasteboardType,
        );

        #[optional]
        #[method(pasteboardChangedOwner:)]
        unsafe fn pasteboardChangedOwner(&self, sender: &NSPasteboard);
    }

    unsafe impl ProtocolType for dyn NSPasteboardTypeOwner {}
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPasteboardWritingOptions(pub NSUInteger);
impl NSPasteboardWritingOptions {
    pub const NSPasteboardWritingPromised: Self = Self(1 << 9);
}

unsafe impl Encode for NSPasteboardWritingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPasteboardWritingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait NSPasteboardWriting: NSObjectProtocol {
        #[method_id(@__retain_semantics Other writableTypesForPasteboard:)]
        unsafe fn writableTypesForPasteboard(
            &self,
            pasteboard: &NSPasteboard,
        ) -> Id<NSArray<NSPasteboardType>>;

        #[optional]
        #[method(writingOptionsForType:pasteboard:)]
        unsafe fn writingOptionsForType_pasteboard(
            &self,
            r#type: &NSPasteboardType,
            pasteboard: &NSPasteboard,
        ) -> NSPasteboardWritingOptions;

        #[method_id(@__retain_semantics Other pasteboardPropertyListForType:)]
        unsafe fn pasteboardPropertyListForType(
            &self,
            r#type: &NSPasteboardType,
        ) -> Option<Id<AnyObject>>;
    }

    unsafe impl ProtocolType for dyn NSPasteboardWriting {}
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPasteboardReadingOptions(pub NSUInteger);
impl NSPasteboardReadingOptions {
    pub const NSPasteboardReadingAsData: Self = Self(0);
    pub const NSPasteboardReadingAsString: Self = Self(1 << 0);
    pub const NSPasteboardReadingAsPropertyList: Self = Self(1 << 1);
    pub const NSPasteboardReadingAsKeyedArchive: Self = Self(1 << 2);
}

unsafe impl Encode for NSPasteboardReadingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPasteboardReadingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait NSPasteboardReading: NSObjectProtocol {
        #[method_id(@__retain_semantics Other readableTypesForPasteboard:)]
        unsafe fn readableTypesForPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Id<NSArray<NSPasteboardType>>;

        #[optional]
        #[method(readingOptionsForType:pasteboard:)]
        unsafe fn readingOptionsForType_pasteboard(
            r#type: &NSPasteboardType,
            pasteboard: &NSPasteboard,
        ) -> NSPasteboardReadingOptions;

        #[optional]
        #[method_id(@__retain_semantics Init initWithPasteboardPropertyList:ofType:)]
        unsafe fn initWithPasteboardPropertyList_ofType(
            this: Allocated<Self>,
            property_list: &AnyObject,
            r#type: &NSPasteboardType,
        ) -> Option<Id<Self>>;
    }

    unsafe impl ProtocolType for dyn NSPasteboardReading {}
);

extern_category!(
    /// Category "NSPasteboardSupport" on [`NSURL`].
    #[doc(alias = "NSPasteboardSupport")]
    pub unsafe trait NSURLNSPasteboardSupport {
        #[method_id(@__retain_semantics Other URLFromPasteboard:)]
        unsafe fn URLFromPasteboard(paste_board: &NSPasteboard) -> Option<Id<NSURL>>;

        #[method(writeToPasteboard:)]
        unsafe fn writeToPasteboard(&self, paste_board: &NSPasteboard);
    }

    unsafe impl NSURLNSPasteboardSupport for NSURL {}
);

unsafe impl NSPasteboardReading for NSURL {}

unsafe impl NSPasteboardWriting for NSURL {}

unsafe impl NSPasteboardReading for NSString {}

unsafe impl NSPasteboardWriting for NSString {}

extern_methods!(
    /// NSFileContents
    unsafe impl NSPasteboard {
        #[method(writeFileContents:)]
        pub unsafe fn writeFileContents(&self, filename: &NSString) -> bool;

        #[method_id(@__retain_semantics Other readFileContentsType:toFile:)]
        pub unsafe fn readFileContentsType_toFile(
            &self,
            r#type: Option<&NSPasteboardType>,
            filename: &NSString,
        ) -> Option<Id<NSString>>;

        #[method(writeFileWrapper:)]
        pub unsafe fn writeFileWrapper(&self, wrapper: &NSFileWrapper) -> bool;

        #[method_id(@__retain_semantics Other readFileWrapper)]
        pub unsafe fn readFileWrapper(&self) -> Option<Id<NSFileWrapper>>;
    }
);

extern "C" {
    pub static NSFileContentsPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub fn NSCreateFilenamePboardType(file_type: &NSString) -> *mut NSPasteboardType;
}

extern "C" {
    pub fn NSCreateFileContentsPboardType(file_type: &NSString) -> *mut NSPasteboardType;
}

extern "C" {
    pub fn NSGetFileType(pboard_type: &NSPasteboardType) -> *mut NSString;
}

extern "C" {
    pub fn NSGetFileTypes(pboard_types: &NSArray<NSPasteboardType>) -> *mut NSArray<NSString>;
}

extern "C" {
    pub static NSStringPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSFilenamesPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSTIFFPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSRTFPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSTabularTextPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSFontPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSRulerPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSColorPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSRTFDPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSHTMLPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSURLPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPDFPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSMultipleTextSelectionPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPostScriptPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSVCardPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSInkTextPboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSFilesPromisePboardType: &'static NSPasteboardType;
}

extern "C" {
    pub static NSPasteboardTypeFindPanelSearchOptions: &'static NSPasteboardType;
}

extern "C" {
    pub static NSGeneralPboard: &'static NSPasteboardName;
}

extern "C" {
    pub static NSFontPboard: &'static NSPasteboardName;
}

extern "C" {
    pub static NSRulerPboard: &'static NSPasteboardName;
}

extern "C" {
    pub static NSFindPboard: &'static NSPasteboardName;
}

extern "C" {
    pub static NSDragPboard: &'static NSPasteboardName;
}

extern "C" {
    pub static NSPICTPboardType: &'static NSPasteboardType;
}
