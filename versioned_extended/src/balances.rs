use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Balances {
    /// stores the amount given address deposited
    pub deposited: u128,
    /// stores the amount given address deposited plus the earned shares
    pub total: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BalancesV1 {
    /// stores the amount given address deposited
    pub deposited: u128,
    /// stores the amount given address deposited plus the earned shares
    pub total: u128,
    /// new data
    pub earned: u128,
}

/// An example of a versioned struct. In v1, earned is added to state
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Clone)]
pub enum VersionedBalances {
    V0(Balances),
    V1(BalancesV1),
}

impl VersionedBalances {
    /// upgrade VersionedBalances to newer version
    pub fn upgrade(&self) -> Self {
        match self {
            VersionedBalances::V0(bal) => {
                // upgrade state to V1
                let new_bal: BalancesV1 = BalancesV1 {
                    deposited: bal.deposited,
                    total: bal.total,
                    earned: 0,
                };
                VersionedBalances::V1(new_bal)
            }
            // no upgrade required
            VersionedBalances::V1(bal) => VersionedBalances::V1(bal.clone()),
        }
    }

    pub fn need_upgrade(&self) -> bool {
        match self {
            Self::V0(_) => true,
            Self::V1(_) => false,
        }
    }

    pub fn get_balance(self) -> BalancesV1 {
        match self {
            VersionedBalances::V1(bal) => bal,
            _ => unimplemented!(),
        }
    }
}
