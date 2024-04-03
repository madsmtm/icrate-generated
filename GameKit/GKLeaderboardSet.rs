//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKLeaderboardSet;

    unsafe impl ClassType for GKLeaderboardSet {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for GKLeaderboardSet {}

unsafe impl NSObjectProtocol for GKLeaderboardSet {}

unsafe impl NSSecureCoding for GKLeaderboardSet {}

extern_methods!(
    unsafe impl GKLeaderboardSet {
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other groupIdentifier)]
        pub unsafe fn groupIdentifier(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[cfg(feature = "block2")]
        #[method(loadLeaderboardSetsWithCompletionHandler:)]
        pub unsafe fn loadLeaderboardSetsWithCompletionHandler(
            completion_handler: Option<
                &Block<dyn Fn(*mut NSArray<GKLeaderboardSet>, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "GameKit_GKLeaderboard", feature = "block2"))]
        #[method(loadLeaderboardsWithHandler:)]
        pub unsafe fn loadLeaderboardsWithHandler(
            &self,
            handler: &Block<dyn Fn(*mut NSArray<GKLeaderboard>, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKLeaderboardSet {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// Deprecated
    unsafe impl GKLeaderboardSet {
        #[cfg(all(feature = "GameKit_GKLeaderboard", feature = "block2"))]
        #[deprecated]
        #[method(loadLeaderboardsWithCompletionHandler:)]
        pub unsafe fn loadLeaderboardsWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSArray<GKLeaderboard>, *mut NSError)>>,
        );
    }
);

extern_methods!(
    /// UI
    unsafe impl GKLeaderboardSet {
        #[cfg(all(feature = "block2", feature = "objc2-app-kit"))]
        #[method(loadImageWithCompletionHandler:)]
        pub unsafe fn loadImageWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSImage, *mut NSError)>>,
        );
    }
);
