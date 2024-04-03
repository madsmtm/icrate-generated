//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentStoreRequest")]
    pub struct NSBatchInsertRequest;

    #[cfg(feature = "CoreData_NSPersistentStoreRequest")]
    unsafe impl ClassType for NSBatchInsertRequest {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSPersistentStoreRequest")]
unsafe impl NSCopying for NSBatchInsertRequest {}

#[cfg(feature = "CoreData_NSPersistentStoreRequest")]
unsafe impl NSObjectProtocol for NSBatchInsertRequest {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentStoreRequest")]
    unsafe impl NSBatchInsertRequest {
        #[method_id(@__retain_semantics Other entityName)]
        pub unsafe fn entityName(&self) -> Id<NSString>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Option<Id<NSEntityDescription>>;

        #[method_id(@__retain_semantics Other objectsToInsert)]
        pub unsafe fn objectsToInsert(
            &self,
        ) -> Option<Id<NSArray<NSDictionary<NSString, AnyObject>>>>;

        #[method(setObjectsToInsert:)]
        pub unsafe fn setObjectsToInsert(
            &self,
            objects_to_insert: Option<&NSArray<NSDictionary<NSString, AnyObject>>>,
        );

        #[cfg(feature = "block2")]
        #[method(dictionaryHandler)]
        pub unsafe fn dictionaryHandler(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<NSMutableDictionary<NSString, AnyObject>>) -> Bool>;

        #[cfg(feature = "block2")]
        #[method(setDictionaryHandler:)]
        pub unsafe fn setDictionaryHandler(
            &self,
            dictionary_handler: Option<
                &Block<dyn Fn(NonNull<NSMutableDictionary<NSString, AnyObject>>) -> Bool>,
            >,
        );

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "block2"))]
        #[method(managedObjectHandler)]
        pub unsafe fn managedObjectHandler(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<NSManagedObject>) -> Bool>;

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "block2"))]
        #[method(setManagedObjectHandler:)]
        pub unsafe fn setManagedObjectHandler(
            &self,
            managed_object_handler: Option<&Block<dyn Fn(NonNull<NSManagedObject>) -> Bool>>,
        );

        #[cfg(feature = "CoreData_NSPersistentStoreResult")]
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchInsertRequestResultType;

        #[cfg(feature = "CoreData_NSPersistentStoreResult")]
        #[method(setResultType:)]
        pub unsafe fn setResultType(&self, result_type: NSBatchInsertRequestResultType);

        #[method_id(@__retain_semantics Other batchInsertRequestWithEntityName:objects:)]
        pub unsafe fn batchInsertRequestWithEntityName_objects(
            entity_name: &NSString,
            dictionaries: &NSArray<NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other batchInsertRequestWithEntityName:dictionaryHandler:)]
        pub unsafe fn batchInsertRequestWithEntityName_dictionaryHandler(
            entity_name: &NSString,
            handler: &Block<dyn Fn(NonNull<NSMutableDictionary<NSString, AnyObject>>) -> Bool>,
        ) -> Id<Self>;

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "block2"))]
        #[method_id(@__retain_semantics Other batchInsertRequestWithEntityName:managedObjectHandler:)]
        pub unsafe fn batchInsertRequestWithEntityName_managedObjectHandler(
            entity_name: &NSString,
            handler: &Block<dyn Fn(NonNull<NSManagedObject>) -> Bool>,
        ) -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithEntityName:objects:)]
        pub unsafe fn initWithEntityName_objects(
            this: Allocated<Self>,
            entity_name: &NSString,
            dictionaries: &NSArray<NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Init initWithEntity:objects:)]
        pub unsafe fn initWithEntity_objects(
            this: Allocated<Self>,
            entity: &NSEntityDescription,
            dictionaries: &NSArray<NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "CoreData_NSEntityDescription", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithEntity:dictionaryHandler:)]
        pub unsafe fn initWithEntity_dictionaryHandler(
            this: Allocated<Self>,
            entity: &NSEntityDescription,
            handler: &Block<dyn Fn(NonNull<NSMutableDictionary<NSString, AnyObject>>) -> Bool>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "CoreData_NSManagedObject",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithEntity:managedObjectHandler:)]
        pub unsafe fn initWithEntity_managedObjectHandler(
            this: Allocated<Self>,
            entity: &NSEntityDescription,
            handler: &Block<dyn Fn(NonNull<NSManagedObject>) -> Bool>,
        ) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithEntityName:dictionaryHandler:)]
        pub unsafe fn initWithEntityName_dictionaryHandler(
            this: Allocated<Self>,
            entity_name: &NSString,
            handler: &Block<dyn Fn(NonNull<NSMutableDictionary<NSString, AnyObject>>) -> Bool>,
        ) -> Id<Self>;

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithEntityName:managedObjectHandler:)]
        pub unsafe fn initWithEntityName_managedObjectHandler(
            this: Allocated<Self>,
            entity_name: &NSString,
            handler: &Block<dyn Fn(NonNull<NSManagedObject>) -> Bool>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSPersistentStoreRequest")]
    unsafe impl NSBatchInsertRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
