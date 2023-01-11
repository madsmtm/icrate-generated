//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSOperationQueuePriority {
        NSOperationQueuePriorityVeryLow = -8,
        NSOperationQueuePriorityLow = -4,
        NSOperationQueuePriorityNormal = 0,
        NSOperationQueuePriorityHigh = 4,
        NSOperationQueuePriorityVeryHigh = 8,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSOperation")]
    pub struct NSOperation;

    #[cfg(feature = "Foundation_NSOperation")]
    unsafe impl ClassType for NSOperation {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSOperation")]
    unsafe impl NSOperation {
        #[method(start)]
        pub unsafe fn start(&self);

        #[method(main)]
        pub unsafe fn main(&self);

        #[method(isCancelled)]
        pub unsafe fn isCancelled(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(isExecuting)]
        pub unsafe fn isExecuting(&self) -> bool;

        #[method(isFinished)]
        pub unsafe fn isFinished(&self) -> bool;

        #[method(isConcurrent)]
        pub unsafe fn isConcurrent(&self) -> bool;

        #[method(isAsynchronous)]
        pub unsafe fn isAsynchronous(&self) -> bool;

        #[method(isReady)]
        pub unsafe fn isReady(&self) -> bool;

        #[method(addDependency:)]
        pub unsafe fn addDependency(&self, op: &NSOperation);

        #[method(removeDependency:)]
        pub unsafe fn removeDependency(&self, op: &NSOperation);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other dependencies)]
        pub unsafe fn dependencies(&self) -> Id<NSArray<NSOperation>, Shared>;

        #[method(queuePriority)]
        pub unsafe fn queuePriority(&self) -> NSOperationQueuePriority;

        #[method(setQueuePriority:)]
        pub unsafe fn setQueuePriority(&self, queuePriority: NSOperationQueuePriority);

        #[method(completionBlock)]
        pub unsafe fn completionBlock(&self) -> *mut Block<(), ()>;

        #[method(setCompletionBlock:)]
        pub unsafe fn setCompletionBlock(&self, completionBlock: Option<&Block<(), ()>>);

        #[method(waitUntilFinished)]
        pub unsafe fn waitUntilFinished(&self);

        #[method(threadPriority)]
        pub unsafe fn threadPriority(&self) -> c_double;

        #[method(setThreadPriority:)]
        pub unsafe fn setThreadPriority(&self, threadPriority: c_double);

        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSBlockOperation")]
    pub struct NSBlockOperation;

    #[cfg(feature = "Foundation_NSBlockOperation")]
    unsafe impl ClassType for NSBlockOperation {
        #[inherits(NSObject)]
        type Super = NSOperation;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSBlockOperation")]
    unsafe impl NSBlockOperation {
        #[method_id(@__retain_semantics Other blockOperationWithBlock:)]
        pub unsafe fn blockOperationWithBlock(block: &Block<(), ()>) -> Id<Self, Shared>;

        #[method(addExecutionBlock:)]
        pub unsafe fn addExecutionBlock(&self, block: &Block<(), ()>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSInvocationOperation")]
    pub struct NSInvocationOperation;

    #[cfg(feature = "Foundation_NSInvocationOperation")]
    unsafe impl ClassType for NSInvocationOperation {
        #[inherits(NSObject)]
        type Super = NSOperation;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSInvocationOperation")]
    unsafe impl NSInvocationOperation {
        #[method_id(@__retain_semantics Init initWithTarget:selector:object:)]
        pub unsafe fn initWithTarget_selector_object(
            this: Option<Allocated<Self>>,
            target: &Object,
            sel: Sel,
            arg: Option<&Object>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSInvocation")]
        #[method_id(@__retain_semantics Init initWithInvocation:)]
        pub unsafe fn initWithInvocation(
            this: Option<Allocated<Self>>,
            inv: &NSInvocation,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSInvocation")]
        #[method_id(@__retain_semantics Other invocation)]
        pub unsafe fn invocation(&self) -> Id<NSInvocation, Shared>;

        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;
    }
);

extern_static!(NSInvocationOperationVoidResultException: &'static NSExceptionName);

extern_static!(NSInvocationOperationCancelledException: &'static NSExceptionName);

extern_static!(NSOperationQueueDefaultMaxConcurrentOperationCount: NSInteger = -1);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSOperationQueue")]
    pub struct NSOperationQueue;

    #[cfg(feature = "Foundation_NSOperationQueue")]
    unsafe impl ClassType for NSOperationQueue {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSOperationQueue")]
    unsafe impl NSOperationQueue {
        #[cfg(feature = "Foundation_NSProgress")]
        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Id<NSProgress, Shared>;

        #[cfg(feature = "Foundation_NSOperation")]
        #[method(addOperation:)]
        pub unsafe fn addOperation(&self, op: &NSOperation);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSOperation"))]
        #[method(addOperations:waitUntilFinished:)]
        pub unsafe fn addOperations_waitUntilFinished(
            &self,
            ops: &NSArray<NSOperation>,
            wait: bool,
        );

        #[method(addOperationWithBlock:)]
        pub unsafe fn addOperationWithBlock(&self, block: &Block<(), ()>);

        #[method(addBarrierBlock:)]
        pub unsafe fn addBarrierBlock(&self, barrier: &Block<(), ()>);

        #[method(maxConcurrentOperationCount)]
        pub unsafe fn maxConcurrentOperationCount(&self) -> NSInteger;

        #[method(setMaxConcurrentOperationCount:)]
        pub unsafe fn setMaxConcurrentOperationCount(&self, maxConcurrentOperationCount: NSInteger);

        #[method(isSuspended)]
        pub unsafe fn isSuspended(&self) -> bool;

        #[method(setSuspended:)]
        pub unsafe fn setSuspended(&self, suspended: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService);

        #[method(cancelAllOperations)]
        pub unsafe fn cancelAllOperations(&self);

        #[method(waitUntilAllOperationsAreFinished)]
        pub unsafe fn waitUntilAllOperationsAreFinished(&self);

        #[method_id(@__retain_semantics Other currentQueue)]
        pub unsafe fn currentQueue() -> Option<Id<NSOperationQueue, Shared>>;

        #[method_id(@__retain_semantics Other mainQueue)]
        pub unsafe fn mainQueue() -> Id<NSOperationQueue, Shared>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSOperationQueue")]
    unsafe impl NSOperationQueue {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSOperation"))]
        #[method_id(@__retain_semantics Other operations)]
        pub unsafe fn operations(&self) -> Id<NSArray<NSOperation>, Shared>;

        #[method(operationCount)]
        pub unsafe fn operationCount(&self) -> NSUInteger;
    }
);
