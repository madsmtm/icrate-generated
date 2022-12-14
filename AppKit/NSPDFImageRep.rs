//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPDFImageRep;

    unsafe impl ClassType for NSPDFImageRep {
        #[inherits(NSObject)]
        type Super = NSImageRep;
    }
);

extern_methods!(
    unsafe impl NSPDFImageRep {
        #[method_id(@__retain_semantics Other imageRepWithData:)]
        pub unsafe fn imageRepWithData(pdfData: &NSData) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            pdfData: &NSData,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other PDFRepresentation)]
        pub unsafe fn PDFRepresentation(&self) -> Id<NSData, Shared>;

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> NSRect;

        #[method(currentPage)]
        pub unsafe fn currentPage(&self) -> NSInteger;

        #[method(setCurrentPage:)]
        pub unsafe fn setCurrentPage(&self, currentPage: NSInteger);

        #[method(pageCount)]
        pub unsafe fn pageCount(&self) -> NSInteger;
    }
);
