//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSOrderedCollectionDifferenceCalculationOptions {
        NSOrderedCollectionDifferenceCalculationOmitInsertedObjects = 1 << 0,
        NSOrderedCollectionDifferenceCalculationOmitRemovedObjects = 1 << 1,
        NSOrderedCollectionDifferenceCalculationInferMoves = 1 << 2,
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSOrderedCollectionDifference<
        ObjectType: Message = Object,
        ObjectTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSOrderedCollectionDifference<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSOrderedCollectionDifference<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Init initWithChanges:)]
        pub unsafe fn initWithChanges(
            this: Option<Allocated<Self>>,
            changes: &NSArray<NSOrderedCollectionChange<ObjectType>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithInsertIndexes:insertedObjects:removeIndexes:removedObjects:additionalChanges:)]
        pub unsafe fn initWithInsertIndexes_insertedObjects_removeIndexes_removedObjects_additionalChanges(
            this: Option<Allocated<Self>>,
            inserts: &NSIndexSet,
            insertedObjects: Option<&NSArray<ObjectType>>,
            removes: &NSIndexSet,
            removedObjects: Option<&NSArray<ObjectType>>,
            changes: &NSArray<NSOrderedCollectionChange<ObjectType>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithInsertIndexes:insertedObjects:removeIndexes:removedObjects:)]
        pub unsafe fn initWithInsertIndexes_insertedObjects_removeIndexes_removedObjects(
            this: Option<Allocated<Self>>,
            inserts: &NSIndexSet,
            insertedObjects: Option<&NSArray<ObjectType>>,
            removes: &NSIndexSet,
            removedObjects: Option<&NSArray<ObjectType>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other insertions)]
        pub unsafe fn insertions(
            &self,
        ) -> Id<NSArray<NSOrderedCollectionChange<ObjectType>>, Shared>;

        #[method_id(@__retain_semantics Other removals)]
        pub unsafe fn removals(&self)
            -> Id<NSArray<NSOrderedCollectionChange<ObjectType>>, Shared>;

        #[method(hasChanges)]
        pub unsafe fn hasChanges(&self) -> bool;

        #[method_id(@__retain_semantics Other differenceByTransformingChangesWithBlock:)]
        pub unsafe fn differenceByTransformingChangesWithBlock(
            &self,
            block: &Block<
                (NonNull<NSOrderedCollectionChange<ObjectType>>,),
                NonNull<NSOrderedCollectionChange<Object>>,
            >,
        ) -> Id<NSOrderedCollectionDifference<Object>, Shared>;

        #[method_id(@__retain_semantics Other inverseDifference)]
        pub unsafe fn inverseDifference(&self) -> Id<Self, Shared>;
    }
);
