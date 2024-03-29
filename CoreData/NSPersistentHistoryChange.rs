//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersistentHistoryChangeType(pub NSInteger);
impl NSPersistentHistoryChangeType {
    #[doc(alias = "NSPersistentHistoryChangeTypeInsert")]
    pub const Insert: Self = Self(0);
    #[doc(alias = "NSPersistentHistoryChangeTypeUpdate")]
    pub const Update: Self = Self(1);
    #[doc(alias = "NSPersistentHistoryChangeTypeDelete")]
    pub const Delete: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSPersistentHistoryChangeType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSPersistentHistoryChangeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentHistoryChange;

    unsafe impl ClassType for NSPersistentHistoryChange {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSPersistentHistoryChange {}

unsafe impl NSObjectProtocol for NSPersistentHistoryChange {}

extern_methods!(
    unsafe impl NSPersistentHistoryChange {
        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "CoreData_NSManagedObjectContext"
        ))]
        #[method_id(@__retain_semantics Other entityDescriptionWithContext:)]
        pub unsafe fn entityDescriptionWithContext(
            context: &NSManagedObjectContext,
        ) -> Option<Id<NSEntityDescription>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entityDescription)]
        pub unsafe fn entityDescription() -> Option<Id<NSEntityDescription>>;

        #[cfg(all(
            feature = "CoreData_NSFetchRequest",
            feature = "CoreData_NSPersistentStoreRequest"
        ))]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest() -> Option<Id<NSFetchRequest>>;

        #[method(changeID)]
        pub unsafe fn changeID(&self) -> i64;

        #[cfg(feature = "CoreData_NSManagedObjectID")]
        #[method_id(@__retain_semantics Other changedObjectID)]
        pub unsafe fn changedObjectID(&self) -> Id<NSManagedObjectID>;

        #[method(changeType)]
        pub unsafe fn changeType(&self) -> NSPersistentHistoryChangeType;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other tombstone)]
        pub unsafe fn tombstone(&self) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "CoreData_NSPersistentHistoryTransaction")]
        #[method_id(@__retain_semantics Other transaction)]
        pub unsafe fn transaction(&self) -> Option<Id<NSPersistentHistoryTransaction>>;

        #[cfg(all(
            feature = "CoreData_NSPropertyDescription",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other updatedProperties)]
        pub unsafe fn updatedProperties(&self) -> Option<Id<NSSet<NSPropertyDescription>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPersistentHistoryChange {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
