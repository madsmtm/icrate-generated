//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPropertyMapping;

    unsafe impl ClassType for NSPropertyMapping {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSPropertyMapping {}

extern_methods!(
    unsafe impl NSPropertyMapping {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Other valueExpression)]
        pub unsafe fn valueExpression(&self) -> Option<Id<NSExpression>>;

        #[cfg(feature = "Foundation_NSExpression")]
        #[method(setValueExpression:)]
        pub unsafe fn setValueExpression(&self, value_expression: Option<&NSExpression>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPropertyMapping {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
