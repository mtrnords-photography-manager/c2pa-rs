// Copyright 2022 Adobe. All rights reserved.
// This file is licensed to you under the Apache License,
// Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// or the MIT license (http://opensource.org/licenses/MIT),
// at your option.

// Unless required by applicable law or agreed to in writing,
// this software is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR REPRESENTATIONS OF ANY KIND, either express or
// implied. See the LICENSE-MIT and LICENSE-APACHE files for the
// specific language governing permissions and limitations under
// each license.

use crate::Result;

/// The `Signer` trait generates a cryptographic signature over a byte array.
///
/// This trait exists to allow the signature mechanism to be extended.
pub trait Signer {
    /// Returns a new byte array which is a signature over the original.
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>>;

    /// Returns the algorithm of the Signer.
    fn alg(&self) -> Option<String>;

    /// Returns the certificates as a Vec containing a Vec of DER bytes for each certificate.
    fn certs(&self) -> Result<Vec<Vec<u8>>>;

    /// Returns the size in bytes of the largest possible expected signature.
    /// Signing will fail if the result of the `sign` function is larger
    /// than this value.
    fn reserve_size(&self) -> usize;

    /// URL for time authority to time stamp the signature
    fn time_authority_url(&self) -> Option<String> {
        None
    }

    /// OCSP response for the signing cert if available
    /// This is the only C2PA supported cert revocation method.
    /// By pre-querying the value for a your signing cert the value can
    /// be cached taking pressure off of the CA (recommended by C2PA spec)
    fn ocsp_val(&self) -> Option<Vec<u8>> {
        None
    }
}

/// Trait to allow loading of signing credential from external sources
pub trait ConfigurableSigner: Signer + Sized {
    /// Create signer form credential files
    fn from_files<P: AsRef<std::path::Path>>(
        signcert_path: P,
        pkey_path: P,
        alg: String,
        tsa_url: Option<String>,
    ) -> Result<Self>;

    /// Create signer from credentials data
    fn from_signcert_and_pkey(
        signcert: &[u8],
        pkey: &[u8],
        alg: String,
        tsa_url: Option<String>,
    ) -> Result<Self>;
}

/// The `Placeholder` implementation provides a placeholder "signer" for use
/// in testing and development contexts where a valid signature is not required.
/// To state the obvious, claims signed using this implementation will not verify.
pub struct Placeholder {}

impl Signer for Placeholder {
    // sign the provided bytes
    fn sign(&self, _data: &[u8]) -> Result<Vec<u8>> {
        Ok(b"invalid signature".to_vec())
    }

    // algoritim iddentifer string for this Signer
    fn alg(&self) -> Option<String> {
        None
    }

    // list of certificates in der format, with last being cert that signed the claim
    fn certs(&self) -> Result<Vec<Vec<u8>>> {
        Ok(Vec::new())
    }

    // bytes to reserve for a fully signed claim
    fn reserve_size(&self) -> usize {
        128
    }
}

#[cfg(feature = "async_signer")]
use async_trait::async_trait;

/// The `AsyncSigner` trait generates a cryptographic signature over a byte array.
///
/// This trait exists to allow the signature mechanism to be extended.
///
/// Use this when the implementation is asynchronous.
#[cfg(feature = "async_signer")]
#[async_trait]
pub trait AsyncSigner: Sync {
    /// Returns a new byte array which is a signature over the original.
    async fn sign(&self, data: &[u8]) -> Result<Vec<u8>>;

    /// Returns the size in bytes of the largest possible expected signature.
    /// Signing will fail if the result of the `sign` function is larger
    /// than this value.
    fn reserve_size(&self) -> usize;
}

/// The `AsyncPlaceholder` implementation provides a placeholder "async signer"
/// for use in testing and development contexts where a valid signature is not
/// required. To state the obvious, claims signed using this implementation
/// will not verify.
#[cfg(feature = "async_signer")]
pub struct AsyncPlaceholder {}

#[cfg(feature = "async_signer")]
#[async_trait]
impl AsyncSigner for AsyncPlaceholder {
    async fn sign(&self, _data: &[u8]) -> Result<Vec<u8>> {
        Ok(b"invalid signature".to_vec())
    }

    fn reserve_size(&self) -> usize {
        128
    }
}