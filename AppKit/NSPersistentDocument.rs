//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentDocument;

    unsafe impl ClassType for NSPersistentDocument {
        #[inherits(NSObject)]
        type Super = NSDocument;
    }
);

extern_methods!(
    unsafe impl NSPersistentDocument {
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Option<Id<NSManagedObjectContext, Shared>>;

        #[method(setManagedObjectContext:)]
        pub unsafe fn setManagedObjectContext(
            &self,
            managedObjectContext: Option<&NSManagedObjectContext>,
        );

        #[method_id(@__retain_semantics Other managedObjectModel)]
        pub unsafe fn managedObjectModel(&self) -> Option<Id<NSManagedObjectModel, Shared>>;

        #[method(configurePersistentStoreCoordinatorForURL:ofType:modelConfiguration:storeOptions:error:_)]
        pub unsafe fn configurePersistentStoreCoordinatorForURL_ofType_modelConfiguration_storeOptions_error(
            &self,
            url: &NSURL,
            fileType: &NSString,
            configuration: Option<&NSString>,
            storeOptions: Option<&NSDictionary<NSString, Object>>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other persistentStoreTypeForFileType:)]
        pub unsafe fn persistentStoreTypeForFileType(
            &self,
            fileType: &NSString,
        ) -> Id<NSString, Shared>;

        #[method(writeToURL:ofType:forSaveOperation:originalContentsURL:error:_)]
        pub unsafe fn writeToURL_ofType_forSaveOperation_originalContentsURL_error(
            &self,
            absoluteURL: &NSURL,
            typeName: &NSString,
            saveOperation: NSSaveOperationType,
            absoluteOriginalContentsURL: Option<&NSURL>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(readFromURL:ofType:error:_)]
        pub unsafe fn readFromURL_ofType_error(
            &self,
            absoluteURL: &NSURL,
            typeName: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(revertToContentsOfURL:ofType:error:_)]
        pub unsafe fn revertToContentsOfURL_ofType_error(
            &self,
            inAbsoluteURL: &NSURL,
            inTypeName: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSPersistentDocument {
        #[method(configurePersistentStoreCoordinatorForURL:ofType:error:_)]
        pub unsafe fn configurePersistentStoreCoordinatorForURL_ofType_error(
            &self,
            url: Option<&NSURL>,
            fileType: Option<&NSString>,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDocument`
    unsafe impl NSPersistentDocument {
        #[method_id(@__retain_semantics Init initWithType:error:_)]
        pub unsafe fn initWithType_error(
            this: Option<Allocated<Self>>,
            typeName: &NSString,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:ofType:error:_)]
        pub unsafe fn initWithContentsOfURL_ofType_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            typeName: &NSString,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initForURL:withContentsOfURL:ofType:error:_)]
        pub unsafe fn initForURL_withContentsOfURL_ofType_error(
            this: Option<Allocated<Self>>,
            urlOrNil: Option<&NSURL>,
            contentsURL: &NSURL,
            typeName: &NSString,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDocument`
    ///
    /// NSDeprecated
    unsafe impl NSPersistentDocument {
        #[method_id(@__retain_semantics Init initWithContentsOfFile:ofType:)]
        pub unsafe fn initWithContentsOfFile_ofType(
            this: Option<Allocated<Self>>,
            absolutePath: &NSString,
            typeName: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:ofType:)]
        pub unsafe fn initWithContentsOfURL_ofType(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            typeName: &NSString,
        ) -> Option<Id<Self, Shared>>;
    }
);
