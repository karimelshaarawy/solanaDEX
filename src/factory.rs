use std::collections::HashMap;
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest, Sha256VarCore};
use sha2::digest::core_api::{CoreWrapper, CtVariableCoreWrapper};
use sha2::digest::generic_array::GenericArray;
use sha2::digest::Output;
use sha2::digest::typenum::bit::{B0, B1};
use generic_array::typenum::U5;

pub struct factory<'a>{
    pair_addresses :HashMap<&'a str,GenericArray<u8,U5>>,
    fee_to:&'a str,
    fee_to_setter:&'a str,

}

impl<'a> factory<'a>{
    // fn create_pair(&mut self, token_A:&str, token_B:&str) ->GenericArray<u8,U5> {
    //     assert!(token_A != token_B,"IDENTICAL_ADDRESSES");
    //     let key: &mut str;
    //     if token_A<token_B {
    //         key = &mut *[token_A, token_B].join("\n");
    //     }
    //      else {
    //          key =  &mut *[token_B, token_A].join("\n");
    //      }
    //     assert!(!self.pair_addresses.contains_key(key),"Already exists");
    //     let mut hasher = Sha256::new();
    //     hasher.update(key);
    //     let result = hasher.finalize() ;
    //     self.pair_addresses.insert(key,result);
    //     result
    // }

    fn set_fee_to (&mut self, fee_to:&'a str){
        self.fee_to= fee_to;
    }

    fn set_fee_to_setter(&mut self, fee_to_setter:&'a str){
        self.fee_to_setter= fee_to_setter;
    }
}
