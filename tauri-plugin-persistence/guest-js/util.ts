import { Result as CommandResult, Error } from "./commands";

export class Result<S = any, E = any> {
    public constructor(public base: CommandResult<S, E>) {}

    public static ok<T, R = any>(value: T): Result<T, R> {
        return new Result<T, any>({ status: "ok", data: value });
    }

    public static err<T, R = any>(error: T): Result<R, T> {
        return new Result<any, T>({ status: "error", error });
    }

    public static async awrap<X, Y>(
        result: Promise<CommandResult<X, Y>>
    ): Promise<Result<X, Y>> {
        return new Result<X, Y>(await result);
    }

    public static wrap<X, Y>(result: CommandResult<X, Y>): Result<X, Y> {
        return new Result<X, Y>(result);
    }

    public data(): S | null {
        if (this.base.status === "ok") {
            return this.base.data;
        } else {
            return null;
        }
    }

    public error(): E | null {
        if (this.base.status === "error") {
            return this.base.error;
        } else {
            return null;
        }
    }

    public ok_or<D = S>(fallback: D): D | S {
        return this.data() ?? fallback;
    }

    public ok_or_else<D = S>(fallback: (err: E) => D): D | S {
        return this.data() ?? fallback(this.error() as E);
    }

    public map<MS = S, ME = E>(
        success: (data: S) => Result<MS, ME>,
        error: (error: E) => Result<MS, ME>
    ): Result<MS, ME> {
        return this.base.status === "ok"
            ? success(this.base.data)
            : error(this.base.error);
    }

    public and_then<T = S>(transform: (data: S) => T): Result<T, E> {
        if (this.base.status === "ok") {
            return Result.ok<T, E>(transform(this.base.data));
        } else {
            return Result.err<E, T>(this.base.error);
        }
    }

    public or_else<T = E>(transform: (data: E) => T): Result<S, T> {
        if (this.base.status === "error") {
            return Result.err<T, S>(transform(this.base.error));
        } else {
            return Result.ok<S, T>(this.base.data);
        }
    }
}

export type Res<T> = Promise<Result<T, Error>>;
