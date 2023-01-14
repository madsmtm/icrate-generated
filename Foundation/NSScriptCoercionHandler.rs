//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCoercionHandler")]
    pub struct NSScriptCoercionHandler;

    #[cfg(feature = "Foundation_NSScriptCoercionHandler")]
    unsafe impl ClassType for NSScriptCoercionHandler {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCoercionHandler")]
    unsafe impl NSScriptCoercionHandler {
        #[method_id(@__retain_semantics Other sharedCoercionHandler)]
        pub unsafe fn sharedCoercionHandler() -> Id<NSScriptCoercionHandler, Shared>;

        #[method_id(@__retain_semantics Other coerceValue:toClass:)]
        pub unsafe fn coerceValue_toClass(
            &self,
            value: &Object,
            to_class: &Class,
        ) -> Option<Id<Object, Shared>>;

        #[method(registerCoercer:selector:toConvertFromClass:toClass:)]
        pub unsafe fn registerCoercer_selector_toConvertFromClass_toClass(
            &self,
            coercer: &Object,
            selector: Sel,
            from_class: &Class,
            to_class: &Class,
        );
    }
);
