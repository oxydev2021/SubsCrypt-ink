use ink_env::AccountId;
use ink_storage::traits::{PackedLayout, SpreadLayout};

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
