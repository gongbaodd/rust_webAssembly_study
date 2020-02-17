'use strict';

const argv = process.argv.splice('2');
console.log(`This is from node, ${argv.join('_')}.`);