//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCLSTMDescriptor")]
    pub struct MLCLSTMDescriptor;

    #[cfg(feature = "MLCompute_MLCLSTMDescriptor")]
    unsafe impl ClassType for MLCLSTMDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCLSTMDescriptor")]
unsafe impl NSCopying for MLCLSTMDescriptor {}

#[cfg(feature = "MLCompute_MLCLSTMDescriptor")]
unsafe impl NSObjectProtocol for MLCLSTMDescriptor {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCLSTMDescriptor")]
    unsafe impl MLCLSTMDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method(inputSize)]
        pub unsafe fn inputSize(&self) -> NSUInteger;

        #[method(hiddenSize)]
        pub unsafe fn hiddenSize(&self) -> NSUInteger;

        #[method(layerCount)]
        pub unsafe fn layerCount(&self) -> NSUInteger;

        #[method(usesBiases)]
        pub unsafe fn usesBiases(&self) -> bool;

        #[method(batchFirst)]
        pub unsafe fn batchFirst(&self) -> bool;

        #[method(isBidirectional)]
        pub unsafe fn isBidirectional(&self) -> bool;

        #[method(returnsSequences)]
        pub unsafe fn returnsSequences(&self) -> bool;

        #[method(dropout)]
        pub unsafe fn dropout(&self) -> c_float;

        #[method(resultMode)]
        pub unsafe fn resultMode(&self) -> MLCLSTMResultMode;

        #[method_id(@__retain_semantics Other descriptorWithInputSize:hiddenSize:layerCount:)]
        pub unsafe fn descriptorWithInputSize_hiddenSize_layerCount(
            input_size: NSUInteger,
            hidden_size: NSUInteger,
            layer_count: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other descriptorWithInputSize:hiddenSize:layerCount:usesBiases:isBidirectional:dropout:)]
        pub unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_isBidirectional_dropout(
            input_size: NSUInteger,
            hidden_size: NSUInteger,
            layer_count: NSUInteger,
            uses_biases: bool,
            is_bidirectional: bool,
            dropout: c_float,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other descriptorWithInputSize:hiddenSize:layerCount:usesBiases:batchFirst:isBidirectional:dropout:)]
        pub unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_batchFirst_isBidirectional_dropout(
            input_size: NSUInteger,
            hidden_size: NSUInteger,
            layer_count: NSUInteger,
            uses_biases: bool,
            batch_first: bool,
            is_bidirectional: bool,
            dropout: c_float,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other descriptorWithInputSize:hiddenSize:layerCount:usesBiases:batchFirst:isBidirectional:returnsSequences:dropout:)]
        pub unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_batchFirst_isBidirectional_returnsSequences_dropout(
            input_size: NSUInteger,
            hidden_size: NSUInteger,
            layer_count: NSUInteger,
            uses_biases: bool,
            batch_first: bool,
            is_bidirectional: bool,
            returns_sequences: bool,
            dropout: c_float,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other descriptorWithInputSize:hiddenSize:layerCount:usesBiases:batchFirst:isBidirectional:returnsSequences:dropout:resultMode:)]
        pub unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_batchFirst_isBidirectional_returnsSequences_dropout_resultMode(
            input_size: NSUInteger,
            hidden_size: NSUInteger,
            layer_count: NSUInteger,
            uses_biases: bool,
            batch_first: bool,
            is_bidirectional: bool,
            returns_sequences: bool,
            dropout: c_float,
            result_mode: MLCLSTMResultMode,
        ) -> Id<Self>;
    }
);
