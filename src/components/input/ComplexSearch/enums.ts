/* eslint-disable no-shadow */
export enum InputSymbol {
    Default = 'default',
    Include = '+',
    Exclude = '-',
    LeftBracket = '[',
    RightBracket = ']',
}

export enum InputStatus {
    Initial,
    PrefixOperator, // -, +
    TagName, // tag, left bracket
    LeftBracket,
}
