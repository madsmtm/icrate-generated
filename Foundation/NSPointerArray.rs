//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSPointerArray")]
    pub struct NSPointerArray;

    #[cfg(feature = "Foundation_NSPointerArray")]
    unsafe impl ClassType for NSPointerArray {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSPointerArray")]
    unsafe impl NSPointerArray {
        #[method_id(@__retain_semantics Init initWithOptions:)]
        pub unsafe fn initWithOptions(
            this: Option<Allocated<Self>>,
            options: NSPointerFunctionsOptions,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Init initWithPointerFunctions:)]
        pub unsafe fn initWithPointerFunctions(
            this: Option<Allocated<Self>>,
            functions: &NSPointerFunctions,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other pointerArrayWithOptions:)]
        pub unsafe fn pointerArrayWithOptions(
            options: NSPointerFunctionsOptions,
        ) -> Id<NSPointerArray, Shared>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Other pointerArrayWithPointerFunctions:)]
        pub unsafe fn pointerArrayWithPointerFunctions(
            functions: &NSPointerFunctions,
        ) -> Id<NSPointerArray, Shared>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Other pointerFunctions)]
        pub unsafe fn pointerFunctions(&self) -> Id<NSPointerFunctions, Shared>;

        #[method(pointerAtIndex:)]
        pub unsafe fn pointerAtIndex(&self, index: NSUInteger) -> *mut c_void;

        #[method(addPointer:)]
        pub unsafe fn addPointer(&self, pointer: *mut c_void);

        #[method(removePointerAtIndex:)]
        pub unsafe fn removePointerAtIndex(&self, index: NSUInteger);

        #[method(insertPointer:atIndex:)]
        pub unsafe fn insertPointer_atIndex(&self, item: *mut c_void, index: NSUInteger);

        #[method(replacePointerAtIndex:withPointer:)]
        pub unsafe fn replacePointerAtIndex_withPointer(
            &self,
            index: NSUInteger,
            item: *mut c_void,
        );

        #[method(compact)]
        pub unsafe fn compact(&self);

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method(setCount:)]
        pub unsafe fn setCount(&self, count: NSUInteger);
    }
);

extern_methods!(
    /// NSPointerArrayConveniences
    #[cfg(feature = "Foundation_NSPointerArray")]
    unsafe impl NSPointerArray {
        #[method_id(@__retain_semantics Other pointerArrayWithStrongObjects)]
        pub unsafe fn pointerArrayWithStrongObjects() -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other pointerArrayWithWeakObjects)]
        pub unsafe fn pointerArrayWithWeakObjects() -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other strongObjectsPointerArray)]
        pub unsafe fn strongObjectsPointerArray() -> Id<NSPointerArray, Shared>;

        #[method_id(@__retain_semantics Other weakObjectsPointerArray)]
        pub unsafe fn weakObjectsPointerArray() -> Id<NSPointerArray, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allObjects)]
        pub unsafe fn allObjects(&self) -> Id<NSArray, Shared>;
    }
);
