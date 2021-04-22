const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {
    mode: 'development',

    entry: './src/index.ts',

    output: {
        path: path.resolve(__dirname, 'build'),
        filename: 'index.js',
    },

    module: {
        rules: [
            {
                test: /\.ts/,
                loader: 'ts-loader',
            },
        ],
    },

    resolve: {
        extensions: [ '.ts', '.js', '.wasm' ],
    },

    plugins: [
        new HtmlWebpackPlugin()
    ],

    experiments: {
        asyncWebAssembly: true,
    },
}