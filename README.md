# Solana Wallet API (Actix-Web + Solana Rust SDK)

This is a simple Solana Devnet wallet API built using **Rust**, **Actix-Web**, and the **Solana Rust SDK**.

It allows you to:
- Get the SOL balance of any public key.
- Fetch detailed transaction history for a given public key.
- Send SOL between two accounts.

![Screenshot](./Screenshot-from-2025-08-12-01-56-42.png)

---

## 📦 Features
- **GET /balance/{pubkey}** → Returns SOL balance of a given public key.
- **GET /transactions/{pubkey}** → Returns full parsed transaction history (using `get_transaction` with `JsonParsed`).
- **POST /transfer** → Sends SOL from a payer to a receiver and returns updated balances.

---

## 🛠 Setup

1. **Clone the repo**
```bash
git clone https://github.com/YOUR_ORG/YOUR_REPO.git
cd YOUR_REPO
```

2. **Install Rust (if not already installed)**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. **Run the server**
```bash
cargo run
```

---

## 📡 API Endpoints

### **1️⃣ Get Balance**
**Request**
```
GET /balance/{pubkey}
```
**Response**
```json
{
  "pubkey": "YourPublicKeyHere",
  "balance_sol": 7.00545555
}
```

---

### **2️⃣ Get Full Transaction History**
Fetches recent signatures for the given address, then retrieves full transaction details for each signature.

**Request**
```
GET /transaction/full/{pubkey}
```

**Example Response**
```json
[
    {
        "slot": 400583546,
        "transaction": {
            "signatures": [
                "2VNcgbBaREwM6TeMvqaoP2RS8qkvm3RLXjWq7VY7fQvRSqbh1pHnpqYJ5NAtfRap5PfxnSAWKTcWTXnBZDqnc4QW"
            ],
            "message": {
                "accountKeys": [
                    {
                        "pubkey": "3KNrwyYGR4dsTbAUqPrDm1uT6WYkWDxTKk2HLsGgkH7Y",
                        "writable": true,
                        "signer": true,
                        "source": "transaction"
                    },
                    {
                        "pubkey": "9jEv35V6E8fYhyMzrK2bHHicsjTB9WzRYfFGrTYWQk1A",
                        "writable": true,
                        "signer": false,
                        "source": "transaction"
                    },
                    {
                        "pubkey": "11111111111111111111111111111111",
                        "writable": false,
                        "signer": false,
                        "source": "transaction"
                    }
                ],
                "recentBlockhash": "BjehXvtRsuSAa5fNWXnRXVGkec22b8ZY18dYU7ksSU6K",
                "instructions": [
                    {
                        "program": "system",
                        "programId": "11111111111111111111111111111111",
                        "parsed": {
                            "info": {
                                "destination": "9jEv35V6E8fYhyMzrK2bHHicsjTB9WzRYfFGrTYWQk1A",
                                "lamports": 1000000000,
                                "source": "3KNrwyYGR4dsTbAUqPrDm1uT6WYkWDxTKk2HLsGgkH7Y"
                            },
                            "type": "transfer"
                        },
                        "stackHeight": 1
                    }
                ]
            }
        }
    }
]
```

---

### **3️⃣Get Transaction History**
Fetches recent signatures for the given address, then retrieves full transaction details for each signature.

**Request**
```
GET /transaction/{pubkey}
```

**Example Response**
```json
[
  {
    "signature": "5hR8X2...abc",
    "slot": 184030300,
    "block_time": 1710888888,
    "meta": {
      "fee": 5000,
      "pre_balances": [...],
      "post_balances": [...]
    },
    "transaction": {
      "message": { ... },
      "signatures": [ "5hR8X2...abc" ]
    }
  }
]
```

---

### **4️⃣ Transfer SOL**
**Request**
```
POST /transfer
Content-Type: application/json

{
  "payer_id": "/path/to/payer/keypair.json",
  "receiver_id": "ReceiverPublicKeyHere",
  "amount_in_sol": 0.01
}
```

**Response**
```json
{
  "signature": "3nH4s8...xyz",
  "sender_balance_sol": 7.00445555,
  "receiver_balance_sol": 18.015
}
```

---

## 📌 Notes
- `payer_id` in `/transfer` should be the **path to the keypair file**, not the private key string.
- Uses Solana **Devnet** RPC URL by default (`https://api.devnet.solana.com`).
- Balances are returned in SOL, not lamports.
