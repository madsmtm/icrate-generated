//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum {
        NSNoScriptError = 0,
        NSReceiverEvaluationScriptError = 1,
        NSKeySpecifierEvaluationScriptError = 2,
        NSArgumentEvaluationScriptError = 3,
        NSReceiversCantHandleCommandScriptError = 4,
        NSRequiredArgumentsMissingScriptError = 5,
        NSArgumentsWrongScriptError = 6,
        NSUnknownKeyScriptError = 7,
        NSInternalScriptError = 8,
        NSOperationNotSupportedForKeyScriptError = 9,
        NSCannotCreateScriptCommandError = 10,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommand")]
    pub struct NSScriptCommand;

    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl ClassType for NSScriptCommand {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSScriptCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            command_def: &NSScriptCommandDescription,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            in_coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Other commandDescription)]
        pub unsafe fn commandDescription(&self) -> Id<NSScriptCommandDescription, Shared>;

        #[method_id(@__retain_semantics Other directParameter)]
        pub unsafe fn directParameter(&self) -> Option<Id<Object, Shared>>;

        #[method(setDirectParameter:)]
        pub unsafe fn setDirectParameter(&self, direct_parameter: Option<&Object>);

        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method_id(@__retain_semantics Other receiversSpecifier)]
        pub unsafe fn receiversSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>>;

        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(
            &self,
            receivers_specifier: Option<&NSScriptObjectSpecifier>,
        );

        #[method_id(@__retain_semantics Other evaluatedReceivers)]
        pub unsafe fn evaluatedReceivers(&self) -> Option<Id<Object, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setArguments:)]
        pub unsafe fn setArguments(&self, arguments: Option<&NSDictionary<NSString, Object>>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other evaluatedArguments)]
        pub unsafe fn evaluatedArguments(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(isWellFormed)]
        pub unsafe fn isWellFormed(&self) -> bool;

        #[method_id(@__retain_semantics Other performDefaultImplementation)]
        pub unsafe fn performDefaultImplementation(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other executeCommand)]
        pub unsafe fn executeCommand(&self) -> Option<Id<Object, Shared>>;

        #[method(scriptErrorNumber)]
        pub unsafe fn scriptErrorNumber(&self) -> NSInteger;

        #[method(setScriptErrorNumber:)]
        pub unsafe fn setScriptErrorNumber(&self, script_error_number: NSInteger);

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other scriptErrorOffendingObjectDescriptor)]
        pub unsafe fn scriptErrorOffendingObjectDescriptor(
            &self,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method(setScriptErrorOffendingObjectDescriptor:)]
        pub unsafe fn setScriptErrorOffendingObjectDescriptor(
            &self,
            script_error_offending_object_descriptor: Option<&NSAppleEventDescriptor>,
        );

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other scriptErrorExpectedTypeDescriptor)]
        pub unsafe fn scriptErrorExpectedTypeDescriptor(
            &self,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method(setScriptErrorExpectedTypeDescriptor:)]
        pub unsafe fn setScriptErrorExpectedTypeDescriptor(
            &self,
            script_error_expected_type_descriptor: Option<&NSAppleEventDescriptor>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other scriptErrorString)]
        pub unsafe fn scriptErrorString(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setScriptErrorString:)]
        pub unsafe fn setScriptErrorString(&self, script_error_string: Option<&NSString>);

        #[method_id(@__retain_semantics Other currentCommand)]
        pub unsafe fn currentCommand() -> Option<Id<NSScriptCommand, Shared>>;

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other appleEvent)]
        pub unsafe fn appleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>>;

        #[method(suspendExecution)]
        pub unsafe fn suspendExecution(&self);

        #[method(resumeExecutionWithResult:)]
        pub unsafe fn resumeExecutionWithResult(&self, result: Option<&Object>);
    }
);
