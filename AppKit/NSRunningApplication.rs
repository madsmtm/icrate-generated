//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSApplicationActivationOptions(pub NSUInteger);
impl NSApplicationActivationOptions {
    pub const NSApplicationActivateAllWindows: Self = Self(1 << 0);
    #[deprecated = "ignoringOtherApps is deprecated in macOS 14 and will have no effect."]
    pub const NSApplicationActivateIgnoringOtherApps: Self = Self(1 << 1);
}

unsafe impl Encode for NSApplicationActivationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSApplicationActivationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSApplicationActivationPolicy(pub NSInteger);
impl NSApplicationActivationPolicy {
    #[doc(alias = "NSApplicationActivationPolicyRegular")]
    pub const Regular: Self = Self(0);
    #[doc(alias = "NSApplicationActivationPolicyAccessory")]
    pub const Accessory: Self = Self(1);
    #[doc(alias = "NSApplicationActivationPolicyProhibited")]
    pub const Prohibited: Self = Self(2);
}

unsafe impl Encode for NSApplicationActivationPolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSApplicationActivationPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRunningApplication;

    unsafe impl ClassType for NSRunningApplication {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSRunningApplication {}

extern_methods!(
    unsafe impl NSRunningApplication {
        #[method(isTerminated)]
        pub unsafe fn isTerminated(&self) -> bool;

        #[method(isFinishedLaunching)]
        pub unsafe fn isFinishedLaunching(&self) -> bool;

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(ownsMenuBar)]
        pub unsafe fn ownsMenuBar(&self) -> bool;

        #[method(activationPolicy)]
        pub unsafe fn activationPolicy(&self) -> NSApplicationActivationPolicy;

        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other bundleIdentifier)]
        pub unsafe fn bundleIdentifier(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other bundleURL)]
        pub unsafe fn bundleURL(&self) -> Option<Id<NSURL>>;

        #[method_id(@__retain_semantics Other executableURL)]
        pub unsafe fn executableURL(&self) -> Option<Id<NSURL>>;

        #[method_id(@__retain_semantics Other launchDate)]
        pub unsafe fn launchDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<NSImage>>;

        #[method(executableArchitecture)]
        pub unsafe fn executableArchitecture(&self) -> NSInteger;

        #[method(hide)]
        pub unsafe fn hide(&self) -> bool;

        #[method(unhide)]
        pub unsafe fn unhide(&self) -> bool;

        #[method(activateFromApplication:options:)]
        pub unsafe fn activateFromApplication_options(
            &self,
            application: &NSRunningApplication,
            options: NSApplicationActivationOptions,
        ) -> bool;

        #[method(activateWithOptions:)]
        pub unsafe fn activateWithOptions(&self, options: NSApplicationActivationOptions) -> bool;

        #[method(terminate)]
        pub unsafe fn terminate(&self) -> bool;

        #[method(forceTerminate)]
        pub unsafe fn forceTerminate(&self) -> bool;

        #[method_id(@__retain_semantics Other runningApplicationsWithBundleIdentifier:)]
        pub unsafe fn runningApplicationsWithBundleIdentifier(
            bundle_identifier: &NSString,
        ) -> Id<NSArray<NSRunningApplication>>;

        #[method_id(@__retain_semantics Other currentApplication)]
        pub unsafe fn currentApplication() -> Id<NSRunningApplication>;

        #[method(terminateAutomaticallyTerminableApplications)]
        pub unsafe fn terminateAutomaticallyTerminableApplications();
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSRunningApplication {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSWorkspaceRunningApplications
    #[cfg(feature = "AppKit_NSWorkspace")]
    unsafe impl NSWorkspace {
        #[method_id(@__retain_semantics Other runningApplications)]
        pub unsafe fn runningApplications(&self) -> Id<NSArray<NSRunningApplication>>;
    }
);
