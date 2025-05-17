import {
    CollectionSpecifier,
    commands,
    ContextSpecifier,
    DatabaseSpecifier,
    FileHandleMode,
    FileHandleSpecifier,
    JsonValue,
    OperationCount,
    PathInformation,
    PathMetadata,
    UpdateResult,
} from "./commands";
import { Res, Result } from "./util";

export class Context {
    public constructor(public name: string, public path: string) {}

    public get specifier(): ContextSpecifier {
        return { alias: this.name };
    }

    public static async open(name: string, path: string): Res<Context> {
        return Result.wrap(
            await commands.context({ alias: name, path })
        ).and_then((info) => new Context(info.name, info.path));
    }

    public static async get(name: string): Res<Context> {
        return Result.wrap(await commands.context({ alias: name })).and_then(
            (info) => new Context(info.name, info.path)
        );
    }

    public async get_base_path(): Res<string> {
        return Result.wrap(await commands.getContextBasePath(this.specifier));
    }

    public async get_absolute_path_to(context_path: string): Res<string> {
        return Result.wrap(
            await commands.getAbsolutePathTo(this.specifier, context_path)
        );
    }

    public async create_directory(path: string, parents: boolean): Res<null> {
        return Result.wrap(
            await commands.createDirectory(this.specifier, path, parents)
        );
    }

    public async remove_directory(path: string): Res<null> {
        return Result.wrap(
            await commands.removeDirectory(this.specifier, path)
        );
    }

    public async remove_file(path: string): Res<null> {
        return Result.wrap(await commands.removeFile(this.specifier, path));
    }

    public async database(name: string, path?: string): Res<Database> {
        if (path) {
            return await Database.open(this, name, path);
        } else {
            return await Database.get(this, name);
        }
    }

    public async open_file(
        path: string,
        mode: FileHandleMode
    ): Res<FileHandle> {
        return await FileHandle.open(this, path, mode);
    }

    public async get_file_handle(id: string): Res<FileHandle> {
        return await FileHandle.get(this, id);
    }

    public async get_file_metadata(path: string): Res<PathMetadata> {
        return Result.wrap(await commands.fileMetadata(this.specifier, path));
    }

    public async list_directory(path: string): Res<PathInformation[]> {
        return Result.wrap(await commands.listDirectory(this.specifier, path));
    }

    public async close(): Res<null> {
        return Result.wrap(await commands.closeContext(this.specifier));
    }
}

export class Transaction {
    public constructor(
        public database: Database,
        public context: Context,
        public id: string
    ) {}

    public collection<T extends object>(name: string): Collection<T> {
        return new Collection<T>(this.database, this.context, name, this.id);
    }

    public get specifiers(): [ContextSpecifier, DatabaseSpecifier, string] {
        return [this.context.specifier, this.database.specifier, this.id];
    }

    public async commit(): Res<null> {
        return Result.wrap(
            await commands.databaseCommitTransaction(...this.specifiers)
        );
    }

    public async rollback(): Res<null> {
        return Result.wrap(
            await commands.databaseRollbackTransaction(...this.specifiers)
        );
    }
}

export class Collection<T extends object> {
    public constructor(
        public database: Database,
        public context: Context,
        public name: string,
        public transaction_id: string | null
    ) {}

    public get specifier(): CollectionSpecifier {
        return this.transaction_id
            ? { transaction: this.transaction_id, name: this.name }
            : { name: this.name };
    }

    public get specifiers(): [
        ContextSpecifier,
        DatabaseSpecifier,
        CollectionSpecifier
    ] {
        return [
            this.context.specifier,
            this.database.specifier,
            this.specifier,
        ];
    }

    public async count_documents(): Res<number> {
        return Result.wrap(
            await commands.collectionCountDocuments(...this.specifiers)
        );
    }

    public async update(
        query: JsonValue,
        update: JsonValue,
        operations: OperationCount,
        upsert: boolean
    ): Res<UpdateResult> {
        return Result.wrap(
            await commands.collectionUpdateDocuments(
                ...this.specifiers,
                query,
                update,
                operations,
                upsert
            )
        );
    }

    public async update_one(
        query: JsonValue,
        update: JsonValue
    ): Res<UpdateResult> {
        return await this.update(query, update, "one", false);
    }

    public async update_many(
        query: JsonValue,
        update: JsonValue
    ): Res<UpdateResult> {
        return await this.update(query, update, "many", false);
    }

    public async upsert_one(
        query: JsonValue,
        update: JsonValue
    ): Res<UpdateResult> {
        return await this.update(query, update, "one", true);
    }

    public async upsert_many(
        query: JsonValue,
        update: JsonValue
    ): Res<UpdateResult> {
        return await this.update(query, update, "many", true);
    }

    public async delete(
        query: JsonValue,
        operations: OperationCount
    ): Res<number> {
        return Result.wrap(
            await commands.collectionDeleteDocuments(
                ...this.specifiers,
                query,
                operations
            )
        );
    }

    public async delete_one(query: JsonValue): Res<boolean> {
        return (await this.delete(query, "one")).and_then((d) => d >= 1);
    }

    public async delete_many(query: JsonValue): Res<number> {
        return await this.delete(query, "many");
    }

    public async create_index(
        keys: JsonValue,
        name: string,
        unique?: boolean
    ): Res<null> {
        return Result.wrap(
            await commands.collectionCreateIndex(
                ...this.specifiers,
                keys,
                name,
                unique ?? null
            )
        );
    }

    public async drop_index(name: string): Res<null> {
        return Result.wrap(
            await commands.collectionDropIndex(...this.specifiers, name)
        );
    }

    public async drop(): Res<null> {
        return Result.wrap(await commands.collectionDrop(...this.specifiers));
    }

    public async insert(
        ...documents: JsonValue[]
    ): Res<Partial<{ [key: number]: JsonValue }>> {
        return Result.wrap(
            await commands.collectionInsertDocuments(
                ...this.specifiers,
                documents
            )
        );
    }

    public async find_one(filter: JsonValue): Res<T | null> {
        return Result.wrap(
            await commands.collectionFindOneDocument(...this.specifiers, filter)
        ).and_then((d) => d as T | null);
    }

    public async find(
        filter: JsonValue,
        skip?: number | null,
        limit?: number | null,
        sort?: JsonValue | null
    ): Res<T[]> {
        return Result.wrap(
            await commands.collectionFindManyDocuments(
                ...this.specifiers,
                filter,
                skip ?? null,
                limit ?? null,
                sort ?? null
            )
        ).and_then((r) => r as T[]);
    }
}

export class Database {
    public constructor(
        public parent: Context,
        public name: string,
        public path: string
    ) {}

    public get specifier(): DatabaseSpecifier {
        return { alias: this.name };
    }

    public static async open(
        context: Context,
        name: string,
        path: string
    ): Res<Database> {
        return Result.wrap(
            await commands.database(context.specifier, { alias: name, path })
        ).and_then((info) => new Database(context, name, path));
    }

    public static async get(context: Context, name: string): Res<Database> {
        return Result.wrap(
            await commands.database(context.specifier, { alias: name })
        ).and_then((info) => new Database(context, info.name, info.path));
    }

    public collection<T extends object>(name: string): Collection<T> {
        return new Collection<T>(this, this.parent, name, null);
    }

    public async collections(): Res<string[]> {
        return Result.wrap(
            await commands.databaseGetCollections(
                this.parent.specifier,
                this.specifier
            )
        );
    }

    public async start_transaction(): Res<Transaction> {
        return Result.wrap(
            await commands.databaseStartTransaction(
                this.parent.specifier,
                this.specifier
            )
        ).and_then(
            ((id: string) => new Transaction(this, this.parent, id)).bind(this)
        );
    }

    public transaction(id: string): Transaction {
        return new Transaction(this, this.parent, id);
    }
}

export class FileHandle {
    public constructor(
        public parent: Context,
        public id: string,
        public path: string,
        public mode: FileHandleMode
    ) {}

    public get specifier(): FileHandleSpecifier {
        return { id: this.id };
    }

    public static async open(
        context: Context,
        path: string,
        mode: FileHandleMode
    ): Res<FileHandle> {
        return Result.wrap(
            await commands.fileHandle(context.specifier, { path, mode })
        ).and_then((info) => new FileHandle(context, info.id, path, mode));
    }

    public static async get(context: Context, id: string): Res<FileHandle> {
        return Result.wrap(
            await commands.fileHandle(context.specifier, { id })
        ).and_then((info) => new FileHandle(context, id, info.path, info.mode));
    }

    public async close(): Res<null> {
        return Result.wrap(
            await commands.fileClose(this.parent.specifier, this.specifier)
        );
    }

    public async read_text(max_length?: number): Res<string> {
        return Result.wrap(
            await commands.fileReadText(
                this.parent.specifier,
                this.specifier,
                max_length ?? null
            )
        );
    }

    public async read_bytes(max_length?: number): Res<Uint8Array> {
        return Result.wrap(
            await commands.fileReadBytes(
                this.parent.specifier,
                this.specifier,
                max_length ?? null
            )
        ).and_then((bytes) => new Uint8Array(bytes));
    }

    public async write_text(data: string): Res<null> {
        return Result.wrap(
            await commands.fileWriteText(
                this.parent.specifier,
                this.specifier,
                data
            )
        );
    }

    public async write_bytes(data: Uint8Array): Res<null> {
        return Result.wrap(
            await commands.fileWriteBytes(
                this.parent.specifier,
                this.specifier,
                Array.from(data)
            )
        );
    }
}
