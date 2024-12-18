use crate::*;

//return lamport
pub fn get_token_price(token_amount: u64) -> u64 {
    if token_amount <= 200_000_000_000 {
        return 33
    }
    if token_amount <= 400_000_000_000 {
        return 66
    }
    if token_amount <= 600_000_000_000 {
        return 99
    }
    if token_amount <= 800_000_000_000 {
        return 132
    }
    return 165
}

//token_amount: cutted decimal
pub fn calculate_cost_from_zero_supply(token_amount: u64) -> u64 {
    if token_amount <= 200_000_000 {
        return 33 * token_amount;
    }
    if token_amount <= 400_000_000 {
        return 33 * 200_000_000 + 66 * (token_amount - 200_000_000);
    }
    if token_amount <= 600_000_000 {
        return 99 * 200_000_000 + 99*(token_amount - 400_000_000);
    }
    if token_amount <= 800_000_000 {
        return 198 * 200_000_000 + 132*(token_amount - 600_000_000);
    }
    if token_amount <= 1_000_000_000 {
        return 330 * 200_000_000 + 165*(token_amount - 800_000_000);
    }
    return 495 * 200_000_000
}

pub fn calculate_token_amount_from_lamport(sol_amount: u64) -> u64{
    if sol_amount <= 33*200_000_000{
        return sol_amount / 33;
    }
    if sol_amount <= 99*200_000_000 {
        return 200_000_000 + (sol_amount - 33 * 200_000_000)/66;
    }
    if sol_amount <= 198*200_000_000 {
        return 2 * 200_000_000 +  (sol_amount - 99*200_000_000)/99;
    }
    if sol_amount <= 330* 200_000_000 {
        return 3 * 200_000_000 +  (sol_amount - 198*200_000_000)/132;
    }
    if sol_amount <= 495 * 200_000_000 {
        return 4 * 200_000_000 +  (sol_amount - 330*200_000_000)/165;
    }
    return 5*200_000_000;
}
pub fn calculate_cost(current_supply: u64, tokens_to_buy: u64) -> u64 {
    return calculate_cost_from_zero_supply(tokens_to_buy + current_supply) -
        calculate_cost_from_zero_supply(current_supply);
}

pub fn calculate_token_amount(current_supply: u64, sol_amount: u64) -> u64 {
    let total = sol_amount + calculate_cost_from_zero_supply(current_supply);
    return calculate_token_amount_from_lamport(total) - current_supply;
}