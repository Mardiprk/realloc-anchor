# 📝 realloc-anchor

A simple **Solana Anchor program** that shows how to use  
`#[account(realloc = ...)]` for dynamically resizing accounts.  

We build a **Note-taking dApp** where a user can:
- Initialize a note PDA with some text
- Append more text later by reallocating space

---

## ⚡ Tech Stack
- [Solana](https://solana.com) — High-performance blockchain  
- [Anchor Framework](https://www.anchor-lang.com/) — Developer framework for Solana  
- [Rust](https://www.rust-lang.org/) — Smart contract language  

---

## 📂 Project Structure
programs/note/src/lib.rs -> Smart contract (Anchor + Rust)
tests/note.ts -> Mocha tests with Anchor

yaml
Copy code

---

## 🔨 How it Works
1. **Initialize** — creates a new PDA (`noteAccount`) with initial text.  
2. **Append** — reallocates account space and appends new text.  

---

## ▶️ Run Locally

```bash
# Install deps
yarn install

# Build the program
anchor build

# Run tests
anchor test
✅ Example Output
yaml
Copy code
User: 6eRRxAcNXE1FB1TXQrU8wgvj31LMoKvzSMfii9Xem52r
Note PDA: 66gLCJhm6gLbrGJJ4LkYAGxdEhmVmP2Q3dKBTfeRSDTA

☑️ Initialized!
Stored content: Hello

☑️ Append!
Updated content: Hello Solana is very Fast!

Made with ❤️ using Solana + Anchor + Rust
