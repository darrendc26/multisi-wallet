# Solana Multisig Program
A secure and efficient multisig (multi-signature) program built on Solana using the Anchor framework. This program allows multiple parties to collectively control funds and execute transactions with configurable approval thresholds.

## Features
Multi-signature wallet creation with configurable owners and threshold

Transaction proposal system for any Solana instruction

Approval workflow requiring threshold consensus before execution

Secure execution with PDA (Program Derived Address) signing

SOL and SPL token support through generic instruction handling

Approval removal functionality for changed decisions

Event emission for off-chain monitoring and indexing

## Program Overview
Core Components
Multisig Account: Stores owners, threshold, and metadata

Transaction Account: Stores proposed instructions and approvals

Instruction Handlers: Create, propose, approve, execute, and remove approvals

Prerequisites
Rust 1.70+

Solana CLI 1.16+

Anchor Framework 0.31.1

Node.js 16+

## Clone and Build
```bash
# Clone the repository
git clone https://github.com/darrendc26/multisig-wallet
cd multisig-wallet

# Install dependencies
npm install

# Build the program
anchor build

# Run tests
anchor test
```

## Deploy
```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet-beta (use with caution)
anchor deploy --provider.cluster mainnet-beta
```

## Architecture
### Program Instructions
create_multisig: Create a new multisig account 
propose_txn: Propose a new transaction
approve_txn: Approve a proposed transaction
execute_txn: Execute an approved transaction
remove_approval: Remove an existing approval

### Program Accounts
Multisig: Stores owners, threshold, and metadata
Transaction: Stores proposed instructions and approvals

### Program State
#### Multisig
- owners: List of owners
- creator: Creator of the multisig account
- threshold: Minimum number of approvals required for transaction execution
- nonce: Incremented for each transaction
- bump: PDA bump for multisig account

#### Transaction
- multisig: Multisig account that created the transaction
- proposer: Proposer of the transaction
- instructions: List of proposed instructions
- signers: List of signers
- executed: Indicates if the transaction has been executed
- nonce: Incremented for each transaction
- bump: PDA bump for transaction account

#### Instruction Data
- program_id: Solana program ID
- accounts: List of accounts involved in the instruction
- data: Instruction data

## Usage
### Create a Multisig Account
To create a new multisig account, you need to provide a list of owners and a threshold.

### Propose a Transaction
To propose a new transaction, you need to provide a list of instructions and the multisig account.

### Approve a Transaction
To approve a proposed transaction, you need to provide the multisig account and the approver.

### Execute a Transaction
To execute an approved transaction, you need to provide the multisig account and the executor.

### Remove an Approval
To remove an approval, you need to provide the multisig account and the remover.


## Security Features
Threshold-based execution: Requires M-of-N approvals

PDA signing: Only the multisig PDA can execute instructions

Owner validation: Only registered owners can propose/approve

Replay protection: Nonce-based transaction uniqueness

State validation: Comprehensive constraint checks

Duplicate prevention: Owners cannot approve twice

## Error Codes
- 6000	InvalidNumberOfOwners	
- 6001	InvalidThreshold	
- 6002	DuplicateOwners	
- 6003	Unauthorized	
- 6004	EmptyInstructions	
- 6005	InvalidNumberOfInstructions	
- 6006	TransactionExecuted	
- 6007	AlreadySigned	
- 6008	NotEnoughSigners	
- 6009	MissingAccount	
- 6010	NotSigned	

