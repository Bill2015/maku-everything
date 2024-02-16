/* eslint-disable import/no-extraneous-dependencies */
import path from 'path';
import { defineConfig, type PluginOption } from 'vite';
import { visualizer } from "rollup-plugin-visualizer";
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [
        react(),
        visualizer() as PluginOption,
    ],
    css: {
        preprocessorOptions: {
          scss: {
            additionalData: `@import "./src/_mantine";`,
          },
        },
    },
    // Path alias
    resolve: {
        alias: {
            '@':           path.resolve(__dirname, './src'),
            '@assets':     path.resolve(__dirname, './src/assets'),
            '@store':      path.resolve(__dirname, './src/store'),
            '@hooks':      path.resolve(__dirname, './src/hooks'),
            '@declares':   path.resolve(__dirname, './src/declares'),
            '@utils':      path.resolve(__dirname, './src/utils'),
            '@router':     path.resolve(__dirname, './src/router'),
            '@modals':     path.resolve(__dirname, './src/modals'),
            '@modules':    path.resolve(__dirname, './src/modules'),
            '@api':        path.resolve(__dirname, './src/api'),
            '@pages':      path.resolve(__dirname, './src/pages'),
            '@components': path.resolve(__dirname, './src/components'),
            '@__test__':   path.resolve(__dirname, './src/__test__'),
        },
    },
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    // prevent vite from obscuring rust errors
    clearScreen: false,
    // tauri expects a fixed port, fail if that port is not available
    server:      {
        port:       1420,
        strictPort: true,
    },
    // to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ['VITE_', 'TAURI_'],
    build:     {
        // Tauri supports es2021
        target:    process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
        // don't minify for debug builds
        minify:    !process.env.TAURI_DEBUG ? true : false,
        // produce sourcemaps for debug builds
        sourcemap: !!process.env.TAURI_DEBUG,
    },
}));
