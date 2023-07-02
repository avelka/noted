module.exports = {
  globDirectory: "dist/",
  globPatterns: ["**/*.{html,wasm,js,css,svg,json}"],
  swDest: "dist/sw.js",
  ignoreURLParametersMatching: [/^utm_/, /^fbclid$/],
};
