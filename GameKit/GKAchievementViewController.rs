//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    #[deprecated = "Use GKGameCenterViewController instead"]
    pub struct GKAchievementViewController;

    #[cfg(feature = "GameKit_GKAchievementViewController")]
    unsafe impl ClassType for GKAchievementViewController {
        #[inherits(NSViewController, NSResponder, NSObject)]
        type Super = GKGameCenterViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl GKViewController for GKAchievementViewController {}

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl NSCoding for GKAchievementViewController {}

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl NSEditor for GKAchievementViewController {}

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl NSObjectProtocol for GKAchievementViewController {}

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl NSSeguePerforming for GKAchievementViewController {}

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl NSUserInterfaceItemIdentification for GKAchievementViewController {}

extern_methods!(
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    unsafe impl GKAchievementViewController {
        #[deprecated = "Use GKGameCenterViewController instead"]
        #[method_id(@__retain_semantics Other achievementDelegate)]
        pub unsafe fn achievementDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn GKAchievementViewControllerDelegate>>>;

        #[deprecated = "Use GKGameCenterViewController instead"]
        #[method(setAchievementDelegate:)]
        pub unsafe fn setAchievementDelegate(
            &self,
            achievement_delegate: Option<&(impl GKAchievementViewControllerDelegate + Message)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    unsafe impl GKAchievementViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    unsafe impl GKAchievementViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    unsafe impl GKAchievementViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    #[deprecated = "Use GKGameCenterViewController instead"]
    pub unsafe trait GKAchievementViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "GameKit_GKAchievementViewController")]
        #[deprecated = "Use GKGameCenterViewController instead"]
        #[method(achievementViewControllerDidFinish:)]
        unsafe fn achievementViewControllerDidFinish(
            &self,
            view_controller: Option<&GKAchievementViewController>,
        );
    }

    unsafe impl ProtocolType for dyn GKAchievementViewControllerDelegate {}
);
