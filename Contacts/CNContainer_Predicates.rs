//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_methods!(
    /// Predicates
    #[cfg(feature = "Contacts_CNContainer")]
    unsafe impl CNContainer {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSPredicate",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other predicateForContainersWithIdentifiers:)]
        pub unsafe fn predicateForContainersWithIdentifiers(
            identifiers: &NSArray<NSString>,
        ) -> Id<NSPredicate>;

        #[cfg(all(feature = "Foundation_NSPredicate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other predicateForContainerOfContactWithIdentifier:)]
        pub unsafe fn predicateForContainerOfContactWithIdentifier(
            contact_identifier: &NSString,
        ) -> Id<NSPredicate>;

        #[cfg(all(feature = "Foundation_NSPredicate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other predicateForContainerOfGroupWithIdentifier:)]
        pub unsafe fn predicateForContainerOfGroupWithIdentifier(
            group_identifier: &NSString,
        ) -> Id<NSPredicate>;
    }
);
