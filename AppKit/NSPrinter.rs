//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSPrinterTableStatus {
        NSPrinterTableOK = 0,
        NSPrinterTableNotFound = 1,
        NSPrinterTableError = 2,
    }
);

pub type NSPrinterTypeName = NSString;

pub type NSPrinterPaperName = NSString;

extern_class!(
    #[derive(Debug)]
    pub struct NSPrinter;

    unsafe impl ClassType for NSPrinter {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPrinter {
        #[method_id(@__retain_semantics Other printerNames)]
        pub unsafe fn printerNames() -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other printerTypes)]
        pub unsafe fn printerTypes() -> Id<NSArray<NSPrinterTypeName>, Shared>;

        #[method_id(@__retain_semantics Other printerWithName:)]
        pub unsafe fn printerWithName(name: &NSString) -> Option<Id<NSPrinter, Shared>>;

        #[method_id(@__retain_semantics Other printerWithType:)]
        pub unsafe fn printerWithType(type_: &NSPrinterTypeName) -> Option<Id<NSPrinter, Shared>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn type_(&self) -> Id<NSPrinterTypeName, Shared>;

        #[method(languageLevel)]
        pub unsafe fn languageLevel(&self) -> NSInteger;

        #[method(pageSizeForPaper:)]
        pub unsafe fn pageSizeForPaper(&self, paperName: &NSPrinterPaperName) -> NSSize;

        #[method_id(@__retain_semantics Other deviceDescription)]
        pub unsafe fn deviceDescription(
            &self,
        ) -> Id<NSDictionary<NSDeviceDescriptionKey, Object>, Shared>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSPrinter {
        #[method(statusForTable:)]
        pub unsafe fn statusForTable(&self, tableName: &NSString) -> NSPrinterTableStatus;

        #[method(isKey:inTable:)]
        pub unsafe fn isKey_inTable(&self, key: Option<&NSString>, table: &NSString) -> bool;

        #[method(booleanForKey:inTable:)]
        pub unsafe fn booleanForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> bool;

        #[method(floatForKey:inTable:)]
        pub unsafe fn floatForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> c_float;

        #[method(intForKey:inTable:)]
        pub unsafe fn intForKey_inTable(&self, key: Option<&NSString>, table: &NSString) -> c_int;

        #[method(rectForKey:inTable:)]
        pub unsafe fn rectForKey_inTable(&self, key: Option<&NSString>, table: &NSString)
            -> NSRect;

        #[method(sizeForKey:inTable:)]
        pub unsafe fn sizeForKey_inTable(&self, key: Option<&NSString>, table: &NSString)
            -> NSSize;

        #[method_id(@__retain_semantics Other stringForKey:inTable:)]
        pub unsafe fn stringForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other stringListForKey:inTable:)]
        pub unsafe fn stringListForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> Option<Id<NSArray, Shared>>;

        #[method(imageRectForPaper:)]
        pub unsafe fn imageRectForPaper(&self, paperName: Option<&NSString>) -> NSRect;

        #[method(acceptsBinary)]
        pub unsafe fn acceptsBinary(&self) -> bool;

        #[method(isColor)]
        pub unsafe fn isColor(&self) -> bool;

        #[method(isFontAvailable:)]
        pub unsafe fn isFontAvailable(&self, faceName: Option<&NSString>) -> bool;

        #[method(isOutputStackInReverseOrder)]
        pub unsafe fn isOutputStackInReverseOrder(&self) -> bool;

        #[method_id(@__retain_semantics Other printerWithName:domain:includeUnavailable:)]
        pub unsafe fn printerWithName_domain_includeUnavailable(
            name: &NSString,
            domain: Option<&NSString>,
            flag: bool,
        ) -> Option<Id<NSPrinter, Shared>>;

        #[method_id(@__retain_semantics Other domain)]
        pub unsafe fn domain(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other host)]
        pub unsafe fn host(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other note)]
        pub unsafe fn note(&self) -> Id<NSString, Shared>;
    }
);
