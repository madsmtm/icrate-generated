//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static GKErrorDomain: &'static NSString;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKErrorCode(pub NSInteger);
impl GKErrorCode {
    pub const GKErrorUnknown: Self = Self(1);
    pub const GKErrorCancelled: Self = Self(2);
    pub const GKErrorCommunicationsFailure: Self = Self(3);
    pub const GKErrorUserDenied: Self = Self(4);
    pub const GKErrorInvalidCredentials: Self = Self(5);
    pub const GKErrorNotAuthenticated: Self = Self(6);
    pub const GKErrorAuthenticationInProgress: Self = Self(7);
    pub const GKErrorInvalidPlayer: Self = Self(8);
    pub const GKErrorScoreNotSet: Self = Self(9);
    pub const GKErrorParentalControlsBlocked: Self = Self(10);
    pub const GKErrorPlayerStatusExceedsMaximumLength: Self = Self(11);
    pub const GKErrorPlayerStatusInvalid: Self = Self(12);
    pub const GKErrorMatchRequestInvalid: Self = Self(13);
    pub const GKErrorUnderage: Self = Self(14);
    pub const GKErrorGameUnrecognized: Self = Self(15);
    pub const GKErrorNotSupported: Self = Self(16);
    pub const GKErrorInvalidParameter: Self = Self(17);
    pub const GKErrorUnexpectedConnection: Self = Self(18);
    pub const GKErrorChallengeInvalid: Self = Self(19);
    pub const GKErrorTurnBasedMatchDataTooLarge: Self = Self(20);
    pub const GKErrorTurnBasedTooManySessions: Self = Self(21);
    pub const GKErrorTurnBasedInvalidParticipant: Self = Self(22);
    pub const GKErrorTurnBasedInvalidTurn: Self = Self(23);
    pub const GKErrorTurnBasedInvalidState: Self = Self(24);
    pub const GKErrorInvitationsDisabled: Self = Self(25);
    pub const GKErrorPlayerPhotoFailure: Self = Self(26);
    pub const GKErrorUbiquityContainerUnavailable: Self = Self(27);
    pub const GKErrorMatchNotConnected: Self = Self(28);
    pub const GKErrorGameSessionRequestInvalid: Self = Self(29);
    pub const GKErrorRestrictedToAutomatch: Self = Self(30);
    pub const GKErrorAPINotAvailable: Self = Self(31);
    pub const GKErrorNotAuthorized: Self = Self(32);
    pub const GKErrorConnectionTimeout: Self = Self(33);
    pub const GKErrorAPIObsolete: Self = Self(34);
    pub const GKErrorICloudUnavailable: Self = Self(35);
    pub const GKErrorLockdownMode: Self = Self(36);
    pub const GKErrorAppUnlisted: Self = Self(37);
    pub const GKErrorFriendListDescriptionMissing: Self = Self(100);
    pub const GKErrorFriendListRestricted: Self = Self(101);
    pub const GKErrorFriendListDenied: Self = Self(102);
    pub const GKErrorFriendRequestNotAvailable: Self = Self(103);
}

unsafe impl Encode for GKErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GKErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
