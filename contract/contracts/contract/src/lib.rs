#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Tracks overall file storage statistics on the platform
#[contracttype]
#[derive(Clone)]
pub struct StorageStats {
    pub total_files: u64,   // Total number of files uploaded
    pub active_files: u64,  // Files currently active (not deleted)
    pub deleted_files: u64, // Files that have been soft-deleted
}

// Symbol key to reference the global StorageStats struct
const ALL_FILES: Symbol = symbol_short!("ALL_FILES");

// Maps a unique file ID to its FileRecord
#[contracttype]
pub enum FileBook {
    FileRecord(u64),
}

// Counter key for generating unique file IDs
const COUNT_FILE: Symbol = symbol_short!("C_FILE");

// Represents a file uploaded to the dApp
#[contracttype]
#[derive(Clone)]
pub struct FileRecord {
    pub file_id: u64,          // Unique identifier for the file
    pub owner: String,         // Owner identifier (e.g., wallet address as string)
    pub file_name: String,     // Name of the file
    pub file_hash: String,     // IPFS hash or content hash of the file
    pub upload_time: u64,      // Ledger timestamp at upload
    pub is_deleted: bool,      // Soft-delete flag
}

#[contract]
pub struct FileStorageContract;

#[contractimpl]
impl FileStorageContract {

    // Upload a new file record to the dApp
    // Returns the unique file_id assigned to the new file
    pub fn upload_file(env: Env, owner: String, file_name: String, file_hash: String) -> u64 {
        let mut count_file: u64 = env.storage().instance().get(&COUNT_FILE).unwrap_or(0);
        count_file += 1;

        let time = env.ledger().timestamp();

        let mut stats = Self::view_storage_stats(env.clone());

        let file = FileRecord {
            file_id: count_file,
            owner,
            file_name,
            file_hash,
            upload_time: time,
            is_deleted: false,
        };

        // Persist the file record
        env.storage().instance().set(&FileBook::FileRecord(count_file), &file);

        // Update global stats
        stats.total_files += 1;
        stats.active_files += 1;
        env.storage().instance().set(&ALL_FILES, &stats);

        // Update the file counter
        env.storage().instance().set(&COUNT_FILE, &count_file);

        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "File uploaded with File-ID: {}", count_file);
        count_file
    }

    // Retrieve a specific file record by its unique file ID
    pub fn get_file(env: Env, file_id: u64) -> FileRecord {
        let key = FileBook::FileRecord(file_id);
        env.storage().instance().get(&key).unwrap_or(FileRecord {
            file_id: 0,
            owner: String::from_str(&env, "Not_Found"),
            file_name: String::from_str(&env, "Not_Found"),
            file_hash: String::from_str(&env, "Not_Found"),
            upload_time: 0,
            is_deleted: true,
        })
    }

    // Soft-delete a file by its file ID (marks is_deleted = true)
    // Only meaningful when called by the rightful owner (ownership enforcement
    // can be added once Address/auth primitives are integrated)
    pub fn delete_file(env: Env, file_id: u64) {
        let mut file = Self::get_file(env.clone(), file_id);

        if file.file_id == 0 {
            log!(&env, "File-ID: {} does not exist", file_id);
            panic!("File not found!");
        }

        if file.is_deleted {
            log!(&env, "File-ID: {} is already deleted", file_id);
            panic!("File is already deleted!");
        }

        file.is_deleted = true;

        // Persist the updated record
        env.storage().instance().set(&FileBook::FileRecord(file_id), &file);

        // Update global stats
        let mut stats = Self::view_storage_stats(env.clone());
        if stats.active_files > 0 {
            stats.active_files -= 1;
        }
        stats.deleted_files += 1;
        env.storage().instance().set(&ALL_FILES, &stats);

        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "File-ID: {} has been deleted", file_id);
    }

    // View global storage statistics (total, active, deleted file counts)
    pub fn view_storage_stats(env: Env) -> StorageStats {
        env.storage().instance().get(&ALL_FILES).unwrap_or(StorageStats {
            total_files: 0,
            active_files: 0,
            deleted_files: 0,
        })
    }
}