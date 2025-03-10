#![allow(unused)]
use soroban_sdk::{Env, Address};

use crate::{storage::BackstopEmissionData, PoolBalance, UserBalance};


pub fn fixed_div_ceil(x: i128, y: i128, denominator: i128) -> i128 {
    let r = x * y;
    let initial_result = r/ denominator;
    //check if threre is no rest
    let final_result; 
    if initial_result * denominator == initial_result {
        final_result = initial_result;
    } else{
        final_result = initial_result +1;
    }

    final_result
}

pub fn fixed_div_floor(x: i128, y: i128, denominator: i128) -> i128 {
    let result = (x * denominator) / y;
    result
}

pub fn fixed_mul_floor(x: i128, y: i128, denominator: i128) -> i128 { //@audit-issue implement right
    let result = (x * y) / denominator;
    result
}


