use borsh::{BorshDeserialize, BorshSerialize};
use base64;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Swap {
    Saber,
    SaberAddDecimalsDeposit,
    SaberAddDecimalsWithdraw,
    TokenSwap,
    Sencha,
    Step,
    Cropper,
    Raydium,
    Crema { a_to_b: bool },
    Lifinity,
    Mercurial,
    Cykura,
    Serum { side: Side },
    MarinadeDeposit,
    MarinadeUnstake,
    Aldrin { side: Side },
    AldrinV2 { side: Side },
    Whirlpool { a_to_b: bool },
    Invariant { x_to_y: bool },
    Meteora,
    GooseFX,
    DeltaFi { stable: bool },
    Balansol,
    MarcoPolo { x_to_y: bool },
    Dradex { side: Side },
    LifinityV2,
    RaydiumClmm,
    Openbook { side: Side },
    Phoenix { side: Side },
    Symmetry { from_token_id: u64, to_token_id: u64 },
    TokenSwapV2,
    HeliumTreasuryManagementRedeemV0,
    StakeDexStakeWrappedSol,
    StakeDexSwapViaStake { bridge_stake_seed: u32 },
    GooseFXV2,
    Perps,
    PerpsAddLiquidity,
    PerpsRemoveLiquidity,
    MeteoraDlmm,
    OpenBookV2 { side: Side },
    RaydiumClmmV2,
    StakeDexPrefundWithdrawStakeAndDepositStake { bridge_stake_seed: u32 },
    Clone { pool_index: u8, quantity_is_input: bool, quantity_is_collateral: bool },
    SanctumS { src_lst_value_calc_accs: u8, dst_lst_value_calc_accs: u8, src_lst_index: u32, dst_lst_index: u32 },
    SanctumSAddLiquidity { lst_value_calc_accs: u8, lst_index: u32 },
    SanctumSRemoveLiquidity { lst_value_calc_accs: u8, lst_index: u32 },
    RaydiumCP,
    WhirlpoolSwapV2 { a_to_b: bool, remaining_accounts_info: Option<RemainingAccountsInfo> },
    OneIntro,
    PumpdotfunWrappedBuy,
    PumpdotfunWrappedSell,
    PerpsV2,
    PerpsV2AddLiquidity,
    PerpsV2RemoveLiquidity,
    MoonshotWrappedBuy,
    MoonshotWrappedSell,
    StabbleStableSwap,
    StabbleWeightedSwap,
    Obric { x_to_y: bool },
    FoxBuyFromEstimatedCost,
    FoxClaimPartial { is_y: bool },
    SolFi { is_quote_to_base: bool },
}

// 其他相关的结构体和枚举
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RemainingAccountsInfo {
    slices: Vec<RemainingAccountsSlice>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RemainingAccountsSlice {
    accounts_type: AccountsType,
    length: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum AccountsType {
    TransferHookA,
    TransferHookB,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct RoutePlanStep {
    swap: Swap,
    percent: u8,
    input_index: u8,
    output_index: u8,
}

// 定义一个结构体
#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Route {
    route_plan: Vec<RoutePlanStep>,
    in_amount: u64,
    quoted_out_amount: u64,
    slippage_bps: u16,
    platform_fee_bps: u8,
}

fn main() {
    let data = base64::decode("5RfLl3rjrSoBAAAALwAAZAABsgAAAAAAAAAgAwAAAAAAAGQAFA==").unwrap();
    let route: Route = Route::try_from_slice(&data[8..]).expect("Deserialization failed");
    println!("{:?}", route);

    let data2 = route.try_to_vec().expect("Serialization failed");
    let part1 = &data[..8];
    let mut new_data = part1.to_vec(); // 将切片转换为 Vec
    new_data.extend(&data2);
    let after_base64 = base64::encode(&new_data);
    println!("{}", after_base64);
}
