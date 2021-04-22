const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {
    mode: 'development',

    entry: './src/index.ts',

    output: {
        path: path.resolve(__dirname, 'build'),
        filename: 'index.min.js',
    },

    module: {
        rules: [
            {
                test: /\.ts$/,
                loader: 'ts-loader',
            },
            {
                test: /\.html$/,
                loader: 'html-loader',
            },
        ],
    },

    resolve: {
        extensions: [ '.ts', '.js', '.wasm' ],
    },

    plugins: [
        new HtmlWebpackPlugin({ template: './src/index.html' })
    ],

    experiments: {
        asyncWebAssembly: true,
    },
}