const COMMANDS: &[&str] = &[
    "context",
    "database",
    "file_handle",
    "database_get_collections",
    "database_close",
    "database_start_transaction",
    "database_commit_transaction",
    "database_rollback_transaction",
    "collection_count_documents",
    "collection_update_documents",
    "collection_delete_documents",
    "collection_create_index",
    "collection_drop_index",
    "collection_drop",
    "collection_insert_documents",
    "collection_find_many_documents",
    "collection_find_one_document",
    "file_close",
    "file_write_text",
    "file_write_bytes",
    "file_read_text",
    "file_read_bytes",
    "get_context_base_path",
    "get_absolute_path_to",
    "create_directory",
    "remove_directory",
    "remove_file",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
