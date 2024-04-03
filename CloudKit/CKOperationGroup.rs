//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKOperationGroupTransferSize(pub NSInteger);
impl CKOperationGroupTransferSize {
    #[doc(alias = "CKOperationGroupTransferSizeUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CKOperationGroupTransferSizeKilobytes")]
    pub const Kilobytes: Self = Self(1);
    #[doc(alias = "CKOperationGroupTransferSizeMegabytes")]
    pub const Megabytes: Self = Self(2);
    #[doc(alias = "CKOperationGroupTransferSizeTensOfMegabytes")]
    pub const TensOfMegabytes: Self = Self(3);
    #[doc(alias = "CKOperationGroupTransferSizeHundredsOfMegabytes")]
    pub const HundredsOfMegabytes: Self = Self(4);
    #[doc(alias = "CKOperationGroupTransferSizeGigabytes")]
    pub const Gigabytes: Self = Self(5);
    #[doc(alias = "CKOperationGroupTransferSizeTensOfGigabytes")]
    pub const TensOfGigabytes: Self = Self(6);
    #[doc(alias = "CKOperationGroupTransferSizeHundredsOfGigabytes")]
    pub const HundredsOfGigabytes: Self = Self(7);
}

unsafe impl Encode for CKOperationGroupTransferSize {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKOperationGroupTransferSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKOperationGroup;

    unsafe impl ClassType for CKOperationGroup {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CKOperationGroup {}

unsafe impl NSObjectProtocol for CKOperationGroup {}

unsafe impl NSSecureCoding for CKOperationGroup {}

extern_methods!(
    unsafe impl CKOperationGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, a_decoder: &NSCoder) -> Id<Self>;

        #[method_id(@__retain_semantics Other operationGroupID)]
        pub unsafe fn operationGroupID(&self) -> Id<NSString>;

        #[cfg(feature = "CloudKit_CKOperation")]
        #[method_id(@__retain_semantics Other defaultConfiguration)]
        pub unsafe fn defaultConfiguration(&self) -> Id<CKOperationConfiguration>;

        #[cfg(feature = "CloudKit_CKOperation")]
        #[method(setDefaultConfiguration:)]
        pub unsafe fn setDefaultConfiguration(
            &self,
            default_configuration: Option<&CKOperationConfiguration>,
        );

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(quantity)]
        pub unsafe fn quantity(&self) -> NSUInteger;

        #[method(setQuantity:)]
        pub unsafe fn setQuantity(&self, quantity: NSUInteger);

        #[method(expectedSendSize)]
        pub unsafe fn expectedSendSize(&self) -> CKOperationGroupTransferSize;

        #[method(setExpectedSendSize:)]
        pub unsafe fn setExpectedSendSize(&self, expected_send_size: CKOperationGroupTransferSize);

        #[method(expectedReceiveSize)]
        pub unsafe fn expectedReceiveSize(&self) -> CKOperationGroupTransferSize;

        #[method(setExpectedReceiveSize:)]
        pub unsafe fn setExpectedReceiveSize(
            &self,
            expected_receive_size: CKOperationGroupTransferSize,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CKOperationGroup {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
