//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSObjectController")]
    pub struct NSObjectController;

    #[cfg(feature = "AppKit_NSObjectController")]
    unsafe impl ClassType for NSObjectController {
        #[inherits(NSObject)]
        type Super = NSController;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSObjectController")]
    unsafe impl NSObjectController {
        #[method_id(@__retain_semantics Init initWithContent:)]
        pub unsafe fn initWithContent(
            this: Option<Allocated<Self>>,
            content: Option<&Object>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other content)]
        pub unsafe fn content(&self) -> Option<Id<Object, Shared>>;

        #[method(setContent:)]
        pub unsafe fn setContent(&self, content: Option<&Object>);

        #[method_id(@__retain_semantics Other selection)]
        pub unsafe fn selection(&self) -> Id<Object, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other selectedObjects)]
        pub unsafe fn selectedObjects(&self) -> Id<NSArray, Shared>;

        #[method(automaticallyPreparesContent)]
        pub unsafe fn automaticallyPreparesContent(&self) -> bool;

        #[method(setAutomaticallyPreparesContent:)]
        pub unsafe fn setAutomaticallyPreparesContent(&self, automatically_prepares_content: bool);

        #[method(prepareContent)]
        pub unsafe fn prepareContent(&self);

        #[method(objectClass)]
        pub unsafe fn objectClass(&self) -> Option<&'static Class>;

        #[method(setObjectClass:)]
        pub unsafe fn setObjectClass(&self, object_class: Option<&Class>);

        #[method_id(@__retain_semantics New newObject)]
        pub unsafe fn newObject(&self) -> Id<Object, Shared>;

        #[method(addObject:)]
        pub unsafe fn addObject(&self, object: &Object);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, object: &Object);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(add:)]
        pub unsafe fn add(&self, sender: Option<&Object>);

        #[method(canAdd)]
        pub unsafe fn canAdd(&self) -> bool;

        #[method(remove:)]
        pub unsafe fn remove(&self, sender: Option<&Object>);

        #[method(canRemove)]
        pub unsafe fn canRemove(&self) -> bool;

        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(&self, item: &NSValidatedUserInterfaceItem)
            -> bool;
    }
);

extern_methods!(
    /// NSManagedController
    #[cfg(feature = "AppKit_NSObjectController")]
    unsafe impl NSObjectController {
        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Option<Id<NSManagedObjectContext, Shared>>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method(setManagedObjectContext:)]
        pub unsafe fn setManagedObjectContext(
            &self,
            managed_object_context: Option<&NSManagedObjectContext>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other entityName)]
        pub unsafe fn entityName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setEntityName:)]
        pub unsafe fn setEntityName(&self, entity_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other fetchPredicate)]
        pub unsafe fn fetchPredicate(&self) -> Option<Id<NSPredicate, Shared>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(setFetchPredicate:)]
        pub unsafe fn setFetchPredicate(&self, fetch_predicate: Option<&NSPredicate>);

        #[cfg(all(feature = "CoreData_NSFetchRequest", feature = "Foundation_NSError"))]
        #[method(fetchWithRequest:merge:error:_)]
        pub unsafe fn fetchWithRequest_merge_error(
            &self,
            fetch_request: Option<&NSFetchRequest>,
            merge: bool,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(fetch:)]
        pub unsafe fn fetch(&self, sender: Option<&Object>);

        #[method(usesLazyFetching)]
        pub unsafe fn usesLazyFetching(&self) -> bool;

        #[method(setUsesLazyFetching:)]
        pub unsafe fn setUsesLazyFetching(&self, uses_lazy_fetching: bool);

        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Other defaultFetchRequest)]
        pub unsafe fn defaultFetchRequest(&self) -> Id<NSFetchRequest, Shared>;
    }
);
