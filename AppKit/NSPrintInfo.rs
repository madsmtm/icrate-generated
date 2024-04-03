//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPaperOrientation(pub NSInteger);
impl NSPaperOrientation {
    #[doc(alias = "NSPaperOrientationPortrait")]
    pub const Portrait: Self = Self(0);
    #[doc(alias = "NSPaperOrientationLandscape")]
    pub const Landscape: Self = Self(1);
}

unsafe impl Encode for NSPaperOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPaperOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPrintingPaginationMode(pub NSUInteger);
impl NSPrintingPaginationMode {
    #[doc(alias = "NSPrintingPaginationModeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "NSPrintingPaginationModeFit")]
    pub const Fit: Self = Self(1);
    #[doc(alias = "NSPrintingPaginationModeClip")]
    pub const Clip: Self = Self(2);
}

unsafe impl Encode for NSPrintingPaginationMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPrintingPaginationMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSPrintInfoAttributeKey = NSString;

extern "C" {
    pub static NSPrintPaperName: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintPaperSize: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintOrientation: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintScalingFactor: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintLeftMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintRightMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintTopMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintBottomMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintHorizontallyCentered: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintVerticallyCentered: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintHorizontalPagination: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintVerticalPagination: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintPrinter: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintCopies: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintAllPages: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintFirstPage: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintLastPage: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintMustCollate: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintReversePageOrder: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintJobDisposition: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintPagesAcross: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintPagesDown: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintTime: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintDetailedErrorReporting: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintFaxNumber: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintPrinterName: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintSelectionOnly: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintJobSavingURL: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintJobSavingFileNameExtensionHidden: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    pub static NSPrintHeaderAndFooter: &'static NSPrintInfoAttributeKey;
}

// NS_TYPED_ENUM
pub type NSPrintJobDispositionValue = NSString;

extern "C" {
    pub static NSPrintSpoolJob: &'static NSPrintJobDispositionValue;
}

extern "C" {
    pub static NSPrintPreviewJob: &'static NSPrintJobDispositionValue;
}

extern "C" {
    pub static NSPrintSaveJob: &'static NSPrintJobDispositionValue;
}

extern "C" {
    pub static NSPrintCancelJob: &'static NSPrintJobDispositionValue;
}

pub type NSPrintInfoSettingKey = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPrintInfo;

    unsafe impl ClassType for NSPrintInfo {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSPrintInfo {}

unsafe impl NSCopying for NSPrintInfo {}

unsafe impl NSObjectProtocol for NSPrintInfo {}

extern_methods!(
    unsafe impl NSPrintInfo {
        #[method_id(@__retain_semantics Other sharedPrintInfo)]
        pub unsafe fn sharedPrintInfo() -> Id<NSPrintInfo>;

        #[method(setSharedPrintInfo:)]
        pub unsafe fn setSharedPrintInfo(shared_print_info: &NSPrintInfo);

        #[method_id(@__retain_semantics Init initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            this: Allocated<Self>,
            attributes: &NSDictionary<NSPrintInfoAttributeKey, AnyObject>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other dictionary)]
        pub unsafe fn dictionary(
            &self,
        ) -> Id<NSMutableDictionary<NSPrintInfoAttributeKey, AnyObject>>;

        #[cfg(feature = "AppKit_NSPrinter")]
        #[method_id(@__retain_semantics Other paperName)]
        pub unsafe fn paperName(&self) -> Option<Id<NSPrinterPaperName>>;

        #[cfg(feature = "AppKit_NSPrinter")]
        #[method(setPaperName:)]
        pub unsafe fn setPaperName(&self, paper_name: Option<&NSPrinterPaperName>);

        #[method(paperSize)]
        pub unsafe fn paperSize(&self) -> NSSize;

        #[method(setPaperSize:)]
        pub unsafe fn setPaperSize(&self, paper_size: NSSize);

        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSPaperOrientation;

        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSPaperOrientation);

        #[method(scalingFactor)]
        pub unsafe fn scalingFactor(&self) -> CGFloat;

        #[method(setScalingFactor:)]
        pub unsafe fn setScalingFactor(&self, scaling_factor: CGFloat);

        #[method(leftMargin)]
        pub unsafe fn leftMargin(&self) -> CGFloat;

        #[method(setLeftMargin:)]
        pub unsafe fn setLeftMargin(&self, left_margin: CGFloat);

        #[method(rightMargin)]
        pub unsafe fn rightMargin(&self) -> CGFloat;

        #[method(setRightMargin:)]
        pub unsafe fn setRightMargin(&self, right_margin: CGFloat);

        #[method(topMargin)]
        pub unsafe fn topMargin(&self) -> CGFloat;

        #[method(setTopMargin:)]
        pub unsafe fn setTopMargin(&self, top_margin: CGFloat);

        #[method(bottomMargin)]
        pub unsafe fn bottomMargin(&self) -> CGFloat;

        #[method(setBottomMargin:)]
        pub unsafe fn setBottomMargin(&self, bottom_margin: CGFloat);

        #[method(isHorizontallyCentered)]
        pub unsafe fn isHorizontallyCentered(&self) -> bool;

        #[method(setHorizontallyCentered:)]
        pub unsafe fn setHorizontallyCentered(&self, horizontally_centered: bool);

        #[method(isVerticallyCentered)]
        pub unsafe fn isVerticallyCentered(&self) -> bool;

        #[method(setVerticallyCentered:)]
        pub unsafe fn setVerticallyCentered(&self, vertically_centered: bool);

        #[method(horizontalPagination)]
        pub unsafe fn horizontalPagination(&self) -> NSPrintingPaginationMode;

        #[method(setHorizontalPagination:)]
        pub unsafe fn setHorizontalPagination(
            &self,
            horizontal_pagination: NSPrintingPaginationMode,
        );

        #[method(verticalPagination)]
        pub unsafe fn verticalPagination(&self) -> NSPrintingPaginationMode;

        #[method(setVerticalPagination:)]
        pub unsafe fn setVerticalPagination(&self, vertical_pagination: NSPrintingPaginationMode);

        #[method_id(@__retain_semantics Other jobDisposition)]
        pub unsafe fn jobDisposition(&self) -> Id<NSPrintJobDispositionValue>;

        #[method(setJobDisposition:)]
        pub unsafe fn setJobDisposition(&self, job_disposition: &NSPrintJobDispositionValue);

        #[cfg(feature = "AppKit_NSPrinter")]
        #[method_id(@__retain_semantics Other printer)]
        pub unsafe fn printer(&self) -> Id<NSPrinter>;

        #[cfg(feature = "AppKit_NSPrinter")]
        #[method(setPrinter:)]
        pub unsafe fn setPrinter(&self, printer: &NSPrinter);

        #[method(setUpPrintOperationDefaultValues)]
        pub unsafe fn setUpPrintOperationDefaultValues(&self);

        #[method(imageablePageBounds)]
        pub unsafe fn imageablePageBounds(&self) -> NSRect;

        #[method_id(@__retain_semantics Other localizedPaperName)]
        pub unsafe fn localizedPaperName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "AppKit_NSPrinter")]
        #[method_id(@__retain_semantics Other defaultPrinter)]
        pub unsafe fn defaultPrinter() -> Option<Id<NSPrinter>>;

        #[method_id(@__retain_semantics Other printSettings)]
        pub unsafe fn printSettings(
            &self,
        ) -> Id<NSMutableDictionary<NSPrintInfoSettingKey, AnyObject>>;

        #[method(PMPrintSession)]
        pub unsafe fn PMPrintSession(&self) -> NonNull<c_void>;

        #[method(PMPageFormat)]
        pub unsafe fn PMPageFormat(&self) -> NonNull<c_void>;

        #[method(PMPrintSettings)]
        pub unsafe fn PMPrintSettings(&self) -> NonNull<c_void>;

        #[method(updateFromPMPageFormat)]
        pub unsafe fn updateFromPMPageFormat(&self);

        #[method(updateFromPMPrintSettings)]
        pub unsafe fn updateFromPMPrintSettings(&self);

        #[method(isSelectionOnly)]
        pub unsafe fn isSelectionOnly(&self) -> bool;

        #[method(setSelectionOnly:)]
        pub unsafe fn setSelectionOnly(&self, selection_only: bool);

        #[cfg(feature = "AppKit_NSPDFInfo")]
        #[method(takeSettingsFromPDFInfo:)]
        pub unsafe fn takeSettingsFromPDFInfo(&self, in_pdf_info: &NSPDFInfo);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPrintInfo {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSPrintInfo {
        #[cfg(feature = "AppKit_NSPrinter")]
        #[deprecated = "NSPrintInfo's implementation has no effect"]
        #[method(setDefaultPrinter:)]
        pub unsafe fn setDefaultPrinter(printer: Option<&NSPrinter>);

        #[cfg(feature = "AppKit_NSPrinter")]
        #[deprecated = "Use -[NSPrinter pageSizeForPaper:] instead"]
        #[method(sizeForPaperName:)]
        pub unsafe fn sizeForPaperName(name: Option<&NSPrinterPaperName>) -> NSSize;
    }
);

extern "C" {
    pub static NSPrintFormName: &'static NSString;
}

extern "C" {
    pub static NSPrintJobFeatures: &'static NSString;
}

extern "C" {
    pub static NSPrintManualFeed: &'static NSString;
}

extern "C" {
    pub static NSPrintPagesPerSheet: &'static NSString;
}

extern "C" {
    pub static NSPrintPaperFeed: &'static NSString;
}

extern "C" {
    pub static NSPrintSavePath: &'static NSString;
}

// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPrintingOrientation(pub NSUInteger);
impl NSPrintingOrientation {
    #[deprecated]
    pub const NSPortraitOrientation: Self = Self(0);
    #[deprecated]
    pub const NSLandscapeOrientation: Self = Self(1);
}

unsafe impl Encode for NSPrintingOrientation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPrintingOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub static NSAutoPagination: NSPrintingPaginationMode =
    NSPrintingPaginationMode(NSPrintingPaginationMode::Automatic.0);

pub static NSFitPagination: NSPrintingPaginationMode =
    NSPrintingPaginationMode(NSPrintingPaginationMode::Fit.0);

pub static NSClipPagination: NSPrintingPaginationMode =
    NSPrintingPaginationMode(NSPrintingPaginationMode::Clip.0);
