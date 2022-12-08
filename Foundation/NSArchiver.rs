//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSArchiver;

    unsafe impl ClassType for NSArchiver {
        type Super = NSCoder;
    }
);

extern_methods!(
    unsafe impl NSArchiver {
        #[method_id(@__retain_semantics Init initForWritingWithMutableData:)]
        pub unsafe fn initForWritingWithMutableData(
            this: Option<Allocated<Self>>,
            mdata: &NSMutableData,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other archiverData)]
        pub unsafe fn archiverData(&self) -> Id<NSMutableData, Shared>;

        #[method(encodeRootObject:)]
        pub unsafe fn encodeRootObject(&self, rootObject: &Object);

        #[method(encodeConditionalObject:)]
        pub unsafe fn encodeConditionalObject(&self, object: Option<&Object>);

        #[method_id(@__retain_semantics Other archivedDataWithRootObject:)]
        pub unsafe fn archivedDataWithRootObject(rootObject: &Object) -> Id<NSData, Shared>;

        #[method(archiveRootObject:toFile:)]
        pub unsafe fn archiveRootObject_toFile(rootObject: &Object, path: &NSString) -> bool;

        #[method(encodeClassName:intoClassName:)]
        pub unsafe fn encodeClassName_intoClassName(
            &self,
            trueName: &NSString,
            inArchiveName: &NSString,
        );

        #[method_id(@__retain_semantics Other classNameEncodedForTrueClassName:)]
        pub unsafe fn classNameEncodedForTrueClassName(
            &self,
            trueName: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method(replaceObject:withObject:)]
        pub unsafe fn replaceObject_withObject(&self, object: &Object, newObject: &Object);
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSUnarchiver;

    unsafe impl ClassType for NSUnarchiver {
        type Super = NSCoder;
    }
);

extern_methods!(
    unsafe impl NSUnarchiver {
        #[method_id(@__retain_semantics Init initForReadingWithData:)]
        pub unsafe fn initForReadingWithData(
            this: Option<Allocated<Self>>,
            data: &NSData,
        ) -> Option<Id<Self, Shared>>;

        #[method(setObjectZone:)]
        pub unsafe fn setObjectZone(&self, zone: *mut NSZone);

        #[method(objectZone)]
        pub unsafe fn objectZone(&self) -> *mut NSZone;

        #[method(isAtEnd)]
        pub unsafe fn isAtEnd(&self) -> bool;

        #[method(systemVersion)]
        pub unsafe fn systemVersion(&self) -> c_uint;

        #[method_id(@__retain_semantics Other unarchiveObjectWithData:)]
        pub unsafe fn unarchiveObjectWithData(data: &NSData) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other unarchiveObjectWithFile:)]
        pub unsafe fn unarchiveObjectWithFile(path: &NSString) -> Option<Id<Object, Shared>>;

        #[method(replaceObject:withObject:)]
        pub unsafe fn replaceObject_withObject(&self, object: &Object, newObject: &Object);
    }
);

extern_methods!(
    /// NSArchiverCallback
    unsafe impl NSObject {
        #[method(classForArchiver)]
        pub unsafe fn classForArchiver(&self) -> Option<&'static Class>;

        #[method_id(@__retain_semantics Other replacementObjectForArchiver:)]
        pub unsafe fn replacementObjectForArchiver(
            &self,
            archiver: &NSArchiver,
        ) -> Option<Id<Object, Shared>>;
    }
);
