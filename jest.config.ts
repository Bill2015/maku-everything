/* eslint-disable quote-props */
/* eslint-disable import/no-extraneous-dependencies */
/* eslint-disable import/no-import-module-exports */
/* eslint-disable import/extensions */
import { pathsToModuleNameMapper } from 'ts-jest';
import { compilerOptions } from './tsconfig.json';
/** @type {import('ts-jest').JestConfigWithTsJest} */

module.exports = {
    preset:             'ts-jest',
    testEnvironment:    'jsdom',
    
    setupFilesAfterEnv: [
        '<rootDir>/src/__test__/setups/global.tsx',
        '<rootDir>/src/__test__/setups/matchMedia.js',
        '<rootDir>/src/__test__/setups/i18next.ts',
    ],
    /**
     * Default: null
     * A map from regular expressions to module names or to arrays of module names that allow to
     * stub out resources, like images or styles with a single module.
     *
     * Modules that are mapped to an alias are unmocked by default, regardless of whether
     * automocking is enabled or not.
     *
     * Use <rootDir> string token to refer to rootDir value if you want to use file paths.
     * Additionally, you can substitute captured regex groups using numbered backreferences.
     */
    moduleNameMapper: {
        '\\.(jpg|jpeg|png|gif|eot|otf|webp|ttf|woff|woff2|mp4|webm|wav|mp3|m4a|aac|oga)$': '<rootDir>/src/__test__/mocks/fileMock.js',
        // https://github.com/gregberge/svgr/issues/83#issuecomment-785996587
        // SVG Mock
        '\\.(svg)$': '<rootDir>/src/__test__/mocks/svgMock.tsx',
        '\\.(css|less|scss)$': '<rootDir>/src/__test__/mocks/styleMock.js',
        ...pathsToModuleNameMapper(compilerOptions.paths, { prefix: '<rootDir>' }),
    },
    /**
     * An array of regexp pattern strings that are matched against all test paths before executing
     * the test. If the test path matches any of the patterns, it will be skipped.
     */
    testPathIgnorePatterns: [
        '/node_modules/',
        '(.*)index.ts$',
        '(.*)testcase.ts$',
        '/src-tauri/',
    ],
    collectCoverageFrom: [
        '<rootDir>/src/**/*.{ts,tsx}',
    ],
    coveragePathIgnorePatterns: [
        '/node_modules/',
        '/__test__/',
        '(.*)index.ts$',
        '(.*).d.ts$'
    ]
};
