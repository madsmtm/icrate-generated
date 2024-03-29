//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSViewController",
        feature = "GameKit_GKGameCenterViewController"
    ))]
    #[deprecated]
    pub struct GKLeaderboardViewController;

    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSViewController",
        feature = "GameKit_GKGameCenterViewController"
    ))]
    unsafe impl ClassType for GKLeaderboardViewController {
        #[inherits(NSViewController, NSResponder, NSObject)]
        type Super = GKGameCenterViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "GameKit_GKDialogController",
    feature = "GameKit_GKGameCenterViewController"
))]
unsafe impl GKViewController for GKLeaderboardViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "Foundation_NSObject",
    feature = "GameKit_GKGameCenterViewController"
))]
unsafe impl NSCoding for GKLeaderboardViewController {}

#[cfg(all(
    feature = "AppKit_NSKeyValueBinding",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "GameKit_GKGameCenterViewController"
))]
unsafe impl NSEditor for GKLeaderboardViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "GameKit_GKGameCenterViewController"
))]
unsafe impl NSObjectProtocol for GKLeaderboardViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSStoryboardSegue",
    feature = "AppKit_NSViewController",
    feature = "GameKit_GKGameCenterViewController"
))]
unsafe impl NSSeguePerforming for GKLeaderboardViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSViewController",
    feature = "GameKit_GKGameCenterViewController"
))]
unsafe impl NSUserInterfaceItemIdentification for GKLeaderboardViewController {}

extern_methods!(
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSViewController",
        feature = "GameKit_GKGameCenterViewController"
    ))]
    unsafe impl GKLeaderboardViewController {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSViewController",
        feature = "GameKit_GKGameCenterViewController"
    ))]
    unsafe impl GKLeaderboardViewController {
        #[cfg(all(
            feature = "AppKit_NSNib",
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSViewController",
        feature = "GameKit_GKGameCenterViewController"
    ))]
    unsafe impl GKLeaderboardViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSViewController",
        feature = "GameKit_GKGameCenterViewController"
    ))]
    unsafe impl GKLeaderboardViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSViewController",
        feature = "GameKit_GKGameCenterViewController"
    ))]
    unsafe impl GKLeaderboardViewController {
        #[cfg(feature = "GameKit_GKLeaderboard")]
        #[deprecated]
        #[method(timeScope)]
        pub unsafe fn timeScope(&self) -> GKLeaderboardTimeScope;

        #[cfg(feature = "GameKit_GKLeaderboard")]
        #[deprecated]
        #[method(setTimeScope:)]
        pub unsafe fn setTimeScope(&self, time_scope: GKLeaderboardTimeScope);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
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
        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSViewController",
            feature = "GameKit_GKGameCenterViewController"
        ))]
        #[deprecated]
        #[method(leaderboardViewControllerDidFinish:)]
        unsafe fn leaderboardViewControllerDidFinish(
            &self,
            view_controller: Option<&GKLeaderboardViewController>,
        );
    }

    unsafe impl ProtocolType for dyn GKLeaderboardViewControllerDelegate {}
);
