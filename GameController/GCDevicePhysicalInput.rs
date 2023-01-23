//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_protocol!(
    pub struct GCDevicePhysicalInput;

    unsafe impl ProtocolType for GCDevicePhysicalInput {
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Id<GCDevice, Shared>>;

        #[method(elementValueDidChangeHandler)]
        pub unsafe fn elementValueDidChangeHandler(
            &self,
        ) -> *mut Block<
            (
                NonNull<GCDevicePhysicalInput>,
                NonNull<GCPhysicalInputElement>,
            ),
            (),
        >;

        #[method(setElementValueDidChangeHandler:)]
        pub unsafe fn setElementValueDidChangeHandler(
            &self,
            element_value_did_change_handler: Option<
                &Block<
                    (
                        NonNull<GCDevicePhysicalInput>,
                        NonNull<GCPhysicalInputElement>,
                    ),
                    (),
                >,
            >,
        );

        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Id<GCDevicePhysicalInputState, Shared>;

        #[method(inputStateAvailableHandler)]
        pub unsafe fn inputStateAvailableHandler(
            &self,
        ) -> *mut Block<(NonNull<GCDevicePhysicalInput>,), ()>;

        #[method(setInputStateAvailableHandler:)]
        pub unsafe fn setInputStateAvailableHandler(
            &self,
            input_state_available_handler: Option<&Block<(NonNull<GCDevicePhysicalInput>,), ()>>,
        );

        #[method(inputStateQueueDepth)]
        pub unsafe fn inputStateQueueDepth(&self) -> NSInteger;

        #[method(setInputStateQueueDepth:)]
        pub unsafe fn setInputStateQueueDepth(&self, input_state_queue_depth: NSInteger);

        #[method_id(@__retain_semantics Other nextInputState)]
        pub unsafe fn nextInputState(&self) -> Option<Id<TodoProtocols, Shared>>;
    }
);
