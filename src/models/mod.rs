use ink_env::AccountId;
use ink_storage::traits::{PackedLayout, SpreadLayout};

use super::utils::prelude::*;

/// This struct represents a provider
/// # fields:
/// * plans
/// * money_address : provider earned money will be sent to this address
/// * payment_manager : struct for handling refund requests
#[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Provider {
    pub(crate) plans: Vec<PlanConsts>,
    pub(crate) money_address: AccountId,
    pub(crate) payment_manager: LinkedList,
}

/// This struct represents a user
/// # fields:
/// * list_of_providers : list of providers that the user subscribed to
/// * subs_crypt_pass_hash : pass hash for retrieve data in subscrypt user dashboard
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct User {
    pub list_of_providers: Vec<AccountId>,
    pub subs_crypt_pass_hash: [u8; 32],
}

/// Struct that represents amount of money that can be withdraw after its due date passed.
#[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct DailyLockedAmount {
    pub(crate) amount: u128,
    pub(crate) next_day: u64,
}

pub struct ProcessReturningData {
    pub(crate) withdrawing_amount: u128,
    pub(crate) current_linked_list_head: u64,
    pub(crate) reduced_length: u128,
}

/// This struct stores configs of plan which is set by provider
/// # Note
/// `max_refund_permille_policy` is out of 1000
#[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PlanConsts {
    pub(crate) duration: u64,
    pub(crate) active_session_limit: u128,
    pub(crate) price: u128,
    pub(crate) max_refund_permille_policy: u128,
    pub(crate) disabled: bool,
}

/// This struct represents a subscription record
/// # fields:
/// * provider
/// * plan
/// * plan_index
/// * subscription_time : this stores start time of each subscription (used in linkedList)
/// * meta_data_encrypted
/// * refunded
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct SubscriptionRecord {
    pub(crate) provider: AccountId,
    pub(crate) plan: PlanConsts,
    pub(crate) plan_index: u128,
    pub(crate) subscription_time: u64,
    pub(crate) meta_data_encrypted: String,
    //encrypted Data with public key of provider
    pub refunded: bool,
}

/// This struct stores user plan records
/// # fields:
/// * subscription_records
/// * pass_hash : hash of (token + pass_phrase) for authenticating user without wallet
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PlanRecord {
    pub(crate) subscription_records: Vec<SubscriptionRecord>,
    pub(crate) pass_hash: [u8; 32],
}
