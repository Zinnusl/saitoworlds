const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const importRetry = require("webpack-plugin-import-retry");

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin(['index.html']),
        new importRetry()
    ],
};
