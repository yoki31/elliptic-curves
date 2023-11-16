#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"
)]
#![forbid(unsafe_code)]
#![warn(
    clippy::mod_module_files,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::checked_conversions,
    clippy::implicit_saturating_sub,
    clippy::integer_arithmetic,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

#[cfg(feature = "alloc")]
#[allow(unused_extern_crates)]
extern crate alloc;

#[cfg(feature = "arithmetic")]
mod arithmetic;

#[cfg(any(feature = "test-vectors", test))]
pub mod test_vectors;

pub use elliptic_curve::{self, bigint::U256};

#[cfg(feature = "arithmetic")]
pub use arithmetic::{scalar::Scalar, AffinePoint, ProjectivePoint};

#[cfg(feature = "pkcs8")]
pub use elliptic_curve::pkcs8;

use elliptic_curve::{
    bigint::ArrayEncoding,
    consts::{U32, U33},
    generic_array::GenericArray,
    FieldBytesEncoding,
};

/// Order of BIGN P-256's elliptic curve group (i.e. scalar modulus) in hexadecimal.
const ORDER_HEX: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFD95C8ED60DFB4DFC7E5ABF99263D6607";

/// BIGN P-256 elliptic curve.
///
/// This curve is also known as bign-curve256v1
/// and is specified in [STB 34.101.45-2013]:
/// Recommendations for Discrete Logarithm-based Cryptography:
/// Elliptic Curve Domain Parameters.
///
///
/// Its equation is `y² = x³ + ax + b` over a ~256-bit prime field.
///
/// ```text
/// a = 115792089237316195423570985008687907853269984665640564039457584007913129639744
/// b = 54189945433829174764701416670523239872420438478408031144987871676190519198705
/// ```
///
/// [STB 34.101.45-2013]: https://apmi.bsu.by/assets/files/std/bign-spec294.pdf
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct BignP256;

impl elliptic_curve::Curve for BignP256 {
    /// 256-bit integer type used for internally representing field elements.
    type FieldBytesSize = U32;
    type Uint = U256;

    /// Order of BIGN P-256's elliptic curve group (i.e. scalar modulus).
    const ORDER: U256 = U256::from_be_hex(ORDER_HEX);
}

impl elliptic_curve::PrimeCurve for BignP256 {}

impl elliptic_curve::point::PointCompression for BignP256 {
    /// BIGN P-256 points are typically uncompressed.
    const COMPRESS_POINTS: bool = false;
}

impl elliptic_curve::point::PointCompaction for BignP256 {
    /// BIGN P-256 points are typically uncompressed.
    const COMPACT_POINTS: bool = false;
}

#[cfg(feature = "pkcs8")]
impl pkcs8::AssociatedOid for BignP256 {
    const OID: pkcs8::ObjectIdentifier =
        pkcs8::ObjectIdentifier::new_unwrap("1.2.112.0.2.0.34.101.45.1");
}

/// Compressed SEC1-encoded BIGN P256 curve point.
pub type CompressedPoint = GenericArray<u8, U33>;

/// BIGN P-256 field element serialized as bytes.
///
/// Byte array containing a serialized field element value (base field or scalar).
pub type FieldBytes = elliptic_curve::FieldBytes<BignP256>;

/// SEC1 encoded point.
pub type EncodedPoint = elliptic_curve::sec1::EncodedPoint<BignP256>;

impl FieldBytesEncoding<BignP256> for U256 {
    fn decode_field_bytes(field_bytes: &FieldBytes) -> Self {
        U256::from_be_byte_array(*field_bytes)
    }

    fn encode_field_bytes(&self) -> FieldBytes {
        self.to_be_byte_array()
    }
}

/// Non-zero scalar field element.
#[cfg(feature = "arithmetic")]
pub type NonZeroScalar = elliptic_curve::NonZeroScalar<BignP256>;

/// BIGN P-256 public key.
#[cfg(feature = "arithmetic")]
pub type PublicKey = elliptic_curve::PublicKey<BignP256>;

/// BIGN P-256 secret key.
pub type SecretKey = elliptic_curve::SecretKey<BignP256>;

#[cfg(not(feature = "arithmetic"))]
impl elliptic_curve::sec1::ValidatePublicKey for BignP256 {}

/// Bit representation of a BIGN P-256 scalar field element.
#[cfg(feature = "bits")]
pub type ScalarBits = elliptic_curve::ScalarBits<BignP256>;
