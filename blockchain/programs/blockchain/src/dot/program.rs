#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Customer {
    pub customer: Pubkey,
    pub name_array: [u16; 32],
    pub location_array: [u16; 64],
    pub info_array: [u16; 256],
    pub gender: u8,
    pub phone_array: [u8; 10],
    pub age: u8,
    pub seed_random: u64,
}

impl<'info, 'entrypoint> Customer {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedCustomer<'info, 'entrypoint>> {
        let customer = account.customer.clone();
        let name_array = Mutable::new(account.name_array.clone());
        let location_array = Mutable::new(account.location_array.clone());
        let info_array = Mutable::new(account.info_array.clone());
        let gender = account.gender;
        let phone_array = Mutable::new(account.phone_array.clone());
        let age = account.age;
        let seed_random = account.seed_random;

        Mutable::new(LoadedCustomer {
            __account__: account,
            __programs__: programs_map,
            customer,
            name_array,
            location_array,
            info_array,
            gender,
            phone_array,
            age,
            seed_random,
        })
    }

    pub fn store(loaded: Mutable<LoadedCustomer>) {
        let mut loaded = loaded.borrow_mut();
        let customer = loaded.customer.clone();

        loaded.__account__.customer = customer;

        let name_array = loaded.name_array.borrow().clone();

        loaded.__account__.name_array = name_array;

        let location_array = loaded.location_array.borrow().clone();

        loaded.__account__.location_array = location_array;

        let info_array = loaded.info_array.borrow().clone();

        loaded.__account__.info_array = info_array;

        let gender = loaded.gender;

        loaded.__account__.gender = gender;

        let phone_array = loaded.phone_array.borrow().clone();

        loaded.__account__.phone_array = phone_array;

        let age = loaded.age;

        loaded.__account__.age = age;

        let seed_random = loaded.seed_random;

        loaded.__account__.seed_random = seed_random;
    }
}

#[derive(Debug)]
pub struct LoadedCustomer<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Customer>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub customer: Pubkey,
    pub name_array: Mutable<[u16; 32]>,
    pub location_array: Mutable<[u16; 64]>,
    pub info_array: Mutable<[u16; 256]>,
    pub gender: u8,
    pub phone_array: Mutable<[u8; 10]>,
    pub age: u8,
    pub seed_random: u64,
}

#[account]
#[derive(Debug)]
pub struct GymClass {
    pub company: Pubkey,
    pub trainer: Pubkey,
    pub customer: Pubkey,
    pub name_array: [u16; 32],
    pub info_array: [u16; 256],
    pub price: u64,
    pub flag: u8,
    pub review_array: [u16; 128],
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
        let info_array = Mutable::new(account.info_array.clone());
        let price = account.price;
        let flag = account.flag;
        let review_array = Mutable::new(account.review_array.clone());
        let seed_sha256 = account.seed_sha256;

        Mutable::new(LoadedGymClass {
            __account__: account,
            __programs__: programs_map,
            company,
            trainer,
            customer,
            name_array,
            info_array,
            price,
            flag,
            review_array,
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

        let info_array = loaded.info_array.borrow().clone();

        loaded.__account__.info_array = info_array;

        let price = loaded.price;

        loaded.__account__.price = price;

        let flag = loaded.flag;

        loaded.__account__.flag = flag;

        let review_array = loaded.review_array.borrow().clone();

        loaded.__account__.review_array = review_array;

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
    pub name_array: Mutable<[u16; 32]>,
    pub info_array: Mutable<[u16; 256]>,
    pub price: u64,
    pub flag: u8,
    pub review_array: Mutable<[u16; 128]>,
    pub seed_sha256: u64,
}

#[account]
#[derive(Debug)]
pub struct Trainer {
    pub trainer: Pubkey,
    pub name_array: [u16; 32],
    pub location_array: [u16; 64],
    pub info_array: [u16; 256],
    pub seed_random: u64,
    pub gender: u8,
    pub is_active: u8,
    pub phone_array: [u8; 10],
    pub age: u8,
}

impl<'info, 'entrypoint> Trainer {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedTrainer<'info, 'entrypoint>> {
        let trainer = account.trainer.clone();
        let name_array = Mutable::new(account.name_array.clone());
        let location_array = Mutable::new(account.location_array.clone());
        let info_array = Mutable::new(account.info_array.clone());
        let seed_random = account.seed_random;
        let gender = account.gender;
        let is_active = account.is_active;
        let phone_array = Mutable::new(account.phone_array.clone());
        let age = account.age;

        Mutable::new(LoadedTrainer {
            __account__: account,
            __programs__: programs_map,
            trainer,
            name_array,
            location_array,
            info_array,
            seed_random,
            gender,
            is_active,
            phone_array,
            age,
        })
    }

    pub fn store(loaded: Mutable<LoadedTrainer>) {
        let mut loaded = loaded.borrow_mut();
        let trainer = loaded.trainer.clone();

        loaded.__account__.trainer = trainer;

        let name_array = loaded.name_array.borrow().clone();

        loaded.__account__.name_array = name_array;

        let location_array = loaded.location_array.borrow().clone();

        loaded.__account__.location_array = location_array;

        let info_array = loaded.info_array.borrow().clone();

        loaded.__account__.info_array = info_array;

        let seed_random = loaded.seed_random;

        loaded.__account__.seed_random = seed_random;

        let gender = loaded.gender;

        loaded.__account__.gender = gender;

        let is_active = loaded.is_active;

        loaded.__account__.is_active = is_active;

        let phone_array = loaded.phone_array.borrow().clone();

        loaded.__account__.phone_array = phone_array;

        let age = loaded.age;

        loaded.__account__.age = age;
    }
}

#[derive(Debug)]
pub struct LoadedTrainer<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Trainer>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub trainer: Pubkey,
    pub name_array: Mutable<[u16; 32]>,
    pub location_array: Mutable<[u16; 64]>,
    pub info_array: Mutable<[u16; 256]>,
    pub seed_random: u64,
    pub gender: u8,
    pub is_active: u8,
    pub phone_array: Mutable<[u8; 10]>,
    pub age: u8,
}

pub fn customer_confirm_done_handler<'info>(
    mut customer: SeahorseSigner<'info, '_>,
    mut trainer: SeahorseSigner<'info, '_>,
    mut company: SeahorseSigner<'info, '_>,
    mut gymclass: Mutable<LoadedGymClass<'info, '_>>,
    mut review_array: [u16; 128],
) -> () {
    if !(customer.key() == gymclass.borrow().customer) {
        panic!("gymclass student not match");
    }

    if !(gymclass.borrow().flag == 1) {
        panic!("PT not confirm yet");
    }

    assign!(gymclass.borrow_mut().flag, 2);

    let mut pt_money = (gymclass.borrow().price / 100) * 70;
    let mut customer_reward = (gymclass.borrow().price / 100) * 20;

    assign!(gymclass.borrow_mut().review_array, Mutable::<[u16; 128]>::new(review_array));

    {
        let amount = pt_money.clone();

        **gymclass
            .borrow()
            .__account__
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() -= amount;

        **customer
            .clone()
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() += amount;
    };

    {
        let amount = customer_reward.clone();

        **gymclass
            .borrow()
            .__account__
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() -= amount;

        **trainer
            .clone()
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() += amount;
    };

    {
        let amount = ((gymclass.borrow().price - pt_money) - customer_reward);

        **gymclass
            .borrow()
            .__account__
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() -= amount;

        **company
            .clone()
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() += amount;
    };
}

pub fn customer_join_gymclass_handler<'info>(
    mut customer: SeahorseSigner<'info, '_>,
    mut gymclass: Mutable<LoadedGymClass<'info, '_>>,
) -> () {
    if !(gymclass.borrow().__account__.key() == gymclass.borrow().customer) {
        panic!("gymclass already have customer!");
    }

    if !(gymclass.borrow().flag < 2) {
        panic!("gymclass already done!");
    }

    assign!(gymclass.borrow_mut().customer, customer.key());

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &customer.key(),
            &gymclass.borrow().__account__.key(),
            gymclass.borrow().price.clone(),
        ),
        &[
            customer.to_account_info(),
            gymclass.borrow().__account__.to_account_info(),
            customer.programs.get("system_program").clone(),
        ],
    )
    .unwrap();
}

pub fn hide_trainer_account_handler<'info>(
    mut trainer: SeahorseSigner<'info, '_>,
    mut trainer_account: Mutable<LoadedTrainer<'info, '_>>,
) -> () {
    if !(trainer.key() == trainer_account.borrow().trainer) {
        panic!("you are not allowed to hide this account");
    }

    assign!(trainer_account.borrow_mut().is_active, 0);
}

pub fn init_customer_account_handler<'info>(
    mut customer: SeahorseSigner<'info, '_>,
    mut customer_account: Empty<Mutable<LoadedCustomer<'info, '_>>>,
    mut seed_random: u64,
    mut name_array: [u16; 32],
    mut location_array: [u16; 64],
    mut info_array: [u16; 256],
    mut phone_array: [u8; 10],
    mut gender: u8,
) -> () {
    let mut customer_account = customer_account.account.clone();

    assign!(customer_account.borrow_mut().name_array, Mutable::<[u16; 32]>::new(name_array));

    assign!(customer_account.borrow_mut().location_array, Mutable::<[u16; 64]>::new(location_array));

    assign!(customer_account.borrow_mut().info_array, Mutable::<[u16; 256]>::new(info_array));

    assign!(customer_account.borrow_mut().phone_array, Mutable::<[u8; 10]>::new(phone_array));

    assign!(customer_account.borrow_mut().gender, gender);
}

pub fn init_gymclass_handler<'info>(
    mut trainer: SeahorseSigner<'info, '_>,
    mut company: SeahorseSigner<'info, '_>,
    mut gymclass: Empty<Mutable<LoadedGymClass<'info, '_>>>,
    mut seed_sha256: u64,
    mut price: u64,
    mut name_array: [u16; 32],
    mut info_array: [u16; 256],
) -> () {
    let mut gymclass = gymclass.account.clone();

    assign!(gymclass.borrow_mut().trainer, trainer.key());

    assign!(gymclass.borrow_mut().company, company.key());

    assign!(
        gymclass.borrow_mut().customer,
        gymclass.borrow().__account__.key()
    );

    assign!(gymclass.borrow_mut().price, price);

    assign!(gymclass.borrow_mut().name_array, Mutable::<[u16; 32]>::new(name_array));

    assign!(gymclass.borrow_mut().info_array, Mutable::<[u16; 256]>::new(info_array));

    assign!(gymclass.borrow_mut().flag, 0);

    assign!(gymclass.borrow_mut().seed_sha256, seed_sha256);

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &trainer.key(),
            &company.clone().key(),
            100000000,
        ),
        &[
            trainer.to_account_info(),
            company.clone().to_account_info(),
            trainer.programs.get("system_program").clone(),
        ],
    )
    .unwrap();
}

pub fn init_trainer_account_handler<'info>(
    mut trainer: SeahorseSigner<'info, '_>,
    mut trainer_account: Empty<Mutable<LoadedTrainer<'info, '_>>>,
    mut seed_random: u64,
    mut name_array: [u16; 32],
    mut location_array: [u16; 64],
    mut info_array: [u16; 256],
    mut phone_array: [u8; 10],
    mut age: u8,
    mut gender: u8,
) -> () {
    let mut trainer_account = trainer_account.account.clone();

    assign!(trainer_account.borrow_mut().name_array, Mutable::<[u16; 32]>::new(name_array));

    assign!(trainer_account.borrow_mut().location_array, Mutable::<[u16; 64]>::new(location_array));

    assign!(trainer_account.borrow_mut().info_array, Mutable::<[u16; 256]>::new(info_array));

    assign!(trainer_account.borrow_mut().phone_array, Mutable::<[u8; 10]>::new(phone_array));

    assign!(trainer_account.borrow_mut().gender, gender);

    assign!(trainer_account.borrow_mut().age, age);

    assign!(trainer_account.borrow_mut().is_active, 1);
}

pub fn pt_confirm_done_handler<'info>(
    mut trainer: SeahorseSigner<'info, '_>,
    mut gymclass: Mutable<LoadedGymClass<'info, '_>>,
) -> () {
    if !(trainer.key() == gymclass.borrow().trainer) {
        panic!("gymclass trainer not");
    }

    if !(gymclass.borrow().flag == 0) {
        panic!("customer cannot confirm before PT");
    }

    assign!(gymclass.borrow_mut().flag, 1);
}

pub fn pt_decline_customer_handler<'info>(
    mut trainer: SeahorseSigner<'info, '_>,
    mut customer: SeahorseSigner<'info, '_>,
    mut gymclass: Mutable<LoadedGymClass<'info, '_>>,
) -> () {
    if !(trainer.key() == gymclass.borrow().trainer) {
        panic!("gymclass trainer not same!");
    }

    if !(customer.key() == gymclass.borrow().customer) {
        panic!("Customer is not in gymclass!");
    }

    if !(gymclass.borrow().flag < 2) {
        panic!("gymclass already done!");
    }

    {
        let amount = gymclass.borrow().price.clone();

        **gymclass
            .borrow()
            .__account__
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() -= amount;

        **customer
            .clone()
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() += amount;
    };

    assign!(
        gymclass.borrow_mut().customer,
        gymclass.borrow().__account__.key()
    );
}

pub fn trainer_hide_gymclass_handler<'info>(
    mut trainer: SeahorseSigner<'info, '_>,
    mut gymclass: Mutable<LoadedGymClass<'info, '_>>,
) -> () {
    if !(trainer.key() == gymclass.borrow().trainer) {
        panic!("you are not trainer");
    }

    assign!(gymclass.borrow_mut().flag, 3);
}

pub fn update_customer_account_handler<'info>(
    mut customer: SeahorseSigner<'info, '_>,
    mut customer_account: Mutable<LoadedCustomer<'info, '_>>,
    mut name_array: [u16; 32],
    mut location_array: [u16; 64],
    mut info_array: [u16; 256],
    mut phone_array: [u8; 10],
) -> () {
    if !(customer.key() == customer_account.borrow().customer) {
        panic!("you are not allowed to update this account");
    }

    assign!(customer_account.borrow_mut().name_array, Mutable::<[u16; 32]>::new(name_array));

    assign!(customer_account.borrow_mut().location_array, Mutable::<[u16; 64]>::new(location_array));

    assign!(customer_account.borrow_mut().info_array, Mutable::<[u16; 256]>::new(info_array));

    assign!(customer_account.borrow_mut().phone_array, Mutable::<[u8; 10]>::new(phone_array));
}

pub fn update_gymclass_handler<'info>(
    mut trainer: SeahorseSigner<'info, '_>,
    mut gymclass: Mutable<LoadedGymClass<'info, '_>>,
    mut price: u64,
    mut name_array: [u16; 32],
    mut info_array: [u16; 256],
    mut flag: u8,
) -> () {
    if !(trainer.key() == gymclass.borrow().trainer) {
        panic!("gymclass trainer not");
    }

    assign!(gymclass.borrow_mut().price, price);

    assign!(gymclass.borrow_mut().name_array, Mutable::<[u16; 32]>::new(name_array));

    assign!(gymclass.borrow_mut().info_array, Mutable::<[u16; 256]>::new(info_array));

    assign!(gymclass.borrow_mut().flag, flag);
}

pub fn update_trainer_account_handler<'info>(
    mut trainer: SeahorseSigner<'info, '_>,
    mut trainer_account: Mutable<LoadedTrainer<'info, '_>>,
    mut name_array: [u16; 32],
    mut location_array: [u16; 64],
    mut info_array: [u16; 256],
    mut phone_array: [u8; 10],
) -> () {
    if !(trainer.key() == trainer_account.borrow().trainer) {
        panic!("you are not allowed to update this account");
    }

    assign!(trainer_account.borrow_mut().name_array, Mutable::<[u16; 32]>::new(name_array));

    assign!(trainer_account.borrow_mut().location_array, Mutable::<[u16; 64]>::new(location_array));

    assign!(trainer_account.borrow_mut().info_array, Mutable::<[u16; 256]>::new(info_array));

    assign!(trainer_account.borrow_mut().phone_array, Mutable::<[u8; 10]>::new(phone_array));
}
