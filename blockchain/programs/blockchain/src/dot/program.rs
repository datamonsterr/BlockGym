#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct GymClass {
    pub company: Pubkey,
    pub trainer: Pubkey,
    pub customer: Pubkey,
    pub name_array: [u8; 32],
    pub location_array: [u8; 64],
    pub info_array: [u8; 256],
    pub price: u64,
    pub is_done: u8,
    pub seed_sha256: u64,
}

impl<'info, 'entrypoint> GymClass {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedGymClass<'info, 'entrypoint>> {
        let company = account.company.clone();
        let trainer = account.trainer.clone();
        let customer = account.customer.clone();
        let name_array = Mutable::new(account.name_array.clone());
        let location_array = Mutable::new(account.location_array.clone());
        let info_array = Mutable::new(account.info_array.clone());
        let price = account.price;
        let is_done = account.is_done;
        let seed_sha256 = account.seed_sha256;

        Mutable::new(LoadedGymClass {
            __account__: account,
            __programs__: programs_map,
            company,
            trainer,
            customer,
            name_array,
            location_array,
            info_array,
            price,
            is_done,
            seed_sha256,
        })
    }

    pub fn store(loaded: Mutable<LoadedGymClass>) {
        let mut loaded = loaded.borrow_mut();
        let company = loaded.company.clone();

        loaded.__account__.company = company;

        let trainer = loaded.trainer.clone();

        loaded.__account__.trainer = trainer;

        let customer = loaded.customer.clone();

        loaded.__account__.customer = customer;

        let name_array = loaded.name_array.borrow().clone();

        loaded.__account__.name_array = name_array;

        let location_array = loaded.location_array.borrow().clone();

        loaded.__account__.location_array = location_array;

        let info_array = loaded.info_array.borrow().clone();

        loaded.__account__.info_array = info_array;

        let price = loaded.price;

        loaded.__account__.price = price;

        let is_done = loaded.is_done;

        loaded.__account__.is_done = is_done;

        let seed_sha256 = loaded.seed_sha256;

        loaded.__account__.seed_sha256 = seed_sha256;
    }
}

#[derive(Debug)]
pub struct LoadedGymClass<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, GymClass>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub company: Pubkey,
    pub trainer: Pubkey,
    pub customer: Pubkey,
    pub name_array: Mutable<[u8; 32]>,
    pub location_array: Mutable<[u8; 64]>,
    pub info_array: Mutable<[u8; 256]>,
    pub price: u64,
    pub is_done: u8,
    pub seed_sha256: u64,
}

pub fn customer_confirm_done_handler<'info>(
    mut customer: SeahorseSigner<'info, '_>,
    mut trainer: SeahorseSigner<'info, '_>,
    mut company: SeahorseSigner<'info, '_>,
    mut gymclass: Mutable<LoadedGymClass<'info, '_>>,
) -> () {
    if !(customer.key() == gymclass.borrow().customer) {
        panic!("gymclass student not");
    }

    if !(gymclass.borrow().is_done == 1) {
        panic!("PT not confirm yet");
    }

    assign!(gymclass.borrow_mut().is_done, 2);

    let mut pt_money = (gymclass.borrow().price / 100) * 70;
    let mut customer_reward = (gymclass.borrow().price / 100) * 20;

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &company.key(),
            &customer.clone().key(),
            pt_money.clone(),
        ),
        &[
            company.to_account_info(),
            customer.clone().to_account_info(),
            company.programs.get("system_program").clone(),
        ],
    )
    .unwrap();

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &company.key(),
            &trainer.clone().key(),
            customer_reward.clone(),
        ),
        &[
            company.to_account_info(),
            trainer.clone().to_account_info(),
            company.programs.get("system_program").clone(),
        ],
    )
    .unwrap();
}

pub fn customer_join_gymclass_handler<'info>(
    mut customer: SeahorseSigner<'info, '_>,
    mut company: SeahorseSigner<'info, '_>,
    mut gymclass: Mutable<LoadedGymClass<'info, '_>>,
) -> () {
    if !(gymclass.borrow().__account__.key() == gymclass.borrow().customer) {
        panic!("gymclass already have customer!");
    }

    if !(gymclass.borrow().is_done < 2) {
        panic!("gymclass already done!");
    }

    assign!(gymclass.borrow_mut().customer, customer.key());

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &customer.key(),
            &company.clone().key(),
            gymclass.borrow().price.clone(),
        ),
        &[
            customer.to_account_info(),
            company.clone().to_account_info(),
            customer.programs.get("system_program").clone(),
        ],
    )
    .unwrap();
}

pub fn init_gymclass_handler<'info>(
    mut trainer: SeahorseSigner<'info, '_>,
    mut company: SeahorseSigner<'info, '_>,
    mut gymclass: Empty<Mutable<LoadedGymClass<'info, '_>>>,
    mut seed_sha256: u64,
    mut price: u64,
    mut location_array: [u8; 64],
    mut name_array: [u8; 32],
    mut info_array: [u8; 256],
) -> () {
    let mut gymclass = gymclass.account.clone();

    assign!(gymclass.borrow_mut().trainer, trainer.key());

    assign!(gymclass.borrow_mut().company, company.key());

    assign!(
        gymclass.borrow_mut().customer,
        gymclass.borrow().__account__.key()
    );

    assign!(gymclass.borrow_mut().price, price);

    assign!(gymclass.borrow_mut().location_array, Mutable::<[u8; 64]>::new(location_array));

    assign!(gymclass.borrow_mut().name_array, Mutable::<[u8; 32]>::new(name_array));

    assign!(gymclass.borrow_mut().info_array, Mutable::<[u8; 256]>::new(info_array));

    assign!(gymclass.borrow_mut().is_done, 0);

    assign!(gymclass.borrow_mut().seed_sha256, seed_sha256);

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &trainer.key(),
            &company.clone().key(),
            1000000000,
        ),
        &[
            trainer.to_account_info(),
            company.clone().to_account_info(),
            trainer.programs.get("system_program").clone(),
        ],
    )
    .unwrap();
}

pub fn pt_confirm_done_handler<'info>(
    mut trainer: SeahorseSigner<'info, '_>,
    mut gymclass: Mutable<LoadedGymClass<'info, '_>>,
) -> () {
    if !(trainer.key() == gymclass.borrow().trainer) {
        panic!("gymclass trainer not");
    }

    if !(gymclass.borrow().is_done == 0) {
        panic!("customer cannot confirm before PT");
    }

    assign!(gymclass.borrow_mut().is_done, 1);
}

pub fn pt_decline_customer_handler<'info>(
    mut trainer: SeahorseSigner<'info, '_>,
    mut customer: SeahorseSigner<'info, '_>,
    mut company: SeahorseSigner<'info, '_>,
    mut gymclass: Mutable<LoadedGymClass<'info, '_>>,
) -> () {
    if !(trainer.key() == gymclass.borrow().trainer) {
        panic!("gymclass trainer not same!");
    }

    if !(customer.key() == gymclass.borrow().customer) {
        panic!("Customer is not in gymclass!");
    }

    if !(gymclass.borrow().is_done < 2) {
        panic!("gymclass already done!");
    }

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &company.key(),
            &customer.clone().key(),
            gymclass.borrow().price.clone(),
        ),
        &[
            company.to_account_info(),
            customer.clone().to_account_info(),
            company.programs.get("system_program").clone(),
        ],
    )
    .unwrap();

    assign!(
        gymclass.borrow_mut().customer,
        gymclass.borrow().__account__.key()
    );
}
