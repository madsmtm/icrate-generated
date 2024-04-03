//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWInterface;

    unsafe impl ClassType for CWInterface {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CWInterface {}

extern_methods!(
    unsafe impl CWInterface {
        #[method_id(@__retain_semantics Other interfaceName)]
        pub unsafe fn interfaceName(&self) -> Option<Id<NSString>>;

        #[method(powerOn)]
        pub unsafe fn powerOn(&self) -> bool;

        #[cfg(feature = "CoreWLAN_CWChannel")]
        #[method_id(@__retain_semantics Other supportedWLANChannels)]
        pub unsafe fn supportedWLANChannels(&self) -> Option<Id<NSSet<CWChannel>>>;

        #[cfg(feature = "CoreWLAN_CWChannel")]
        #[method_id(@__retain_semantics Other wlanChannel)]
        pub unsafe fn wlanChannel(&self) -> Option<Id<CWChannel>>;

        #[cfg(feature = "CoreWLAN_CoreWLANTypes")]
        #[method(activePHYMode)]
        pub unsafe fn activePHYMode(&self) -> CWPHYMode;

        #[method_id(@__retain_semantics Other ssid)]
        pub unsafe fn ssid(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other ssidData)]
        pub unsafe fn ssidData(&self) -> Option<Id<NSData>>;

        #[method_id(@__retain_semantics Other bssid)]
        pub unsafe fn bssid(&self) -> Option<Id<NSString>>;

        #[method(rssiValue)]
        pub unsafe fn rssiValue(&self) -> NSInteger;

        #[method(noiseMeasurement)]
        pub unsafe fn noiseMeasurement(&self) -> NSInteger;

        #[cfg(feature = "CoreWLAN_CoreWLANTypes")]
        #[method(security)]
        pub unsafe fn security(&self) -> CWSecurity;

        #[method(transmitRate)]
        pub unsafe fn transmitRate(&self) -> c_double;

        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "CoreWLAN_CoreWLANTypes")]
        #[method(interfaceMode)]
        pub unsafe fn interfaceMode(&self) -> CWInterfaceMode;

        #[method(transmitPower)]
        pub unsafe fn transmitPower(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other hardwareAddress)]
        pub unsafe fn hardwareAddress(&self) -> Option<Id<NSString>>;

        #[method(serviceActive)]
        pub unsafe fn serviceActive(&self) -> bool;

        #[cfg(feature = "CoreWLAN_CWNetwork")]
        #[method_id(@__retain_semantics Other cachedScanResults)]
        pub unsafe fn cachedScanResults(&self) -> Option<Id<NSSet<CWNetwork>>>;

        #[cfg(feature = "CoreWLAN_CWConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Option<Id<CWConfiguration>>;

        #[deprecated = "Use -[CWWiFiClient interfaceNames] instead"]
        #[method_id(@__retain_semantics Other interfaceNames)]
        pub unsafe fn interfaceNames() -> Option<Id<NSSet<NSString>>>;

        #[deprecated = "Use -[CWWiFiClient interface] instead"]
        #[method_id(@__retain_semantics Other interface)]
        pub unsafe fn interface() -> Id<Self>;

        #[deprecated = "Use -[CWWiFiClient interfaceWithName:] instead"]
        #[method_id(@__retain_semantics Other interfaceWithName:)]
        pub unsafe fn interfaceWithName(name: Option<&NSString>) -> Id<Self>;

        #[deprecated = "Use -[CWWiFiClient interfaceWithName:] instead"]
        #[method_id(@__retain_semantics Init initWithInterfaceName:)]
        pub unsafe fn initWithInterfaceName(
            this: Allocated<Self>,
            name: Option<&NSString>,
        ) -> Id<Self>;

        #[method(setPower:error:_)]
        pub unsafe fn setPower_error(&self, power: bool) -> Result<(), Id<NSError>>;

        #[cfg(feature = "CoreWLAN_CWChannel")]
        #[method(setWLANChannel:error:_)]
        pub unsafe fn setWLANChannel_error(&self, channel: &CWChannel) -> Result<(), Id<NSError>>;

        #[method(setPairwiseMasterKey:error:_)]
        pub unsafe fn setPairwiseMasterKey_error(
            &self,
            key: Option<&NSData>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "CoreWLAN_CoreWLANTypes")]
        #[method(setWEPKey:flags:index:error:_)]
        pub unsafe fn setWEPKey_flags_index_error(
            &self,
            key: Option<&NSData>,
            flags: CWCipherKeyFlags,
            index: NSInteger,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "CoreWLAN_CWNetwork")]
        #[method_id(@__retain_semantics Other scanForNetworksWithSSID:error:_)]
        pub unsafe fn scanForNetworksWithSSID_error(
            &self,
            ssid: Option<&NSData>,
        ) -> Result<Id<NSSet<CWNetwork>>, Id<NSError>>;

        #[cfg(feature = "CoreWLAN_CWNetwork")]
        #[method_id(@__retain_semantics Other scanForNetworksWithSSID:includeHidden:error:_)]
        pub unsafe fn scanForNetworksWithSSID_includeHidden_error(
            &self,
            ssid: Option<&NSData>,
            include_hidden: bool,
        ) -> Result<Id<NSSet<CWNetwork>>, Id<NSError>>;

        #[cfg(feature = "CoreWLAN_CWNetwork")]
        #[method_id(@__retain_semantics Other scanForNetworksWithName:error:_)]
        pub unsafe fn scanForNetworksWithName_error(
            &self,
            network_name: Option<&NSString>,
        ) -> Result<Id<NSSet<CWNetwork>>, Id<NSError>>;

        #[cfg(feature = "CoreWLAN_CWNetwork")]
        #[method_id(@__retain_semantics Other scanForNetworksWithName:includeHidden:error:_)]
        pub unsafe fn scanForNetworksWithName_includeHidden_error(
            &self,
            network_name: Option<&NSString>,
            include_hidden: bool,
        ) -> Result<Id<NSSet<CWNetwork>>, Id<NSError>>;

        #[cfg(feature = "CoreWLAN_CWNetwork")]
        #[method(associateToNetwork:password:error:_)]
        pub unsafe fn associateToNetwork_password_error(
            &self,
            network: &CWNetwork,
            password: Option<&NSString>,
        ) -> Result<(), Id<NSError>>;

        #[method(disassociate)]
        pub unsafe fn disassociate(&self);

        #[cfg(feature = "CoreWLAN_CoreWLANTypes")]
        #[deprecated]
        #[method(startIBSSModeWithSSID:security:channel:password:error:_)]
        pub unsafe fn startIBSSModeWithSSID_security_channel_password_error(
            &self,
            ssid_data: &NSData,
            security: CWIBSSModeSecurity,
            channel: NSUInteger,
            password: Option<&NSString>,
        ) -> Result<(), Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWInterface {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
