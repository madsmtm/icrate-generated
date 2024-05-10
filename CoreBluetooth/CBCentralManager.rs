//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[deprecated = "Use CBManagerState instead"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CBCentralManagerState(pub NSInteger);
impl CBCentralManagerState {
    #[cfg(feature = "CBManager")]
    #[deprecated = "Use CBManagerState instead"]
    #[doc(alias = "CBCentralManagerStateUnknown")]
    pub const Unknown: Self = Self(CBManagerState::Unknown.0);
    #[cfg(feature = "CBManager")]
    #[deprecated = "Use CBManagerState instead"]
    #[doc(alias = "CBCentralManagerStateResetting")]
    pub const Resetting: Self = Self(CBManagerState::Resetting.0);
    #[cfg(feature = "CBManager")]
    #[deprecated = "Use CBManagerState instead"]
    #[doc(alias = "CBCentralManagerStateUnsupported")]
    pub const Unsupported: Self = Self(CBManagerState::Unsupported.0);
    #[cfg(feature = "CBManager")]
    #[deprecated = "Use CBManagerState instead"]
    #[doc(alias = "CBCentralManagerStateUnauthorized")]
    pub const Unauthorized: Self = Self(CBManagerState::Unauthorized.0);
    #[cfg(feature = "CBManager")]
    #[deprecated = "Use CBManagerState instead"]
    #[doc(alias = "CBCentralManagerStatePoweredOff")]
    pub const PoweredOff: Self = Self(CBManagerState::PoweredOff.0);
    #[cfg(feature = "CBManager")]
    #[deprecated = "Use CBManagerState instead"]
    #[doc(alias = "CBCentralManagerStatePoweredOn")]
    pub const PoweredOn: Self = Self(CBManagerState::PoweredOn.0);
}

unsafe impl Encode for CBCentralManagerState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CBCentralManagerState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CBConnectionEvent(pub NSInteger);
impl CBConnectionEvent {
    #[doc(alias = "CBConnectionEventPeerDisconnected")]
    pub const PeerDisconnected: Self = Self(0);
    #[doc(alias = "CBConnectionEventPeerConnected")]
    pub const PeerConnected: Self = Self(1);
}

unsafe impl Encode for CBConnectionEvent {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CBConnectionEvent {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CBCentralManagerFeature(pub NSUInteger);
impl CBCentralManagerFeature {
    #[doc(alias = "CBCentralManagerFeatureExtendedScanAndConnect")]
    pub const ExtendedScanAndConnect: Self = Self(1 << 0);
}

unsafe impl Encode for CBCentralManagerFeature {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CBCentralManagerFeature {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CBManager")]
    pub struct CBCentralManager;

    #[cfg(feature = "CBManager")]
    unsafe impl ClassType for CBCentralManager {
        #[inherits(NSObject)]
        type Super = CBManager;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CBManager")]
unsafe impl NSObjectProtocol for CBCentralManager {}

extern_methods!(
    #[cfg(feature = "CBManager")]
    unsafe impl CBCentralManager {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn CBCentralManagerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CBCentralManagerDelegate>>,
        );

        #[method(isScanning)]
        pub unsafe fn isScanning(&self) -> bool;

        #[method(supportsFeatures:)]
        pub unsafe fn supportsFeatures(features: CBCentralManagerFeature) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "CBPeer", feature = "CBPeripheral"))]
        #[method_id(@__retain_semantics Other retrievePeripheralsWithIdentifiers:)]
        pub unsafe fn retrievePeripheralsWithIdentifiers(
            &self,
            identifiers: &NSArray<NSUUID>,
        ) -> Id<NSArray<CBPeripheral>>;

        #[cfg(all(feature = "CBPeer", feature = "CBPeripheral", feature = "CBUUID"))]
        #[method_id(@__retain_semantics Other retrieveConnectedPeripheralsWithServices:)]
        pub unsafe fn retrieveConnectedPeripheralsWithServices(
            &self,
            service_uui_ds: &NSArray<CBUUID>,
        ) -> Id<NSArray<CBPeripheral>>;

        #[cfg(feature = "CBUUID")]
        #[method(scanForPeripheralsWithServices:options:)]
        pub unsafe fn scanForPeripheralsWithServices_options(
            &self,
            service_uui_ds: Option<&NSArray<CBUUID>>,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[method(stopScan)]
        pub unsafe fn stopScan(&self);

        #[cfg(all(feature = "CBPeer", feature = "CBPeripheral"))]
        #[method(connectPeripheral:options:)]
        pub unsafe fn connectPeripheral_options(
            &self,
            peripheral: &CBPeripheral,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(all(feature = "CBPeer", feature = "CBPeripheral"))]
        #[method(cancelPeripheralConnection:)]
        pub unsafe fn cancelPeripheralConnection(&self, peripheral: &CBPeripheral);

        #[cfg(feature = "CBCentralManagerConstants")]
        #[method(registerForConnectionEventsWithOptions:)]
        pub unsafe fn registerForConnectionEventsWithOptions(
            &self,
            options: Option<&NSDictionary<CBConnectionEventMatchingOption, AnyObject>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CBManager")]
    unsafe impl CBCentralManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait CBCentralManagerDelegate: NSObjectProtocol {
        #[cfg(feature = "CBManager")]
        #[method(centralManagerDidUpdateState:)]
        unsafe fn centralManagerDidUpdateState(&self, central: &CBCentralManager);

        #[cfg(feature = "CBManager")]
        #[optional]
        #[method(centralManager:willRestoreState:)]
        unsafe fn centralManager_willRestoreState(
            &self,
            central: &CBCentralManager,
            dict: &NSDictionary<NSString, AnyObject>,
        );

        #[cfg(all(feature = "CBManager", feature = "CBPeer", feature = "CBPeripheral"))]
        #[optional]
        #[method(centralManager:didDiscoverPeripheral:advertisementData:RSSI:)]
        unsafe fn centralManager_didDiscoverPeripheral_advertisementData_RSSI(
            &self,
            central: &CBCentralManager,
            peripheral: &CBPeripheral,
            advertisement_data: &NSDictionary<NSString, AnyObject>,
            rssi: &NSNumber,
        );

        #[cfg(all(feature = "CBManager", feature = "CBPeer", feature = "CBPeripheral"))]
        #[optional]
        #[method(centralManager:didConnectPeripheral:)]
        unsafe fn centralManager_didConnectPeripheral(
            &self,
            central: &CBCentralManager,
            peripheral: &CBPeripheral,
        );

        #[cfg(all(feature = "CBManager", feature = "CBPeer", feature = "CBPeripheral"))]
        #[optional]
        #[method(centralManager:didFailToConnectPeripheral:error:)]
        unsafe fn centralManager_didFailToConnectPeripheral_error(
            &self,
            central: &CBCentralManager,
            peripheral: &CBPeripheral,
            error: Option<&NSError>,
        );

        #[cfg(all(feature = "CBManager", feature = "CBPeer", feature = "CBPeripheral"))]
        #[optional]
        #[method(centralManager:didDisconnectPeripheral:error:)]
        unsafe fn centralManager_didDisconnectPeripheral_error(
            &self,
            central: &CBCentralManager,
            peripheral: &CBPeripheral,
            error: Option<&NSError>,
        );

        #[cfg(all(feature = "CBManager", feature = "CBPeer", feature = "CBPeripheral"))]
        #[optional]
        #[method(centralManager:connectionEventDidOccur:forPeripheral:)]
        unsafe fn centralManager_connectionEventDidOccur_forPeripheral(
            &self,
            central: &CBCentralManager,
            event: CBConnectionEvent,
            peripheral: &CBPeripheral,
        );

        #[cfg(all(feature = "CBManager", feature = "CBPeer", feature = "CBPeripheral"))]
        #[optional]
        #[method(centralManager:didUpdateANCSAuthorizationForPeripheral:)]
        unsafe fn centralManager_didUpdateANCSAuthorizationForPeripheral(
            &self,
            central: &CBCentralManager,
            peripheral: &CBPeripheral,
        );
    }

    unsafe impl ProtocolType for dyn CBCentralManagerDelegate {}
);