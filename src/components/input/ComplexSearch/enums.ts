/**
 * Input Symbol Definition \
 * **Default:** any string exclude defined. */
/* eslint-disable no-shadow */
export enum InputSymbol {
    Default = 'default',
    Include = '+',
    Exclude = '-',
    LeftBracket = '[',
    RightBracket = ']',
}

/**
 * Input Status \
 * define the status mechine here
 * @see useInputStatusMechine */
export enum InputStatus {
    Initial,
    PrefixOperator, // -, +
    TagName, // tag, left bracket
    LeftBracket,
}
