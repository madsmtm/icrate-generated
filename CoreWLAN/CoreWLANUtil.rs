//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    #[cfg(feature = "CoreWLAN_CWNetwork")]
    pub fn CWMergeNetworks(networks: &NSSet<CWNetwork>) -> NonNull<NSSet<CWNetwork>>;
}
