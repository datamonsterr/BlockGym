import uvicorn
import os
from dotenv import load_dotenv
load_dotenv()
from solathon import Client, PublicKey

from config import *
import fastapi

app = fastapi.FastAPI()
PORT = int(os.getenv("PORT", 8000))
client = Client("https://api.devnet.solana.com")

@app.get("/init_gym_class")
def init_gym_class():
    return {"title": "World"}

@app.get("/get_balance")
def get_balance(pubkey: str):
    pubkey = PublicKey(pubkey)
    balance = client.get_balance(pubkey)
    return {"balance": balance}

if __name__ == "__main__":
    uvicorn.run(app, host="localhost", port=PORT)