use crate::errors::InsightArenaError;

// ── Constants ─────────────────────────────────────────────────────────────────

/// Minimum liquidity to prevent division by zero and manipulation.
pub const MIN_LIQUIDITY: i128 = 1000;

/// Default trading fee in basis points (0.3% = 30 bps).
pub const DEFAULT_FEE_BPS: u32 = 30;

// ── AMM Math Functions ────────────────────────────────────────────────────────

/// Calculate output amount for a swap using constant product formula.
///
/// Formula: amount_out = (amount_in * reserve_out) / (reserve_in + amount_in)
/// Then apply trading fee: amount_out_with_fee = amount_out * (1 - fee_bps/10000)
pub fn calculate_swap_output(
    amount_in: i128,
    reserve_in: i128,
    reserve_out: i128,
    fee_bps: u32,
) -> Result<i128, InsightArenaError> {
    if amount_in <= 0 || reserve_in <= 0 || reserve_out <= 0 {
        return Err(InsightArenaError::InvalidInput);
    }

    let numerator = amount_in
        .checked_mul(reserve_out)
        .ok_or(InsightArenaError::Overflow)?;

    let denominator = reserve_in
        .checked_add(amount_in)
        .ok_or(InsightArenaError::Overflow)?;

    let amount_out = numerator
        .checked_div(denominator)
        .ok_or(InsightArenaError::Overflow)?;

    let fee_multiplier = 10_000i128
        .checked_sub(fee_bps as i128)
        .ok_or(InsightArenaError::Overflow)?;

    let amount_out_with_fee = amount_out
        .checked_mul(fee_multiplier)
        .ok_or(InsightArenaError::Overflow)?
        .checked_div(10_000)
        .ok_or(InsightArenaError::Overflow)?;

    Ok(amount_out_with_fee)
}

// ── Helper Functions ──────────────────────────────────────────────────────────

/// Calculate liquidity value for LP tokens (for withdrawal)
pub fn calculate_liquidity_value(
    lp_tokens: i128,
    total_liquidity: i128,
    total_lp_supply: i128,
) -> Result<i128, InsightArenaError> {
    if lp_tokens <= 0 || total_lp_supply <= 0 {
        return Err(InsightArenaError::InvalidInput);
    }

    if lp_tokens > total_lp_supply {
        return Err(InsightArenaError::InsufficientBalance);
    }

    let value = lp_tokens
        .checked_mul(total_liquidity)
        .ok_or(InsightArenaError::Overflow)?
        .checked_div(total_lp_supply)
        .ok_or(InsightArenaError::Overflow)?;

    Ok(value)
}

// ── Liquidity Management ──────────────────────────────────────────────────────

/// Calculate LP tokens to mint for a deposit
pub fn calculate_lp_tokens(
    deposit_amount: i128,
    total_liquidity: i128,
    total_lp_supply: i128,
) -> Result<i128, InsightArenaError> {
    if deposit_amount <= 0 {
        return Err(InsightArenaError::InvalidInput);
    }

    // First deposit: mint tokens equal to deposit
    if total_lp_supply == 0 || total_liquidity == 0 {
        return Ok(deposit_amount);
    }

    // Subsequent deposits: mint proportionally
    let lp_tokens = deposit_amount
        .checked_mul(total_lp_supply)
        .ok_or(InsightArenaError::Overflow)?
        .checked_div(total_liquidity)
        .ok_or(InsightArenaError::Overflow)?;

    Ok(lp_tokens)
}

// TODO: add_liquidity
// TODO: remove_liquidity

// ── Trading Functions ─────────────────────────────────────────────────────────

// TODO: swap_outcome
// TODO: get_outcome_price

// ── Analytics ─────────────────────────────────────────────────────────────────

// TODO: get_pool_stats
// TODO: get_lp_position

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::InsightArenaError;

    #[test]
    fn test_calculate_swap_output_zero_input_fails() {
        // Should return InvalidInput error
        let result = calculate_swap_output(0, 1000, 1000, 30);
        assert_eq!(result, Err(InsightArenaError::InvalidInput));
    }

    #[test]
    fn test_calculate_swap_output_zero_reserve_fails() {
        // Should return InvalidInput error
        let result_in = calculate_swap_output(100, 0, 1000, 30);
        assert_eq!(result_in, Err(InsightArenaError::InvalidInput));

        let result_out = calculate_swap_output(100, 1000, 0, 30);
        assert_eq!(result_out, Err(InsightArenaError::InvalidInput));
    }

    #[test]
    fn test_calculate_swap_output_overflow_protection() {
        // Try: i128::MAX → Should return Overflow error
        let result = calculate_swap_output(i128::MAX, 1000, 1000, 30);
        assert_eq!(result, Err(InsightArenaError::Overflow));
    }

    #[test]
    fn test_calculate_lp_tokens_first_deposit() {
        // Deposit: 1000, Liquidity: 0, Supply: 0 → Expected: 1000
        assert_eq!(calculate_lp_tokens(1000, 0, 0), Ok(1000));
    }

    #[test]
    fn test_calculate_lp_tokens_second_deposit_equal() {
        // Deposit: 1000, Liquidity: 1000, Supply: 1000 → Expected: 1000
        assert_eq!(calculate_lp_tokens(1000, 1000, 1000), Ok(1000));
    }

    #[test]
    fn test_calculate_lp_tokens_second_deposit_half() {
        // Deposit: 500, Liquidity: 1000, Supply: 1000 → Expected: 500
        assert_eq!(calculate_lp_tokens(500, 1000, 1000), Ok(500));
    }

    #[test]
    fn test_calculate_lp_tokens_second_deposit_double() {
        // Deposit: 2000, Liquidity: 1000, Supply: 1000 → Expected: 2000
        assert_eq!(calculate_lp_tokens(2000, 1000, 1000), Ok(2000));
    }

    // ── Issue #373: Basic Liquidity Value Calculation Tests ──────────────────

    #[test]
    fn test_calculate_liquidity_value_full_withdrawal() {
        // LP: 1000, Liquidity: 1000, Supply: 1000 → Expected: 1000
        assert_eq!(
            calculate_liquidity_value(1000, 1000, 1000),
            Ok(1000)
        );
    }

    #[test]
    fn test_calculate_liquidity_value_half_withdrawal() {
        // LP: 500, Liquidity: 1000, Supply: 1000 → Expected: 500
        assert_eq!(
            calculate_liquidity_value(500, 1000, 1000),
            Ok(500)
        );
    }

    #[test]
    fn test_calculate_liquidity_value_quarter_withdrawal() {
        // LP: 250, Liquidity: 1000, Supply: 1000 → Expected: 250
        assert_eq!(
            calculate_liquidity_value(250, 1000, 1000),
            Ok(250)
        );
    }

    #[test]
    fn test_calculate_liquidity_value_with_fees() {
        // LP: 1000, Liquidity: 1100, Supply: 1000 → Expected: 1100
        assert_eq!(
            calculate_liquidity_value(1000, 1100, 1000),
            Ok(1100)
        );
    }

    // ── Issue #374: Edge Case Tests ──────────────────────────────────────────

    #[test]
    fn test_calculate_liquidity_value_multiple_lps() {
        // LP: 300, Liquidity: 1500, Supply: 1000 → Expected: 450
        assert_eq!(
            calculate_liquidity_value(300, 1500, 1000),
            Ok(450)
        );
    }

    #[test]
    fn test_calculate_liquidity_value_small_withdrawal() {
        // LP: 1, Liquidity: 1_000_000, Supply: 1_000_000 → Expected: 1
        assert_eq!(
            calculate_liquidity_value(1, 1_000_000, 1_000_000),
            Ok(1)
        );
    }

    #[test]
    fn test_calculate_liquidity_value_large_pool() {
        // LP: 100, Liquidity: 10_000_000, Supply: 1_000_000 → Expected: 1000
        assert_eq!(
            calculate_liquidity_value(100, 10_000_000, 1_000_000),
            Ok(1000)
        );
    }

    #[test]
    fn test_calculate_liquidity_value_after_trading() {
        // LP: 500, Liquidity: 1050, Supply: 1000 → Expected: 525
        assert_eq!(
            calculate_liquidity_value(500, 1050, 1000),
            Ok(525)
        );
    }

    // ── Issue #375: Validation Tests ─────────────────────────────────────────

    #[test]
    fn test_calculate_liquidity_value_zero_tokens_fails() {
        // Should return InvalidInput error
        let result = calculate_liquidity_value(0, 1000, 1000);
        assert_eq!(result, Err(InsightArenaError::InvalidInput));
    }

    #[test]
    fn test_calculate_liquidity_value_negative_tokens_fails() {
        // Should return InvalidInput error
        let result = calculate_liquidity_value(-1, 1000, 1000);
        assert_eq!(result, Err(InsightArenaError::InvalidInput));
    }

    #[test]
    fn test_calculate_liquidity_value_exceeds_supply_fails() {
        // LP: 1500, Supply: 1000 → Should return InsufficientBalance error
        let result = calculate_liquidity_value(1500, 1000, 1000);
        assert_eq!(result, Err(InsightArenaError::InsufficientBalance));
    }

    #[test]
    fn test_calculate_liquidity_value_overflow_protection() {
        // Try: i128::MAX as lp_tokens → Should return Overflow error
        let result = calculate_liquidity_value(i128::MAX, i128::MAX, 1000);
        assert_eq!(result, Err(InsightArenaError::Overflow));
    }

    // ── Issue #376: Precision Test ───────────────────────────────────────────

    #[test]
    fn test_calculate_liquidity_value_precision() {
        // LP: 333, Liquidity: 1000, Supply: 1000 → Expected: 333 (no rounding errors)
        assert_eq!(
            calculate_liquidity_value(333, 1000, 1000),
            Ok(333)
        );
    }
}
