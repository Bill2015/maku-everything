/**
 * Input Symbol Definition \
 * **Default:** any string exclude defined. */
/* eslint-disable no-shadow */
export enum InputSymbol {
    Default = 'default',
    Include = '+',
    Exclude = '-',
    LeftGroupBracket = '[',
    RightGroupBracket = ']',
    LeftAttrBracket = '{',
    RightAttrBracket = '}',
}

// eslint-disable-next-line @typescript-eslint/no-namespace, no-redeclare
export namespace InputSymbol {
    export function isValid(val: string) {
        if (val === 'default') {
            return false;
        }
        return Object.values(InputSymbol).indexOf(val as InputSymbol) > 0;
    }

    export function isPrefix(val: string) {
        return (val === InputSymbol.Exclude || val === InputSymbol.Include);
    }
}

/**
 * Search Status \
 * define the status mechine here
 * @see useInputStatusMechine */
export enum SearchStatus {
    Initial,
    PrefixOperator, // -, +
    TagName, // tag, left bracket
    Attribute,
    Group,
}
