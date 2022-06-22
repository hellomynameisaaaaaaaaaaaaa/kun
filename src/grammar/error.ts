export interface Error {
    line: number;
    col: number;
    errorType: Errors;
    errorCode: number;
}

export enum Errors { // Types of errors
    SyntaxError // Codes 0000 - 001F
}