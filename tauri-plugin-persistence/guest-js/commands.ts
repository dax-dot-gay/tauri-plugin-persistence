
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
async context(context: ContextSpecifier) : Promise<Result<ContextInfo, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|context", { context }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async database(context: ContextSpecifier, database: DatabaseSpecifier) : Promise<Result<DatabaseInfo, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|database", { context, database }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async fileHandle(context: ContextSpecifier, fileHandle: FileHandleSpecifier) : Promise<Result<FileHandleInfo, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|file_handle", { context, fileHandle }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async databaseGetCollections(context: ContextSpecifier, database: DatabaseSpecifier) : Promise<Result<string[], Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|database_get_collections", { context, database }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async databaseClose(context: ContextSpecifier, database: DatabaseSpecifier) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|database_close", { context, database }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async databaseStartTransaction(context: ContextSpecifier, database: DatabaseSpecifier) : Promise<Result<string, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|database_start_transaction", { context, database }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async databaseCommitTransaction(context: ContextSpecifier, database: DatabaseSpecifier, transaction: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|database_commit_transaction", { context, database, transaction }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async databaseRollbackTransaction(context: ContextSpecifier, database: DatabaseSpecifier, transaction: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|database_rollback_transaction", { context, database, transaction }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async collectionCountDocuments(context: ContextSpecifier, database: DatabaseSpecifier, collection: CollectionSpecifier) : Promise<Result<number, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|collection_count_documents", { context, database, collection }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async collectionUpdateDocuments(context: ContextSpecifier, database: DatabaseSpecifier, collection: CollectionSpecifier, query: null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }>, update: null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }>, operations: OperationCount, upsert: boolean) : Promise<Result<UpdateResult, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|collection_update_documents", { context, database, collection, query, update, operations, upsert }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async collectionDeleteDocuments(context: ContextSpecifier, database: DatabaseSpecifier, collection: CollectionSpecifier, query: null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }>, operations: OperationCount) : Promise<Result<number, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|collection_delete_documents", { context, database, collection, query, operations }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async collectionCreateIndex(context: ContextSpecifier, database: DatabaseSpecifier, collection: CollectionSpecifier, keys: null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }>, name: string | null, unique: boolean | null) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|collection_create_index", { context, database, collection, keys, name, unique }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async collectionDropIndex(context: ContextSpecifier, database: DatabaseSpecifier, collection: CollectionSpecifier, name: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|collection_drop_index", { context, database, collection, name }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async collectionDrop(context: ContextSpecifier, database: DatabaseSpecifier, collection: CollectionSpecifier) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|collection_drop", { context, database, collection }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async collectionInsertDocuments(context: ContextSpecifier, database: DatabaseSpecifier, collection: CollectionSpecifier, documents: (null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }>)[]) : Promise<Result<Partial<{ [key in number]: null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }> }>, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|collection_insert_documents", { context, database, collection, documents }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async collectionFindManyDocuments(context: ContextSpecifier, database: DatabaseSpecifier, collection: CollectionSpecifier, filter: null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }>, skip: number | null, limit: number | null, sort: null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }> | null) : Promise<Result<(null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }>)[], Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|collection_find_many_documents", { context, database, collection, filter, skip, limit, sort }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async collectionFindOneDocument(context: ContextSpecifier, database: DatabaseSpecifier, collection: CollectionSpecifier, filter: null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }>) : Promise<Result<null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }> | null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|collection_find_one_document", { context, database, collection, filter }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async fileClose(context: ContextSpecifier, fileHandle: FileHandleSpecifier) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|file_close", { context, fileHandle }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async fileWriteText(context: ContextSpecifier, fileHandle: FileHandleSpecifier, data: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|file_write_text", { context, fileHandle, data }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async fileWriteBytes(context: ContextSpecifier, fileHandle: FileHandleSpecifier, data: number[]) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|file_write_bytes", { context, fileHandle, data }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async fileReadText(context: ContextSpecifier, fileHandle: FileHandleSpecifier, size: number | null) : Promise<Result<string, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|file_read_text", { context, fileHandle, size }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async fileReadBytes(context: ContextSpecifier, fileHandle: FileHandleSpecifier, size: number | null) : Promise<Result<number[], Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|file_read_bytes", { context, fileHandle, size }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getContextBasePath(context: ContextSpecifier) : Promise<Result<string, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|get_context_base_path", { context }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getAbsolutePathTo(context: ContextSpecifier, path: string) : Promise<Result<string, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|get_absolute_path_to", { context, path }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async createDirectory(context: ContextSpecifier, path: string, parents: boolean) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|create_directory", { context, path, parents }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async removeDirectory(context: ContextSpecifier, path: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|remove_directory", { context, path }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async removeFile(context: ContextSpecifier, path: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|remove_file", { context, path }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async fileMetadata(context: ContextSpecifier, path: string) : Promise<Result<PathMetadata, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|file_metadata", { context, path }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async listDirectory(context: ContextSpecifier, path: string) : Promise<Result<PathInformation[], Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|list_directory", { context, path }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async closeContext(context: ContextSpecifier) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|close_context", { context }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async cleanup() : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:persistence|cleanup") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}

/** user-defined events **/



/** user-defined constants **/



/** user-defined types **/

/**
 * A model used to specify a collection
 */
export type CollectionSpecifier = 
/**
 * Open a collection in a transaction
 */
{ 
/**
 * Transaction ID
 */
transaction: string; 
/**
 * Collection name
 */
name: string } | 
/**
 * Open a non-transacted collection
 */
{ 
/**
 * Collection name
 */
name: string }
/**
 * A model containing serializable information about a [crate::Context]
 */
export type ContextInfo = { 
/**
 * Context name
 */
name: string; 
/**
 * Context path
 */
path: string }
/**
 * A model used to specify an existing or closed context
 */
export type ContextSpecifier = 
/**
 * Open a new context
 */
{ alias: string; path: string } | 
/**
 * Return an existing context
 */
{ alias: string }
/**
 * A model containing serializable information about a [crate::Database]
 */
export type DatabaseInfo = { 
/**
 * Database name
 */
name: string; 
/**
 * Database path
 */
path: string }
/**
 * A model used to specify an existing or closed database
 */
export type DatabaseSpecifier = 
/**
 * Open a new database
 */
{ alias: string; path: string } | 
/**
 * Return an existing database
 */
{ alias: string }
export type Error = { kind: "unknown"; reason: string } | { kind: "open_context"; name: string; path: string; reason: string } | { kind: "open_database"; name: string; context: string; path: string; reason: string } | { kind: "open_file_handle"; path: string; context: string; reason: string } | { kind: "unknown_context"; reason: string } | { kind: "unknown_database"; reason: string } | { kind: "unknown_file_handle"; reason: string } | { kind: "unknown_transaction"; reason: string } | { kind: "invalid_path"; reason: string } | { kind: "no_absolute_paths"; reason: string } | { kind: "path_escapes_context"; reason: string } | { kind: "database_error"; reason: string } | { kind: "serialization_error"; reason: string } | { kind: "deserialization_error"; reason: string } | { kind: "io_error"; reason: string } | { kind: "string_encoding_error"; reason: string } | { kind: "filesystem_error"; operation: string; reason: string }
/**
 * A model containing serializable information about a [crate::FileHandle]
 */
export type FileHandleInfo = { 
/**
 * File handle ID
 */
id: string; 
/**
 * File handle path
 */
path: string; 
/**
 * Open mode
 */
mode: FileHandleMode }
export type FileHandleMode = { mode: "create"; new: boolean; overwrite: boolean } | { mode: "write"; overwrite: boolean } | { mode: "read" }
/**
 * A model used to specify an existing or closed file handle
 */
export type FileHandleSpecifier = 
/**
 * Return an existing file handle
 */
{ id: string } | 
/**
 * Open a new file handle
 */
{ path: string; mode: FileHandleMode }
export type JsonValue = null | boolean | number | string | JsonValue[] | Partial<{ [key in string]: JsonValue }>
/**
 * Whether to do one operation or multiple (in a database context)
 */
export type OperationCount = "one" | "many"
/**
 * Description of the type of a file/directory/symlink
 */
export type PathFileType = "directory" | "file" | "symlink"
/**
 * General info about a path
 */
export type PathInformation = { file_name: string; absolute_path: string; media_type: string }
/**
 * File or folder metadata
 */
export type PathMetadata = { file_type: PathFileType; size: number; last_modified: string | null; last_accessed: string | null; created: string | null }
/**
 * Serializable version of [polodb_core::results::UpdateResult]
 */
export type UpdateResult = { 
/**
 * How many documents matched the filter
 */
matched: number; 
/**
 * How many documents were updated
 */
modified: number }

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: null extends T
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
