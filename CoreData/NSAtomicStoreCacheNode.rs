//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAtomicStoreCacheNode;

    unsafe impl ClassType for NSAtomicStoreCacheNode {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAtomicStoreCacheNode {
        #[method_id(@__retain_semantics Init initWithObjectID:)]
        pub unsafe fn initWithObjectID(
            this: Option<Allocated<Self>>,
            moid: &NSManagedObjectID,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other objectID)]
        pub unsafe fn objectID(&self) -> Id<NSManagedObjectID, Shared>;

        #[method_id(@__retain_semantics Other propertyCache)]
        pub unsafe fn propertyCache(
            &self,
        ) -> Option<Id<NSMutableDictionary<NSString, Object>, Owned>>;

        #[method(setPropertyCache:)]
        pub unsafe fn setPropertyCache(
            &self,
            propertyCache: Option<&NSMutableDictionary<NSString, Object>>,
        );

        #[method_id(@__retain_semantics Other valueForKey:)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Option<Id<Object, Shared>>;

        #[method(setValue:forKey:)]
        pub unsafe fn setValue_forKey(&self, value: Option<&Object>, key: &NSString);
    }
);
