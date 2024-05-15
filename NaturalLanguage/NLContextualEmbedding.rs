//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NLContextualEmbeddingKey = NSString;

extern "C" {
    pub static NLContextualEmbeddingKeyLanguages: &'static NLContextualEmbeddingKey;
}

extern "C" {
    pub static NLContextualEmbeddingKeyScripts: &'static NLContextualEmbeddingKey;
}

extern "C" {
    pub static NLContextualEmbeddingKeyRevision: &'static NLContextualEmbeddingKey;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NLContextualEmbeddingAssetsResult(pub NSInteger);
impl NLContextualEmbeddingAssetsResult {
    #[doc(alias = "NLContextualEmbeddingAssetsResultAvailable")]
    pub const Available: Self = Self(0);
    #[doc(alias = "NLContextualEmbeddingAssetsResultNotAvailable")]
    pub const NotAvailable: Self = Self(1);
    #[doc(alias = "NLContextualEmbeddingAssetsResultError")]
    pub const Error: Self = Self(2);
}

unsafe impl Encode for NLContextualEmbeddingAssetsResult {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NLContextualEmbeddingAssetsResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NLContextualEmbedding;

    unsafe impl ClassType for NLContextualEmbedding {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NLContextualEmbedding {}

extern_methods!(
    unsafe impl NLContextualEmbedding {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other contextualEmbeddingWithModelIdentifier:)]
        pub unsafe fn contextualEmbeddingWithModelIdentifier(
            model_identifier: &NSString,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other contextualEmbeddingsForValues:)]
        pub unsafe fn contextualEmbeddingsForValues(
            values_dictionary: &NSDictionary<NLContextualEmbeddingKey, AnyObject>,
        ) -> Id<NSArray<NLContextualEmbedding>>;

        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Other contextualEmbeddingWithLanguage:)]
        pub unsafe fn contextualEmbeddingWithLanguage(
            language: &NLLanguage,
        ) -> Option<Id<NLContextualEmbedding>>;

        #[cfg(feature = "NLScript")]
        #[method_id(@__retain_semantics Other contextualEmbeddingWithScript:)]
        pub unsafe fn contextualEmbeddingWithScript(
            script: &NLScript,
        ) -> Option<Id<NLContextualEmbedding>>;

        #[method_id(@__retain_semantics Other modelIdentifier)]
        pub unsafe fn modelIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Other languages)]
        pub unsafe fn languages(&self) -> Id<NSArray<NLLanguage>>;

        #[cfg(feature = "NLScript")]
        #[method_id(@__retain_semantics Other scripts)]
        pub unsafe fn scripts(&self) -> Id<NSArray<NLScript>>;

        #[method(revision)]
        pub unsafe fn revision(&self) -> NSUInteger;

        #[method(dimension)]
        pub unsafe fn dimension(&self) -> NSUInteger;

        #[method(maximumSequenceLength)]
        pub unsafe fn maximumSequenceLength(&self) -> NSUInteger;

        #[method(loadWithError:_)]
        pub unsafe fn loadWithError(&self) -> Result<(), Id<NSError>>;

        #[method(unload)]
        pub unsafe fn unload(&self);

        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Other embeddingResultForString:language:error:_)]
        pub unsafe fn embeddingResultForString_language_error(
            &self,
            string: &NSString,
            language: Option<&NLLanguage>,
        ) -> Result<Id<NLContextualEmbeddingResult>, Id<NSError>>;

        #[method(hasAvailableAssets)]
        pub unsafe fn hasAvailableAssets(&self) -> bool;

        #[cfg(feature = "block2")]
        #[method(requestEmbeddingAssetsWithCompletionHandler:)]
        pub unsafe fn requestEmbeddingAssetsWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<
                dyn Fn(NLContextualEmbeddingAssetsResult, *mut NSError),
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NLContextualEmbedding {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NLContextualEmbeddingResult;

    unsafe impl ClassType for NLContextualEmbeddingResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NLContextualEmbeddingResult {}

extern_methods!(
    unsafe impl NLContextualEmbeddingResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Id<NSString>;

        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Other language)]
        pub unsafe fn language(&self) -> Id<NLLanguage>;

        #[method(sequenceLength)]
        pub unsafe fn sequenceLength(&self) -> NSUInteger;

        #[cfg(feature = "block2")]
        #[method(enumerateTokenVectorsInRange:usingBlock:)]
        pub unsafe fn enumerateTokenVectorsInRange_usingBlock(
            &self,
            range: NSRange,
            block: &block2::Block<dyn Fn(NonNull<NSArray<NSNumber>>, NSRange, NonNull<Bool>) + '_>,
        );

        #[method_id(@__retain_semantics Other tokenVectorAtIndex:tokenRange:)]
        pub unsafe fn tokenVectorAtIndex_tokenRange(
            &self,
            character_index: NSUInteger,
            token_range: NSRangePointer,
        ) -> Option<Id<NSArray<NSNumber>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NLContextualEmbeddingResult {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
