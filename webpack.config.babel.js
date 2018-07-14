// webpack.config.js
import webpack from 'webpack';
const path = require('path');

module.exports = {
  entry: ['react-hot-loader/patch', './dist/index.js'],
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: 'index.js',
  },
  mode: 'development',
  plugins: [
    new webpack.HotModuleReplacementPlugin(),
  ],
};
