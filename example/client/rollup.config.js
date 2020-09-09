import rust from "@wasm-tool/rollup-plugin-rust";
import serve from 'rollup-plugin-serve';
import liveReload from 'rollup-plugin-livereload';
import { terser } from "rollup-plugin-terser";

const isDev = process.env.ROLLUP_WATCH;

export default {
    input: {
        index: "Cargo.toml",
    },
    output: {
        dir: "dist/js",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: "js/",
            debug: isDev,
        }),
        isDev && serve({
            contentBase: "dist",
            verbose: true,
            open: true,
        }),
        isDev && liveReload({
            watch: "dist"
        }),
        !isDev && terser(), 
    ],
};