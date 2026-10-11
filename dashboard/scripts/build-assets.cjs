const { execFileSync } = require('node:child_process');
const { copyFileSync } = require('node:fs');
execFileSync(process.execPath, ['node_modules/@tailwindcss/cli/dist/index.mjs', '-i', 'styles.css', '-o', 'assets/witness.css', '--minify'], { stdio: 'inherit' });
copyFileSync('node_modules/alpinejs/dist/cdn.min.js', 'assets/alpine.js');
// Alpine's published package omits its license; the exact upstream MIT notice is vendored in assets/ALPINE-LICENSE.md.
copyFileSync('node_modules/tailwindcss/LICENSE', 'assets/TAILWIND-LICENSE.txt');
