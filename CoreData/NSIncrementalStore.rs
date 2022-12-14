//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSIncrementalStore;

    unsafe impl ClassType for NSIncrementalStore {
        #[inherits(NSObject)]
        type Super = NSPersistentStore;
    }
);

extern_methods!(
    unsafe impl NSIncrementalStore {
        #[method(loadMetadata:_)]
        pub unsafe fn loadMetadata(&self) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other executeRequest:withContext:error:_)]
        pub unsafe fn executeRequest_withContext_error(
            &self,
            request: &NSPersistentStoreRequest,
            context: Option<&NSManagedObjectContext>,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics New newValuesForObjectWithID:withContext:error:_)]
        pub unsafe fn newValuesForObjectWithID_withContext_error(
            &self,
            objectID: &NSManagedObjectID,
            context: &NSManagedObjectContext,
        ) -> Result<Id<NSIncrementalStoreNode, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics New newValueForRelationship:forObjectWithID:withContext:error:_)]
        pub unsafe fn newValueForRelationship_forObjectWithID_withContext_error(
            &self,
            relationship: &NSRelationshipDescription,
            objectID: &NSManagedObjectID,
            context: Option<&NSManagedObjectContext>,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other identifierForNewStoreAtURL:)]
        pub unsafe fn identifierForNewStoreAtURL(storeURL: &NSURL) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other obtainPermanentIDsForObjects:error:_)]
        pub unsafe fn obtainPermanentIDsForObjects_error(
            &self,
            array: &NSArray<NSManagedObject>,
        ) -> Result<Id<NSArray<NSManagedObjectID>, Shared>, Id<NSError, Shared>>;

        #[method(managedObjectContextDidRegisterObjectsWithIDs:)]
        pub unsafe fn managedObjectContextDidRegisterObjectsWithIDs(
            &self,
            objectIDs: &NSArray<NSManagedObjectID>,
        );

        #[method(managedObjectContextDidUnregisterObjectsWithIDs:)]
        pub unsafe fn managedObjectContextDidUnregisterObjectsWithIDs(
            &self,
            objectIDs: &NSArray<NSManagedObjectID>,
        );

        #[method_id(@__retain_semantics New newObjectIDForEntity:referenceObject:)]
        pub unsafe fn newObjectIDForEntity_referenceObject(
            &self,
            entity: &NSEntityDescription,
            data: &Object,
        ) -> Id<NSManagedObjectID, Shared>;

        #[method_id(@__retain_semantics Other referenceObjectForObjectID:)]
        pub unsafe fn referenceObjectForObjectID(
            &self,
            objectID: &NSManagedObjectID,
        ) -> Id<Object, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSPersistentStore`
    unsafe impl NSIncrementalStore {
        #[method_id(@__retain_semantics Init initWithPersistentStoreCoordinator:configurationName:URL:options:)]
        pub unsafe fn initWithPersistentStoreCoordinator_configurationName_URL_options(
            this: Option<Allocated<Self>>,
            root: Option<&NSPersistentStoreCoordinator>,
            name: Option<&NSString>,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Id<Self, Shared>;
    }
);
