// @ts-ignore
import { Error } from './error';

export interface Statement { // statement types
    type: StatementTypes;
    data: object;
    info: {
        line: number;
        col: number;
    }
    error?: Error;
};

export enum StatementTypes {
    DefineVariable,
    Block,
    Expression
}