import fastapi.middleware
import fastapi.middleware.cors
import uvicorn
from dotenv import load_dotenv
load_dotenv()
import json
from baseAPI import *
from hotaSolana.hotaSolanaData import *

from config import *
import fastapi

app = fastapi.FastAPI()
client = HotaSolanaRPC(PROGRAM_ID, False, "devnet")

origins = ["*"]

app.add_middleware(
    fastapi.middleware.cors.CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
@BaseInstructionDataClass(name="init_gymclass")
class GymClassInitInstruction:
    seed_sha256=HotaUint64()
    price=HotaUint64()
    location=HotaStringUTF16(lenArr=64)
    name= HotaStringUTF16(lenArr=32)
    info=HotaStringUTF16(lenArr=256)

@BaseStructClass
class GymClassData:
    company = HotaPublicKey() 
    trainer = HotaPublicKey()
    customer = HotaPublicKey()
    name= HotaStringUTF16(lenArr=32)
    location= HotaStringUTF16(lenArr=64)
    info= HotaStringUTF16(lenArr=256)
    price =HotaUint64()
    is_done =HotaUint8()
    seed_sha256 =HotaUint64()

class InitGymClassModel(BaseModel):
    name: str
    location: str
    info: str
    price: int

@app.post("/init-gymclass")
async def init_gym_class(
    trainerPrivateKey: str,
    initGymClassModel: InitGymClassModel,
):
    def fun():
        company_keypair = makeKeyPair(OWNER_PRIVATE_KEY)

        instruction_data = GymClassInitInstruction()
        instruction_data.get("seed_sha256").random()
        instruction_data.get("price").object2struct(initGymClassModel.price)
        instruction_data.get("location").object2struct(initGymClassModel.location)
        instruction_data.get("name").object2struct(initGymClassModel.name)
        instruction_data.get("info").object2struct(initGymClassModel.info)

        gym_class_pubkey = findProgramAddress(createBytesFromArrayBytes(
            company_keypair.public_key.byte_value,
            "gymclass".encode("utf-8"),
            bytes(instruction_data.get("seed_sha256").serialize()),
        ), client.program_id)
        
        instruction_address = client.send_transaction(
            instruction_data=instruction_data,
            pubkeys=[
                makeKeyPair(trainerPrivateKey).public_key,
                company_keypair.public_key,
                gym_class_pubkey,
                makePublicKey(sysvar_rent),
                makePublicKey(system_program),
            ],
            keypairs=[
                makeKeyPair(trainerPrivateKey),
                company_keypair,
            ],
            fee_payer=company_keypair.public_key
        )

        return {
            "instruction_address": instruction_address,
            "gym_class_public_key": bs58.encode(gym_class_pubkey.byte_value),
        }
    return make_response_auto_catch(fun)

@app.get("/get-all-gym-classes-data")
async def get_all_gym_classes_data():
    def fun():
        accounts = client.get_program_accounts()
        data = []
        for i in range(len(accounts)):
            try:
                account_data = client.get_account_data(accounts[i].pubkey, GymClassData, [8, 4])
                data.append(account_data)
            except Exception as e:
                pass
        return data
    return make_response_auto_catch(fun)


#### Common function1
@app.post("/convert-keypair-to-private-key")
async def convert_keypair_to_private_key(file: fastapi.UploadFile):
    # Bytes to string
    result = file.file.read()
    keypair_json = json.loads(result)
    keypair_bytes = bytes(keypair_json)
    return {
        "public_key": bs58.encode(keypair_bytes[32:]),
        "private_key": bs58.encode(keypair_bytes),
    }

@app.get("/get-gym-class-info")
async def get_gym_class_info(public_key: str):
    return make_response_auto_catch(lambda: client.get_account_info(PublicKey(public_key)))

@app.get("/get-gym-class-data")
async def get_parking_area_data(public_key: str):
    return make_response_auto_catch(lambda: client.get_account_data(PublicKey(public_key), GymClassData, [8, 4]))
@app.get("/get-balance")
async def get_balance(public_key: str):
    return make_response_auto_catch(client.get_balance(public_key))

@app.post("/airdrop")
async def airdrop(public_key: str, amount: int = 1):
    return make_response_auto_catch(client.drop_sol(public_key, amount))

if __name__ == "__main__":
    uvicorn.run("main:app", host="localhost", port=8000, reload=True)