//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSSpellServer;

    unsafe impl ClassType for NSSpellServer {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSSpellServer {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSpellServerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSpellServerDelegate>);

        #[method(registerLanguage:byVendor:)]
        pub unsafe fn registerLanguage_byVendor(
            &self,
            language: Option<&NSString>,
            vendor: Option<&NSString>,
        ) -> bool;

        #[method(isWordInUserDictionaries:caseSensitive:)]
        pub unsafe fn isWordInUserDictionaries_caseSensitive(
            &self,
            word: &NSString,
            flag: bool,
        ) -> bool;

        #[method(run)]
        pub unsafe fn run(&self);
    }
);

extern_static!(NSGrammarRange: &'static NSString);

extern_static!(NSGrammarUserDescription: &'static NSString);

extern_static!(NSGrammarCorrections: &'static NSString);

extern_protocol!(
    pub struct NSSpellServerDelegate;

    unsafe impl NSSpellServerDelegate {
        #[optional]
        #[method(spellServer:findMisspelledWordInString:language:wordCount:countOnly:)]
        pub unsafe fn spellServer_findMisspelledWordInString_language_wordCount_countOnly(
            &self,
            sender: &NSSpellServer,
            stringToCheck: &NSString,
            language: &NSString,
            wordCount: NonNull<NSInteger>,
            countOnly: bool,
        ) -> NSRange;

        #[optional]
        #[method_id(@__retain_semantics Other spellServer:suggestGuessesForWord:inLanguage:)]
        pub unsafe fn spellServer_suggestGuessesForWord_inLanguage(
            &self,
            sender: &NSSpellServer,
            word: &NSString,
            language: &NSString,
        ) -> Option<Id<NSArray<NSString>, Shared>>;

        #[optional]
        #[method(spellServer:didLearnWord:inLanguage:)]
        pub unsafe fn spellServer_didLearnWord_inLanguage(
            &self,
            sender: &NSSpellServer,
            word: &NSString,
            language: &NSString,
        );

        #[optional]
        #[method(spellServer:didForgetWord:inLanguage:)]
        pub unsafe fn spellServer_didForgetWord_inLanguage(
            &self,
            sender: &NSSpellServer,
            word: &NSString,
            language: &NSString,
        );

        #[optional]
        #[method_id(@__retain_semantics Other spellServer:suggestCompletionsForPartialWordRange:inString:language:)]
        pub unsafe fn spellServer_suggestCompletionsForPartialWordRange_inString_language(
            &self,
            sender: &NSSpellServer,
            range: NSRange,
            string: &NSString,
            language: &NSString,
        ) -> Option<Id<NSArray<NSString>, Shared>>;

        #[optional]
        #[method(spellServer:checkGrammarInString:language:details:)]
        pub unsafe fn spellServer_checkGrammarInString_language_details(
            &self,
            sender: &NSSpellServer,
            stringToCheck: &NSString,
            language: Option<&NSString>,
            details: *mut *mut NSArray<NSDictionary<NSString, Object>>,
        ) -> NSRange;

        #[optional]
        #[method_id(@__retain_semantics Other spellServer:checkString:offset:types:options:orthography:wordCount:)]
        pub unsafe fn spellServer_checkString_offset_types_options_orthography_wordCount(
            &self,
            sender: &NSSpellServer,
            stringToCheck: &NSString,
            offset: NSUInteger,
            checkingTypes: NSTextCheckingTypes,
            options: Option<&NSDictionary<NSString, Object>>,
            orthography: Option<&NSOrthography>,
            wordCount: NonNull<NSInteger>,
        ) -> Option<Id<NSArray<NSTextCheckingResult>, Shared>>;

        #[optional]
        #[method(spellServer:recordResponse:toCorrection:forWord:language:)]
        pub unsafe fn spellServer_recordResponse_toCorrection_forWord_language(
            &self,
            sender: &NSSpellServer,
            response: NSUInteger,
            correction: &NSString,
            word: &NSString,
            language: &NSString,
        );
    }
);
