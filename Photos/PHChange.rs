//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::Photos::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHChange")]
    pub struct PHChange;

    #[cfg(feature = "Photos_PHChange")]
    unsafe impl ClassType for PHChange {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Photos_PHChange")]
unsafe impl Send for PHChange {}

#[cfg(feature = "Photos_PHChange")]
unsafe impl Sync for PHChange {}

#[cfg(feature = "Photos_PHChange")]
unsafe impl NSObjectProtocol for PHChange {}

extern_methods!(
    #[cfg(feature = "Photos_PHChange")]
    unsafe impl PHChange {
        #[cfg(all(feature = "Photos_PHObject", feature = "Photos_PHObjectChangeDetails"))]
        #[method_id(@__retain_semantics Other changeDetailsForObject:)]
        pub unsafe fn changeDetailsForObject(
            &self,
            object: &PHObject,
        ) -> Option<Id<PHObjectChangeDetails>>;

        #[cfg(all(
            feature = "Photos_PHFetchResult",
            feature = "Photos_PHFetchResultChangeDetails"
        ))]
        #[method_id(@__retain_semantics Other changeDetailsForFetchResult:)]
        pub unsafe fn changeDetailsForFetchResult(
            &self,
            object: &PHFetchResult,
        ) -> Option<Id<PHFetchResultChangeDetails>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Photos_PHChange")]
    unsafe impl PHChange {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHObjectChangeDetails")]
    pub struct PHObjectChangeDetails<ObjectType: ?Sized = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut ObjectType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "Photos_PHObjectChangeDetails")]
    unsafe impl<ObjectType: ?Sized + Message> ClassType for PHObjectChangeDetails<ObjectType> {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "Photos_PHObjectChangeDetails")]
unsafe impl<ObjectType: ?Sized> NSObjectProtocol for PHObjectChangeDetails<ObjectType> {}

extern_methods!(
    #[cfg(feature = "Photos_PHObjectChangeDetails")]
    unsafe impl<ObjectType: Message> PHObjectChangeDetails<ObjectType> {
        #[method_id(@__retain_semantics Other objectBeforeChanges)]
        pub unsafe fn objectBeforeChanges(&self) -> Id<ObjectType>;

        #[method_id(@__retain_semantics Other objectAfterChanges)]
        pub unsafe fn objectAfterChanges(&self) -> Option<Id<ObjectType>>;

        #[method(assetContentChanged)]
        pub unsafe fn assetContentChanged(&self) -> bool;

        #[method(objectWasDeleted)]
        pub unsafe fn objectWasDeleted(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Photos_PHObjectChangeDetails")]
    unsafe impl<ObjectType: Message> PHObjectChangeDetails<ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHFetchResultChangeDetails")]
    pub struct PHFetchResultChangeDetails<ObjectType: ?Sized = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut ObjectType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "Photos_PHFetchResultChangeDetails")]
    unsafe impl<ObjectType: ?Sized + Message> ClassType for PHFetchResultChangeDetails<ObjectType> {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "Photos_PHFetchResultChangeDetails")]
unsafe impl<ObjectType: ?Sized> NSObjectProtocol for PHFetchResultChangeDetails<ObjectType> {}

extern_methods!(
    #[cfg(feature = "Photos_PHFetchResultChangeDetails")]
    unsafe impl<ObjectType: Message> PHFetchResultChangeDetails<ObjectType> {
        #[cfg(feature = "Photos_PHFetchResult")]
        #[method_id(@__retain_semantics Other fetchResultBeforeChanges)]
        pub unsafe fn fetchResultBeforeChanges(&self) -> Id<PHFetchResult<ObjectType>>;

        #[cfg(feature = "Photos_PHFetchResult")]
        #[method_id(@__retain_semantics Other fetchResultAfterChanges)]
        pub unsafe fn fetchResultAfterChanges(&self) -> Id<PHFetchResult<ObjectType>>;

        #[method(hasIncrementalChanges)]
        pub unsafe fn hasIncrementalChanges(&self) -> bool;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other removedIndexes)]
        pub unsafe fn removedIndexes(&self) -> Option<Id<NSIndexSet>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other removedObjects)]
        pub unsafe fn removedObjects(&self) -> Id<NSArray<ObjectType>>;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other insertedIndexes)]
        pub unsafe fn insertedIndexes(&self) -> Option<Id<NSIndexSet>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other insertedObjects)]
        pub unsafe fn insertedObjects(&self) -> Id<NSArray<ObjectType>>;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other changedIndexes)]
        pub unsafe fn changedIndexes(&self) -> Option<Id<NSIndexSet>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other changedObjects)]
        pub unsafe fn changedObjects(&self) -> Id<NSArray<ObjectType>>;

        #[method(enumerateMovesWithBlock:)]
        pub unsafe fn enumerateMovesWithBlock(
            &self,
            handler: &Block<dyn Fn(NSUInteger, NSUInteger)>,
        );

        #[method(hasMoves)]
        pub unsafe fn hasMoves(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Photos_PHFetchResult"))]
        #[method_id(@__retain_semantics Other changeDetailsFromFetchResult:toFetchResult:changedObjects:)]
        pub unsafe fn changeDetailsFromFetchResult_toFetchResult_changedObjects(
            from_result: &PHFetchResult<ObjectType>,
            to_result: &PHFetchResult<ObjectType>,
            changed_objects: &NSArray<ObjectType>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Photos_PHFetchResultChangeDetails")]
    unsafe impl<ObjectType: Message> PHFetchResultChangeDetails<ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
