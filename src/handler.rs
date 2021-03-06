use crate::unid::utils::data_t::DataT;
use crate::allocator;
use cstr_core::c_char;

pub struct UNiDHandler {
    memory_alloc      : Option<extern "C" fn(u32) -> *mut allocator::c_void>,
    memory_dealloc    : Option<extern "C" fn(*mut allocator::c_void)>,
    debug_message     : Option<extern "C" fn(u32, *mut c_char)>,
    aes_encryptor     : Option<extern "C" fn(*mut DataT, *mut DataT, *mut DataT, *mut u8, u32)>,
    aes_decryptor     : Option<extern "C" fn(*mut DataT, *mut DataT, *mut DataT, *mut u8, u32)>,
    ecdsa_signer      : Option<extern "C" fn(*mut c_char, *mut u8, *mut c_char, *mut c_char)>,
    ecdsa_verifier    : Option<extern "C" fn(*mut c_char, *mut c_char, *mut c_char, *mut c_char, *mut DataT, *mut i32)>,
    crypto_trng       : Option<extern "C" fn(u32) -> *mut c_char>,
    https_post_request: Option<extern "C" fn(*mut c_char, *mut c_char, *mut c_char) -> *mut c_char>,
}

impl UNiDHandler {
    pub const fn new() -> UNiDHandler {
        UNiDHandler {
            memory_alloc      : None,
            memory_dealloc    : None,
            debug_message     : None,
            aes_encryptor     : None,
            aes_decryptor     : None,
            ecdsa_signer      : None,
            ecdsa_verifier    : None,
            crypto_trng       : None,
            https_post_request: None,
        }
    }

    // setter: memory_alloc_handler
    pub fn set_memory_alloc_handler(&mut self, handler: extern "C" fn(u32) -> *mut allocator::c_void) {
        self.memory_alloc = Some(handler)
    }

    // getter: memory_alloc_handler
    pub fn get_memory_alloc_handler(&self) -> Option<extern "C" fn(u32) -> *mut allocator::c_void> {
        self.memory_alloc
    }

    // setter: memory_dealloc_handler
    pub fn set_memory_dealloc_handler(&mut self, handler: extern "C" fn(*mut allocator::c_void)) {
        self.memory_dealloc = Some(handler)
    }

    // getter: memory_dealloc_handler
    pub fn get_memory_dealloc_handler(&self) -> Option<extern "C" fn(*mut allocator::c_void)> {
        self.memory_dealloc
    }

    // setter: aes_encryptor_handler
    pub fn set_aes_encryptor_handler(&mut self, handler: extern "C" fn(*mut DataT, *mut DataT, *mut DataT, *mut u8, u32)) {
        self.aes_encryptor = Some(handler)
    }

    // getter: aes_encryptor_handler
    pub fn get_aes_encryptor_handler(&self) -> Option<extern "C" fn(*mut DataT, *mut DataT, *mut DataT, *mut u8, u32)> {
        self.aes_encryptor
    }

    // setter: aes_decryptor_handler
    pub fn set_aes_decryptor_handler(&mut self, handler: extern "C" fn(*mut DataT, *mut DataT, *mut DataT, *mut u8, u32)) {
        self.aes_decryptor = Some(handler)
    }

    // getter: aes_decryptor_handler
    pub fn get_aes_decryptor_handler(&self) -> Option<extern "C" fn(*mut DataT, *mut DataT, *mut DataT, *mut u8, u32)> {
        self.aes_decryptor
    }

    // setter: ecdsa_signer_handler
    pub fn set_ecdsa_signer_handler(&mut self, handler: extern "C" fn(*mut c_char, *mut u8, *mut c_char, *mut c_char)) {
        self.ecdsa_signer = Some(handler)
    }

    // getter: ecdsa_signer_handler
    pub fn get_ecdsa_signer_handler(&self) -> Option<extern "C" fn(*mut c_char, *mut u8, *mut c_char, *mut c_char)> {
        self.ecdsa_signer
    }

    // setter: ecdsa_verifier_handler
    pub fn set_ecdsa_verifier_handler(&mut self, handler: extern "C" fn(*mut c_char, *mut c_char, *mut c_char, *mut c_char, *mut DataT, *mut i32)) {
        self.ecdsa_verifier = Some(handler)
    }

    // getter: ecdsa_verifier_handler
    pub fn get_ecdsa_verifier_handler(&self) -> Option<extern "C" fn(*mut c_char, *mut c_char, *mut c_char, *mut c_char, *mut DataT, *mut i32)> {
        self.ecdsa_verifier
    }

    // setter: debug_message_handler
    pub fn set_debug_message_handler(&mut self, handler: extern "C" fn(u32, *mut c_char)) {
        self.debug_message = Some(handler)
    }

    // getter: debug_message_handler
    pub fn get_debug_message_handler(&self) -> Option<extern "C" fn(u32, *mut c_char)> {
        self.debug_message
    }

    // setter: crypto_trng
    pub fn set_crypto_trng(&mut self, handler: extern "C" fn(u32) -> *mut c_char) {
        self.crypto_trng = Some(handler)
    }

    // getter: crypto_trng
    pub fn get_crypto_trng(&self) -> Option<extern "C" fn(u32) -> *mut c_char> {
        self.crypto_trng
    }

    // setter: https_get_request
    pub fn set_https_post_request(&mut self, handler: extern "C" fn(*mut c_char, *mut c_char, *mut c_char) -> *mut c_char) {
        self.https_post_request = Some(handler)
    }

    // getter: https_post_request
    pub fn get_https_post_request(&self) -> Option<extern "C" fn(*mut c_char, *mut c_char, *mut c_char) -> *mut c_char> {
        self.https_post_request
    }
}