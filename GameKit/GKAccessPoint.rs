//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKAccessPointLocation(pub NSInteger);
impl GKAccessPointLocation {
    #[doc(alias = "GKAccessPointLocationTopLeading")]
    pub const TopLeading: Self = Self(0);
    #[doc(alias = "GKAccessPointLocationTopTrailing")]
    pub const TopTrailing: Self = Self(1);
    #[doc(alias = "GKAccessPointLocationBottomLeading")]
    pub const BottomLeading: Self = Self(2);
    #[doc(alias = "GKAccessPointLocationBottomTrailing")]
    pub const BottomTrailing: Self = Self(3);
}

unsafe impl Encode for GKAccessPointLocation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GKAccessPointLocation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKAccessPoint;

    unsafe impl ClassType for GKAccessPoint {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for GKAccessPoint {}

extern_methods!(
    unsafe impl GKAccessPoint {
        #[method_id(@__retain_semantics Other shared)]
        pub unsafe fn shared() -> Retained<GKAccessPoint>;

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[method(isFocused)]
        pub unsafe fn isFocused(&self) -> bool;

        #[method(setFocused:)]
        pub unsafe fn setFocused(&self, focused: bool);

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(isPresentingGameCenter)]
        pub unsafe fn isPresentingGameCenter(&self) -> bool;

        #[method(showHighlights)]
        pub unsafe fn showHighlights(&self) -> bool;

        #[method(setShowHighlights:)]
        pub unsafe fn setShowHighlights(&self, show_highlights: bool);

        #[method(location)]
        pub unsafe fn location(&self) -> GKAccessPointLocation;

        #[method(setLocation:)]
        pub unsafe fn setLocation(&self, location: GKAccessPointLocation);

        #[method(frameInScreenCoordinates)]
        pub unsafe fn frameInScreenCoordinates(&self) -> NSRect;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other parentWindow)]
        pub unsafe fn parentWindow(&self, mtm: MainThreadMarker) -> Option<Retained<NSWindow>>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(setParentWindow:)]
        pub unsafe fn setParentWindow(&self, parent_window: Option<&NSWindow>);

        #[cfg(feature = "block2")]
        #[method(triggerAccessPointWithHandler:)]
        pub unsafe fn triggerAccessPointWithHandler(&self, handler: &block2::Block<dyn Fn()>);

        #[cfg(all(feature = "GKGameCenterViewController", feature = "block2"))]
        #[method(triggerAccessPointWithState:handler:)]
        pub unsafe fn triggerAccessPointWithState_handler(
            &self,
            state: GKGameCenterViewControllerState,
            handler: &block2::Block<dyn Fn()>,
        );

        #[cfg(feature = "block2")]
        #[method(triggerAccessPointWithAchievementID:handler:)]
        pub unsafe fn triggerAccessPointWithAchievementID_handler(
            &self,
            achievement_id: &NSString,
            handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(feature = "block2")]
        #[method(triggerAccessPointWithLeaderboardSetID:handler:)]
        pub unsafe fn triggerAccessPointWithLeaderboardSetID_handler(
            &self,
            leaderboard_set_id: &NSString,
            handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(all(feature = "GKLeaderboard", feature = "block2"))]
        #[method(triggerAccessPointWithLeaderboardID:playerScope:timeScope:handler:)]
        pub unsafe fn triggerAccessPointWithLeaderboardID_playerScope_timeScope_handler(
            &self,
            leaderboard_id: &NSString,
            player_scope: GKLeaderboardPlayerScope,
            time_scope: GKLeaderboardTimeScope,
            handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer", feature = "block2"))]
        #[method(triggerAccessPointWithPlayer:handler:)]
        pub unsafe fn triggerAccessPointWithPlayer_handler(
            &self,
            player: &GKPlayer,
            handler: Option<&block2::Block<dyn Fn()>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKAccessPoint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
