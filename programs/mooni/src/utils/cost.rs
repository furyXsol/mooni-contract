//token_amount
//return: lamports
pub fn calculate_cost_from_zero_supply(token_amount: u64) -> u64 {
  if token_amount <= 160_000_000 {
      return 125 * token_amount;
  }
  if token_amount <= 160_000_000 + 144_000_000 {
      return 125 * 160_000_000 + 139 * (token_amount - 160_000_000);
  }
  if token_amount <= 160_000_000 + 144_000_000 + 128_000_000 {
      return 125 * 160_000_000 +
              139 * 144_000_000 +
              156*(token_amount - 160_000_000 - 144_000_000);
  }
  if token_amount <= 160_000_000 + 144_000_000 + 128_000_000 + 112_000_000 {
      return 125 * 160_000_000 +
              139 * 144_000_000 +
              156 * 128_000_000 +
              179*(token_amount - 160_000_000 - 144_000_000 - 128_000_000);
  }
  if token_amount <= 160_000_000 + 144_000_000 + 128_000_000 + 112_000_000 + 96_000_000 {
      return 125 * 160_000_000 +
              139 * 144_000_000 +
              156 * 128_000_000 +
              179 * 90_000_000 +
              208 * (token_amount - 160_000_000 - 144_000_000 - 128_000_000 - 112_000_000)
  }
  return 208 * 1_000_000_000;
}

// return:  token_amount
pub fn calculate_token_amount_from_lamport(lamports: u64) -> u64{
  if lamports <= 125*160_000_000{
      return lamports / 125;
  }
  if lamports <= 125*160_000_000 + 139*144_000_000 {
      return 160_000_000 + (lamports - 125 * 160_000_000)/139;
  }
  if lamports <= 125*160_000_000 + 139*144_000_000 + 156*128_000_000 {
      return 160_000_000 + 144_000_000 +  (lamports - 125 * 160_000_000 - 139*144_000_000)/156;
  }
  if lamports <= 125*160_000_000 + 139*144_000_000 + 156*128_000_000 + 179*112_000_000 {
      return 160_000_000 +
          144_000_000 +
          128_000_000+
          (lamports - 125 * 160_000_000 - 139*144_000_000 - 156 * 128_000_000)/179;
  }
  if lamports <= 100_000_000_000{
      return
          160_000_000 +
          144_000_000 +
          128_000_000 +
          112_000_000 +
          (lamports - 125 * 160_000_000 - 139*144_000_000 - 156 * 128_000_000 - 179*112_000_000)/208;
  }
  return 640_000_000;
}

pub fn calculate_token_amount(current_supply: u64, sol_amount: u64) -> u64 {
  let total = sol_amount + calculate_cost_from_zero_supply(current_supply);
  return calculate_token_amount_from_lamport(total) - current_supply;
}