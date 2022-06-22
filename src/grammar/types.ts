export interface Literal<T> {
    type: Type;
    value: T;
}

export type Value = number | string | boolean | null;

export enum Type {
    Number,
    String,
    Bool,
    Null
}

export class convert {
    static toNumber(x: string[]): Literal<number> {
        return {
            type: Type.Number,
            value: parseFloat(x[0])
        }
    }

    static toString(x: string[]): Literal<string> {
        return {
            type: Type.String,
            value: x[0].slice(1, -1)
        }
    }

    static toBool(x: string[]): Literal<boolean> {
        return {
            type: Type.Bool,
            value: x[0] == 'true'
        }
    } 

    static toNull(x: string[0]): Literal<null> {
        return {
            type: Type.Null,
            value: null
        }
    }
}