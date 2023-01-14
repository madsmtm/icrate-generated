//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSSnapshotEventType {
        NSSnapshotEventUndoInsertion = 1 << 1,
        NSSnapshotEventUndoDeletion = 1 << 2,
        NSSnapshotEventUndoUpdate = 1 << 3,
        NSSnapshotEventRollback = 1 << 4,
        NSSnapshotEventRefresh = 1 << 5,
        NSSnapshotEventMergePolicy = 1 << 6,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSManagedObject")]
    pub struct NSManagedObject;

    #[cfg(feature = "CoreData_NSManagedObject")]
    unsafe impl ClassType for NSManagedObject {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSManagedObject")]
    unsafe impl NSManagedObject {
        #[method(contextShouldIgnoreUnmodeledPropertyChanges)]
        pub unsafe fn contextShouldIgnoreUnmodeledPropertyChanges() -> bool;

        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest() -> Id<NSFetchRequest, Shared>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "CoreData_NSManagedObjectContext"
        ))]
        #[method_id(@__retain_semantics Init initWithEntity:insertIntoManagedObjectContext:)]
        pub unsafe fn initWithEntity_insertIntoManagedObjectContext(
            this: Option<Allocated<Self>>,
            entity: &NSEntityDescription,
            context: Option<&NSManagedObjectContext>,
        ) -> Id<NSManagedObject, Shared>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Init initWithContext:)]
        pub unsafe fn initWithContext(
            this: Option<Allocated<Self>>,
            moc: &NSManagedObjectContext,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Option<Id<NSManagedObjectContext, Shared>>;

        #[cfg(feature = "CoreData_NSManagedObjectID")]
        #[method_id(@__retain_semantics Other objectID)]
        pub unsafe fn objectID(&self) -> Id<NSManagedObjectID, Shared>;

        #[method(isInserted)]
        pub unsafe fn isInserted(&self) -> bool;

        #[method(isUpdated)]
        pub unsafe fn isUpdated(&self) -> bool;

        #[method(isDeleted)]
        pub unsafe fn isDeleted(&self) -> bool;

        #[method(hasChanges)]
        pub unsafe fn hasChanges(&self) -> bool;

        #[method(hasPersistentChangedValues)]
        pub unsafe fn hasPersistentChangedValues(&self) -> bool;

        #[method(isFault)]
        pub unsafe fn isFault(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(hasFaultForRelationshipNamed:)]
        pub unsafe fn hasFaultForRelationshipNamed(&self, key: &NSString) -> bool;

        #[cfg(all(
            feature = "CoreData_NSManagedObjectID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other objectIDsForRelationshipNamed:)]
        pub unsafe fn objectIDsForRelationshipNamed(
            &self,
            key: &NSString,
        ) -> Id<NSArray<NSManagedObjectID>, Shared>;

        #[method(faultingState)]
        pub unsafe fn faultingState(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method(willAccessValueForKey:)]
        pub unsafe fn willAccessValueForKey(&self, key: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(didAccessValueForKey:)]
        pub unsafe fn didAccessValueForKey(&self, key: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(willChangeValueForKey:)]
        pub unsafe fn willChangeValueForKey(&self, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(didChangeValueForKey:)]
        pub unsafe fn didChangeValueForKey(&self, key: &NSString);

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method(willChangeValueForKey:withSetMutation:usingObjects:)]
        pub unsafe fn willChangeValueForKey_withSetMutation_usingObjects(
            &self,
            in_key: &NSString,
            in_mutation_kind: NSKeyValueSetMutationKind,
            in_objects: &NSSet,
        );

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method(didChangeValueForKey:withSetMutation:usingObjects:)]
        pub unsafe fn didChangeValueForKey_withSetMutation_usingObjects(
            &self,
            in_key: &NSString,
            in_mutation_kind: NSKeyValueSetMutationKind,
            in_objects: &NSSet,
        );

        #[method(awakeFromFetch)]
        pub unsafe fn awakeFromFetch(&self);

        #[method(awakeFromInsert)]
        pub unsafe fn awakeFromInsert(&self);

        #[method(awakeFromSnapshotEvents:)]
        pub unsafe fn awakeFromSnapshotEvents(&self, flags: NSSnapshotEventType);

        #[method(prepareForDeletion)]
        pub unsafe fn prepareForDeletion(&self);

        #[method(willSave)]
        pub unsafe fn willSave(&self);

        #[method(didSave)]
        pub unsafe fn didSave(&self);

        #[method(willTurnIntoFault)]
        pub unsafe fn willTurnIntoFault(&self);

        #[method(didTurnIntoFault)]
        pub unsafe fn didTurnIntoFault(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueForKey:)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:forKey:)]
        pub unsafe fn setValue_forKey(&self, value: Option<&Object>, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other primitiveValueForKey:)]
        pub unsafe fn primitiveValueForKey(&self, key: &NSString) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPrimitiveValue:forKey:)]
        pub unsafe fn setPrimitiveValue_forKey(&self, value: Option<&Object>, key: &NSString);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other committedValuesForKeys:)]
        pub unsafe fn committedValuesForKeys(
            &self,
            keys: Option<&NSArray<NSString>>,
        ) -> Id<NSDictionary<NSString, Object>, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other changedValues)]
        pub unsafe fn changedValues(&self) -> Id<NSDictionary<NSString, Object>, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other changedValuesForCurrentEvent)]
        pub unsafe fn changedValuesForCurrentEvent(
            &self,
        ) -> Id<NSDictionary<NSString, Object>, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(validateValue:forKey:error:_)]
        pub unsafe fn validateValue_forKey_error(
            &self,
            value: NonNull<*mut Object>,
            key: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(validateForDelete:_)]
        pub unsafe fn validateForDelete(&self) -> Result<(), Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(validateForInsert:_)]
        pub unsafe fn validateForInsert(&self) -> Result<(), Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(validateForUpdate:_)]
        pub unsafe fn validateForUpdate(&self) -> Result<(), Id<NSError, Shared>>;

        #[method(setObservationInfo:)]
        pub unsafe fn setObservationInfo(&self, in_observation_info: *mut c_void);

        #[method(observationInfo)]
        pub unsafe fn observationInfo(&self) -> *mut c_void;
    }
);
