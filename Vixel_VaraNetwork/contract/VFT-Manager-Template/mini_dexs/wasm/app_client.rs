// Code generated by sails-client-gen. DO NOT EDIT.
#[allow(unused_imports)]
use sails_rs::collections::BTreeMap;
#[allow(unused_imports)]
use sails_rs::{
    calls::{Activation, Call, Query, Remoting, RemotingAction},
    prelude::*,
    String,
};
pub struct AppFactory<R> {
    #[allow(dead_code)]
    remoting: R,
}
impl<R> AppFactory<R> {
    #[allow(unused)]
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::AppFactory for AppFactory<R> {
    type Args = R::Args;
    fn new(&self) -> impl Activation<Args = R::Args> {
        RemotingAction::<_, app_factory::io::New>::new(self.remoting.clone(), ())
    }
    fn new_with_data(
        &self,
        vft_contract_id: Option<ActorId>,
        min_tokens_to_add: u128,
        tokens_per_vara: u128,
    ) -> impl Activation<Args = R::Args> {
        RemotingAction::<_, app_factory::io::NewWithData>::new(
            self.remoting.clone(),
            (vft_contract_id, min_tokens_to_add, tokens_per_vara),
        )
    }
}

pub mod app_factory {
    use super::*;
    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct New(());
        impl New {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <New as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for New {
            const ROUTE: &'static [u8] = &[12, 78, 101, 119];
            type Params = ();
            type Reply = ();
        }
        pub struct NewWithData(());
        impl NewWithData {
            #[allow(dead_code)]
            pub fn encode_call(
                vft_contract_id: Option<ActorId>,
                min_tokens_to_add: u128,
                tokens_per_vara: u128,
            ) -> Vec<u8> {
                <NewWithData as ActionIo>::encode_call(&(
                    vft_contract_id,
                    min_tokens_to_add,
                    tokens_per_vara,
                ))
            }
        }
        impl ActionIo for NewWithData {
            const ROUTE: &'static [u8] = &[44, 78, 101, 119, 87, 105, 116, 104, 68, 97, 116, 97];
            type Params = (Option<ActorId>, u128, u128);
            type Reply = ();
        }
    }
}
pub struct MiniDeXs<R> {
    remoting: R,
}
impl<R> MiniDeXs<R> {
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::MiniDeXs for MiniDeXs<R> {
    type Args = R::Args;
    /// ## Add an amount of tokens to the vft contract for this contract
    /// Only the contract owner can perform this action
    fn add_tokens_to_contract(
        &mut self,
        tokens_to_add: u128,
    ) -> impl Call<Output = MiniDexsEvents, Args = R::Args> {
        RemotingAction::<_, mini_de_xs::io::AddTokensToContract>::new(
            self.remoting.clone(),
            tokens_to_add,
        )
    }
    /// ## Change the minimum number of tokens to add to the contract
    /// Only the contract owner can perform this action
    fn set_min_tokens_to_add(
        &mut self,
        min_tokens_to_add: u128,
    ) -> impl Call<Output = MiniDexsEvents, Args = R::Args> {
        RemotingAction::<_, mini_de_xs::io::SetMinTokensToAdd>::new(
            self.remoting.clone(),
            min_tokens_to_add,
        )
    }
    /// ## Change the number of tokens to exchange for one rod
    /// Only the contract owner can perform this action
    fn set_tokens_per_vara(
        &mut self,
        tokens_per_vara: u128,
    ) -> impl Call<Output = MiniDexsEvents, Args = R::Args> {
        RemotingAction::<_, mini_de_xs::io::SetTokensPerVara>::new(
            self.remoting.clone(),
            tokens_per_vara,
        )
    }
    /// ## Change vft contract id
    /// Only the contract owner can perform this action
    fn set_vft_contract_id(
        &mut self,
        vft_contract_id: ActorId,
    ) -> impl Call<Output = MiniDexsEvents, Args = R::Args> {
        RemotingAction::<_, mini_de_xs::io::SetVftContractId>::new(
            self.remoting.clone(),
            vft_contract_id,
        )
    }
    /// ## Swap Varas for tokens
    /// Receive a certain amount of varas and then make a swap for a certain number of tokens
    fn swap_tokens_by_num_of_varas(
        &mut self,
    ) -> impl Call<Output = MiniDexsEvents, Args = R::Args> {
        RemotingAction::<_, mini_de_xs::io::SwapTokensByNumOfVaras>::new(self.remoting.clone(), ())
    }
    /// ## Swap tokens for Varas
    fn swap_tokens_to_varas(
        &mut self,
        amount_of_tokens: u128,
    ) -> impl Call<Output = MiniDexsEvents, Args = R::Args> {
        RemotingAction::<_, mini_de_xs::io::SwapTokensToVaras>::new(
            self.remoting.clone(),
            amount_of_tokens,
        )
    }
    /// ## Varas stored in contract
    fn contract_total_varas_stored(
        &self,
    ) -> impl Query<Output = MiniDexsQueryEvents, Args = R::Args> {
        RemotingAction::<_, mini_de_xs::io::ContractTotalVarasStored>::new(
            self.remoting.clone(),
            (),
        )
    }
    fn tokens_to_swap_one_vara(&self) -> impl Query<Output = MiniDexsQueryEvents, Args = R::Args> {
        RemotingAction::<_, mini_de_xs::io::TokensToSwapOneVara>::new(self.remoting.clone(), ())
    }
    /// ## Returns the total number of tokens in the contract (In U256 format)
    fn total_tokens_to_swap(&self) -> impl Query<Output = MiniDexsQueryEvents, Args = R::Args> {
        RemotingAction::<_, mini_de_xs::io::TotalTokensToSwap>::new(self.remoting.clone(), ())
    }
    /// ## Returns the total number of tokens in the contract (In u128 format)
    fn total_tokens_to_swap_as_u_128(
        &self,
    ) -> impl Query<Output = MiniDexsQueryEvents, Args = R::Args> {
        RemotingAction::<_, mini_de_xs::io::TotalTokensToSwapAsU128>::new(self.remoting.clone(), ())
    }
}

pub mod mini_de_xs {
    use super::*;

    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct AddTokensToContract(());
        impl AddTokensToContract {
            #[allow(dead_code)]
            pub fn encode_call(tokens_to_add: u128) -> Vec<u8> {
                <AddTokensToContract as ActionIo>::encode_call(&tokens_to_add)
            }
        }
        impl ActionIo for AddTokensToContract {
            const ROUTE: &'static [u8] = &[
                32, 77, 105, 110, 105, 68, 101, 88, 115, 76, 65, 100, 100, 84, 111, 107, 101, 110,
                115, 84, 111, 67, 111, 110, 116, 114, 97, 99, 116,
            ];
            type Params = u128;
            type Reply = super::MiniDexsEvents;
        }
        pub struct SetMinTokensToAdd(());
        impl SetMinTokensToAdd {
            #[allow(dead_code)]
            pub fn encode_call(min_tokens_to_add: u128) -> Vec<u8> {
                <SetMinTokensToAdd as ActionIo>::encode_call(&min_tokens_to_add)
            }
        }
        impl ActionIo for SetMinTokensToAdd {
            const ROUTE: &'static [u8] = &[
                32, 77, 105, 110, 105, 68, 101, 88, 115, 68, 83, 101, 116, 77, 105, 110, 84, 111,
                107, 101, 110, 115, 84, 111, 65, 100, 100,
            ];
            type Params = u128;
            type Reply = super::MiniDexsEvents;
        }
        pub struct SetTokensPerVara(());
        impl SetTokensPerVara {
            #[allow(dead_code)]
            pub fn encode_call(tokens_per_vara: u128) -> Vec<u8> {
                <SetTokensPerVara as ActionIo>::encode_call(&tokens_per_vara)
            }
        }
        impl ActionIo for SetTokensPerVara {
            const ROUTE: &'static [u8] = &[
                32, 77, 105, 110, 105, 68, 101, 88, 115, 64, 83, 101, 116, 84, 111, 107, 101, 110,
                115, 80, 101, 114, 86, 97, 114, 97,
            ];
            type Params = u128;
            type Reply = super::MiniDexsEvents;
        }
        pub struct SetVftContractId(());
        impl SetVftContractId {
            #[allow(dead_code)]
            pub fn encode_call(vft_contract_id: ActorId) -> Vec<u8> {
                <SetVftContractId as ActionIo>::encode_call(&vft_contract_id)
            }
        }
        impl ActionIo for SetVftContractId {
            const ROUTE: &'static [u8] = &[
                32, 77, 105, 110, 105, 68, 101, 88, 115, 64, 83, 101, 116, 86, 102, 116, 67, 111,
                110, 116, 114, 97, 99, 116, 73, 100,
            ];
            type Params = ActorId;
            type Reply = super::MiniDexsEvents;
        }
        pub struct SwapTokensByNumOfVaras(());
        impl SwapTokensByNumOfVaras {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <SwapTokensByNumOfVaras as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for SwapTokensByNumOfVaras {
            const ROUTE: &'static [u8] = &[
                32, 77, 105, 110, 105, 68, 101, 88, 115, 88, 83, 119, 97, 112, 84, 111, 107, 101,
                110, 115, 66, 121, 78, 117, 109, 79, 102, 86, 97, 114, 97, 115,
            ];
            type Params = ();
            type Reply = super::MiniDexsEvents;
        }
        pub struct SwapTokensToVaras(());
        impl SwapTokensToVaras {
            #[allow(dead_code)]
            pub fn encode_call(amount_of_tokens: u128) -> Vec<u8> {
                <SwapTokensToVaras as ActionIo>::encode_call(&amount_of_tokens)
            }
        }
        impl ActionIo for SwapTokensToVaras {
            const ROUTE: &'static [u8] = &[
                32, 77, 105, 110, 105, 68, 101, 88, 115, 68, 83, 119, 97, 112, 84, 111, 107, 101,
                110, 115, 84, 111, 86, 97, 114, 97, 115,
            ];
            type Params = u128;
            type Reply = super::MiniDexsEvents;
        }
        pub struct ContractTotalVarasStored(());
        impl ContractTotalVarasStored {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <ContractTotalVarasStored as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for ContractTotalVarasStored {
            const ROUTE: &'static [u8] = &[
                32, 77, 105, 110, 105, 68, 101, 88, 115, 96, 67, 111, 110, 116, 114, 97, 99, 116,
                84, 111, 116, 97, 108, 86, 97, 114, 97, 115, 83, 116, 111, 114, 101, 100,
            ];
            type Params = ();
            type Reply = super::MiniDexsQueryEvents;
        }
        pub struct TokensToSwapOneVara(());
        impl TokensToSwapOneVara {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <TokensToSwapOneVara as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for TokensToSwapOneVara {
            const ROUTE: &'static [u8] = &[
                32, 77, 105, 110, 105, 68, 101, 88, 115, 76, 84, 111, 107, 101, 110, 115, 84, 111,
                83, 119, 97, 112, 79, 110, 101, 86, 97, 114, 97,
            ];
            type Params = ();
            type Reply = super::MiniDexsQueryEvents;
        }
        pub struct TotalTokensToSwap(());
        impl TotalTokensToSwap {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <TotalTokensToSwap as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for TotalTokensToSwap {
            const ROUTE: &'static [u8] = &[
                32, 77, 105, 110, 105, 68, 101, 88, 115, 68, 84, 111, 116, 97, 108, 84, 111, 107,
                101, 110, 115, 84, 111, 83, 119, 97, 112,
            ];
            type Params = ();
            type Reply = super::MiniDexsQueryEvents;
        }
        pub struct TotalTokensToSwapAsU128(());
        impl TotalTokensToSwapAsU128 {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <TotalTokensToSwapAsU128 as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for TotalTokensToSwapAsU128 {
            const ROUTE: &'static [u8] = &[
                32, 77, 105, 110, 105, 68, 101, 88, 115, 92, 84, 111, 116, 97, 108, 84, 111, 107,
                101, 110, 115, 84, 111, 83, 119, 97, 112, 65, 115, 85, 49, 50, 56,
            ];
            type Params = ();
            type Reply = super::MiniDexsQueryEvents;
        }
    }
}
#[derive(PartialEq, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum MiniDexsEvents {
    RefundOfVaras(u128),
    VFTContractIdSet,
    MinTokensToAddSet,
    TokensAdded,
    SetTokensPerVaras,
    TotalSwapInVaras(u128),
    TokensSwapSuccessfully {
        total_tokens: u128,
        total_varas: u128,
    },
    Error(MiniDexsErrors),
}
#[derive(PartialEq, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum MiniDexsErrors {
    MinTokensToAdd(u128),
    CantSwapTokens {
        tokens_in_vft_contract: U256,
    },
    CantSwapUserTokens {
        user_tokens: U256,
        tokens_to_swap: U256,
    },
    ContractCantMint,
    CantSwapTokensWithAmount {
        min_amount: u128,
        actual_amount: u128,
    },
    OnlyOwnerCanDoThatAction,
    VftContractIdNotSet,
    ErrorInVFTContract,
    ErrorInGetNumOfVarasToSwap,
    OperationWasNotPerformed,
}
#[derive(PartialEq, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum MiniDexsQueryEvents {
    ContractBalanceInVaras(u128),
    UserTotalTokensAsU128(u128),
    UserTotalTokens(U256),
    TotalTokensToSwap(U256),
    TotalTokensToSwapAsU128(u128),
    TokensToSwapOneVara(u128),
    NumOfTokensForOneVara(u128),
    Error(MiniDexsErrors),
}

pub mod traits {
    use super::*;
    #[allow(dead_code)]
    pub trait AppFactory {
        type Args;
        #[allow(clippy::new_ret_no_self)]
        #[allow(clippy::wrong_self_convention)]
        fn new(&self) -> impl Activation<Args = Self::Args>;
        fn new_with_data(
            &self,
            vft_contract_id: Option<ActorId>,
            min_tokens_to_add: u128,
            tokens_per_vara: u128,
        ) -> impl Activation<Args = Self::Args>;
    }

    #[allow(clippy::type_complexity)]
    pub trait MiniDeXs {
        type Args;
        fn add_tokens_to_contract(
            &mut self,
            tokens_to_add: u128,
        ) -> impl Call<Output = MiniDexsEvents, Args = Self::Args>;
        fn set_min_tokens_to_add(
            &mut self,
            min_tokens_to_add: u128,
        ) -> impl Call<Output = MiniDexsEvents, Args = Self::Args>;
        fn set_tokens_per_vara(
            &mut self,
            tokens_per_vara: u128,
        ) -> impl Call<Output = MiniDexsEvents, Args = Self::Args>;
        fn set_vft_contract_id(
            &mut self,
            vft_contract_id: ActorId,
        ) -> impl Call<Output = MiniDexsEvents, Args = Self::Args>;
        fn swap_tokens_by_num_of_varas(
            &mut self,
        ) -> impl Call<Output = MiniDexsEvents, Args = Self::Args>;
        fn swap_tokens_to_varas(
            &mut self,
            amount_of_tokens: u128,
        ) -> impl Call<Output = MiniDexsEvents, Args = Self::Args>;
        fn contract_total_varas_stored(
            &self,
        ) -> impl Query<Output = MiniDexsQueryEvents, Args = Self::Args>;
        fn tokens_to_swap_one_vara(
            &self,
        ) -> impl Query<Output = MiniDexsQueryEvents, Args = Self::Args>;
        fn total_tokens_to_swap(
            &self,
        ) -> impl Query<Output = MiniDexsQueryEvents, Args = Self::Args>;
        fn total_tokens_to_swap_as_u_128(
            &self,
        ) -> impl Query<Output = MiniDexsQueryEvents, Args = Self::Args>;
    }
}

#[cfg(feature = "with_mocks")]
#[cfg(not(target_arch = "wasm32"))]
extern crate std;

#[cfg(feature = "with_mocks")]
#[cfg(not(target_arch = "wasm32"))]
pub mod mockall {
    use super::*;
    use sails_rs::mockall::*;
    mock! { pub MiniDeXs<A> {} #[allow(refining_impl_trait)] #[allow(clippy::type_complexity)] impl<A> traits::MiniDeXs for MiniDeXs<A> { type Args = A; fn add_tokens_to_contract (&mut self, tokens_to_add: u128,) -> MockCall<A, MiniDexsEvents>;fn set_min_tokens_to_add (&mut self, min_tokens_to_add: u128,) -> MockCall<A, MiniDexsEvents>;fn set_tokens_per_vara (&mut self, tokens_per_vara: u128,) -> MockCall<A, MiniDexsEvents>;fn set_vft_contract_id (&mut self, vft_contract_id: ActorId,) -> MockCall<A, MiniDexsEvents>;fn swap_tokens_by_num_of_varas (&mut self, ) -> MockCall<A, MiniDexsEvents>;fn swap_tokens_to_varas (&mut self, amount_of_tokens: u128,) -> MockCall<A, MiniDexsEvents>;fn contract_total_varas_stored (& self, ) -> MockQuery<A, MiniDexsQueryEvents>;fn tokens_to_swap_one_vara (& self, ) -> MockQuery<A, MiniDexsQueryEvents>;fn total_tokens_to_swap (& self, ) -> MockQuery<A, MiniDexsQueryEvents>;fn total_tokens_to_swap_as_u_128 (& self, ) -> MockQuery<A, MiniDexsQueryEvents>; } }
}
