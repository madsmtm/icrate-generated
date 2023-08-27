//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_static!(CLLocationPushServiceErrorDomain: Option<&'static NSErrorDomain>);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum CLLocationPushServiceError {
        CLLocationPushServiceErrorUnknown = 0,
        CLLocationPushServiceErrorMissingPushExtension = 1,
        CLLocationPushServiceErrorMissingPushServerEnvironment = 2,
        CLLocationPushServiceErrorMissingEntitlement = 3,
        CLLocationPushServiceErrorUnsupportedPlatform = 4,
    }
);
