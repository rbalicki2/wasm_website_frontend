// webpack.config.js
const path = require('path');

module.exports = {
  entry: "./dist/index.js",
  output: {
    path: path.resolve(__dirname, "dist_prod"),
    filename: "index.js",
  },
  mode: "development"
};