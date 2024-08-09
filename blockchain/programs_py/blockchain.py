# blockchain
# Built with Seahorse v0.2.7

from seahorse.prelude import *

declare_id('poTfdHjWbSsodLv1npNwAAtN4Cpa1hHTwHJJ9jXbvad')

class GymClass(Account):
    company: Pubkey # 32 bytes # Program
    trainer: Pubkey # 32 bytes # PT
    customer: Pubkey # 32 bytes # User
    name_u16_32_array: Array[u16, 32] # 32 characters max = 32 bytes
    location_u16_64_array: Array[u16, 64] # 64 characters max = 64 bytes # City, Nation
    info_u16_256_array: Array[u16, 256] # 256 characters max = 256 bytes
    price: u64 # 8 bytes # x (sol) 
    is_done: u8 # 1 byte
    seed_sha256: u64 # 8 bytes
    # total 32 + 32 + 32 + 32 + 64 + 256 + 8 + 1 = 417 bytes
    

# PT create a class by signing a contract with our company
@instruction
def init_gymclass(
    trainer: Signer,
    company: Signer,
    gymclass: Empty[GymClass],
    seed_sha256: u64,
    price: u64,
    location_u16_64_array: Array[u16, 64],
    name_u16_32_array: Array[u16, 32],
    info_u16_256_array: Array[u16, 256],
):
    gymclass = gymclass.init(payer = company, seeds = [company, "gymclass", seed_sha256])
    gymclass.trainer = trainer.key()
    gymclass.company = company.key()
    gymclass.customer = gymclass.key()
    gymclass.price = price
    gymclass.location_u16_64_array = location_u16_64_array
    gymclass.name_u16_32_array = name_u16_32_array
    gymclass.info_u16_256_array = info_u16_256_array
    gymclass.is_done = 0
    gymclass.seed_sha256 = seed_sha256
    
    trainer.transfer_lamports(company, 100000000) # 0.1 sol for creating class

@instruction
def customer_join_gymclass(
    customer: Signer,
    gymclass: GymClass
):
    assert gymclass.key() == gymclass.customer, "gymclass already have customer!"
    assert gymclass.is_done < 2, "gymclass alr0eady done!"
    gymclass.customer = customer.key()
    customer.transfer_lamports(gymclass, gymclass.price)

@instruction
def pt_decline_customer(
    trainer: Signer,
    customer: Signer,
    company: Signer,
    gymclass: GymClass
):
    assert trainer.key() == gymclass.trainer, "gymclass trainer not same!"
    assert customer.key() == gymclass.customer, "Customer is not in gymclass!"
    assert gymclass.is_done < 2, "gymclass already done!"

    gymclass.transfer_lamports(customer, gymclass.price)   
    gymclass.customer = gymclass.key()

@instruction
def pt_confirm_done(
    trainer: Signer,
    gymclass: GymClass
):
    assert trainer.key() == gymclass.trainer, "gymclass trainer not"
    assert gymclass.is_done == 0, "customer cannot confirm before PT"
    gymclass.is_done = 1
    
@instruction
def customer_confirm_done(
    customer: Signer,
    trainer: Signer,
    company: Signer,
    gymclass: GymClass
):
    assert customer.key() == gymclass.customer, "gymclass student not"
    assert gymclass.is_done == 1, "PT not confirm yet"

    gymclass.is_done = 2
    pt_money = gymclass.price // 100 * 70 
    customer_reward = gymclass.price // 100 * 20
    gymclass.transfer_lamports(customer, pt_money)
    gymclass.transfer_lamports(trainer, customer_reward)
    gymclass.transfer_lamports(company, gymclass.price - pt_money - customer_reward)

