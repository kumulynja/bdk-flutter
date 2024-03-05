use crate::api::descriptor::DescriptorBase;
use crate::api::types::{AddressBase, AddressIndex, AddressInfo, Balance, ChangeSpendPolicy, DatabaseConfig, Input, KeychainKind, LocalUtxo, Network, OutPoint, PsbtSigHashType, RbfValue, ScriptAmount, ScriptBufBase, SignOptions, TransactionDetails};
use std::ops::Deref;
use std::str::FromStr;

use crate::api::blockchain::BlockchainBase;
use crate::api::error::BdkError;
use crate::api::psbt::PsbtBase;
use bdk::bitcoin::{Sequence, Txid};
pub use bdk::database::any::AnyDatabase;
use bdk::database::ConfigurableDatabase;
pub use std::sync::Mutex;
use std::sync::MutexGuard;
use bdk::bitcoin::script::PushBytesBuf;
use crate::frb_generated::RustOpaque;

/// A Bitcoin wallet.
/// The Wallet acts as a way of coherently interfacing with output descriptors and related transactions. Its main components are:
///     1. Output descriptors from which it can derive addresses.
///     2. A Database where it tracks transactions and utxos related to the descriptors.
///     3. Signers that can contribute signatures to addresses instantiated from the descriptors.
#[derive(Debug)]
pub struct WalletBase {
    pub ptr: RustOpaque<Mutex<bdk::Wallet<AnyDatabase>>>,
}
impl WalletBase {
    pub fn new(
        descriptor: DescriptorBase,
        change_descriptor: Option<DescriptorBase>,
        network: Network,
        database_config: DatabaseConfig,
    ) -> Result<Self, BdkError> {
        let database = AnyDatabase::from_config(&database_config.into())?;
        let descriptor: String = descriptor.as_string_private();
        let change_descriptor: Option<String> = change_descriptor.map(|d| d.as_string_private());

        let wallet = bdk::Wallet::new(
            &descriptor,
            change_descriptor.as_ref(),
            network.into(),
            database,
        )?;
        Ok(WalletBase { ptr: RustOpaque::new( Mutex::new(wallet)) })
    }
    fn get_wallet(&self) -> MutexGuard<bdk::Wallet<AnyDatabase>> {
        self.ptr.lock().expect("")
    }

    /// Get the Bitcoin network the wallet is using.
    pub fn network(&self) -> Network {
        self.get_wallet().network().into()
    }
    /// Return whether or not a script is part of this wallet (either internal or external).
    pub fn is_mine(&self, script: ScriptBufBase) -> Result<bool, BdkError> {
        self.get_wallet()
            .is_mine(<ScriptBufBase as Into<bdk::bitcoin::ScriptBuf>>::into(script).as_script())
            .map_err(|e| e.into())
    }
    /// Return a derived address using the external descriptor, see AddressIndex for available address index selection
    /// strategies. If none of the keys in the descriptor are derivable (i.e. the descriptor does not end with a * character)
    /// then the same address will always be returned for any AddressIndex.
    pub fn get_address(&self, address_index: AddressIndex) -> Result<AddressInfo, BdkError> {
        self.get_wallet()
            .get_address(address_index.into())
            .map(AddressInfo::from)
            .map_err(|e| e.into())
    }

    /// Return a derived address using the internal (change) descriptor.
    ///
    /// If the wallet doesn't have an internal descriptor it will use the external descriptor.
    ///
    /// see [AddressIndex] for available address index selection strategies. If none of the keys
    /// in the descriptor are derivable (i.e. does not end with /*) then the same address will always
    /// be returned for any [AddressIndex].
    pub fn get_internal_address(
        &self,
        address_index: AddressIndex,
    ) -> Result<AddressInfo, BdkError> {
        self.get_wallet()
            .get_internal_address(address_index.into())
            .map(|e| e.into())
            .map_err(|e| e.into())
    }

    /// Return the balance, meaning the sum of this wallet’s unspent outputs’ values. Note that this method only operates
    /// on the internal database, which first needs to be Wallet.sync manually.
    pub fn get_balance(&self) -> Result<Balance, BdkError> {
        self.get_wallet()
            .get_balance()
            .map(|b| b.into())
            .map_err(|e| e.into())
    }
    /// Return the list of transactions made and received by the wallet. Note that this method only operate on the internal database, which first needs to be [Wallet.sync] manually.
    pub fn list_transactions(
        &self,
        include_raw: bool,
    ) -> Result<Vec<TransactionDetails>, BdkError> {
        let transaction_details = self.get_wallet().list_transactions(include_raw)?;
        Ok(transaction_details
            .into_iter()
            .map(TransactionDetails::from)
            .collect())
    }

    /// Return the list of unspent outputs of this wallet. Note that this method only operates on the internal database,
    /// which first needs to be Wallet.sync manually.
    pub fn list_unspent(&self) -> Result<Vec<LocalUtxo>, BdkError> {
        let unspent: Vec<bdk::LocalUtxo> = self.get_wallet().list_unspent()?;
        Ok(unspent.into_iter().map(LocalUtxo::from).collect())
    }

    /// Sign a transaction with all the wallet's signers. This function returns an encapsulated bool that
    /// has the value true if the PSBT was finalized, or false otherwise.
    ///
    /// The [SignOptions] can be used to tweak the behavior of the software signers, and the way
    /// the transaction is finalized at the end. Note that it can't be guaranteed that *every*
    /// signers will follow the options, but the "software signers" (WIF keys and `xprv`) defined
    /// in this library will.
    pub fn sign(
        &self,
        psbt: PsbtBase,
        sign_options: Option<SignOptions>,
    ) -> Result<bool, BdkError> {
        let mut psbt = psbt.ptr.lock().unwrap();
        self.get_wallet()
            .sign(
                &mut psbt,
                sign_options.map(SignOptions::into).unwrap_or_default(),
            )
            .map_err(|e| e.into())
    }
    /// Sync the internal database with the blockchain.
    pub fn sync(&self, blockchain: BlockchainBase) -> Result<(), BdkError> {
        let blockchain = blockchain.get_blockchain();
        self.get_wallet()
            .sync(blockchain.deref(), bdk::SyncOptions::default())
            .map_err(|e| e.into())
    }
    ///get the corresponding PSBT Input for a LocalUtxo
    pub fn get_psbt_input(
        &self,
        utxo: LocalUtxo,
        only_witness_utxo: bool,
        sighash_type: Option<PsbtSigHashType>,
    ) -> anyhow::Result<Input, BdkError> {
        let input = self.get_wallet()
            .get_psbt_input(utxo.into(),sighash_type.map(|e| e.into()), only_witness_utxo)?;
        Ok(input.into())
    }
    ///Returns the descriptor used to create addresses for a particular keychain.
    pub fn get_descriptor_for_keychain(
        &self,
        keychain: KeychainKind,
    ) -> anyhow::Result<DescriptorBase, BdkError> {
        let wallet = self.get_wallet();
        let extended_descriptor = wallet.get_descriptor_for_keychain(keychain.into());
        DescriptorBase::new(extended_descriptor.to_string(), wallet.network().into())
    }
}

pub fn finish_bump_fee_tx_builder(
    txid: String,
    fee_rate: f32,
    allow_shrinking: Option<AddressBase>,
    wallet: WalletBase,
    enable_rbf: bool,
    n_sequence: Option<u32>,
) -> anyhow::Result<(PsbtBase, TransactionDetails), BdkError> {
    let txid = Txid::from_str(txid.as_str()).unwrap();
    let bdk_wallet = wallet.get_wallet();

    let mut tx_builder = bdk_wallet.build_fee_bump(txid)?;
    tx_builder.fee_rate(bdk::FeeRate::from_sat_per_vb(fee_rate));
    if let Some(allow_shrinking) = &allow_shrinking {
        let address = allow_shrinking.0.clone();
        let script = address.script_pubkey();
        tx_builder.allow_shrinking(script).unwrap();
    }
    if let Some(n_sequence) = n_sequence {
        tx_builder.enable_rbf_with_sequence(Sequence(n_sequence));
    }
    if enable_rbf {
        tx_builder.enable_rbf();
    }
    return match tx_builder.finish() {
        Ok(e) => Ok((e.0.into(), TransactionDetails::from(&e.1))),
        Err(e) => Err(e.into()),
    };
}

pub fn tx_builder_finish(
    wallet: WalletBase,
    recipients: Vec<ScriptAmount>,
    utxos: Vec<OutPoint>,
    foreign_utxo: Option<(OutPoint, Input, usize)>,
    un_spendable: Vec<OutPoint>,
    change_policy: ChangeSpendPolicy,
    manually_selected_only: bool,
    fee_rate: Option<f32>,
    fee_absolute: Option<u64>,
    drain_wallet: bool,
    drain_to: Option<ScriptBufBase>,
    rbf: Option<RbfValue>,
    data: Vec<u8>,
) -> anyhow::Result<(PsbtBase, TransactionDetails), BdkError> {
    let binding = wallet.get_wallet();

    let mut tx_builder = binding.build_tx();

    for e in recipients {
        tx_builder.add_recipient(e.script.into(), e.amount);
    }
    tx_builder.change_policy(change_policy.into());

    if !utxos.is_empty() {
        let bdk_utxos: Vec<bdk::bitcoin::OutPoint> =
            utxos.iter().map(bdk::bitcoin::OutPoint::from).collect();
        let utxos: &[bdk::bitcoin::OutPoint] = &bdk_utxos;
        tx_builder.add_utxos(utxos).unwrap();
    }
    if !un_spendable.is_empty() {
        let bdk_unspendable: Vec<bdk::bitcoin::OutPoint> = un_spendable
            .iter()
            .map(bdk::bitcoin::OutPoint::from)
            .collect();
        tx_builder.unspendable(bdk_unspendable);
    }
    if manually_selected_only {
        tx_builder.manually_selected_only();
    }
    if let Some(sat_per_vb) = fee_rate {
        tx_builder.fee_rate(bdk::FeeRate::from_sat_per_vb(sat_per_vb));
    }
    if let Some(fee_amount) = fee_absolute {
        tx_builder.fee_absolute(fee_amount);
    }
    if drain_wallet {
        tx_builder.drain_wallet();
    }
    if let Some(script_) = drain_to {
        tx_builder.drain_to(script_.into());
    }
    if let Some(utxo) = foreign_utxo {
       let foreign_utxo:bdk::bitcoin::psbt::Input= utxo.1.into();
        tx_builder
            .add_foreign_utxo((&utxo.0).into(), foreign_utxo, utxo.2)?;
    }
    if let Some(rbf) = &rbf {
        match rbf {
            RbfValue::RbfDefault => {
                tx_builder.enable_rbf();
            }
            RbfValue::Value(nsequence) => {
                tx_builder.enable_rbf_with_sequence(Sequence(nsequence.to_owned()));
            }
        }
    }
    if !data.is_empty() {
        let push_bytes = PushBytesBuf::try_from(data.clone()).map_err(|_| {
            BdkError::Generic("Failed to convert data to PushBytes".to_string())
        })?;
        tx_builder.add_data(&push_bytes);
    }

    return match tx_builder.finish() {
        Ok(e) => Ok((
            e.0.into(),
            TransactionDetails::from(&e.1),
        )),
        Err(e) => Err(e.into()),
    };
}
