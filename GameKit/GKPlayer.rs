//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static GKPlayerIDNoLongerAvailable: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKBasePlayer")]
    pub struct GKPlayer;

    #[cfg(feature = "GameKit_GKBasePlayer")]
    unsafe impl ClassType for GKPlayer {
        #[inherits(NSObject)]
        type Super = GKBasePlayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKBasePlayer")]
unsafe impl NSObjectProtocol for GKPlayer {}

extern_methods!(
    #[cfg(feature = "GameKit_GKBasePlayer")]
    unsafe impl GKPlayer {
        #[method(scopedIDsArePersistent)]
        pub unsafe fn scopedIDsArePersistent(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other gamePlayerID)]
        pub unsafe fn gamePlayerID(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other teamPlayerID)]
        pub unsafe fn teamPlayerID(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alias)]
        pub unsafe fn alias(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other anonymousGuestPlayerWithIdentifier:)]
        pub unsafe fn anonymousGuestPlayerWithIdentifier(guest_identifier: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other guestIdentifier)]
        pub unsafe fn guestIdentifier(&self) -> Option<Id<NSString>>;

        #[method(isInvitable)]
        pub unsafe fn isInvitable(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKBasePlayer")]
    unsafe impl GKPlayer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKPhotoSize(pub NSInteger);
impl GKPhotoSize {
    #[doc(alias = "GKPhotoSizeSmall")]
    pub const Small: Self = Self(0);
    #[doc(alias = "GKPhotoSizeNormal")]
    pub const Normal: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for GKPhotoSize {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for GKPhotoSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// UI
    #[cfg(feature = "GameKit_GKBasePlayer")]
    unsafe impl GKPlayer {
        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSError"))]
        #[method(loadPhotoForSize:withCompletionHandler:)]
        pub unsafe fn loadPhotoForSize_withCompletionHandler(
            &self,
            size: GKPhotoSize,
            completion_handler: Option<&Block<dyn Fn(*mut NSImage, *mut NSError)>>,
        );
    }
);

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static GKPlayerDidChangeNotificationName: &'static NSNotificationName;
}

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GameKit_GKBasePlayer")]
    unsafe impl GKPlayer {
        #[deprecated]
        #[method(isFriend)]
        pub unsafe fn isFriend(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use either the gamePlayerID or teamPlayerID property to identify a player."]
        #[method_id(@__retain_semantics Other playerID)]
        pub unsafe fn playerID(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[deprecated]
        #[method(loadPlayersForIdentifiers:withCompletionHandler:)]
        pub unsafe fn loadPlayersForIdentifiers_withCompletionHandler(
            identifiers: &NSArray<NSString>,
            completion_handler: Option<&Block<dyn Fn(*mut NSArray<GKPlayer>, *mut NSError)>>,
        );
    }
);
