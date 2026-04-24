#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct CalculadoraContract;

#[contractimpl]
impl CalculadoraContract {
    pub fn somar(a: u32, b: u32) -> u32 {
        a + b
    }
}