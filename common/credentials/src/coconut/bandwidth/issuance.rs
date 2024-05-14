// Copyright 2024 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::coconut::bandwidth::freepass::FreePassIssuanceData;
use crate::coconut::bandwidth::issued::IssuedBandwidthCredential;
use crate::coconut::bandwidth::voucher::BandwidthVoucherIssuanceData;
use crate::coconut::bandwidth::{
    bandwidth_credential_params, CredentialSigningData, CredentialType,
};
use crate::coconut::utils::scalar_serde_helper;
use crate::error::Error;
use nym_credentials_interface::{
    aggregate_signature_shares, aggregate_signature_shares_and_verify, hash_to_scalar,
    prepare_blind_sign, Attribute, BlindedSerialNumber, BlindedSignature, Parameters,
    PrivateAttribute, PublicAttribute, Signature, SignatureShare, VerificationKey,
};
use nym_crypto::asymmetric::{encryption, identity};
use nym_validator_client::nym_api::EpochId;
use nym_validator_client::signing::AccountData;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub use nym_validator_client::nyxd::{Coin, Hash};

#[derive(Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub enum BandwidthCredentialIssuanceDataVariant {
    Voucher(BandwidthVoucherIssuanceData),
    FreePass(FreePassIssuanceData),
}

impl From<FreePassIssuanceData> for BandwidthCredentialIssuanceDataVariant {
    fn from(value: FreePassIssuanceData) -> Self {
        BandwidthCredentialIssuanceDataVariant::FreePass(value)
    }
}

impl From<BandwidthVoucherIssuanceData> for BandwidthCredentialIssuanceDataVariant {
    fn from(value: BandwidthVoucherIssuanceData) -> Self {
        BandwidthCredentialIssuanceDataVariant::Voucher(value)
    }
}

impl BandwidthCredentialIssuanceDataVariant {
    pub fn info(&self) -> CredentialType {
        match self {
            BandwidthCredentialIssuanceDataVariant::Voucher(..) => CredentialType::Voucher,
            BandwidthCredentialIssuanceDataVariant::FreePass(..) => CredentialType::FreePass,
        }
    }

    // currently this works under the assumption of there being a single unique public attribute for given variant
    pub fn public_value(&self) -> &Attribute {
        match self {
            BandwidthCredentialIssuanceDataVariant::Voucher(voucher) => voucher.value_attribute(),
            BandwidthCredentialIssuanceDataVariant::FreePass(freepass) => {
                freepass.expiry_date_attribute()
            }
        }
    }

    // currently this works under the assumption of there being a single unique public attribute for given variant
    pub fn public_value_plain(&self) -> String {
        match self {
            BandwidthCredentialIssuanceDataVariant::Voucher(voucher) => voucher.value_plain(),
            BandwidthCredentialIssuanceDataVariant::FreePass(freepass) => {
                freepass.expiry_date_plain()
            }
        }
    }

    pub fn voucher_data(&self) -> Option<&BandwidthVoucherIssuanceData> {
        match self {
            BandwidthCredentialIssuanceDataVariant::Voucher(voucher) => Some(voucher),
            _ => None,
        }
    }
}

// all types of bandwidth credentials contain serial number and binding number
#[derive(Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct IssuanceBandwidthCredential {
    // private attributes
    /// a random secret value generated by the client used for double-spending detection
    #[serde(with = "scalar_serde_helper")]
    serial_number: PrivateAttribute,

    /// a random secret value generated by the client used to bind multiple credentials together
    #[serde(with = "scalar_serde_helper")]
    binding_number: PrivateAttribute,

    /// data specific to given bandwidth credential, for example a value for bandwidth voucher and expiry date for the free pass
    variant_data: BandwidthCredentialIssuanceDataVariant,

    /// type of the bandwdith credential hashed onto a scalar
    #[serde(with = "scalar_serde_helper")]
    type_prehashed: PublicAttribute,
}

impl IssuanceBandwidthCredential {
    pub const PUBLIC_ATTRIBUTES: u32 = 2;
    pub const PRIVATE_ATTRIBUTES: u32 = 2;
    pub const ENCODED_ATTRIBUTES: u32 = Self::PUBLIC_ATTRIBUTES + Self::PRIVATE_ATTRIBUTES;

    pub fn default_parameters() -> Parameters {
        // safety: the unwrap is fine here as Self::ENCODED_ATTRIBUTES is non-zero
        Parameters::new(Self::ENCODED_ATTRIBUTES).unwrap()
    }

    pub fn new<B: Into<BandwidthCredentialIssuanceDataVariant>>(variant_data: B) -> Self {
        let variant_data = variant_data.into();
        let type_prehashed = hash_to_scalar(variant_data.info().to_string());

        let params = bandwidth_credential_params();
        let serial_number = params.random_scalar();
        let binding_number = params.random_scalar();

        IssuanceBandwidthCredential {
            serial_number,
            binding_number,
            variant_data,
            type_prehashed,
        }
    }

    pub fn new_voucher(
        value: impl Into<Coin>,
        deposit_tx_hash: Hash,
        signing_key: identity::PrivateKey,
        unused_ed25519: encryption::PrivateKey,
    ) -> Self {
        Self::new(BandwidthVoucherIssuanceData::new(
            value,
            deposit_tx_hash,
            signing_key,
            unused_ed25519,
        ))
    }

    pub fn new_freepass(expiry_date: Option<OffsetDateTime>) -> Self {
        Self::new(FreePassIssuanceData::new(expiry_date))
    }

    pub fn blind_serial_number(&self) -> BlindedSerialNumber {
        (bandwidth_credential_params().gen2() * self.serial_number).into()
    }

    pub fn blinded_serial_number_bs58(&self) -> String {
        use nym_credentials_interface::Base58;

        self.blind_serial_number().to_bs58()
    }

    pub fn typ(&self) -> CredentialType {
        self.variant_data.info()
    }

    pub fn get_private_attributes(&self) -> Vec<&PrivateAttribute> {
        vec![&self.serial_number, &self.binding_number]
    }

    pub fn get_public_attributes(&self) -> Vec<&PublicAttribute> {
        vec![self.variant_data.public_value(), &self.type_prehashed]
    }

    pub fn get_plain_public_attributes(&self) -> Vec<String> {
        vec![
            self.variant_data.public_value_plain(),
            self.typ().to_string(),
        ]
    }

    pub fn get_variant_data(&self) -> &BandwidthCredentialIssuanceDataVariant {
        &self.variant_data
    }

    pub fn get_bandwidth_attribute(&self) -> String {
        self.variant_data.public_value_plain()
    }

    pub fn prepare_for_signing(&self) -> CredentialSigningData {
        let params = bandwidth_credential_params();

        // safety: the creation of the request can only fail if one provided invalid parameters
        // and we created then specific to this type of the credential so the unwrap is fine
        let (pedersen_commitments_openings, blind_sign_request) = prepare_blind_sign(
            params,
            &[&self.serial_number, &self.binding_number],
            &self.get_public_attributes(),
        )
        .unwrap();

        CredentialSigningData {
            pedersen_commitments_openings,
            blind_sign_request,
            public_attributes_plain: self.get_plain_public_attributes(),
            typ: self.typ(),
        }
    }

    pub fn unblind_signature(
        &self,
        validator_vk: &VerificationKey,
        signing_data: &CredentialSigningData,
        blinded_signature: BlindedSignature,
    ) -> Result<Signature, Error> {
        let public_attributes = self.get_public_attributes();
        let private_attributes = self.get_private_attributes();

        let params = bandwidth_credential_params();
        let unblinded_signature = blinded_signature.unblind_and_verify(
            params,
            validator_vk,
            &private_attributes,
            &public_attributes,
            &signing_data.blind_sign_request.get_commitment_hash(),
            &signing_data.pedersen_commitments_openings,
        )?;

        Ok(unblinded_signature)
    }

    pub async fn obtain_partial_freepass_credential(
        &self,
        client: &nym_validator_client::client::NymApiClient,
        account_data: &AccountData,
        validator_vk: &VerificationKey,
        signing_data: impl Into<Option<CredentialSigningData>>,
    ) -> Result<Signature, Error> {
        // if we provided signing data, do use them, otherwise generate fresh data
        let signing_data = signing_data
            .into()
            .unwrap_or_else(|| self.prepare_for_signing());

        let blinded_signature = match &self.variant_data {
            BandwidthCredentialIssuanceDataVariant::FreePass(freepass) => {
                freepass
                    .request_blinded_credential(&signing_data, account_data, client)
                    .await?
            }
            _ => return Err(Error::NotAFreePass),
        };
        self.unblind_signature(validator_vk, &signing_data, blinded_signature)
    }

    // ideally this would have been generic over credential type, but we really don't need secp256k1 keys for bandwidth vouchers
    pub async fn obtain_partial_bandwidth_voucher_credential(
        &self,
        client: &nym_validator_client::client::NymApiClient,
        validator_vk: &VerificationKey,
        signing_data: impl Into<Option<CredentialSigningData>>,
    ) -> Result<Signature, Error> {
        // if we provided signing data, do use them, otherwise generate fresh data
        let signing_data = signing_data
            .into()
            .unwrap_or_else(|| self.prepare_for_signing());

        let blinded_signature = match &self.variant_data {
            BandwidthCredentialIssuanceDataVariant::Voucher(voucher) => {
                // TODO: the request can be re-used between different apis
                let request = voucher.create_blind_sign_request_body(&signing_data);
                voucher.obtain_blinded_credential(client, &request).await?
            }
            _ => return Err(Error::NotABandwdithVoucher),
        };
        self.unblind_signature(validator_vk, &signing_data, blinded_signature)
    }

    pub fn unchecked_aggregate_signature_shares(
        &self,
        shares: &[SignatureShare],
    ) -> Result<Signature, Error> {
        aggregate_signature_shares(shares).map_err(Error::SignatureAggregationError)
    }

    pub fn aggregate_signature_shares(
        &self,
        verification_key: &VerificationKey,
        shares: &[SignatureShare],
    ) -> Result<Signature, Error> {
        let public_attributes = self.get_public_attributes();
        let private_attributes = self.get_private_attributes();

        let params = bandwidth_credential_params();

        let mut attributes = Vec::with_capacity(private_attributes.len() + public_attributes.len());
        attributes.extend_from_slice(&private_attributes);
        attributes.extend_from_slice(&public_attributes);

        aggregate_signature_shares_and_verify(params, verification_key, &attributes, shares)
            .map_err(Error::SignatureAggregationError)
    }

    // also drops self after the conversion
    pub fn into_issued_credential(
        self,
        aggregate_signature: Signature,
        epoch_id: EpochId,
    ) -> IssuedBandwidthCredential {
        self.to_issued_credential(aggregate_signature, epoch_id)
    }

    pub fn to_issued_credential(
        &self,
        aggregate_signature: Signature,
        epoch_id: EpochId,
    ) -> IssuedBandwidthCredential {
        IssuedBandwidthCredential::new(
            self.serial_number,
            self.binding_number,
            aggregate_signature,
            (&self.variant_data).into(),
            self.type_prehashed,
            epoch_id,
        )
    }

    // TODO: is that actually needed?
    pub fn to_recovery_bytes(&self) -> Vec<u8> {
        use bincode::Options;
        // safety: our data format is stable and thus the serialization should not fail
        make_recovery_bincode_serializer().serialize(self).unwrap()
    }

    // TODO: is that actually needed?
    // idea: make it consistent with the issued credential and its vX serde
    pub fn try_from_recovered_bytes(bytes: &[u8]) -> Result<Self, Error> {
        use bincode::Options;
        make_recovery_bincode_serializer()
            .deserialize(bytes)
            .map_err(|source| Error::RecoveryCredentialDeserializationFailure { source })
    }
}

fn make_recovery_bincode_serializer() -> impl bincode::Options {
    use bincode::Options;
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_varint_encoding()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_zeroize_on_drop<T: ZeroizeOnDrop>() {}

    fn assert_zeroize<T: Zeroize>() {}

    #[test]
    fn credential_is_zeroized() {
        assert_zeroize::<IssuanceBandwidthCredential>();
        assert_zeroize_on_drop::<IssuanceBandwidthCredential>();
    }
}
