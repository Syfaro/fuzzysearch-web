const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, 'dist');

module.exports = (_env, argv) => {
    return {
        devServer: {
            historyApiFallback: true,
            compress: argv.mode === 'production',
            port: 8000,
        },
        entry: './bootstrap.js',
        output: {
            publicPath: '/',
            path: distPath,
            filename: 'fuzzysearch.js',
            webassemblyModuleFilename: 'fuzzysearch.wasm',
        },
        module: {
            rules: [
                {
                    test: /\.css$/,
                    use: [
                        'style-loader',
                        'css-loader',
                    ],
                },
                {
                    test: /\.png$/,
                    use: [
                        'url-loader',
                    ],
                },
            ],
        },
        plugins: [
            new CopyWebpackPlugin([{
                from: './static',
                to: distPath,
            }]),
            new WasmPackPlugin({
                crateDirectory: '.',
                extraArgs: '--no-typescript',
                forceMode: 'production',
            }),
        ],
        watch: argv.mode !== 'production',
    }
};
