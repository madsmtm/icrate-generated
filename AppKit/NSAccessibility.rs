//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_category!(
    /// Category "NSAccessibility" on [`NSObject`].
    #[doc(alias = "NSAccessibility")]
    pub unsafe trait NSObjectNSAccessibility {
        #[cfg(all(
            feature = "AppKit_NSAccessibilityConstants",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityAttributeNames)]
        unsafe fn accessibilityAttributeNames(&self) -> Id<NSArray<NSAccessibilityAttributeName>>;

        #[cfg(all(
            feature = "AppKit_NSAccessibilityConstants",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityAttributeValue:)]
        unsafe fn accessibilityAttributeValue(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(
            feature = "AppKit_NSAccessibilityConstants",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method(accessibilityIsAttributeSettable:)]
        unsafe fn accessibilityIsAttributeSettable(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSAccessibilityConstants",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method(accessibilitySetValue:forAttribute:)]
        unsafe fn accessibilitySetValue_forAttribute(
            &self,
            value: Option<&AnyObject>,
            attribute: &NSAccessibilityAttributeName,
        );

        #[cfg(all(
            feature = "AppKit_NSAccessibilityConstants",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityParameterizedAttributeNames)]
        unsafe fn accessibilityParameterizedAttributeNames(
            &self,
        ) -> Id<NSArray<NSAccessibilityParameterizedAttributeName>>;

        #[cfg(all(
            feature = "AppKit_NSAccessibilityConstants",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityAttributeValue:forParameter:)]
        unsafe fn accessibilityAttributeValue_forParameter(
            &self,
            attribute: &NSAccessibilityParameterizedAttributeName,
            parameter: Option<&AnyObject>,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(
            feature = "AppKit_NSAccessibilityConstants",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityActionNames)]
        unsafe fn accessibilityActionNames(&self) -> Id<NSArray<NSAccessibilityActionName>>;

        #[cfg(all(
            feature = "AppKit_NSAccessibilityConstants",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityActionDescription:)]
        unsafe fn accessibilityActionDescription(
            &self,
            action: &NSAccessibilityActionName,
        ) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "AppKit_NSAccessibilityConstants",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method(accessibilityPerformAction:)]
        unsafe fn accessibilityPerformAction(&self, action: &NSAccessibilityActionName);

        #[deprecated = "Use isAccessibilityElement instead"]
        #[method(accessibilityIsIgnored)]
        unsafe fn accessibilityIsIgnored(&self) -> bool;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other accessibilityHitTest:)]
        unsafe fn accessibilityHitTest(&self, point: NSPoint) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other accessibilityFocusedUIElement)]
        unsafe fn accessibilityFocusedUIElement(&self) -> Option<Id<AnyObject>>;

        #[method(accessibilityIndexOfChild:)]
        unsafe fn accessibilityIndexOfChild(&self, child: &AnyObject) -> NSUInteger;

        #[cfg(all(
            feature = "AppKit_NSAccessibilityConstants",
            feature = "Foundation_NSString"
        ))]
        #[method(accessibilityArrayAttributeCount:)]
        unsafe fn accessibilityArrayAttributeCount(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> NSUInteger;

        #[cfg(all(
            feature = "AppKit_NSAccessibilityConstants",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other accessibilityArrayAttributeValues:index:maxCount:)]
        unsafe fn accessibilityArrayAttributeValues_index_maxCount(
            &self,
            attribute: &NSAccessibilityAttributeName,
            index: NSUInteger,
            max_count: NSUInteger,
        ) -> Id<NSArray>;

        #[method(accessibilityNotifiesWhenDestroyed)]
        unsafe fn accessibilityNotifiesWhenDestroyed(&self) -> bool;
    }

    unsafe impl NSObjectNSAccessibility for NSObject {}
);

extern_methods!(
    /// NSWorkspaceAccessibilityDisplay
    #[cfg(feature = "AppKit_NSWorkspace")]
    unsafe impl NSWorkspace {
        #[method(accessibilityDisplayShouldIncreaseContrast)]
        pub unsafe fn accessibilityDisplayShouldIncreaseContrast(&self) -> bool;

        #[method(accessibilityDisplayShouldDifferentiateWithoutColor)]
        pub unsafe fn accessibilityDisplayShouldDifferentiateWithoutColor(&self) -> bool;

        #[method(accessibilityDisplayShouldReduceTransparency)]
        pub unsafe fn accessibilityDisplayShouldReduceTransparency(&self) -> bool;

        #[method(accessibilityDisplayShouldReduceMotion)]
        pub unsafe fn accessibilityDisplayShouldReduceMotion(&self) -> bool;

        #[method(accessibilityDisplayShouldInvertColors)]
        pub unsafe fn accessibilityDisplayShouldInvertColors(&self) -> bool;
    }
);

extern_methods!(
    /// NSWorkspaceAccessibility
    #[cfg(feature = "AppKit_NSWorkspace")]
    unsafe impl NSWorkspace {
        #[method(isVoiceOverEnabled)]
        pub unsafe fn isVoiceOverEnabled(&self) -> bool;

        #[method(isSwitchControlEnabled)]
        pub unsafe fn isSwitchControlEnabled(&self) -> bool;
    }
);

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView",
        feature = "Foundation_NSGeometry"
    ))]
    pub fn NSAccessibilityFrameInView(parent_view: &NSView, frame: NSRect) -> NSRect;
}

extern "C" {
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView",
        feature = "Foundation_NSGeometry"
    ))]
    pub fn NSAccessibilityPointInView(parent_view: &NSView, point: NSPoint) -> NSPoint;
}

extern "C" {
    pub fn NSAccessibilitySetMayContainProtectedContent(flag: Bool) -> Bool;
}

extern "C" {
    #[cfg(all(
        feature = "AppKit_NSAccessibilityConstants",
        feature = "Foundation_NSString"
    ))]
    pub fn NSAccessibilityRoleDescription(
        role: &NSAccessibilityRole,
        subrole: Option<&NSAccessibilitySubrole>,
    ) -> *mut NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub fn NSAccessibilityRoleDescriptionForUIElement(element: &AnyObject) -> *mut NSString;
}

extern "C" {
    #[cfg(all(
        feature = "AppKit_NSAccessibilityConstants",
        feature = "Foundation_NSString"
    ))]
    pub fn NSAccessibilityActionDescription(action: &NSAccessibilityActionName) -> *mut NSString;
}

extern "C" {
    #[cfg(all(
        feature = "AppKit_NSAccessibilityConstants",
        feature = "Foundation_NSString"
    ))]
    #[deprecated = "Exceptions are no longer appropriate for indicating errors in accessibility API. Unexpected values should be handled through appropriate type checking."]
    pub fn NSAccessibilityRaiseBadArgumentException(
        element: Option<&AnyObject>,
        attribute: Option<&NSAccessibilityAttributeName>,
        value: Option<&AnyObject>,
    );
}

extern "C" {
    pub fn NSAccessibilityUnignoredAncestor(element: &AnyObject) -> *mut AnyObject;
}

extern "C" {
    pub fn NSAccessibilityUnignoredDescendant(element: &AnyObject) -> *mut AnyObject;
}

extern "C" {
    #[cfg(feature = "Foundation_NSArray")]
    pub fn NSAccessibilityUnignoredChildren(original_children: &NSArray) -> NonNull<NSArray>;
}

extern "C" {
    #[cfg(feature = "Foundation_NSArray")]
    pub fn NSAccessibilityUnignoredChildrenForOnlyChild(
        original_child: &AnyObject,
    ) -> NonNull<NSArray>;
}

extern "C" {
    #[cfg(all(
        feature = "AppKit_NSAccessibilityConstants",
        feature = "Foundation_NSString"
    ))]
    pub fn NSAccessibilityPostNotification(
        element: &AnyObject,
        notification: &NSAccessibilityNotificationName,
    );
}
