//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCPhysicalInputProfile")]
    pub struct GCPhysicalInputProfile;

    #[cfg(feature = "GameController_GCPhysicalInputProfile")]
    unsafe impl ClassType for GCPhysicalInputProfile {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "GameController_GCPhysicalInputProfile")]
    unsafe impl GCPhysicalInputProfile {
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Id<GCDevice, Shared>>;

        #[method(lastEventTimestamp)]
        pub unsafe fn lastEventTimestamp(&self) -> NSTimeInterval;

        #[method(hasRemappedElements)]
        pub unsafe fn hasRemappedElements(&self) -> bool;

        #[cfg(feature = "GameController_GCControllerElement")]
        #[method(valueDidChangeHandler)]
        pub unsafe fn valueDidChangeHandler(
            &self,
        ) -> *mut Block<
            (
                NonNull<GCPhysicalInputProfile>,
                NonNull<GCControllerElement>,
            ),
            (),
        >;

        #[cfg(feature = "GameController_GCControllerElement")]
        #[method(setValueDidChangeHandler:)]
        pub unsafe fn setValueDidChangeHandler(
            &self,
            value_did_change_handler: Option<
                &Block<
                    (
                        NonNull<GCPhysicalInputProfile>,
                        NonNull<GCControllerElement>,
                    ),
                    (),
                >,
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "GameController_GCControllerElement"
        ))]
        #[method_id(@__retain_semantics Other elements)]
        pub unsafe fn elements(&self) -> Id<NSDictionary<NSString, GCControllerElement>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "GameController_GCControllerButtonInput"
        ))]
        #[method_id(@__retain_semantics Other buttons)]
        pub unsafe fn buttons(&self)
            -> Id<NSDictionary<NSString, GCControllerButtonInput>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "GameController_GCControllerAxisInput"
        ))]
        #[method_id(@__retain_semantics Other axes)]
        pub unsafe fn axes(&self) -> Id<NSDictionary<NSString, GCControllerAxisInput>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "GameController_GCControllerDirectionPad"
        ))]
        #[method_id(@__retain_semantics Other dpads)]
        pub unsafe fn dpads(&self) -> Id<NSDictionary<NSString, GCControllerDirectionPad>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "GameController_GCControllerTouchpad"
        ))]
        #[method_id(@__retain_semantics Other touchpads)]
        pub unsafe fn touchpads(&self) -> Id<NSDictionary<NSString, GCControllerTouchpad>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSSet",
            feature = "GameController_GCControllerElement"
        ))]
        #[method_id(@__retain_semantics Other allElements)]
        pub unsafe fn allElements(&self) -> Id<NSSet<GCControllerElement>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSSet",
            feature = "GameController_GCControllerButtonInput"
        ))]
        #[method_id(@__retain_semantics Other allButtons)]
        pub unsafe fn allButtons(&self) -> Id<NSSet<GCControllerButtonInput>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSSet",
            feature = "GameController_GCControllerAxisInput"
        ))]
        #[method_id(@__retain_semantics Other allAxes)]
        pub unsafe fn allAxes(&self) -> Id<NSSet<GCControllerAxisInput>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSSet",
            feature = "GameController_GCControllerDirectionPad"
        ))]
        #[method_id(@__retain_semantics Other allDpads)]
        pub unsafe fn allDpads(&self) -> Id<NSSet<GCControllerDirectionPad>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSSet",
            feature = "GameController_GCControllerTouchpad"
        ))]
        #[method_id(@__retain_semantics Other allTouchpads)]
        pub unsafe fn allTouchpads(&self) -> Id<NSSet<GCControllerTouchpad>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameController_GCControllerElement"
        ))]
        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(
            &self,
            key: &NSString,
        ) -> Option<Id<GCControllerElement, Shared>>;

        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Id<Self, Shared>;

        #[method(setStateFromPhysicalInput:)]
        pub unsafe fn setStateFromPhysicalInput(&self, physical_input: &GCPhysicalInputProfile);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other mappedElementAliasForPhysicalInputName:)]
        pub unsafe fn mappedElementAliasForPhysicalInputName(
            &self,
            input_name: &NSString,
        ) -> Id<NSString, Shared>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other mappedPhysicalInputNamesForElementAlias:)]
        pub unsafe fn mappedPhysicalInputNamesForElementAlias(
            &self,
            element_alias: &NSString,
        ) -> Id<NSSet<NSString>, Shared>;
    }
);
