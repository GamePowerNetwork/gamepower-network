pub mod currency {
  use primitives::Balance;

  pub const DOLLARS: Balance = 1_000_000_000;
  pub const CENTS: Balance = DOLLARS / 10;
  // 10_000_000_000_000_000
  pub const MILLICENTS: Balance = CENTS / 100;
  // 10_000_000_000_000
  pub const MICROCENTS: Balance = MILLICENTS / 100; // 10_000_000_000
}