# blockchain
# Built with Seahorse v0.2.7

from seahorse.prelude import *

declare_id('FwrxzVAS7hdsWmnGmm1isX9u24wRpNiKiKR3RTaBbSDx')
class Trainer(Account):
    trainer: Pubkey # 32 byte
    name_u16_32_array: Array[u16, 32] # 32 characters max = 32 bytes
    location_u16_64_array: Array[u16, 64] # 64 characters max = 64 bytes # City, Nation
    info_u16_256_array: Array[u16, 256] # 256 characters max = 256 bytes
    seed_random: u64 # 8 bytes
    gender: u8 # 1 byte
    is_active: u8 # 1 byte
    phone_u8_10_array: Array[u8, 10] # 5 characters max = 5 bytes
    age: u8 # 1 byte

class Customer(Account):
    customer: Pubkey # 32 byte
    name_u16_32_array: Array[u16, 32] # 32 characters max = 32 bytes
    location_u16_64_array: Array[u16, 64] # 64 characters max = 64 bytes # City, Nation
    info_u16_256_array: Array[u16, 256] # 256 characters max = 256 bytes
    gender: u8 # 1 byte
    phone_u8_10_array: Array[u8, 10] # 5 characters max = 5 bytes
    age: u8 # 1 byte
    seed_random: u64 # 8 bytes

class GymClass(Account):
    company: Pubkey # 32 bytes # Program
    trainer: Pubkey # 32 bytes # PT
    customer: Pubkey # 32 bytes # User
    name_u16_32_array: Array[u16, 32] # 32 characters max = 32 bytes
    info_u16_256_array: Array[u16, 256] # 256 characters max = 256 bytes
    price: u64 # 8 bytes # x (sol) 
    flag: u8 # 1: PT Confirmed, 2: Customer Reviewed, 3: Hidden
    review_u16_128_array: Array[u16, 128] # 128 characters max = 128 bytes
    seed_sha256: u64 # 8 bytes

    

# PT create a class by signing a contract with our company
@instruction
def init_gymclass(
    trainer: Signer,
    company: Signer,
    gymclass: Empty[GymClass],
    seed_sha256: u64,
    price: u64,
    name_u16_32_array: Array[u16, 32],
    info_u16_256_array: Array[u16, 256],
):
    gymclass = gymclass.init(payer = company, seeds = [company, "gymclass", seed_sha256])
    gymclass.trainer = trainer.key()
    gymclass.company = company.key()
    gymclass.customer = gymclass.key()
    gymclass.price = price
    gymclass.name_u16_32_array = name_u16_32_array
    gymclass.info_u16_256_array = info_u16_256_array
    gymclass.flag = 0
    gymclass.seed_sha256 = seed_sha256
    
    trainer.transfer_lamports(company, 100000000) # 0.1 sol for creating class

@instruction
def customer_join_gymclass(
    customer: Signer,
    gymclass: GymClass
):
    assert gymclass.key() == gymclass.customer, "gymclass already have customer!"
    assert gymclass.flag < 2, "gymclass already done!"
    gymclass.customer = customer.key()
    customer.transfer_lamports(gymclass, gymclass.price)

@instruction
def pt_decline_customer(
    trainer: Signer,
    customer: Signer,
    gymclass: GymClass
):
    assert trainer.key() == gymclass.trainer, "gymclass trainer not same!"
    assert customer.key() == gymclass.customer, "Customer is not in gymclass!"
    assert gymclass.flag < 2, "gymclass already done!"

    gymclass.transfer_lamports(customer, gymclass.price)   
    gymclass.customer = gymclass.key()

@instruction
def update_gymclass(
    trainer: Signer,
    gymclass: GymClass,
    price: u64,
    name_u16_32_array: Array[u16, 32],
    info_u16_256_array: Array[u16, 256],
    flag: u8
):
    assert trainer.key() == gymclass.trainer, "gymclass trainer not"
    gymclass.price = price
    gymclass.name_u16_32_array = name_u16_32_array
    gymclass.info_u16_256_array = info_u16_256_array
    gymclass.flag = flag

@instruction
def pt_confirm_done(
    trainer: Signer,
    gymclass: GymClass
):
    assert trainer.key() == gymclass.trainer, "gymclass trainer not"
    assert gymclass.flag == 0, "customer cannot confirm before PT"
    gymclass.flag = 1
    
@instruction
def customer_confirm_done(
    customer: Signer,
    trainer: Signer,
    company: Signer,
    gymclass: GymClass,
    review_u16_128_array: Array[u16, 128]
):
    assert customer.key() == gymclass.customer, "gymclass student not match"
    assert gymclass.flag == 1, "PT not confirm yet"

    gymclass.flag = 2
    pt_money = gymclass.price // 100 * 70 
    customer_reward = gymclass.price // 100 * 20
    gymclass.review_u16_128_array = review_u16_128_array
    gymclass.transfer_lamports(customer, pt_money)
    gymclass.transfer_lamports(trainer, customer_reward)
    gymclass.transfer_lamports(company, gymclass.price - pt_money - customer_reward)

@instruction
def trainer_hide_gymclass(
    trainer: Signer,
    gymclass: GymClass
):
    assert trainer.key() == gymclass.trainer, "you are not trainer"
    gymclass.flag = 3

@instruction
def init_trainer_account(
    trainer: Signer,
    trainer_account: Empty[Trainer],
    seed_random: u64,
    name_u16_32_array: Array[u16, 32],
    location_u16_64_array: Array[u16, 64],
    info_u16_256_array: Array[u16, 256],
    phone_u8_10_array: Array[u8, 10],
    age: u8,
    gender: u8
):
    trainer_account = trainer_account.init(payer = trainer, seeds = [trainer, "trainer", seed_random])
    trainer_account.name_u16_32_array = name_u16_32_array
    trainer_account.location_u16_64_array = location_u16_64_array
    trainer_account.info_u16_256_array = info_u16_256_array
    trainer_account.phone_u8_10_array = phone_u8_10_array
    trainer_account.gender = gender
    trainer_account.age = age
    trainer_account.is_active = 1
    

@instruction
def update_trainer_account(
    trainer: Signer,
    trainer_account: Trainer,
    name_u16_32_array: Array[u16, 32],
    location_u16_64_array: Array[u16, 64],
    info_u16_256_array: Array[u16, 256],
    phone_u8_10_array: Array[u8, 10],
):
    assert trainer.key() == trainer_account.trainer, "you are not allowed to update this account"
    trainer_account.name_u16_32_array = name_u16_32_array
    trainer_account.location_u16_64_array = location_u16_64_array
    trainer_account.info_u16_256_array = info_u16_256_array
    trainer_account.phone_u8_10_array = phone_u8_10_array

@instruction
def hide_trainer_account(
    trainer: Signer,
    trainer_account: Trainer
):
    assert trainer.key() == trainer_account.trainer, "you are not allowed to hide this account"
    trainer_account.is_active = 0

@instruction
def init_customer_account(
    customer: Signer,
    customer_account: Empty[Customer],
    seed_random: u64,
    name_u16_32_array: Array[u16, 32],
    location_u16_64_array: Array[u16, 64],
    info_u16_256_array: Array[u16, 256],
    phone_u8_10_array: Array[u8, 10],
    gender: u8
):
    customer_account = customer_account.init(payer = customer, seeds = [customer, "customer", seed_random])
    customer_account.name_u16_32_array = name_u16_32_array
    customer_account.location_u16_64_array = location_u16_64_array
    customer_account.info_u16_256_array = info_u16_256_array
    customer_account.phone_u8_10_array = phone_u8_10_array
    customer_account.gender = gender
    

@instruction
def update_customer_account(
    customer: Signer,
    customer_account: Customer,
    name_u16_32_array: Array[u16, 32],
    location_u16_64_array: Array[u16, 64],
    info_u16_256_array: Array[u16, 256],
    phone_u8_10_array: Array[u8, 10],
):
    assert customer.key() == customer_account.customer, "you are not allowed to update this account"
    customer_account.name_u16_32_array = name_u16_32_array
    customer_account.location_u16_64_array = location_u16_64_array
    customer_account.info_u16_256_array = info_u16_256_array
    customer_account.phone_u8_10_array = phone_u8_10_array
