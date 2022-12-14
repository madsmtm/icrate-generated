//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSViewControllerTransitionOptions {
        NSViewControllerTransitionNone = 0x0,
        NSViewControllerTransitionCrossfade = 0x1,
        NSViewControllerTransitionSlideUp = 0x10,
        NSViewControllerTransitionSlideDown = 0x20,
        NSViewControllerTransitionSlideLeft = 0x40,
        NSViewControllerTransitionSlideRight = 0x80,
        NSViewControllerTransitionSlideForward = 0x140,
        NSViewControllerTransitionSlideBackward = 0x180,
        NSViewControllerTransitionAllowUserInteraction = 0x1000,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSViewController;

    unsafe impl ClassType for NSViewController {
        #[inherits(NSObject)]
        type Super = NSResponder;
    }
);

extern_methods!(
    unsafe impl NSViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nibNameOrNil: Option<&NSNibName>,
            nibBundleOrNil: Option<&NSBundle>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other nibName)]
        pub unsafe fn nibName(&self) -> Option<Id<NSNibName, Shared>>;

        #[method_id(@__retain_semantics Other nibBundle)]
        pub unsafe fn nibBundle(&self) -> Option<Id<NSBundle, Shared>>;

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<Object, Shared>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, representedObject: Option<&Object>);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Id<NSView, Shared>;

        #[method(setView:)]
        pub unsafe fn setView(&self, view: &NSView);

        #[method(loadView)]
        pub unsafe fn loadView(&self);

        #[method(commitEditingWithDelegate:didCommitSelector:contextInfo:)]
        pub unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            didCommitSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(commitEditing)]
        pub unsafe fn commitEditing(&self) -> bool;

        #[method(discardEditing)]
        pub unsafe fn discardEditing(&self);

        #[method(viewDidLoad)]
        pub unsafe fn viewDidLoad(&self);

        #[method(isViewLoaded)]
        pub unsafe fn isViewLoaded(&self) -> bool;

        #[method(viewWillAppear)]
        pub unsafe fn viewWillAppear(&self);

        #[method(viewDidAppear)]
        pub unsafe fn viewDidAppear(&self);

        #[method(viewWillDisappear)]
        pub unsafe fn viewWillDisappear(&self);

        #[method(viewDidDisappear)]
        pub unsafe fn viewDidDisappear(&self);

        #[method(preferredContentSize)]
        pub unsafe fn preferredContentSize(&self) -> NSSize;

        #[method(setPreferredContentSize:)]
        pub unsafe fn setPreferredContentSize(&self, preferredContentSize: NSSize);

        #[method(updateViewConstraints)]
        pub unsafe fn updateViewConstraints(&self);

        #[method(viewWillLayout)]
        pub unsafe fn viewWillLayout(&self);

        #[method(viewDidLayout)]
        pub unsafe fn viewDidLayout(&self);
    }
);

extern_methods!(
    /// NSViewControllerPresentation
    unsafe impl NSViewController {
        #[method(presentViewController:animator:)]
        pub unsafe fn presentViewController_animator(
            &self,
            viewController: &NSViewController,
            animator: &NSViewControllerPresentationAnimator,
        );

        #[method(dismissViewController:)]
        pub unsafe fn dismissViewController(&self, viewController: &NSViewController);

        #[method(dismissController:)]
        pub unsafe fn dismissController(&self, sender: Option<&Object>);

        #[method_id(@__retain_semantics Other presentedViewControllers)]
        pub unsafe fn presentedViewControllers(
            &self,
        ) -> Option<Id<NSArray<NSViewController>, Shared>>;

        #[method_id(@__retain_semantics Other presentingViewController)]
        pub unsafe fn presentingViewController(&self) -> Option<Id<NSViewController, Shared>>;
    }
);

extern_methods!(
    /// NSViewControllerPresentationAndTransitionStyles
    unsafe impl NSViewController {
        #[method(presentViewControllerAsSheet:)]
        pub unsafe fn presentViewControllerAsSheet(&self, viewController: &NSViewController);

        #[method(presentViewControllerAsModalWindow:)]
        pub unsafe fn presentViewControllerAsModalWindow(&self, viewController: &NSViewController);

        #[method(presentViewController:asPopoverRelativeToRect:ofView:preferredEdge:behavior:)]
        pub unsafe fn presentViewController_asPopoverRelativeToRect_ofView_preferredEdge_behavior(
            &self,
            viewController: &NSViewController,
            positioningRect: NSRect,
            positioningView: &NSView,
            preferredEdge: NSRectEdge,
            behavior: NSPopoverBehavior,
        );

        #[method(transitionFromViewController:toViewController:options:completionHandler:)]
        pub unsafe fn transitionFromViewController_toViewController_options_completionHandler(
            &self,
            fromViewController: &NSViewController,
            toViewController: &NSViewController,
            options: NSViewControllerTransitionOptions,
            completion: Option<&Block<(), ()>>,
        );
    }
);

extern_methods!(
    /// NSViewControllerContainer
    unsafe impl NSViewController {
        #[method_id(@__retain_semantics Other parentViewController)]
        pub unsafe fn parentViewController(&self) -> Option<Id<NSViewController, Shared>>;

        #[method_id(@__retain_semantics Other childViewControllers)]
        pub unsafe fn childViewControllers(&self) -> Id<NSArray<NSViewController>, Shared>;

        #[method(setChildViewControllers:)]
        pub unsafe fn setChildViewControllers(
            &self,
            childViewControllers: &NSArray<NSViewController>,
        );

        #[method(addChildViewController:)]
        pub unsafe fn addChildViewController(&self, childViewController: &NSViewController);

        #[method(removeFromParentViewController)]
        pub unsafe fn removeFromParentViewController(&self);

        #[method(insertChildViewController:atIndex:)]
        pub unsafe fn insertChildViewController_atIndex(
            &self,
            childViewController: &NSViewController,
            index: NSInteger,
        );

        #[method(removeChildViewControllerAtIndex:)]
        pub unsafe fn removeChildViewControllerAtIndex(&self, index: NSInteger);

        #[method(preferredContentSizeDidChangeForViewController:)]
        pub unsafe fn preferredContentSizeDidChangeForViewController(
            &self,
            viewController: &NSViewController,
        );

        #[method(viewWillTransitionToSize:)]
        pub unsafe fn viewWillTransitionToSize(&self, newSize: NSSize);
    }
);

extern_protocol!(
    pub struct NSViewControllerPresentationAnimator;

    unsafe impl ProtocolType for NSViewControllerPresentationAnimator {
        #[method(animatePresentationOfViewController:fromViewController:)]
        pub unsafe fn animatePresentationOfViewController_fromViewController(
            &self,
            viewController: &NSViewController,
            fromViewController: &NSViewController,
        );

        #[method(animateDismissalOfViewController:fromViewController:)]
        pub unsafe fn animateDismissalOfViewController_fromViewController(
            &self,
            viewController: &NSViewController,
            fromViewController: &NSViewController,
        );
    }
);

extern_methods!(
    /// NSViewControllerStoryboardingMethods
    unsafe impl NSViewController {
        #[method_id(@__retain_semantics Other storyboard)]
        pub unsafe fn storyboard(&self) -> Option<Id<NSStoryboard, Shared>>;
    }
);

extern_methods!(
    /// NSExtensionAdditions
    unsafe impl NSViewController {
        #[method_id(@__retain_semantics Other extensionContext)]
        pub unsafe fn extensionContext(&self) -> Option<Id<NSExtensionContext, Shared>>;

        #[method_id(@__retain_semantics Other sourceItemView)]
        pub unsafe fn sourceItemView(&self) -> Option<Id<NSView, Shared>>;

        #[method(setSourceItemView:)]
        pub unsafe fn setSourceItemView(&self, sourceItemView: Option<&NSView>);

        #[method(preferredScreenOrigin)]
        pub unsafe fn preferredScreenOrigin(&self) -> NSPoint;

        #[method(setPreferredScreenOrigin:)]
        pub unsafe fn setPreferredScreenOrigin(&self, preferredScreenOrigin: NSPoint);

        #[method(preferredMinimumSize)]
        pub unsafe fn preferredMinimumSize(&self) -> NSSize;

        #[method(preferredMaximumSize)]
        pub unsafe fn preferredMaximumSize(&self) -> NSSize;
    }
);
