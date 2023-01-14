//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSAtomicStoreCacheNode")]
    pub struct NSAtomicStoreCacheNode;

    #[cfg(feature = "CoreData_NSAtomicStoreCacheNode")]
    unsafe impl ClassType for NSAtomicStoreCacheNode {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSAtomicStoreCacheNode")]
    unsafe impl NSAtomicStoreCacheNode {
        #[cfg(feature = "CoreData_NSManagedObjectID")]
        #[method_id(@__retain_semantics Init initWithObjectID:)]
        pub unsafe fn initWithObjectID(
            this: Option<Allocated<Self>>,
            moid: &NSManagedObjectID,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "CoreData_NSManagedObjectID")]
        #[method_id(@__retain_semantics Other objectID)]
        pub unsafe fn objectID(&self) -> Id<NSManagedObjectID, Shared>;

        #[cfg(all(
            feature = "Foundation_NSMutableDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other propertyCache)]
        pub unsafe fn propertyCache(
            &self,
        ) -> Option<Id<NSMutableDictionary<NSString, Object>, Owned>>;

        #[cfg(all(
            feature = "Foundation_NSMutableDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(setPropertyCache:)]
        pub unsafe fn setPropertyCache(
            &self,
            property_cache: Option<&NSMutableDictionary<NSString, Object>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueForKey:)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:forKey:)]
        pub unsafe fn setValue_forKey(&self, value: Option<&Object>, key: &NSString);
    }
);
