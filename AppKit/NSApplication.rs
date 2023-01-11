//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSAppKitVersion = c_double;
);

extern_static!(NSAppKitVersionNumber: NSAppKitVersion);

extern_static!(NSAppKitVersionNumber10_0: NSAppKitVersion = 577);

extern_static!(NSAppKitVersionNumber10_1: NSAppKitVersion = 620);

extern_static!(NSAppKitVersionNumber10_2: NSAppKitVersion = 663);

extern_static!(NSAppKitVersionNumber10_2_3: NSAppKitVersion = 663.6);

extern_static!(NSAppKitVersionNumber10_3: NSAppKitVersion = 743);

extern_static!(NSAppKitVersionNumber10_3_2: NSAppKitVersion = 743.14);

extern_static!(NSAppKitVersionNumber10_3_3: NSAppKitVersion = 743.2);

extern_static!(NSAppKitVersionNumber10_3_5: NSAppKitVersion = 743.24);

extern_static!(NSAppKitVersionNumber10_3_7: NSAppKitVersion = 743.33);

extern_static!(NSAppKitVersionNumber10_3_9: NSAppKitVersion = 743.36);

extern_static!(NSAppKitVersionNumber10_4: NSAppKitVersion = 824);

extern_static!(NSAppKitVersionNumber10_4_1: NSAppKitVersion = 824.1);

extern_static!(NSAppKitVersionNumber10_4_3: NSAppKitVersion = 824.23);

extern_static!(NSAppKitVersionNumber10_4_4: NSAppKitVersion = 824.33);

extern_static!(NSAppKitVersionNumber10_4_7: NSAppKitVersion = 824.41);

extern_static!(NSAppKitVersionNumber10_5: NSAppKitVersion = 949);

extern_static!(NSAppKitVersionNumber10_5_2: NSAppKitVersion = 949.27);

extern_static!(NSAppKitVersionNumber10_5_3: NSAppKitVersion = 949.33);

extern_static!(NSAppKitVersionNumber10_6: NSAppKitVersion = 1038);

extern_static!(NSAppKitVersionNumber10_7: NSAppKitVersion = 1138);

extern_static!(NSAppKitVersionNumber10_7_2: NSAppKitVersion = 1138.23);

extern_static!(NSAppKitVersionNumber10_7_3: NSAppKitVersion = 1138.32);

extern_static!(NSAppKitVersionNumber10_7_4: NSAppKitVersion = 1138.47);

extern_static!(NSAppKitVersionNumber10_8: NSAppKitVersion = 1187);

extern_static!(NSAppKitVersionNumber10_9: NSAppKitVersion = 1265);

extern_static!(NSAppKitVersionNumber10_10: NSAppKitVersion = 1343);

extern_static!(NSAppKitVersionNumber10_10_2: NSAppKitVersion = 1344);

extern_static!(NSAppKitVersionNumber10_10_3: NSAppKitVersion = 1347);

extern_static!(NSAppKitVersionNumber10_10_4: NSAppKitVersion = 1348);

extern_static!(NSAppKitVersionNumber10_10_5: NSAppKitVersion = 1348);

extern_static!(NSAppKitVersionNumber10_10_Max: NSAppKitVersion = 1349);

extern_static!(NSAppKitVersionNumber10_11: NSAppKitVersion = 1404);

extern_static!(NSAppKitVersionNumber10_11_1: NSAppKitVersion = 1404.13);

extern_static!(NSAppKitVersionNumber10_11_2: NSAppKitVersion = 1404.34);

extern_static!(NSAppKitVersionNumber10_11_3: NSAppKitVersion = 1404.34);

extern_static!(NSAppKitVersionNumber10_12: NSAppKitVersion = 1504);

extern_static!(NSAppKitVersionNumber10_12_1: NSAppKitVersion = 1504.60);

extern_static!(NSAppKitVersionNumber10_12_2: NSAppKitVersion = 1504.76);

extern_static!(NSAppKitVersionNumber10_13: NSAppKitVersion = 1561);

extern_static!(NSAppKitVersionNumber10_13_1: NSAppKitVersion = 1561.1);

extern_static!(NSAppKitVersionNumber10_13_2: NSAppKitVersion = 1561.2);

extern_static!(NSAppKitVersionNumber10_13_4: NSAppKitVersion = 1561.4);

extern_static!(NSAppKitVersionNumber10_14: NSAppKitVersion = 1671);

extern_static!(NSAppKitVersionNumber10_14_1: NSAppKitVersion = 1671.1);

extern_static!(NSAppKitVersionNumber10_14_2: NSAppKitVersion = 1671.2);

extern_static!(NSAppKitVersionNumber10_14_3: NSAppKitVersion = 1671.3);

extern_static!(NSAppKitVersionNumber10_14_4: NSAppKitVersion = 1671.4);

extern_static!(NSAppKitVersionNumber10_14_5: NSAppKitVersion = 1671.5);

extern_static!(NSAppKitVersionNumber10_15: NSAppKitVersion = 1894);

extern_static!(NSAppKitVersionNumber10_15_1: NSAppKitVersion = 1894.1);

extern_static!(NSAppKitVersionNumber10_15_2: NSAppKitVersion = 1894.2);

extern_static!(NSAppKitVersionNumber10_15_3: NSAppKitVersion = 1894.3);

extern_static!(NSAppKitVersionNumber10_15_4: NSAppKitVersion = 1894.4);

extern_static!(NSAppKitVersionNumber10_15_5: NSAppKitVersion = 1894.5);

extern_static!(NSAppKitVersionNumber10_15_6: NSAppKitVersion = 1894.6);

extern_static!(NSAppKitVersionNumber11_0: NSAppKitVersion = 2022);

extern_static!(NSAppKitVersionNumber11_1: NSAppKitVersion = 2022.2);

extern_static!(NSAppKitVersionNumber11_2: NSAppKitVersion = 2022.3);

extern_static!(NSAppKitVersionNumber11_3: NSAppKitVersion = 2022.4);

extern_static!(NSAppKitVersionNumber11_4: NSAppKitVersion = 2022.5);

extern_static!(NSModalPanelRunLoopMode: &'static NSRunLoopMode);

extern_static!(NSEventTrackingRunLoopMode: &'static NSRunLoopMode);

typed_extensible_enum!(
    pub type NSModalResponse = NSInteger;
);

extern_static!(NSModalResponseStop: NSModalResponse = -1000);

extern_static!(NSModalResponseAbort: NSModalResponse = -1001);

extern_static!(NSModalResponseContinue: NSModalResponse = -1002);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSUpdateWindowsRunLoopOrdering = 500000,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSApplicationPresentationOptions {
        NSApplicationPresentationDefault = 0,
        NSApplicationPresentationAutoHideDock = 1 << 0,
        NSApplicationPresentationHideDock = 1 << 1,
        NSApplicationPresentationAutoHideMenuBar = 1 << 2,
        NSApplicationPresentationHideMenuBar = 1 << 3,
        NSApplicationPresentationDisableAppleMenu = 1 << 4,
        NSApplicationPresentationDisableProcessSwitching = 1 << 5,
        NSApplicationPresentationDisableForceQuit = 1 << 6,
        NSApplicationPresentationDisableSessionTermination = 1 << 7,
        NSApplicationPresentationDisableHideApplication = 1 << 8,
        NSApplicationPresentationDisableMenuBarTransparency = 1 << 9,
        NSApplicationPresentationFullScreen = 1 << 10,
        NSApplicationPresentationAutoHideToolbar = 1 << 11,
        NSApplicationPresentationDisableCursorLocationAssistance = 1 << 12,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSApplicationOcclusionState {
        NSApplicationOcclusionStateVisible = 1 << 1,
    }
);

ns_options!(
    #[underlying(NSInteger)]
    pub enum NSWindowListOptions {
        NSWindowListOrderedFrontToBack = 1 << 0,
    }
);

pub type NSModalSession = *mut c_void;

extern_static!(NSApp: Option<&'static NSApplication>);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSRequestUserAttentionType {
        NSCriticalRequest = 0,
        NSInformationalRequest = 10,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSApplicationDelegateReply {
        NSApplicationDelegateReplySuccess = 0,
        NSApplicationDelegateReplyCancel = 1,
        NSApplicationDelegateReplyFailure = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSApplication")]
    pub struct NSApplication;

    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl ClassType for NSApplication {
        #[inherits(NSObject)]
        type Super = NSResponder;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method_id(@__retain_semantics Other sharedApplication)]
        pub unsafe fn sharedApplication() -> Id<NSApplication, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSApplicationDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSApplicationDelegate>);

        #[method(hide:)]
        pub unsafe fn hide(&self, sender: Option<&Object>);

        #[method(unhide:)]
        pub unsafe fn unhide(&self, sender: Option<&Object>);

        #[method(unhideWithoutActivation)]
        pub unsafe fn unhideWithoutActivation(&self);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other windowWithWindowNumber:)]
        pub unsafe fn windowWithWindowNumber(
            &self,
            windowNum: NSInteger,
        ) -> Option<Id<NSWindow, Shared>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other mainWindow)]
        pub unsafe fn mainWindow(&self) -> Option<Id<NSWindow, Shared>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other keyWindow)]
        pub unsafe fn keyWindow(&self) -> Option<Id<NSWindow, Shared>>;

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(isRunning)]
        pub unsafe fn isRunning(&self) -> bool;

        #[method(deactivate)]
        pub unsafe fn deactivate(&self);

        #[method(activateIgnoringOtherApps:)]
        pub unsafe fn activateIgnoringOtherApps(&self, flag: bool);

        #[method(hideOtherApplications:)]
        pub unsafe fn hideOtherApplications(&self, sender: Option<&Object>);

        #[method(unhideAllApplications:)]
        pub unsafe fn unhideAllApplications(&self, sender: Option<&Object>);

        #[method(finishLaunching)]
        pub unsafe fn finishLaunching(&self);

        #[method(run)]
        pub unsafe fn run(&self);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(runModalForWindow:)]
        pub unsafe fn runModalForWindow(&self, window: &NSWindow) -> NSModalResponse;

        #[method(stop:)]
        pub unsafe fn stop(&self, sender: Option<&Object>);

        #[method(stopModal)]
        pub unsafe fn stopModal(&self);

        #[method(stopModalWithCode:)]
        pub unsafe fn stopModalWithCode(&self, returnCode: NSModalResponse);

        #[method(abortModal)]
        pub unsafe fn abortModal(&self);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other modalWindow)]
        pub unsafe fn modalWindow(&self) -> Option<Id<NSWindow, Shared>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(beginModalSessionForWindow:)]
        pub unsafe fn beginModalSessionForWindow(&self, window: &NSWindow) -> NSModalSession;

        #[method(runModalSession:)]
        pub unsafe fn runModalSession(&self, session: NSModalSession) -> NSModalResponse;

        #[method(endModalSession:)]
        pub unsafe fn endModalSession(&self, session: NSModalSession);

        #[method(terminate:)]
        pub unsafe fn terminate(&self, sender: Option<&Object>);

        #[method(requestUserAttention:)]
        pub unsafe fn requestUserAttention(
            &self,
            requestType: NSRequestUserAttentionType,
        ) -> NSInteger;

        #[method(cancelUserAttentionRequest:)]
        pub unsafe fn cancelUserAttentionRequest(&self, request: NSInteger);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(enumerateWindowsWithOptions:usingBlock:)]
        pub unsafe fn enumerateWindowsWithOptions_usingBlock(
            &self,
            options: NSWindowListOptions,
            block: &Block<(NonNull<NSWindow>, NonNull<Bool>), ()>,
        );

        #[method(preventWindowOrdering)]
        pub unsafe fn preventWindowOrdering(&self);

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other windows)]
        pub unsafe fn windows(&self) -> Id<NSArray<NSWindow>, Shared>;

        #[method(setWindowsNeedUpdate:)]
        pub unsafe fn setWindowsNeedUpdate(&self, needUpdate: bool);

        #[method(updateWindows)]
        pub unsafe fn updateWindows(&self);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other mainMenu)]
        pub unsafe fn mainMenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMainMenu:)]
        pub unsafe fn setMainMenu(&self, mainMenu: Option<&NSMenu>);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other helpMenu)]
        pub unsafe fn helpMenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setHelpMenu:)]
        pub unsafe fn setHelpMenu(&self, helpMenu: Option<&NSMenu>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other applicationIconImage)]
        pub unsafe fn applicationIconImage(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setApplicationIconImage:)]
        pub unsafe fn setApplicationIconImage(&self, applicationIconImage: Option<&NSImage>);

        #[method(activationPolicy)]
        pub unsafe fn activationPolicy(&self) -> NSApplicationActivationPolicy;

        #[method(setActivationPolicy:)]
        pub unsafe fn setActivationPolicy(
            &self,
            activationPolicy: NSApplicationActivationPolicy,
        ) -> bool;

        #[cfg(feature = "AppKit_NSDockTile")]
        #[method_id(@__retain_semantics Other dockTile)]
        pub unsafe fn dockTile(&self) -> Id<NSDockTile, Shared>;

        #[cfg(feature = "Foundation_NSException")]
        #[method(reportException:)]
        pub unsafe fn reportException(&self, exception: &NSException);

        #[method(detachDrawingThread:toTarget:withObject:)]
        pub unsafe fn detachDrawingThread_toTarget_withObject(
            selector: Sel,
            target: &Object,
            argument: Option<&Object>,
        );

        #[method(replyToApplicationShouldTerminate:)]
        pub unsafe fn replyToApplicationShouldTerminate(&self, shouldTerminate: bool);

        #[method(replyToOpenOrPrint:)]
        pub unsafe fn replyToOpenOrPrint(&self, reply: NSApplicationDelegateReply);

        #[method(orderFrontCharacterPalette:)]
        pub unsafe fn orderFrontCharacterPalette(&self, sender: Option<&Object>);

        #[method(presentationOptions)]
        pub unsafe fn presentationOptions(&self) -> NSApplicationPresentationOptions;

        #[method(setPresentationOptions:)]
        pub unsafe fn setPresentationOptions(
            &self,
            presentationOptions: NSApplicationPresentationOptions,
        );

        #[method(currentSystemPresentationOptions)]
        pub unsafe fn currentSystemPresentationOptions(&self) -> NSApplicationPresentationOptions;

        #[method(occlusionState)]
        pub unsafe fn occlusionState(&self) -> NSApplicationOcclusionState;

        #[method(isProtectedDataAvailable)]
        pub unsafe fn isProtectedDataAvailable(&self) -> bool;
    }
);

extern_methods!(
    /// NSAppearanceCustomization
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[cfg(feature = "AppKit_NSAppearance")]
        #[method_id(@__retain_semantics Other appearance)]
        pub unsafe fn appearance(&self) -> Option<Id<NSAppearance, Shared>>;

        #[cfg(feature = "AppKit_NSAppearance")]
        #[method(setAppearance:)]
        pub unsafe fn setAppearance(&self, appearance: Option<&NSAppearance>);

        #[cfg(feature = "AppKit_NSAppearance")]
        #[method_id(@__retain_semantics Other effectiveAppearance)]
        pub unsafe fn effectiveAppearance(&self) -> Id<NSAppearance, Shared>;
    }
);

extern_methods!(
    /// NSEvent
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[cfg(feature = "AppKit_NSEvent")]
        #[method(sendEvent:)]
        pub unsafe fn sendEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(postEvent:atStart:)]
        pub unsafe fn postEvent_atStart(&self, event: &NSEvent, flag: bool);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method_id(@__retain_semantics Other currentEvent)]
        pub unsafe fn currentEvent(&self) -> Option<Id<NSEvent, Shared>>;

        #[cfg(all(feature = "AppKit_NSEvent", feature = "Foundation_NSDate"))]
        #[method_id(@__retain_semantics Other nextEventMatchingMask:untilDate:inMode:dequeue:)]
        pub unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue(
            &self,
            mask: NSEventMask,
            expiration: Option<&NSDate>,
            mode: &NSRunLoopMode,
            deqFlag: bool,
        ) -> Option<Id<NSEvent, Shared>>;

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(discardEventsMatchingMask:beforeEvent:)]
        pub unsafe fn discardEventsMatchingMask_beforeEvent(
            &self,
            mask: NSEventMask,
            lastEvent: Option<&NSEvent>,
        );
    }
);

extern_methods!(
    /// NSResponder
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(sendAction:to:from:)]
        pub unsafe fn sendAction_to_from(
            &self,
            action: Sel,
            target: Option<&Object>,
            sender: Option<&Object>,
        ) -> bool;

        #[method_id(@__retain_semantics Other targetForAction:)]
        pub unsafe fn targetForAction(&self, action: Sel) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other targetForAction:to:from:)]
        pub unsafe fn targetForAction_to_from(
            &self,
            action: Sel,
            target: Option<&Object>,
            sender: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;

        #[method(tryToPerform:with:)]
        pub unsafe fn tryToPerform_with(&self, action: Sel, object: Option<&Object>) -> bool;

        #[method_id(@__retain_semantics Other validRequestorForSendType:returnType:)]
        pub unsafe fn validRequestorForSendType_returnType(
            &self,
            sendType: Option<&NSPasteboardType>,
            returnType: Option<&NSPasteboardType>,
        ) -> Option<Id<Object, Shared>>;
    }
);

extern_methods!(
    /// NSWindowsMenu
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other windowsMenu)]
        pub unsafe fn windowsMenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setWindowsMenu:)]
        pub unsafe fn setWindowsMenu(&self, windowsMenu: Option<&NSMenu>);

        #[method(arrangeInFront:)]
        pub unsafe fn arrangeInFront(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(removeWindowsItem:)]
        pub unsafe fn removeWindowsItem(&self, win: &NSWindow);

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSString"))]
        #[method(addWindowsItem:title:filename:)]
        pub unsafe fn addWindowsItem_title_filename(
            &self,
            win: &NSWindow,
            string: &NSString,
            isFilename: bool,
        );

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSString"))]
        #[method(changeWindowsItem:title:filename:)]
        pub unsafe fn changeWindowsItem_title_filename(
            &self,
            win: &NSWindow,
            string: &NSString,
            isFilename: bool,
        );

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(updateWindowsItem:)]
        pub unsafe fn updateWindowsItem(&self, win: &NSWindow);

        #[method(miniaturizeAll:)]
        pub unsafe fn miniaturizeAll(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// NSFullKeyboardAccess
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(isFullKeyboardAccessEnabled)]
        pub unsafe fn isFullKeyboardAccessEnabled(&self) -> bool;
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSApplicationTerminateReply {
        NSTerminateCancel = 0,
        NSTerminateNow = 1,
        NSTerminateLater = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSApplicationPrintReply {
        NSPrintingCancelled = 0,
        NSPrintingSuccess = 1,
        NSPrintingFailure = 3,
        NSPrintingReplyLater = 2,
    }
);

extern_protocol!(
    pub struct NSApplicationDelegate;

    unsafe impl ProtocolType for NSApplicationDelegate {
        #[optional]
        #[method(applicationShouldTerminate:)]
        pub unsafe fn applicationShouldTerminate(
            &self,
            sender: &NSApplication,
        ) -> NSApplicationTerminateReply;

        #[optional]
        #[method(application:openURLs:)]
        pub unsafe fn application_openURLs(
            &self,
            application: &NSApplication,
            urls: &NSArray<NSURL>,
        );

        #[optional]
        #[method(application:openFile:)]
        pub unsafe fn application_openFile(
            &self,
            sender: &NSApplication,
            filename: &NSString,
        ) -> bool;

        #[optional]
        #[method(application:openFiles:)]
        pub unsafe fn application_openFiles(
            &self,
            sender: &NSApplication,
            filenames: &NSArray<NSString>,
        );

        #[optional]
        #[method(application:openTempFile:)]
        pub unsafe fn application_openTempFile(
            &self,
            sender: &NSApplication,
            filename: &NSString,
        ) -> bool;

        #[optional]
        #[method(applicationShouldOpenUntitledFile:)]
        pub unsafe fn applicationShouldOpenUntitledFile(&self, sender: &NSApplication) -> bool;

        #[optional]
        #[method(applicationOpenUntitledFile:)]
        pub unsafe fn applicationOpenUntitledFile(&self, sender: &NSApplication) -> bool;

        #[optional]
        #[method(application:openFileWithoutUI:)]
        pub unsafe fn application_openFileWithoutUI(
            &self,
            sender: &Object,
            filename: &NSString,
        ) -> bool;

        #[optional]
        #[method(application:printFile:)]
        pub unsafe fn application_printFile(
            &self,
            sender: &NSApplication,
            filename: &NSString,
        ) -> bool;

        #[optional]
        #[method(application:printFiles:withSettings:showPrintPanels:)]
        pub unsafe fn application_printFiles_withSettings_showPrintPanels(
            &self,
            application: &NSApplication,
            fileNames: &NSArray<NSString>,
            printSettings: &NSDictionary<NSPrintInfoAttributeKey, Object>,
            showPrintPanels: bool,
        ) -> NSApplicationPrintReply;

        #[optional]
        #[method(applicationShouldTerminateAfterLastWindowClosed:)]
        pub unsafe fn applicationShouldTerminateAfterLastWindowClosed(
            &self,
            sender: &NSApplication,
        ) -> bool;

        #[optional]
        #[method(applicationShouldHandleReopen:hasVisibleWindows:)]
        pub unsafe fn applicationShouldHandleReopen_hasVisibleWindows(
            &self,
            sender: &NSApplication,
            flag: bool,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other applicationDockMenu:)]
        pub unsafe fn applicationDockMenu(
            &self,
            sender: &NSApplication,
        ) -> Option<Id<NSMenu, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other application:willPresentError:)]
        pub unsafe fn application_willPresentError(
            &self,
            application: &NSApplication,
            error: &NSError,
        ) -> Id<NSError, Shared>;

        #[optional]
        #[method(application:didRegisterForRemoteNotificationsWithDeviceToken:)]
        pub unsafe fn application_didRegisterForRemoteNotificationsWithDeviceToken(
            &self,
            application: &NSApplication,
            deviceToken: &NSData,
        );

        #[optional]
        #[method(application:didFailToRegisterForRemoteNotificationsWithError:)]
        pub unsafe fn application_didFailToRegisterForRemoteNotificationsWithError(
            &self,
            application: &NSApplication,
            error: &NSError,
        );

        #[optional]
        #[method(application:didReceiveRemoteNotification:)]
        pub unsafe fn application_didReceiveRemoteNotification(
            &self,
            application: &NSApplication,
            userInfo: &NSDictionary<NSString, Object>,
        );

        #[optional]
        #[method(applicationSupportsSecureRestorableState:)]
        pub unsafe fn applicationSupportsSecureRestorableState(&self, app: &NSApplication) -> bool;

        #[optional]
        #[method(application:willEncodeRestorableState:)]
        pub unsafe fn application_willEncodeRestorableState(
            &self,
            app: &NSApplication,
            coder: &NSCoder,
        );

        #[optional]
        #[method(application:didDecodeRestorableState:)]
        pub unsafe fn application_didDecodeRestorableState(
            &self,
            app: &NSApplication,
            coder: &NSCoder,
        );

        #[optional]
        #[method(application:willContinueUserActivityWithType:)]
        pub unsafe fn application_willContinueUserActivityWithType(
            &self,
            application: &NSApplication,
            userActivityType: &NSString,
        ) -> bool;

        #[optional]
        #[method(application:continueUserActivity:restorationHandler:)]
        pub unsafe fn application_continueUserActivity_restorationHandler(
            &self,
            application: &NSApplication,
            userActivity: &NSUserActivity,
            restorationHandler: &Block<(NonNull<NSArray<NSUserActivityRestoring>>,), ()>,
        ) -> bool;

        #[optional]
        #[method(application:didFailToContinueUserActivityWithType:error:)]
        pub unsafe fn application_didFailToContinueUserActivityWithType_error(
            &self,
            application: &NSApplication,
            userActivityType: &NSString,
            error: &NSError,
        );

        #[optional]
        #[method(application:didUpdateUserActivity:)]
        pub unsafe fn application_didUpdateUserActivity(
            &self,
            application: &NSApplication,
            userActivity: &NSUserActivity,
        );

        #[optional]
        #[method(application:delegateHandlesKey:)]
        pub unsafe fn application_delegateHandlesKey(
            &self,
            sender: &NSApplication,
            key: &NSString,
        ) -> bool;

        #[optional]
        #[method(applicationShouldAutomaticallyLocalizeKeyEquivalents:)]
        pub unsafe fn applicationShouldAutomaticallyLocalizeKeyEquivalents(
            &self,
            application: &NSApplication,
        ) -> bool;

        #[optional]
        #[method(applicationWillFinishLaunching:)]
        pub unsafe fn applicationWillFinishLaunching(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationDidFinishLaunching:)]
        pub unsafe fn applicationDidFinishLaunching(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationWillHide:)]
        pub unsafe fn applicationWillHide(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationDidHide:)]
        pub unsafe fn applicationDidHide(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationWillUnhide:)]
        pub unsafe fn applicationWillUnhide(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationDidUnhide:)]
        pub unsafe fn applicationDidUnhide(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationWillBecomeActive:)]
        pub unsafe fn applicationWillBecomeActive(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationDidBecomeActive:)]
        pub unsafe fn applicationDidBecomeActive(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationWillResignActive:)]
        pub unsafe fn applicationWillResignActive(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationDidResignActive:)]
        pub unsafe fn applicationDidResignActive(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationWillUpdate:)]
        pub unsafe fn applicationWillUpdate(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationDidUpdate:)]
        pub unsafe fn applicationDidUpdate(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationWillTerminate:)]
        pub unsafe fn applicationWillTerminate(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationDidChangeScreenParameters:)]
        pub unsafe fn applicationDidChangeScreenParameters(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationDidChangeOcclusionState:)]
        pub unsafe fn applicationDidChangeOcclusionState(&self, notification: &NSNotification);

        #[optional]
        #[method(applicationProtectedDataWillBecomeUnavailable:)]
        pub unsafe fn applicationProtectedDataWillBecomeUnavailable(
            &self,
            notification: &NSNotification,
        );

        #[optional]
        #[method(applicationProtectedDataDidBecomeAvailable:)]
        pub unsafe fn applicationProtectedDataDidBecomeAvailable(
            &self,
            notification: &NSNotification,
        );
    }
);

extern_methods!(
    /// NSServicesMenu
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other servicesMenu)]
        pub unsafe fn servicesMenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setServicesMenu:)]
        pub unsafe fn setServicesMenu(&self, servicesMenu: Option<&NSMenu>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(registerServicesMenuSendTypes:returnTypes:)]
        pub unsafe fn registerServicesMenuSendTypes_returnTypes(
            &self,
            sendTypes: &NSArray<NSPasteboardType>,
            returnTypes: &NSArray<NSPasteboardType>,
        );
    }
);

extern_protocol!(
    pub struct NSServicesMenuRequestor;

    unsafe impl ProtocolType for NSServicesMenuRequestor {
        #[optional]
        #[method(writeSelectionToPasteboard:types:)]
        pub unsafe fn writeSelectionToPasteboard_types(
            &self,
            pboard: &NSPasteboard,
            types: &NSArray<NSPasteboardType>,
        ) -> bool;

        #[optional]
        #[method(readSelectionFromPasteboard:)]
        pub unsafe fn readSelectionFromPasteboard(&self, pboard: &NSPasteboard) -> bool;
    }
);

extern_methods!(
    /// NSServicesHandling
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method_id(@__retain_semantics Other servicesProvider)]
        pub unsafe fn servicesProvider(&self) -> Option<Id<Object, Shared>>;

        #[method(setServicesProvider:)]
        pub unsafe fn setServicesProvider(&self, servicesProvider: Option<&Object>);
    }
);

typed_enum!(
    pub type NSAboutPanelOptionKey = NSString;
);

extern_static!(NSAboutPanelOptionCredits: &'static NSAboutPanelOptionKey);

extern_static!(NSAboutPanelOptionApplicationName: &'static NSAboutPanelOptionKey);

extern_static!(NSAboutPanelOptionApplicationIcon: &'static NSAboutPanelOptionKey);

extern_static!(NSAboutPanelOptionVersion: &'static NSAboutPanelOptionKey);

extern_static!(NSAboutPanelOptionApplicationVersion: &'static NSAboutPanelOptionKey);

extern_methods!(
    /// NSStandardAboutPanel
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(orderFrontStandardAboutPanel:)]
        pub unsafe fn orderFrontStandardAboutPanel(&self, sender: Option<&Object>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(orderFrontStandardAboutPanelWithOptions:)]
        pub unsafe fn orderFrontStandardAboutPanelWithOptions(
            &self,
            optionsDictionary: &NSDictionary<NSAboutPanelOptionKey, Object>,
        );
    }
);

extern_methods!(
    /// NSApplicationLayoutDirection
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;
    }
);

extern_methods!(
    /// NSRestorableUserInterface
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(disableRelaunchOnLogin)]
        pub unsafe fn disableRelaunchOnLogin(&self);

        #[method(enableRelaunchOnLogin)]
        pub unsafe fn enableRelaunchOnLogin(&self);
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSRemoteNotificationType {
        NSRemoteNotificationTypeNone = 0,
        NSRemoteNotificationTypeBadge = 1 << 0,
        NSRemoteNotificationTypeSound = 1 << 1,
        NSRemoteNotificationTypeAlert = 1 << 2,
    }
);

extern_methods!(
    /// NSRemoteNotifications
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(registerForRemoteNotifications)]
        pub unsafe fn registerForRemoteNotifications(&self);

        #[method(unregisterForRemoteNotifications)]
        pub unsafe fn unregisterForRemoteNotifications(&self);

        #[method(isRegisteredForRemoteNotifications)]
        pub unsafe fn isRegisteredForRemoteNotifications(&self) -> bool;

        #[method(registerForRemoteNotificationTypes:)]
        pub unsafe fn registerForRemoteNotificationTypes(&self, types: NSRemoteNotificationType);

        #[method(enabledRemoteNotificationTypes)]
        pub unsafe fn enabledRemoteNotificationTypes(&self) -> NSRemoteNotificationType;
    }
);

extern_fn!(
    pub unsafe fn NSApplicationMain(argc: c_int, argv: NonNull<NonNull<c_char>>) -> c_int;
);

extern_fn!(
    pub unsafe fn NSApplicationLoad() -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSShowsServicesMenuItem(itemName: &NSString) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSSetShowsServicesMenuItem(itemName: &NSString, enabled: Bool) -> NSInteger;
);

extern_fn!(
    pub unsafe fn NSUpdateDynamicServices();
);

extern_fn!(
    #[cfg(all(feature = "AppKit_NSPasteboard", feature = "Foundation_NSString"))]
    pub unsafe fn NSPerformService(itemName: &NSString, pboard: Option<&NSPasteboard>) -> Bool;
);

pub type NSServiceProviderName = NSString;

extern_fn!(
    pub unsafe fn NSRegisterServicesProvider(
        provider: Option<&Object>,
        name: &NSServiceProviderName,
    );
);

extern_fn!(
    pub unsafe fn NSUnregisterServicesProvider(name: &NSServiceProviderName);
);

extern_static!(NSApplicationDidBecomeActiveNotification: &'static NSNotificationName);

extern_static!(NSApplicationDidHideNotification: &'static NSNotificationName);

extern_static!(NSApplicationDidFinishLaunchingNotification: &'static NSNotificationName);

extern_static!(NSApplicationDidResignActiveNotification: &'static NSNotificationName);

extern_static!(NSApplicationDidUnhideNotification: &'static NSNotificationName);

extern_static!(NSApplicationDidUpdateNotification: &'static NSNotificationName);

extern_static!(NSApplicationWillBecomeActiveNotification: &'static NSNotificationName);

extern_static!(NSApplicationWillHideNotification: &'static NSNotificationName);

extern_static!(NSApplicationWillFinishLaunchingNotification: &'static NSNotificationName);

extern_static!(NSApplicationWillResignActiveNotification: &'static NSNotificationName);

extern_static!(NSApplicationWillUnhideNotification: &'static NSNotificationName);

extern_static!(NSApplicationWillUpdateNotification: &'static NSNotificationName);

extern_static!(NSApplicationWillTerminateNotification: &'static NSNotificationName);

extern_static!(NSApplicationDidChangeScreenParametersNotification: &'static NSNotificationName);

extern_static!(
    NSApplicationProtectedDataWillBecomeUnavailableNotification: &'static NSNotificationName
);

extern_static!(
    NSApplicationProtectedDataDidBecomeAvailableNotification: &'static NSNotificationName
);

extern_static!(NSApplicationLaunchIsDefaultLaunchKey: &'static NSString);

extern_static!(NSApplicationLaunchUserNotificationKey: &'static NSString);

extern_static!(NSApplicationLaunchRemoteNotificationKey: &'static NSString);

extern_static!(NSApplicationDidChangeOcclusionStateNotification: &'static NSNotificationName);

extern_enum!(
    #[underlying(c_int)]
    pub enum {
        NSRunStoppedResponse = -1000,
        NSRunAbortedResponse = -1001,
        NSRunContinuesResponse = -1002,
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[cfg(feature = "AppKit_NSWindow")]
        #[method(runModalForWindow:relativeToWindow:)]
        pub unsafe fn runModalForWindow_relativeToWindow(
            &self,
            window: Option<&NSWindow>,
            docWindow: Option<&NSWindow>,
        ) -> NSInteger;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(beginModalSessionForWindow:relativeToWindow:)]
        pub unsafe fn beginModalSessionForWindow_relativeToWindow(
            &self,
            window: Option<&NSWindow>,
            docWindow: Option<&NSWindow>,
        ) -> NSModalSession;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(application:printFiles:)]
        pub unsafe fn application_printFiles(
            &self,
            sender: Option<&NSApplication>,
            filenames: Option<&NSArray<NSString>>,
        );

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(beginSheet:modalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheet_modalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            sheet: &NSWindow,
            docWindow: &NSWindow,
            modalDelegate: Option<&Object>,
            didEndSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(endSheet:)]
        pub unsafe fn endSheet(&self, sheet: &NSWindow);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(endSheet:returnCode:)]
        pub unsafe fn endSheet_returnCode(&self, sheet: &NSWindow, returnCode: NSInteger);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other makeWindowsPerform:inOrder:)]
        pub unsafe fn makeWindowsPerform_inOrder(
            &self,
            selector: Sel,
            flag: bool,
        ) -> Option<Id<NSWindow, Shared>>;

        #[cfg(feature = "AppKit_NSGraphicsContext")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Option<Id<NSGraphicsContext, Shared>>;
    }
);
