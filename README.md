# Solana Transfer API (Rust + Actix-Web)

This project is a simple **Solana transaction API** built with Rust and Actix-Web.  
It allows you to:
- Check balances of Solana accounts (in SOL)
- Transfer SOL from one account to another

---

## Features
- **POST /transaction** → Transfer SOL between two accounts
- Uses **Solana Devnet**
- Reads sender's keypair from a file path
- Returns transaction signature and updated balances

---

## API Usage

### 1. POST /transaction
Transfers SOL from a sender's wallet to a receiver.

**Request Body (JSON)**:
```json
{
  "payer_id": "/path/to/your/wallet-keypair.json",
  "reciever_id": "ReceiverPublicKeyHere",
  "amount_in_sol": 1.00
}
```
- `payer_id`: Path to the sender's Solana keypair JSON file  
- `reciever_id`: Base58 Solana public key of the receiver  
- `amount_in_sol`: Amount to send in SOL (can be decimal)

**Example POSTMAN Request:**
- Method: `POST`
- URL: `http://127.0.0.1:8080/transaction`
- Headers: `Content-Type: application/json`
- Body: Raw JSON as shown above

**Example Response:**
```json
{
  "receiver_balance_sol": 20.01,
  "sender_balance_sol": 5.78114092,
  "signature": "5G1d8WGzfQ9Afh4XAzr9QNa1UbX9qric5jsdWUvX4PCk8ZZ4S9rwL279UPwQQHvbSPnf8ziMJHjFp3ua36GeAyoXR"
}
```

---

## Development Setup

### Prerequisites
- Rust (latest stable)
- Cargo
- Solana CLI
- Postman (optional, for testing)

### Clone the repository
```sh
git clone https://github.com/YOUR_ORG/YOUR_REPO.git
cd YOUR_REPO
```

### Run the server
```sh
cargo run
```
Server will start on `http://127.0.0.1:8080`.

---

## Example Postman Request Screenshot
![Postman Example](Screenshot_from_2025-08-12_01-56-42.png)

---

## License
MIT License