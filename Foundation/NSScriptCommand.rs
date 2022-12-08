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
    #[derive(Debug)]
    pub struct NSScriptCommand;

    unsafe impl ClassType for NSScriptCommand {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSScriptCommand {
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            commandDef: &NSScriptCommandDescription,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            inCoder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other commandDescription)]
        pub unsafe fn commandDescription(&self) -> Id<NSScriptCommandDescription, Shared>;

        #[method_id(@__retain_semantics Other directParameter)]
        pub unsafe fn directParameter(&self) -> Option<Id<Object, Shared>>;

        #[method(setDirectParameter:)]
        pub unsafe fn setDirectParameter(&self, directParameter: Option<&Object>);

        #[method_id(@__retain_semantics Other receiversSpecifier)]
        pub unsafe fn receiversSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>>;

        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(
            &self,
            receiversSpecifier: Option<&NSScriptObjectSpecifier>,
        );

        #[method_id(@__retain_semantics Other evaluatedReceivers)]
        pub unsafe fn evaluatedReceivers(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setArguments:)]
        pub unsafe fn setArguments(&self, arguments: Option<&NSDictionary<NSString, Object>>);

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
        pub unsafe fn setScriptErrorNumber(&self, scriptErrorNumber: NSInteger);

        #[method_id(@__retain_semantics Other scriptErrorOffendingObjectDescriptor)]
        pub unsafe fn scriptErrorOffendingObjectDescriptor(
            &self,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;

        #[method(setScriptErrorOffendingObjectDescriptor:)]
        pub unsafe fn setScriptErrorOffendingObjectDescriptor(
            &self,
            scriptErrorOffendingObjectDescriptor: Option<&NSAppleEventDescriptor>,
        );

        #[method_id(@__retain_semantics Other scriptErrorExpectedTypeDescriptor)]
        pub unsafe fn scriptErrorExpectedTypeDescriptor(
            &self,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;

        #[method(setScriptErrorExpectedTypeDescriptor:)]
        pub unsafe fn setScriptErrorExpectedTypeDescriptor(
            &self,
            scriptErrorExpectedTypeDescriptor: Option<&NSAppleEventDescriptor>,
        );

        #[method_id(@__retain_semantics Other scriptErrorString)]
        pub unsafe fn scriptErrorString(&self) -> Option<Id<NSString, Shared>>;

        #[method(setScriptErrorString:)]
        pub unsafe fn setScriptErrorString(&self, scriptErrorString: Option<&NSString>);

        #[method_id(@__retain_semantics Other currentCommand)]
        pub unsafe fn currentCommand() -> Option<Id<NSScriptCommand, Shared>>;

        #[method_id(@__retain_semantics Other appleEvent)]
        pub unsafe fn appleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>>;

        #[method(suspendExecution)]
        pub unsafe fn suspendExecution(&self);

        #[method(resumeExecutionWithResult:)]
        pub unsafe fn resumeExecutionWithResult(&self, result: Option<&Object>);
    }
);
