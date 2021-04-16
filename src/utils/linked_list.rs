use ink_storage::traits::{PackedLayout, SpreadLayout};

/// Struct for handling payments of refund
    /// # Description
    ///
    /// This LinkedList is used for keeping tracking of each subscription that will end in some
    /// specific date in future. We order these subscriptions by their date of expiration, so we
    /// will be able to easily calculate and handle refund - withdraw methods with a minimum
    /// transaction fee. Each entity of the linked-list is `PaymentAdmission` struct.
#[derive(
scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct LinkedList {
    pub head: u64,
    pub back: u64,
    pub length: u128,
}

impl Default for LinkedList {
    fn default() -> Self {
        Self::new()
    }
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList::default()
    }
    pub fn default() -> Self {
        Self {
            back: 0,
            head: 0,
            length: 0,
        }
    }
}
