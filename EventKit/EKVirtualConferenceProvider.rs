//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::EventKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKVirtualConferenceProvider")]
    pub struct EKVirtualConferenceProvider;

    #[cfg(feature = "EventKit_EKVirtualConferenceProvider")]
    unsafe impl ClassType for EKVirtualConferenceProvider {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "EventKit_EKVirtualConferenceProvider")]
    unsafe impl EKVirtualConferenceProvider {
        #[cfg(all(
            feature = "EventKit_EKVirtualConferenceRoomTypeDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchAvailableRoomTypesWithCompletionHandler:)]
        pub unsafe fn fetchAvailableRoomTypesWithCompletionHandler(
            &self,
            completion_handler: &Block<
                (
                    *mut NSArray<EKVirtualConferenceRoomTypeDescriptor>,
                    *mut NSError,
                ),
                (),
            >,
        );

        #[cfg(all(
            feature = "EventKit_EKVirtualConferenceDescriptor",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchVirtualConferenceForIdentifier:completionHandler:)]
        pub unsafe fn fetchVirtualConferenceForIdentifier_completionHandler(
            &self,
            identifier: &EKVirtualConferenceRoomTypeIdentifier,
            completion_handler: &Block<(*mut EKVirtualConferenceDescriptor, *mut NSError), ()>,
        );
    }
);
