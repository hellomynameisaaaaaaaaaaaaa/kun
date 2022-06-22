@preprocessor typescript

@{%
    // Local Imports
    import { Error, Errors } from './error';
    import { Statement, StatementTypes } from './statement';
    import { Literal, Value, Type, convert } from './types';

    // Lib Imports
    import { compile, keywords, error } from 'moo';

    const lexer = compile({
        // Literals
        NUMBER_LIT: /[-]?[1-9]+\.[0-9]*?/,
        STRING_LIT: /'[^'\n]*?'|"[^"\n]*?"/,
        BOOL_LIT: ['true', 'false'],
        NULL_LIT: ['null'],

        // Symbols
        L_PAR: '(',
        R_PAR: ')',
        L_SBR: '[',
        R_SBR: ']',
        L_CBR: '{',
        R_CBR: '}',
        EQ: '=',
        LCOM: '#',
        S_COM: '#=',
        E_COM: '=#',

        // Keywords
        DO: 'do',
        END: 'end',
        FN: 'fn',
        IF: 'if',
        ELSE: 'else',
        ELIF: 'elif',
        WHILE: 'while',

        WS: /\s/,
        EOL: {match: /\r\n|\n/, lineBreaks: true },
        ID: /\w/,
        ERR: error
    })
%}¿@lexer lexer

file -> (statement | __):+

# Statements
statement -> expression
           | comment

variableStatement -> %ID _ %EQ _ value {% ()%}

ifStatement -> %IF _ expression _ expression

# Expressions
expression -> numberLiteral
            | stringLiteral
            | boolLiteral
            | nullLiteral
            | %L_PAR _ %ID _ expression:* _ %R_PAR {% ([s, , fn, , expr, ]) => <Statement>{
                type: StatementTypes.Expression,
                data: {
                    fn: fn.value,
                    expr: expr.value
                },
                info: {
                    line: s.line,
                    col: s.col
                }
            } %}

# Comments
comment -> %LCOM {% () => null %}

# Blocks
block -> %DO __ (expression | __):* __ %END {% ([s, ,expr, , ]) => <Statement>{
    type: StatementTypes.Expression,
    data: {
        exprs: expr
    },
    info: {
        line: s.line,
        col: s.col
    }
}%}

# Literals
numberLiteral -> %NUMBER_LIT {% convert.toNumber %}
stringLiteral -> %STRING_LIT {% convert.toString %}
boolLiteral -> %BOOL_LIT {% convert.toBool %}
nullLiteral -> %NULL_LIT {% convert.toNull %}

# Whitespace
_ -> %WS:* | %EOL:? {% () => null %}
__ -> %WS:+ | %EOL {% () => null %} 