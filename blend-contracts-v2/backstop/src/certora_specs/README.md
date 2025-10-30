# BlendV2 Formal Verification Contest Repo

This repo was submitted by me (BenRai1) for the [Blend V2 Audit + Certora Formal Verification competition](https://code4rena.com/audits/2025-02-blend-v2-audit-certora-formal-verification) on CodeArena running from the 24th of February 2025 to 17th of March 2025.

The goal of the formal verification part of the competition was to formally verify the following contracts using the Certora Sunbeam Prover:

| Rust files                                                                                                                                     |
| ---------------------------------------------------------------------------------------------------------------------------------------------- |
| [withdrawal.rs](https://github.com/BenRai1/BenRai1-2025-02-blend-fv/blob/main/blend-contracts-v2/backstop/src/backstop/withdrawal.rs)          |
| [user.rs](https://github.com/BenRai1/BenRai1-2025-02-blend-fv/blob/main/blend-contracts-v2/backstop/src/backstop/user.rs)                      |
| [deposit.rs](https://github.com/BenRai1/BenRai1-2025-02-blend-fv/blob/main/blend-contracts-v2/backstop/src/backstop/deposit.rs)                |
| [fund_managment.rs](https://github.com/BenRai1/BenRai1-2025-02-blend-fv/blob/main/blend-contracts-v2/backstop/src/backstop/fund_management.rs) |
| [pool.rs](https://github.com/BenRai1/BenRai1-2025-02-blend-fv/blob/main/blend-contracts-v2/backstop/src/backstop/pool.rs)                      |

I wrote a total of [99 rules](https://github.com/BenRai1/BenRai1-2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs) and managed to catch 21 out of 22 mutations used for [evaluating the submissions](https://docs.google.com/spreadsheets/d/1g0_RmhGVQJeQvn5vrAfhew-0hjq0iT0TO8OgsYPmfAg/edit?gid=1970712821#gid=1970712821) which place me 3rd in the FV contest.