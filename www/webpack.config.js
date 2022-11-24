const path = require("path");
const cpPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "public"),
        filename: "output_index.js"
    },
    mode: "development",
    plugins: [
        new cpPlugin({patterns:[{from:"./index.html", to:"./"}]})
    ]
}