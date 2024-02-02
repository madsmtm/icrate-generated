//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::Photos::*;
use crate::UniformTypeIdentifiers::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHAuthorizationStatus {
        PHAuthorizationStatusNotDetermined = 0,
        PHAuthorizationStatusRestricted = 1,
        PHAuthorizationStatusDenied = 2,
        PHAuthorizationStatusAuthorized = 3,
        PHAuthorizationStatusLimited = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHAccessLevel {
        PHAccessLevelAddOnly = 1,
        PHAccessLevelReadWrite = 2,
    }
);

extern_protocol!(
    pub unsafe trait PHPhotoLibraryChangeObserver: NSObjectProtocol {
        #[cfg(feature = "Photos_PHChange")]
        #[method(photoLibraryDidChange:)]
        unsafe fn photoLibraryDidChange(&self, change_instance: &PHChange);
    }

    unsafe impl ProtocolType for dyn PHPhotoLibraryChangeObserver {}
);

extern_protocol!(
    pub unsafe trait PHPhotoLibraryAvailabilityObserver: NSObjectProtocol {
        #[cfg(feature = "Photos_PHPhotoLibrary")]
        #[method(photoLibraryDidBecomeUnavailable:)]
        unsafe fn photoLibraryDidBecomeUnavailable(&self, photo_library: &PHPhotoLibrary);
    }

    unsafe impl ProtocolType for dyn PHPhotoLibraryAvailabilityObserver {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHPhotoLibrary")]
    pub struct PHPhotoLibrary;

    #[cfg(feature = "Photos_PHPhotoLibrary")]
    unsafe impl ClassType for PHPhotoLibrary {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Photos_PHPhotoLibrary")]
unsafe impl Send for PHPhotoLibrary {}

#[cfg(feature = "Photos_PHPhotoLibrary")]
unsafe impl Sync for PHPhotoLibrary {}

#[cfg(feature = "Photos_PHPhotoLibrary")]
unsafe impl NSObjectProtocol for PHPhotoLibrary {}

extern_methods!(
    #[cfg(feature = "Photos_PHPhotoLibrary")]
    unsafe impl PHPhotoLibrary {
        #[method_id(@__retain_semantics Other sharedPhotoLibrary)]
        pub unsafe fn sharedPhotoLibrary() -> Id<PHPhotoLibrary>;

        #[method(authorizationStatusForAccessLevel:)]
        pub unsafe fn authorizationStatusForAccessLevel(
            access_level: PHAccessLevel,
        ) -> PHAuthorizationStatus;

        #[method(requestAuthorizationForAccessLevel:handler:)]
        pub unsafe fn requestAuthorizationForAccessLevel_handler(
            access_level: PHAccessLevel,
            handler: &Block<dyn Fn(PHAuthorizationStatus)>,
        );

        #[deprecated]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> PHAuthorizationStatus;

        #[deprecated]
        #[method(requestAuthorization:)]
        pub unsafe fn requestAuthorization(handler: &Block<dyn Fn(PHAuthorizationStatus)>);

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other unavailabilityReason)]
        pub unsafe fn unavailabilityReason(&self) -> Option<Id<NSError>>;

        #[method(registerAvailabilityObserver:)]
        pub unsafe fn registerAvailabilityObserver(
            &self,
            observer: &ProtocolObject<dyn PHPhotoLibraryAvailabilityObserver>,
        );

        #[method(unregisterAvailabilityObserver:)]
        pub unsafe fn unregisterAvailabilityObserver(
            &self,
            observer: &ProtocolObject<dyn PHPhotoLibraryAvailabilityObserver>,
        );

        #[method(registerChangeObserver:)]
        pub unsafe fn registerChangeObserver(
            &self,
            observer: &ProtocolObject<dyn PHPhotoLibraryChangeObserver>,
        );

        #[method(unregisterChangeObserver:)]
        pub unsafe fn unregisterChangeObserver(
            &self,
            observer: &ProtocolObject<dyn PHPhotoLibraryChangeObserver>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Photos_PHPersistentChangeFetchResult",
            feature = "Photos_PHPersistentChangeToken"
        ))]
        #[method_id(@__retain_semantics Other fetchPersistentChangesSinceToken:error:_)]
        pub unsafe fn fetchPersistentChangesSinceToken_error(
            &self,
            token: &PHPersistentChangeToken,
        ) -> Result<Id<PHPersistentChangeFetchResult>, Id<NSError>>;

        #[cfg(feature = "Photos_PHPersistentChangeToken")]
        #[method_id(@__retain_semantics Other currentChangeToken)]
        pub unsafe fn currentChangeToken(&self) -> Id<PHPersistentChangeToken>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Photos_PHPhotoLibrary")]
    unsafe impl PHPhotoLibrary {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
