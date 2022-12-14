//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSMapTableStrongMemory: NSPointerFunctionsOptions = NSPointerFunctionsStrongMemory);

extern_static!(
    NSMapTableZeroingWeakMemory: NSPointerFunctionsOptions = NSPointerFunctionsZeroingWeakMemory
);

extern_static!(NSMapTableCopyIn: NSPointerFunctionsOptions = NSPointerFunctionsCopyIn);

extern_static!(
    NSMapTableObjectPointerPersonality: NSPointerFunctionsOptions =
        NSPointerFunctionsObjectPointerPersonality
);

extern_static!(NSMapTableWeakMemory: NSPointerFunctionsOptions = NSPointerFunctionsWeakMemory);

pub type NSMapTableOptions = NSUInteger;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMapTable<
        KeyType: Message = Object,
        ObjectType: Message = Object,
        KeyTypeOwnership: Ownership = Shared,
        ObjectTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (KeyType, KeyTypeOwnership)>,
        _inner1: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > ClassType for NSMapTable<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSMapTable<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Init initWithKeyOptions:valueOptions:capacity:)]
        pub unsafe fn initWithKeyOptions_valueOptions_capacity(
            this: Option<Allocated<Self>>,
            keyOptions: NSPointerFunctionsOptions,
            valueOptions: NSPointerFunctionsOptions,
            initialCapacity: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithKeyPointerFunctions:valuePointerFunctions:capacity:)]
        pub unsafe fn initWithKeyPointerFunctions_valuePointerFunctions_capacity(
            this: Option<Allocated<Self>>,
            keyFunctions: &NSPointerFunctions,
            valueFunctions: &NSPointerFunctions,
            initialCapacity: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other mapTableWithKeyOptions:valueOptions:)]
        pub unsafe fn mapTableWithKeyOptions_valueOptions(
            keyOptions: NSPointerFunctionsOptions,
            valueOptions: NSPointerFunctionsOptions,
        ) -> Id<NSMapTable<KeyType, ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other mapTableWithStrongToStrongObjects)]
        pub unsafe fn mapTableWithStrongToStrongObjects() -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other mapTableWithWeakToStrongObjects)]
        pub unsafe fn mapTableWithWeakToStrongObjects() -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other mapTableWithStrongToWeakObjects)]
        pub unsafe fn mapTableWithStrongToWeakObjects() -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other mapTableWithWeakToWeakObjects)]
        pub unsafe fn mapTableWithWeakToWeakObjects() -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other strongToStrongObjectsMapTable)]
        pub unsafe fn strongToStrongObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other weakToStrongObjectsMapTable)]
        pub unsafe fn weakToStrongObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other strongToWeakObjectsMapTable)]
        pub unsafe fn strongToWeakObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other weakToWeakObjectsMapTable)]
        pub unsafe fn weakToWeakObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other keyPointerFunctions)]
        pub unsafe fn keyPointerFunctions(&self) -> Id<NSPointerFunctions, Shared>;

        #[method_id(@__retain_semantics Other valuePointerFunctions)]
        pub unsafe fn valuePointerFunctions(&self) -> Id<NSPointerFunctions, Shared>;

        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(
            &self,
            aKey: Option<&KeyType>,
        ) -> Option<Id<ObjectType, ObjectTypeOwnership>>;

        #[method(removeObjectForKey:)]
        pub unsafe fn removeObjectForKey(&self, aKey: Option<&KeyType>);

        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(
            &self,
            anObject: Option<&ObjectType>,
            aKey: Option<&KeyType>,
        );

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other keyEnumerator)]
        pub unsafe fn keyEnumerator(&self) -> Id<NSEnumerator<KeyType>, Shared>;

        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Option<Id<NSEnumerator<ObjectType>, Shared>>;

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);

        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(
            &self,
        ) -> Id<NSDictionary<KeyType, ObjectType>, Shared>;
    }
);

extern_struct!(
    pub struct NSMapEnumerator {
        _pi: NSUInteger,
        _si: NSUInteger,
        _bs: *mut c_void,
    }
);

extern_fn!(
    pub unsafe fn NSFreeMapTable(table: &NSMapTable);
);

extern_fn!(
    pub unsafe fn NSResetMapTable(table: &NSMapTable);
);

extern_fn!(
    pub unsafe fn NSCompareMapTables(table1: &NSMapTable, table2: &NSMapTable) -> Bool;
);

extern_fn!(
    pub unsafe fn NSCopyMapTableWithZone(
        table: &NSMapTable,
        zone: *mut NSZone,
    ) -> NonNull<NSMapTable>;
);

extern_fn!(
    pub unsafe fn NSMapMember(
        table: &NSMapTable,
        key: NonNull<c_void>,
        originalKey: *mut *mut c_void,
        value: *mut *mut c_void,
    ) -> Bool;
);

extern_fn!(
    pub unsafe fn NSMapGet(table: &NSMapTable, key: *mut c_void) -> *mut c_void;
);

extern_fn!(
    pub unsafe fn NSMapInsert(table: &NSMapTable, key: *mut c_void, value: *mut c_void);
);

extern_fn!(
    pub unsafe fn NSMapInsertKnownAbsent(table: &NSMapTable, key: *mut c_void, value: *mut c_void);
);

extern_fn!(
    pub unsafe fn NSMapInsertIfAbsent(
        table: &NSMapTable,
        key: *mut c_void,
        value: *mut c_void,
    ) -> *mut c_void;
);

extern_fn!(
    pub unsafe fn NSMapRemove(table: &NSMapTable, key: *mut c_void);
);

extern_fn!(
    pub unsafe fn NSEnumerateMapTable(table: &NSMapTable) -> NSMapEnumerator;
);

extern_fn!(
    pub unsafe fn NSNextMapEnumeratorPair(
        enumerator: NonNull<NSMapEnumerator>,
        key: *mut *mut c_void,
        value: *mut *mut c_void,
    ) -> Bool;
);

extern_fn!(
    pub unsafe fn NSEndMapTableEnumeration(enumerator: NonNull<NSMapEnumerator>);
);

extern_fn!(
    pub unsafe fn NSCountMapTable(table: &NSMapTable) -> NSUInteger;
);

extern_fn!(
    pub unsafe fn NSStringFromMapTable(table: &NSMapTable) -> NonNull<NSString>;
);

extern_fn!(
    pub unsafe fn NSAllMapTableKeys(table: &NSMapTable) -> NonNull<NSArray>;
);

extern_fn!(
    pub unsafe fn NSAllMapTableValues(table: &NSMapTable) -> NonNull<NSArray>;
);

extern_struct!(
    pub struct NSMapTableKeyCallBacks {
        pub hash: Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>) -> NSUInteger>,
        pub isEqual: Option<
            unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>, NonNull<c_void>) -> Bool,
        >,
        pub retain: Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
        pub release: Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
        pub describe:
            Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>) -> *mut NSString>,
        pub notAKeyMarker: *mut c_void,
    }
);

extern_struct!(
    pub struct NSMapTableValueCallBacks {
        pub retain: Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
        pub release: Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
        pub describe:
            Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>) -> *mut NSString>,
    }
);

extern_fn!(
    pub unsafe fn NSCreateMapTableWithZone(
        keyCallBacks: NSMapTableKeyCallBacks,
        valueCallBacks: NSMapTableValueCallBacks,
        capacity: NSUInteger,
        zone: *mut NSZone,
    ) -> NonNull<NSMapTable>;
);

extern_fn!(
    pub unsafe fn NSCreateMapTable(
        keyCallBacks: NSMapTableKeyCallBacks,
        valueCallBacks: NSMapTableValueCallBacks,
        capacity: NSUInteger,
    ) -> NonNull<NSMapTable>;
);

extern_static!(NSIntegerMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSNonOwnedPointerMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSNonOwnedPointerOrNullMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSNonRetainedObjectMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSObjectMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSOwnedPointerMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSIntMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSIntegerMapValueCallBacks: NSMapTableValueCallBacks);

extern_static!(NSNonOwnedPointerMapValueCallBacks: NSMapTableValueCallBacks);

extern_static!(NSObjectMapValueCallBacks: NSMapTableValueCallBacks);

extern_static!(NSNonRetainedObjectMapValueCallBacks: NSMapTableValueCallBacks);

extern_static!(NSOwnedPointerMapValueCallBacks: NSMapTableValueCallBacks);

extern_static!(NSIntMapValueCallBacks: NSMapTableValueCallBacks);
