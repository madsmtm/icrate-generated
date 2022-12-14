//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScriptCommandDescription;

    unsafe impl ClassType for NSScriptCommandDescription {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSScriptCommandDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithSuiteName:commandName:dictionary:)]
        pub unsafe fn initWithSuiteName_commandName_dictionary(
            this: Option<Allocated<Self>>,
            suiteName: &NSString,
            commandName: &NSString,
            commandDeclaration: Option<&NSDictionary>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            inCoder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other suiteName)]
        pub unsafe fn suiteName(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other commandName)]
        pub unsafe fn commandName(&self) -> Id<NSString, Shared>;

        #[method(appleEventClassCode)]
        pub unsafe fn appleEventClassCode(&self) -> FourCharCode;

        #[method(appleEventCode)]
        pub unsafe fn appleEventCode(&self) -> FourCharCode;

        #[method_id(@__retain_semantics Other commandClassName)]
        pub unsafe fn commandClassName(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other returnType)]
        pub unsafe fn returnType(&self) -> Option<Id<NSString, Shared>>;

        #[method(appleEventCodeForReturnType)]
        pub unsafe fn appleEventCodeForReturnType(&self) -> FourCharCode;

        #[method_id(@__retain_semantics Other argumentNames)]
        pub unsafe fn argumentNames(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other typeForArgumentWithName:)]
        pub unsafe fn typeForArgumentWithName(
            &self,
            argumentName: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method(appleEventCodeForArgumentWithName:)]
        pub unsafe fn appleEventCodeForArgumentWithName(
            &self,
            argumentName: &NSString,
        ) -> FourCharCode;

        #[method(isOptionalArgumentWithName:)]
        pub unsafe fn isOptionalArgumentWithName(&self, argumentName: &NSString) -> bool;

        #[method_id(@__retain_semantics Other createCommandInstance)]
        pub unsafe fn createCommandInstance(&self) -> Id<NSScriptCommand, Shared>;

        #[method_id(@__retain_semantics Other createCommandInstanceWithZone:)]
        pub unsafe fn createCommandInstanceWithZone(
            &self,
            zone: *mut NSZone,
        ) -> Id<NSScriptCommand, Shared>;
    }
);
