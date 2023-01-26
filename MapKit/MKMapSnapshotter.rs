//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

pub type MKMapSnapshotCompletionHandler = *mut Block<(*mut MKMapSnapshot, *mut NSError), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKMapSnapshotter")]
    pub struct MKMapSnapshotter;

    #[cfg(feature = "MapKit_MKMapSnapshotter")]
    unsafe impl ClassType for MKMapSnapshotter {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKMapSnapshotter")]
unsafe impl NSObjectProtocol for MKMapSnapshotter {}

extern_methods!(
    #[cfg(feature = "MapKit_MKMapSnapshotter")]
    unsafe impl MKMapSnapshotter {
        #[cfg(feature = "MapKit_MKMapSnapshotOptions")]
        #[method_id(@__retain_semantics Init initWithOptions:)]
        pub unsafe fn initWithOptions(
            this: Option<Allocated<Self>>,
            options: &MKMapSnapshotOptions,
        ) -> Id<Self, Shared>;

        #[method(startWithCompletionHandler:)]
        pub unsafe fn startWithCompletionHandler(
            &self,
            completion_handler: MKMapSnapshotCompletionHandler,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(isLoading)]
        pub unsafe fn isLoading(&self) -> bool;
    }
);