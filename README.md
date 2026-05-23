# Stellar Task Tracker DApp

**Stellar Task Tracker DApp** - Blockchain-Based Decentralized Task Management System

## Project Description

Stellar Task Tracker DApp is a decentralized smart contract application built on the Stellar blockchain using Soroban SDK and Rust programming language. This project allows users to manage their daily tasks securely and transparently on the blockchain without relying on centralized servers.

The smart contract enables users to create tasks, view all stored tasks, mark tasks as completed, and delete tasks permanently from storage. Every task is stored directly inside the Stellar blockchain using Soroban instance storage, ensuring transparency, reliability, and tamper-resistant data management.

This project demonstrates the implementation of CRUD (Create, Read, Update, Delete) functionality using blockchain technology in a simple and beginner-friendly way.

---

# Project Vision

Our vision is to create a decentralized productivity system where users fully control their task management data through blockchain technology.

We aim to:

* Eliminate dependency on centralized task management services
* Provide transparent and immutable task tracking
* Improve trust and ownership of personal productivity data
* Introduce blockchain technology in a simple and practical use case
* Encourage students and beginner developers to learn Web3 development using Soroban and Stellar

---

# Key Features

## 1. Create Task

Users can create new tasks with:

* Unique task ID
* Task title
* Completion status

Each task is automatically stored on-chain.

---

## 2. View All Tasks

Retrieve all stored tasks directly from blockchain storage.

Features:

* Fast retrieval
* Real-time blockchain synchronization
* Structured task data

---

## 3. Complete Task

Users can mark tasks as completed.

Benefits:

* Easy progress tracking
* Simple task status management
* On-chain update mechanism

---

## 4. Delete Task

Users can permanently remove tasks from blockchain storage using task ID.

---

## 5. Blockchain Transparency

All task activities are:

* Transparent
* Immutable
* Secure
* Decentralized

Powered by Stellar blockchain infrastructure.

---

# Smart Contract Functions

## `create_task()`

Create and store a new task on the blockchain.

## `get_tasks()`

Retrieve all existing tasks from storage.

## `complete_task()`

Update task status into completed.

## `delete_task()`

Delete a task using its unique ID.

---

# Contract Details

## Smart Contract Address

```txt
CCC66IICDTAHQTLMC6BFBFNB7UY6U26YHMOMALDLMLNJVYSMJNNWK47Y
```

Network:

* Stellar Soroban Testnet

---

# Future Improvements

## Short-Term Features

* Add task categories
* Add task deadlines
* Add priority levels
* Add search and filtering system

## Medium-Term Features

* Wallet-based task ownership
* Multi-user collaboration
* Task sharing between wallets
* Notification integration

## Long-Term Vision

* AI-based productivity assistant
* Decentralized frontend hosting
* Cross-chain compatibility
* DAO governance system
* Mobile application integration

---

# Technical Stack

* Rust Programming Language
* Soroban SDK
* Stellar Blockchain
* Smart Contract Technology

---

# Getting Started

## Requirements

* Rust
* Soroban CLI
* Stellar Wallet
* Soroban SDK

## Deploy Contract

Build and deploy the smart contract using Soroban CLI or Soroban Studio.

## Main Functions

* `create_task()` → Add new task
* `get_tasks()` → View all tasks
* `complete_task()` → Mark task as completed
* `delete_task()` → Delete task

---

# Repository Structure

```txt
project/
├── contract/
│   └── src/
│       ├── lib.rs
│       └── test.rs
├── Cargo.toml
└── README.md
```

---

# Educational Purpose

This project was created as part of Stellar Soroban smart contract learning and workshop practice to understand blockchain-based decentralized applications.

---

# Author

Built using Stellar Soroban Smart Contract with Rust.

---

# Stellar Task Tracker DApp

Manage your productivity securely on the blockchain.
Contract ID = CCC66IICDTAHQTLMC6BFBFNB7UY6U26YHMOMALDLMLNJVYSMJNNWK47Y
