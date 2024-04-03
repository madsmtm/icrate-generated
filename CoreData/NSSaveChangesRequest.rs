//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentStoreRequest")]
    pub struct NSSaveChangesRequest;

    #[cfg(feature = "CoreData_NSPersistentStoreRequest")]
    unsafe impl ClassType for NSSaveChangesRequest {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSPersistentStoreRequest")]
unsafe impl NSCopying for NSSaveChangesRequest {}

#[cfg(feature = "CoreData_NSPersistentStoreRequest")]
unsafe impl NSObjectProtocol for NSSaveChangesRequest {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentStoreRequest")]
    unsafe impl NSSaveChangesRequest {
        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method_id(@__retain_semantics Init initWithInsertedObjects:updatedObjects:deletedObjects:lockedObjects:)]
        pub unsafe fn initWithInsertedObjects_updatedObjects_deletedObjects_lockedObjects(
            this: Allocated<Self>,
            inserted_objects: Option<&NSSet<NSManagedObject>>,
            updated_objects: Option<&NSSet<NSManagedObject>>,
            deleted_objects: Option<&NSSet<NSManagedObject>>,
            locked_objects: Option<&NSSet<NSManagedObject>>,
        ) -> Id<Self>;

        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method_id(@__retain_semantics Other insertedObjects)]
        pub unsafe fn insertedObjects(&self) -> Option<Id<NSSet<NSManagedObject>>>;

        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method_id(@__retain_semantics Other updatedObjects)]
        pub unsafe fn updatedObjects(&self) -> Option<Id<NSSet<NSManagedObject>>>;

        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method_id(@__retain_semantics Other deletedObjects)]
        pub unsafe fn deletedObjects(&self) -> Option<Id<NSSet<NSManagedObject>>>;

        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method_id(@__retain_semantics Other lockedObjects)]
        pub unsafe fn lockedObjects(&self) -> Option<Id<NSSet<NSManagedObject>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSPersistentStoreRequest")]
    unsafe impl NSSaveChangesRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
