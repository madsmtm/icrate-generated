//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKBiologicalSexObject;

    unsafe impl ClassType for HKBiologicalSexObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for HKBiologicalSexObject {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for HKBiologicalSexObject {}

unsafe impl NSObjectProtocol for HKBiologicalSexObject {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for HKBiologicalSexObject {}

extern_methods!(
    unsafe impl HKBiologicalSexObject {
        #[cfg(feature = "HealthKit_HKCharacteristicValues")]
        #[method(biologicalSex)]
        pub unsafe fn biologicalSex(&self) -> HKBiologicalSex;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKBiologicalSexObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKBloodTypeObject;

    unsafe impl ClassType for HKBloodTypeObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for HKBloodTypeObject {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for HKBloodTypeObject {}

unsafe impl NSObjectProtocol for HKBloodTypeObject {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for HKBloodTypeObject {}

extern_methods!(
    unsafe impl HKBloodTypeObject {
        #[cfg(feature = "HealthKit_HKCharacteristicValues")]
        #[method(bloodType)]
        pub unsafe fn bloodType(&self) -> HKBloodType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKBloodTypeObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKFitzpatrickSkinTypeObject;

    unsafe impl ClassType for HKFitzpatrickSkinTypeObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for HKFitzpatrickSkinTypeObject {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for HKFitzpatrickSkinTypeObject {}

unsafe impl NSObjectProtocol for HKFitzpatrickSkinTypeObject {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for HKFitzpatrickSkinTypeObject {}

extern_methods!(
    unsafe impl HKFitzpatrickSkinTypeObject {
        #[cfg(feature = "HealthKit_HKCharacteristicValues")]
        #[method(skinType)]
        pub unsafe fn skinType(&self) -> HKFitzpatrickSkinType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKFitzpatrickSkinTypeObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKWheelchairUseObject;

    unsafe impl ClassType for HKWheelchairUseObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for HKWheelchairUseObject {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for HKWheelchairUseObject {}

unsafe impl NSObjectProtocol for HKWheelchairUseObject {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for HKWheelchairUseObject {}

extern_methods!(
    unsafe impl HKWheelchairUseObject {
        #[cfg(feature = "HealthKit_HKCharacteristicValues")]
        #[method(wheelchairUse)]
        pub unsafe fn wheelchairUse(&self) -> HKWheelchairUse;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKWheelchairUseObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKActivityMoveModeObject;

    unsafe impl ClassType for HKActivityMoveModeObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for HKActivityMoveModeObject {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for HKActivityMoveModeObject {}

unsafe impl NSObjectProtocol for HKActivityMoveModeObject {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for HKActivityMoveModeObject {}

extern_methods!(
    unsafe impl HKActivityMoveModeObject {
        #[cfg(feature = "HealthKit_HKCharacteristicValues")]
        #[method(activityMoveMode)]
        pub unsafe fn activityMoveMode(&self) -> HKActivityMoveMode;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKActivityMoveModeObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
