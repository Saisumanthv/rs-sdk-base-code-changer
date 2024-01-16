use super::{DctTokenPayment, ManagedVec};
use crate::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
        CodecFromSelf,
    },
    types::BigUint,
};

use crate as dharitri_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

/// Encodes any type of payment, which either:
/// - MOA (can be zero in case of no payment whatsoever);
/// - Multi-DCT (one or more DCT transfers).
#[derive(
    TopDecode, TopEncode, TypeAbi, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug,
)]
pub enum MoaOrMultiDctPayment<M: ManagedTypeApi> {
    Moa(BigUint<M>),
    MultiDct(ManagedVec<M, DctTokenPayment<M>>),
}

impl<M> CodecFromSelf for MoaOrMultiDctPayment<M> where M: ManagedTypeApi {}
