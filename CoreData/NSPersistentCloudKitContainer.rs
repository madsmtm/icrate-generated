//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPersistentCloudKitContainerSchemaInitializationOptions {
        NSPersistentCloudKitContainerSchemaInitializationOptionsNone = 0,
        NSPersistentCloudKitContainerSchemaInitializationOptionsDryRun = 1 << 1,
        NSPersistentCloudKitContainerSchemaInitializationOptionsPrintSchema = 1 << 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentCloudKitContainer;

    unsafe impl ClassType for NSPersistentCloudKitContainer {
        #[inherits(NSObject)]
        type Super = NSPersistentContainer;
    }
);

extern_methods!(
    unsafe impl NSPersistentCloudKitContainer {
        #[method(initializeCloudKitSchemaWithOptions:error:_)]
        pub unsafe fn initializeCloudKitSchemaWithOptions_error(
            &self,
            options: NSPersistentCloudKitContainerSchemaInitializationOptions,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(canUpdateRecordForManagedObjectWithID:)]
        pub unsafe fn canUpdateRecordForManagedObjectWithID(
            &self,
            objectID: &NSManagedObjectID,
        ) -> bool;

        #[method(canDeleteRecordForManagedObjectWithID:)]
        pub unsafe fn canDeleteRecordForManagedObjectWithID(
            &self,
            objectID: &NSManagedObjectID,
        ) -> bool;

        #[method(canModifyManagedObjectsInStore:)]
        pub unsafe fn canModifyManagedObjectsInStore(&self, store: &NSPersistentStore) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSPersistentContainer`
    unsafe impl NSPersistentCloudKitContainer {
        #[method_id(@__retain_semantics Other persistentContainerWithName:)]
        pub unsafe fn persistentContainerWithName(name: &NSString) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other persistentContainerWithName:managedObjectModel:)]
        pub unsafe fn persistentContainerWithName_managedObjectModel(
            name: &NSString,
            model: &NSManagedObjectModel,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(
            this: Option<Allocated<Self>>,
            name: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithName:managedObjectModel:)]
        pub unsafe fn initWithName_managedObjectModel(
            this: Option<Allocated<Self>>,
            name: &NSString,
            model: &NSManagedObjectModel,
        ) -> Id<Self, Shared>;
    }
);
