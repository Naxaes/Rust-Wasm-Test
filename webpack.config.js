const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HTMLWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

module.exports = (env, args) => {
    const isProduction = (args.mode === 'production');

    return {
        entry: './index.js',
        output: {
            path: path.resolve(__dirname, 'dist'),
            filename: isProduction ? '[name].[contenthash].js' : '[name].[hash].js',
        },
        plugins: [
            new HTMLWebpackPlugin({
                template: 'index.html',
            }),
            new WasmPackPlugin({
               crateDirectory: path.resolve(__dirname, '.'),
            }),
            new webpack.ProvidePlugin({
                TextDecoder: ['text-encoding', 'TextDecoder'],
                TextEncoder: ['text-encoding', 'TextEncoder'],
            }),
        ]

    }
}