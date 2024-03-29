// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `MailKit` framework
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

#[link(name = "MailKit", kind = "framework")]
extern "C" {}

#[cfg(feature = "MailKit_MEAddressAnnotation")]
#[path = "MEAddressAnnotation.rs"]
mod __MEAddressAnnotation;
#[cfg(feature = "MailKit_MEComposeContext")]
#[path = "MEComposeContext.rs"]
mod __MEComposeContext;
#[cfg(feature = "MailKit_MEComposeSession")]
#[path = "MEComposeSession.rs"]
mod __MEComposeSession;
#[cfg(feature = "MailKit_MEContentBlocker")]
#[path = "MEContentBlocker.rs"]
mod __MEContentBlocker;
#[cfg(feature = "MailKit_MEDecodedMessage")]
#[path = "MEDecodedMessage.rs"]
mod __MEDecodedMessage;
#[cfg(feature = "MailKit_MEDecodedMessageBanner")]
#[path = "MEDecodedMessageBanner.rs"]
mod __MEDecodedMessageBanner;
#[cfg(feature = "MailKit_MEEmailAddress")]
#[path = "MEEmailAddress.rs"]
mod __MEEmailAddress;
#[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
#[path = "MEEncodedOutgoingMessage.rs"]
mod __MEEncodedOutgoingMessage;
#[cfg(feature = "MailKit_MEExtension")]
#[path = "MEExtension.rs"]
mod __MEExtension;
#[cfg(feature = "MailKit_MEExtensionManager")]
#[path = "MEExtensionManager.rs"]
mod __MEExtensionManager;
#[cfg(feature = "MailKit_MEExtensionViewController")]
#[path = "MEExtensionViewController.rs"]
mod __MEExtensionViewController;
#[cfg(feature = "MailKit_MEMessage")]
#[path = "MEMessage.rs"]
mod __MEMessage;
#[cfg(feature = "MailKit_MEMessageAction")]
#[path = "MEMessageAction.rs"]
mod __MEMessageAction;
#[cfg(feature = "MailKit_MEMessageActionDecision")]
#[path = "MEMessageActionDecision.rs"]
mod __MEMessageActionDecision;
#[cfg(feature = "MailKit_MEMessageActionHandler")]
#[path = "MEMessageActionHandler.rs"]
mod __MEMessageActionHandler;
#[cfg(feature = "MailKit_MEMessageDecoder")]
#[path = "MEMessageDecoder.rs"]
mod __MEMessageDecoder;
#[cfg(feature = "MailKit_MEMessageEncoder")]
#[path = "MEMessageEncoder.rs"]
mod __MEMessageEncoder;
#[cfg(feature = "MailKit_MEMessageEncodingResult")]
#[path = "MEMessageEncodingResult.rs"]
mod __MEMessageEncodingResult;
#[cfg(feature = "MailKit_MEMessageSecurityHandler")]
#[path = "MEMessageSecurityHandler.rs"]
mod __MEMessageSecurityHandler;
#[cfg(feature = "MailKit_MEMessageSecurityInformation")]
#[path = "MEMessageSecurityInformation.rs"]
mod __MEMessageSecurityInformation;
#[cfg(feature = "MailKit_MEMessageSigner")]
#[path = "MEMessageSigner.rs"]
mod __MEMessageSigner;
#[cfg(feature = "MailKit_MEOutgoingMessageEncodingStatus")]
#[path = "MEOutgoingMessageEncodingStatus.rs"]
mod __MEOutgoingMessageEncodingStatus;

#[cfg(feature = "MailKit_MEAddressAnnotation")]
pub use self::__MEAddressAnnotation::MEAddressAnnotation;
#[cfg(feature = "MailKit_MEComposeContext")]
pub use self::__MEComposeContext::MEComposeContext;
#[cfg(feature = "MailKit_MEComposeContext")]
pub use self::__MEComposeContext::MEComposeUserAction;
#[cfg(feature = "MailKit_MEComposeSession")]
pub use self::__MEComposeSession::MEComposeSession;
#[cfg(feature = "MailKit_MEComposeSession")]
pub use self::__MEComposeSession::MEComposeSessionErrorCode;
#[cfg(all(
    feature = "Foundation_NSError",
    feature = "Foundation_NSString",
    feature = "MailKit_MEComposeSession"
))]
pub use self::__MEComposeSession::MEComposeSessionErrorDomain;
#[cfg(feature = "MailKit_MEComposeSession")]
pub use self::__MEComposeSession::MEComposeSessionHandler;
#[cfg(feature = "MailKit_MEContentBlocker")]
pub use self::__MEContentBlocker::MEContentBlocker;
#[cfg(feature = "MailKit_MEDecodedMessage")]
pub use self::__MEDecodedMessage::MEDecodedMessage;
#[cfg(feature = "MailKit_MEDecodedMessageBanner")]
pub use self::__MEDecodedMessageBanner::MEDecodedMessageBanner;
#[cfg(feature = "MailKit_MEEmailAddress")]
pub use self::__MEEmailAddress::MEEmailAddress;
#[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
pub use self::__MEEncodedOutgoingMessage::MEEncodedOutgoingMessage;
#[cfg(feature = "MailKit_MEExtension")]
pub use self::__MEExtension::MEExtension;
#[cfg(feature = "MailKit_MEExtensionManager")]
pub use self::__MEExtensionManager::MEExtensionManager;
#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "MailKit_MEExtensionViewController"
))]
pub use self::__MEExtensionViewController::MEExtensionViewController;
#[cfg(feature = "MailKit_MEMessage")]
pub use self::__MEMessage::MEMessage;
#[cfg(feature = "MailKit_MEMessage")]
pub use self::__MEMessage::MEMessageEncryptionState;
#[cfg(feature = "MailKit_MEMessage")]
pub use self::__MEMessage::MEMessageState;
#[cfg(feature = "MailKit_MEMessageAction")]
pub use self::__MEMessageAction::MEMessageAction;
#[cfg(feature = "MailKit_MEMessageAction")]
pub use self::__MEMessageAction::MEMessageActionFlag;
#[cfg(feature = "MailKit_MEMessageAction")]
pub use self::__MEMessageAction::MEMessageActionMessageColor;
#[cfg(feature = "MailKit_MEMessageActionDecision")]
pub use self::__MEMessageActionDecision::MEMessageActionDecision;
#[cfg(feature = "MailKit_MEMessageActionHandler")]
pub use self::__MEMessageActionHandler::MEMessageActionHandler;
#[cfg(feature = "MailKit_MEMessageDecoder")]
pub use self::__MEMessageDecoder::MEMessageDecoder;
#[cfg(feature = "MailKit_MEMessageEncoder")]
pub use self::__MEMessageEncoder::MEMessageEncoder;
#[cfg(feature = "MailKit_MEMessageEncodingResult")]
pub use self::__MEMessageEncodingResult::MEMessageEncodingResult;
#[cfg(feature = "MailKit_MEMessageSecurityHandler")]
pub use self::__MEMessageSecurityHandler::MEMessageSecurityErrorCode;
#[cfg(all(
    feature = "Foundation_NSError",
    feature = "Foundation_NSString",
    feature = "MailKit_MEMessageSecurityHandler"
))]
pub use self::__MEMessageSecurityHandler::MEMessageSecurityErrorDomain;
#[cfg(all(
    feature = "MailKit_MEMessageDecoder",
    feature = "MailKit_MEMessageEncoder",
    feature = "MailKit_MEMessageSecurityHandler"
))]
pub use self::__MEMessageSecurityHandler::MEMessageSecurityHandler;
#[cfg(feature = "MailKit_MEMessageSecurityInformation")]
pub use self::__MEMessageSecurityInformation::MEMessageSecurityInformation;
#[cfg(feature = "MailKit_MEMessageSigner")]
pub use self::__MEMessageSigner::MEMessageSigner;
#[cfg(feature = "MailKit_MEOutgoingMessageEncodingStatus")]
pub use self::__MEOutgoingMessageEncodingStatus::MEOutgoingMessageEncodingStatus;
