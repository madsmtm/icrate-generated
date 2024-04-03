// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "CallKit", kind = "framework")]
extern "C" {}

#[cfg(feature = "CallKit_CXAction")]
#[path = "CXAction.rs"]
mod __CXAction;
#[cfg(feature = "CallKit_CXAnswerCallAction")]
#[path = "CXAnswerCallAction.rs"]
mod __CXAnswerCallAction;
#[cfg(feature = "CallKit_CXBase")]
#[path = "CXBase.rs"]
mod __CXBase;
#[cfg(feature = "CallKit_CXCall")]
#[path = "CXCall.rs"]
mod __CXCall;
#[cfg(feature = "CallKit_CXCallAction")]
#[path = "CXCallAction.rs"]
mod __CXCallAction;
#[cfg(feature = "CallKit_CXCallController")]
#[path = "CXCallController.rs"]
mod __CXCallController;
#[cfg(feature = "CallKit_CXCallDirectory")]
#[path = "CXCallDirectory.rs"]
mod __CXCallDirectory;
#[cfg(feature = "CallKit_CXCallDirectoryExtensionContext")]
#[path = "CXCallDirectoryExtensionContext.rs"]
mod __CXCallDirectoryExtensionContext;
#[cfg(feature = "CallKit_CXCallDirectoryManager")]
#[path = "CXCallDirectoryManager.rs"]
mod __CXCallDirectoryManager;
#[cfg(feature = "CallKit_CXCallDirectoryProvider")]
#[path = "CXCallDirectoryProvider.rs"]
mod __CXCallDirectoryProvider;
#[cfg(feature = "CallKit_CXCallObserver")]
#[path = "CXCallObserver.rs"]
mod __CXCallObserver;
#[cfg(feature = "CallKit_CXCallUpdate")]
#[path = "CXCallUpdate.rs"]
mod __CXCallUpdate;
#[cfg(feature = "CallKit_CXEndCallAction")]
#[path = "CXEndCallAction.rs"]
mod __CXEndCallAction;
#[cfg(feature = "CallKit_CXError")]
#[path = "CXError.rs"]
mod __CXError;
#[cfg(feature = "CallKit_CXHandle")]
#[path = "CXHandle.rs"]
mod __CXHandle;
#[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
#[path = "CXPlayDTMFCallAction.rs"]
mod __CXPlayDTMFCallAction;
#[cfg(feature = "CallKit_CXProvider")]
#[path = "CXProvider.rs"]
mod __CXProvider;
#[cfg(feature = "CallKit_CXProviderConfiguration")]
#[path = "CXProviderConfiguration.rs"]
mod __CXProviderConfiguration;
#[cfg(feature = "CallKit_CXSetGroupCallAction")]
#[path = "CXSetGroupCallAction.rs"]
mod __CXSetGroupCallAction;
#[cfg(feature = "CallKit_CXSetHeldCallAction")]
#[path = "CXSetHeldCallAction.rs"]
mod __CXSetHeldCallAction;
#[cfg(feature = "CallKit_CXSetMutedCallAction")]
#[path = "CXSetMutedCallAction.rs"]
mod __CXSetMutedCallAction;
#[cfg(feature = "CallKit_CXStartCallAction")]
#[path = "CXStartCallAction.rs"]
mod __CXStartCallAction;
#[cfg(feature = "CallKit_CXTransaction")]
#[path = "CXTransaction.rs"]
mod __CXTransaction;

#[cfg(feature = "CallKit_CXAction")]
pub use self::__CXAction::CXAction;
#[cfg(all(
    feature = "CallKit_CXAction",
    feature = "CallKit_CXAnswerCallAction",
    feature = "CallKit_CXCallAction"
))]
pub use self::__CXAnswerCallAction::CXAnswerCallAction;
#[cfg(feature = "CallKit_CXCall")]
pub use self::__CXCall::CXCall;
#[cfg(all(feature = "CallKit_CXAction", feature = "CallKit_CXCallAction"))]
pub use self::__CXCallAction::CXCallAction;
#[cfg(feature = "CallKit_CXCallController")]
pub use self::__CXCallController::CXCallController;
#[cfg(feature = "CallKit_CXCallDirectory")]
pub use self::__CXCallDirectory::CXCallDirectoryPhoneNumber;
#[cfg(feature = "CallKit_CXCallDirectory")]
pub use self::__CXCallDirectory::CXCallDirectoryPhoneNumberMax;
#[cfg(feature = "CallKit_CXCallDirectoryExtensionContext")]
pub use self::__CXCallDirectoryExtensionContext::CXCallDirectoryExtensionContext;
#[cfg(feature = "CallKit_CXCallDirectoryExtensionContext")]
pub use self::__CXCallDirectoryExtensionContext::CXCallDirectoryExtensionContextDelegate;
#[cfg(feature = "CallKit_CXCallDirectoryManager")]
pub use self::__CXCallDirectoryManager::CXCallDirectoryEnabledStatus;
#[cfg(feature = "CallKit_CXCallDirectoryManager")]
pub use self::__CXCallDirectoryManager::CXCallDirectoryManager;
#[cfg(feature = "CallKit_CXCallDirectoryProvider")]
pub use self::__CXCallDirectoryProvider::CXCallDirectoryProvider;
#[cfg(feature = "CallKit_CXCallObserver")]
pub use self::__CXCallObserver::CXCallObserver;
#[cfg(feature = "CallKit_CXCallObserver")]
pub use self::__CXCallObserver::CXCallObserverDelegate;
#[cfg(feature = "CallKit_CXCallUpdate")]
pub use self::__CXCallUpdate::CXCallUpdate;
#[cfg(all(
    feature = "CallKit_CXAction",
    feature = "CallKit_CXCallAction",
    feature = "CallKit_CXEndCallAction"
))]
pub use self::__CXEndCallAction::CXEndCallAction;
#[cfg(feature = "CallKit_CXError")]
pub use self::__CXError::CXErrorCode;
#[cfg(feature = "CallKit_CXError")]
pub use self::__CXError::CXErrorCodeCallDirectoryManagerError;
#[cfg(feature = "CallKit_CXError")]
pub use self::__CXError::CXErrorCodeIncomingCallError;
#[cfg(feature = "CallKit_CXError")]
pub use self::__CXError::CXErrorCodeNotificationServiceExtensionError;
#[cfg(feature = "CallKit_CXError")]
pub use self::__CXError::CXErrorCodeRequestTransactionError;
#[cfg(feature = "CallKit_CXError")]
pub use self::__CXError::CXErrorDomain;
#[cfg(feature = "CallKit_CXError")]
pub use self::__CXError::CXErrorDomainCallDirectoryManager;
#[cfg(feature = "CallKit_CXError")]
pub use self::__CXError::CXErrorDomainIncomingCall;
#[cfg(feature = "CallKit_CXError")]
pub use self::__CXError::CXErrorDomainNotificationServiceExtension;
#[cfg(feature = "CallKit_CXError")]
pub use self::__CXError::CXErrorDomainRequestTransaction;
#[cfg(feature = "CallKit_CXHandle")]
pub use self::__CXHandle::CXHandle;
#[cfg(feature = "CallKit_CXHandle")]
pub use self::__CXHandle::CXHandleType;
#[cfg(all(
    feature = "CallKit_CXAction",
    feature = "CallKit_CXCallAction",
    feature = "CallKit_CXPlayDTMFCallAction"
))]
pub use self::__CXPlayDTMFCallAction::CXPlayDTMFCallAction;
#[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
pub use self::__CXPlayDTMFCallAction::CXPlayDTMFCallActionType;
#[cfg(feature = "CallKit_CXProvider")]
pub use self::__CXProvider::CXCallEndedReason;
#[cfg(feature = "CallKit_CXProvider")]
pub use self::__CXProvider::CXProvider;
#[cfg(feature = "CallKit_CXProvider")]
pub use self::__CXProvider::CXProviderDelegate;
#[cfg(feature = "CallKit_CXProviderConfiguration")]
pub use self::__CXProviderConfiguration::CXProviderConfiguration;
#[cfg(all(
    feature = "CallKit_CXAction",
    feature = "CallKit_CXCallAction",
    feature = "CallKit_CXSetGroupCallAction"
))]
pub use self::__CXSetGroupCallAction::CXSetGroupCallAction;
#[cfg(all(
    feature = "CallKit_CXAction",
    feature = "CallKit_CXCallAction",
    feature = "CallKit_CXSetHeldCallAction"
))]
pub use self::__CXSetHeldCallAction::CXSetHeldCallAction;
#[cfg(all(
    feature = "CallKit_CXAction",
    feature = "CallKit_CXCallAction",
    feature = "CallKit_CXSetMutedCallAction"
))]
pub use self::__CXSetMutedCallAction::CXSetMutedCallAction;
#[cfg(all(
    feature = "CallKit_CXAction",
    feature = "CallKit_CXCallAction",
    feature = "CallKit_CXStartCallAction"
))]
pub use self::__CXStartCallAction::CXStartCallAction;
#[cfg(feature = "CallKit_CXTransaction")]
pub use self::__CXTransaction::CXTransaction;
