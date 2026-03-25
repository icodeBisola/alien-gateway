use crate::errors::EscrowError;
use crate::types::{DataKey, LegacyVault, ScheduledPayment, VaultConfig, VaultState};
use soroban_sdk::{BytesN, Env};

/// Reads a vault's immutable configuration from persistent storage.
///
/// Checks the new `VaultConfig` key first; if absent, falls back to the legacy `Vault` key and
/// projects the combined record into a `VaultConfig` for backward compatibility.
pub fn read_vault_config(env: &Env, commitment: &BytesN<32>) -> Option<VaultConfig> {
    let storage = env.storage().persistent();
    if let Some(config) = storage.get(&DataKey::VaultConfig(commitment.clone())) {
        return Some(config);
    }
    let legacy: LegacyVault = storage.get(&DataKey::Vault(commitment.clone()))?;
    Some(VaultConfig {
        owner: legacy.owner,
        token: legacy.token,
        created_at: legacy.created_at,
    })
}

/// Writes a vault's immutable configuration to persistent storage.
pub fn write_vault_config(env: &Env, commitment: &BytesN<32>, config: &VaultConfig) {
    env.storage()
        .persistent()
        .set(&DataKey::VaultConfig(commitment.clone()), config);
}

/// Reads a vault's mutable state from persistent storage.
///
/// Checks the new `VaultState` key first; if absent, falls back to the legacy `Vault` key and
/// projects the combined record into a `VaultState` for backward compatibility.
pub fn read_vault_state(env: &Env, commitment: &BytesN<32>) -> Option<VaultState> {
    let storage = env.storage().persistent();
    if let Some(state) = storage.get(&DataKey::VaultState(commitment.clone())) {
        return Some(state);
    }
    let legacy: LegacyVault = storage.get(&DataKey::Vault(commitment.clone()))?;
    Some(VaultState {
        balance: legacy.balance,
        is_active: legacy.is_active,
    })
}

/// Writes a vault's mutable state to persistent storage.
pub fn write_vault_state(env: &Env, commitment: &BytesN<32>, state: &VaultState) {
    env.storage()
        .persistent()
        .set(&DataKey::VaultState(commitment.clone()), state);
}

/// Increments the global payment counter and returns the previous ID.
///
/// ### Errors
/// - Returns `EscrowError::PaymentCounterOverflow` if the counter reaches `u32::MAX`.
pub fn increment_payment_id(env: &Env) -> Result<u32, EscrowError> {
    let id: u32 = env
        .storage()
        .instance()
        .get(&DataKey::PaymentCounter)
        .unwrap_or(0);

    let next = id
        .checked_add(1)
        .ok_or(EscrowError::PaymentCounterOverflow)?;

    env.storage()
        .instance()
        .set(&DataKey::PaymentCounter, &next);

    Ok(id)
}

/// Records a new scheduled payment in persistent storage.
pub fn write_scheduled_payment(env: &Env, id: u32, payment: &ScheduledPayment) {
    env.storage()
        .persistent()
        .set(&DataKey::ScheduledPayment(id), payment);
}
