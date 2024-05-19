//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    pub unsafe trait MEMessageEncoder: NSObjectProtocol {
        #[cfg(all(
            feature = "MEComposeContext",
            feature = "MEMessage",
            feature = "MEOutgoingMessageEncodingStatus",
            feature = "block2"
        ))]
        #[method(getEncodingStatusForMessage:composeContext:completionHandler:)]
        unsafe fn getEncodingStatusForMessage_composeContext_completionHandler(
            &self,
            message: &MEMessage,
            compose_context: &MEComposeContext,
            completion_handler: &block2::Block<dyn Fn(NonNull<MEOutgoingMessageEncodingStatus>)>,
        );

        #[cfg(all(
            feature = "MEComposeContext",
            feature = "MEMessage",
            feature = "MEMessageEncodingResult",
            feature = "block2"
        ))]
        #[method(encodeMessage:composeContext:completionHandler:)]
        unsafe fn encodeMessage_composeContext_completionHandler(
            &self,
            message: &MEMessage,
            compose_context: &MEComposeContext,
            completion_handler: &block2::Block<dyn Fn(NonNull<MEMessageEncodingResult>)>,
        );
    }

    unsafe impl ProtocolType for dyn MEMessageEncoder {}
);
