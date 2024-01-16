use dharitri_sc::types::{
    BoxedBytes, MoaOrDctTokenIdentifier, MoaOrDctTokenPayment, DctTokenPayment, ManagedBuffer,
    TokenIdentifier,
};
use dharitri_sc_scenario::{
    api::StaticApi, managed_moa_token_id, managed_test_util::check_managed_top_encode_decode,
    managed_token_id, managed_token_id_wrapped, dharitri_sc,
};

#[test]
fn test_moa() {
    assert!(MoaOrDctTokenIdentifier::<StaticApi>::moa().is_moa());
}

#[test]
fn test_codec() {
    check_managed_top_encode_decode(
        MoaOrDctTokenIdentifier::<StaticApi>::moa(),
        MoaOrDctTokenIdentifier::<StaticApi>::MOA_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &MoaOrDctTokenIdentifier::<StaticApi>::MOA_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        vec![MoaOrDctTokenIdentifier::<StaticApi>::moa()],
        expected.as_slice(),
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_dct_identifier() {
    // valid identifier
    assert!(TokenIdentifier::<StaticApi>::from("ALC-6258d2").is_valid_dct_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::<StaticApi>::from("ALC123-6258d2").is_valid_dct_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::<StaticApi>::from("12345-6258d2").is_valid_dct_identifier());

    // missing dash
    assert!(!TokenIdentifier::<StaticApi>::from("ALC6258d2").is_valid_dct_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::<StaticApi>::from("AL-C6258d2").is_valid_dct_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::<StaticApi>::from("alc-6258d2").is_valid_dct_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::<StaticApi>::from("ALC-6258D2").is_valid_dct_identifier());

    // too many random chars
    assert!(!TokenIdentifier::<StaticApi>::from("ALC-6258d2ff").is_valid_dct_identifier());

    // ticker too short
    assert!(!TokenIdentifier::<StaticApi>::from("AL-6258d2").is_valid_dct_identifier());

    // ticker too long
    assert!(!TokenIdentifier::<StaticApi>::from("ALCCCCCCCCC-6258d2").is_valid_dct_identifier());
}

#[test]
#[rustfmt::skip]
fn test_ticker() {
    // valid identifier
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // valid identifier with numbers in ticker
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC123-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC123"),
    );

    // valid ticker only numbers
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("12345-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("12345"),
    );

    // missing dash
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // wrong dash position
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("AL-C6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL-"),
    );

    // lowercase ticker
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("alc-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("alc"),
    );

    // uppercase random chars
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC-6258D2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // too many random chars
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC-6258d2ff").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC-6"),
    );

    // ticker too short
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("AL-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // ticker too long
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALCCCCCCCCC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALCCCCCCCCC"),
    );
}

#[test]
fn test_is_valid_moa_or_dct() {
    // moa is always valid
    assert!(MoaOrDctTokenIdentifier::<StaticApi>::moa().is_valid());

    // valid dct
    assert!(
        MoaOrDctTokenIdentifier::<StaticApi>::dct(TokenIdentifier::from("ALC-6258d2"))
            .is_valid()
    );

    // invalid dct, see above
    assert!(
        !MoaOrDctTokenIdentifier::<StaticApi>::dct(TokenIdentifier::from("ALCCCCCCCCC-6258d2"))
            .is_valid()
    );
}

#[test]
fn test_token_identifier_eq() {
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("DCT-00000"),
        TokenIdentifier::<StaticApi>::from("DCT-00000")
    );
    assert_ne!(
        TokenIdentifier::<StaticApi>::from("DCT-00001"),
        TokenIdentifier::<StaticApi>::from("DCT-00002")
    );

    assert_eq!(
        MoaOrDctTokenIdentifier::<StaticApi>::dct(TokenIdentifier::from("DCT-00003")),
        TokenIdentifier::<StaticApi>::from("DCT-00003")
    );
    assert_ne!(
        MoaOrDctTokenIdentifier::<StaticApi>::moa(),
        TokenIdentifier::<StaticApi>::from("ANYTHING-1234")
    );
    assert_ne!(
        MoaOrDctTokenIdentifier::<StaticApi>::moa(),
        TokenIdentifier::<StaticApi>::from("MOA")
    );
}

#[test]
fn test_payment_eq() {
    assert_eq!(
        DctTokenPayment::<StaticApi>::new("PAY-00000".into(), 0, 1000u32.into()),
        DctTokenPayment::<StaticApi>::new("PAY-00000".into(), 0, 1000u32.into()),
    );
    assert_ne!(
        DctTokenPayment::<StaticApi>::new("PAY-00001".into(), 0, 1000u32.into()),
        DctTokenPayment::<StaticApi>::new("PAY-00002".into(), 0, 1000u32.into()),
    );
    assert_eq!(
        MoaOrDctTokenPayment::<StaticApi>::no_payment(),
        MoaOrDctTokenPayment::<StaticApi>::no_payment(),
    );
    assert_eq!(
        MoaOrDctTokenPayment::<StaticApi>::new(
            MoaOrDctTokenIdentifier::dct("DCTPAY-00000"),
            0,
            1000u32.into()
        ),
        MoaOrDctTokenPayment::<StaticApi>::new(
            MoaOrDctTokenIdentifier::dct("DCTPAY-00000"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        MoaOrDctTokenPayment::<StaticApi>::new(
            MoaOrDctTokenIdentifier::dct("DCTPAY-00001"),
            0,
            1000u32.into()
        ),
        MoaOrDctTokenPayment::<StaticApi>::new(
            MoaOrDctTokenIdentifier::dct("DCTPAY-00002"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        MoaOrDctTokenPayment::<StaticApi>::new(
            MoaOrDctTokenIdentifier::dct("DCTPAY-00001"),
            0,
            1000u32.into()
        ),
        MoaOrDctTokenPayment::<StaticApi>::no_payment(),
    );
}

#[test]
fn test_managed_token_id_macro() {
    assert_eq!(
        managed_moa_token_id!(),
        MoaOrDctTokenIdentifier::<StaticApi>::moa()
    );
    assert_eq!(
        managed_token_id!(b"ALC-6258d2"),
        TokenIdentifier::<StaticApi>::from("ALC-6258d2")
    );
    assert_eq!(
        managed_token_id_wrapped!(b"ALC-6258d2").unwrap_dct(),
        TokenIdentifier::<StaticApi>::from("ALC-6258d2")
    )
}
