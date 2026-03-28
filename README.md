# file-storage-dapp
# File Storage dApp

---

## Table of Contents

- [Project Title](#file-storage-dapp)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Future Scope](#future-scope)

---

## Project Description

**File Storage dApp** is a decentralized application built on the **Stellar blockchain** using the **Soroban smart contract SDK**. It allows users to upload, retrieve, and manage file records on-chain in a transparent and tamper-proof manner. Instead of storing raw file data on-chain (which would be prohibitively expensive), the contract stores file **metadata** — including the file name, owner identity, upload timestamp, and a **content hash** (e.g., an IPFS CID) that points to the actual file stored off-chain.

The smart contract exposes four clean functions:

| Function | Description |
|---|---|
| `upload_file` | Registers a new file record on-chain and returns its unique ID |
| `get_file` | Retrieves a file record by its unique file ID |
| `delete_file` | Soft-deletes a file record (marks it as deleted without erasing history) |
| `view_storage_stats` | Returns global platform statistics (total, active, deleted file counts) |

---

## Project Vision

The vision behind the **File Storage dApp** is to bring **ownership, transparency, and immutability** to file management using blockchain technology. Traditional centralized storage solutions are prone to data censorship, unauthorized deletion, and single points of failure. By anchoring file metadata on the Stellar blockchain via Soroban smart contracts, this dApp aims to:

- Give users **provable, on-chain proof of file ownership and upload time**.
- Enable **trustless verification** — anyone can confirm whether a file existed at a given timestamp without relying on a central authority.
- Serve as a **foundation layer** for decentralized applications that need auditable file management, such as document notarization, media provenance tracking, and decentralized cloud storage frontends.

---

## Key Features

- **On-chain File Registration** — Upload file metadata (name, owner, content hash, timestamp) to the Stellar blockchain with a unique auto-incremented file ID.
- **Content Hash Storage** — Stores an IPFS hash or any content-addressed hash, enabling integration with decentralized storage networks like IPFS or Filecoin.
- **Soft Delete** — Files are never fully erased from chain history; they are marked as deleted, preserving an immutable audit trail.
- **Global Stats Tracking** — The contract maintains a live count of total, active, and deleted files across the entire platform via the `StorageStats` struct.
- **Lightweight & Efficient** — Only 4 contract functions keeping the contract minimal, gas-efficient, and easy to audit.
- **Default Fallback Values** — All read functions return safe default objects when a record is not found, preventing runtime panics on missing data.

---

## Future Scope

The current contract is intentionally minimal and serves as a strong foundation. Planned enhancements include:

- **Wallet-based Ownership Enforcement** — Integrate Soroban's `Address` and `auth` primitives to ensure only the file owner can delete or update their records.
- **File Update / Re-upload** — Allow owners to update the content hash of an existing file record (e.g., for versioning documents).
- **Access Control Lists (ACLs)** — Enable owners to grant read or write permissions on specific files to other wallet addresses.
- **File Categorization & Tagging** — Support metadata tags and categories so files can be organized and queried by type or topic.
- **Event Emission** — Emit Soroban events on upload, delete, and access so off-chain indexers and frontends can react in real time.
- **IPFS / Filecoin Integration** — Build a companion frontend that automatically pins files to IPFS and writes the resulting CID to the contract.
- **Subscription / Storage Tiers** — Introduce token-based access tiers where users pay XLM or a custom token for expanded storage quotas.
- **Multi-signature File Management** — Require multiple authorized signatures to delete or transfer ownership of critical files.

##contract details ;

 - contract id : CBBDFLQ4SEC3FWLJTFHHCBB2J22CH2M7PBGR3COXYJLL4MWEYMO7U5B3
-  <img width="1913" height="898" alt="image" src="https://github.com/user-attachments/assets/ed4fd44d-f853-4a25-95ba-609e7851dc66" />
