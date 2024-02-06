import React, { Dispatch, SetStateAction } from 'react';

/**
 * @purpose::
 * @topic: `state` can go into **`stale state closure problem`**
 * @know: the `state` is designed to be immutable in React -- every time you setState, you pass in a **new copy**
 * @pb: but when the `state` is in a **closure** (eg: `useEffect`), the reference of the state can be **stale**
 * @soln-not_desired: you need to use `dependency array` to update the `state` in `useEffect`
 * @soln-desired: you can use `ref` -- every time you `.current =`, the ref reference will still **stay the same**.
 * (-- @note: the value you pass in to the `.current` can still be a new copy -- but that wont matter)
 *
 * @main-res: https://stackoverflow.com/questions/66603920/combining-usestate-and-useref-in-reactjs
 * @dk still dk is this proper // Dont know if there is a better way.
 * @param initialState
 * @returns
 * state: use this to get the state & for rerender
 * getRefValue(): use this to get the state value  (-- with no `stale state closure problem`)
 * setStateRef(): use this to change state & ref (both sync)
 * - Dont use `ref.current` to change the ref (Only) -- to avoid that, `ref` is not returned (encapsulated), only getRefValue() is provided. */
export function useStateRef<S>(initialState: S | (() => S)): readonly [S, Dispatch<SetStateAction<S>>, () => S] {
    const [state, setState] = React.useState<S>(initialState);

    const ref: React.MutableRefObject<S> = React.useRef<S>(state); // dont just passin the initalState -- it can be a Function

    // @note https://stackoverflow.com/questions/45472944/how-to-define-the-type-of-a-named-function
    // @note https://stackoverflow.com/questions/70053133/how-to-apply-a-function-type-to-a-function-declaration
    const setStateRef: Dispatch<SetStateAction<S>> = (newState) => {
        // #>> state initializer function
        if (typeof newState === 'function') {
            if (typeGuardS2S<S>(newState)) {
                setState((prevState) => {
                    const result = newState(prevState);
                    ref.current = result;
                    return result;
                });
            }
            else {
                throw new TypeError(`typeof newState is a function, but not in the structure of ((prevState: S) => S): ${newState}`);
            }
        }
        // #>> state
        else {
            ref.current = newState;
            setState(newState);
        }
    };

    /** (for encapsulation on ref) */
    function getRefValue(): S {
        return ref.current;
    }

    // ;encapsulate; return [state, setStateRef, ref as React.MutableRefObject/RefObject<S>] as const;
    return [state, setStateRef, getRefValue] as const;
}

/**
 * This is actually unsafe & incompleted.
 * There is only limited things you can test with type guard on functions.
 * @param funcTest
 * @returns
 */
function typeGuardS2S<S>(funcTest: SetStateAction<S>): funcTest is (prevState: S) => S {
    if (typeof funcTest === 'function') {
        return true;
    }
    return false;
}
