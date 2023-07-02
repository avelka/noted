module.exports = {
	globDirectory: 'dist/',
	globPatterns: [
		'**/*.{html,wasm,js,css}'
	],
	swDest: 'dist/sw.js',
	ignoreURLParametersMatching: [
		/^utm_/,
		/^fbclid$/
	]
};