//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    #[deprecated]
    pub struct GKLeaderboardViewController;

    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl ClassType for GKLeaderboardViewController {
        #[inherits(NSViewController, NSResponder, NSObject)]
        type Super = GKGameCenterViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "GKDialogController",
    feature = "GKGameCenterViewController",
    feature = "objc2-app-kit"
))]
#[cfg(target_os = "macos")]
unsafe impl GKViewController for GKLeaderboardViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for GKLeaderboardViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSEditor for GKLeaderboardViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for GKLeaderboardViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for GKLeaderboardViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for GKLeaderboardViewController {}

extern_methods!(
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKLeaderboardViewController {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKLeaderboardViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKLeaderboardViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKLeaderboardViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKLeaderboardViewController {
        #[cfg(feature = "GKLeaderboard")]
        #[deprecated]
        #[method(timeScope)]
        pub unsafe fn timeScope(&self) -> GKLeaderboardTimeScope;

        #[cfg(feature = "GKLeaderboard")]
        #[deprecated]
        #[method(setTimeScope:)]
        pub unsafe fn setTimeScope(&self, time_scope: GKLeaderboardTimeScope);

        #[deprecated]
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setCategory:)]
        pub unsafe fn setCategory(&self, category: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other leaderboardDelegate)]
        pub unsafe fn leaderboardDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn GKLeaderboardViewControllerDelegate>>>;

        #[deprecated]
        #[method(setLeaderboardDelegate:)]
        pub unsafe fn setLeaderboardDelegate(
            &self,
            leaderboard_delegate: Option<&ProtocolObject<dyn GKLeaderboardViewControllerDelegate>>,
        );
    }
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait GKLeaderboardViewControllerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[method(leaderboardViewControllerDidFinish:)]
        unsafe fn leaderboardViewControllerDidFinish(
            &self,
            view_controller: Option<&GKLeaderboardViewController>,
        );
    }

    unsafe impl ProtocolType for dyn GKLeaderboardViewControllerDelegate {}
);
