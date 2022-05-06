// const rustbuch = require('rustbuch_hello');
// const hello = rustbuch.hello;

import { hello, helloAsync } from 'rustbuch_hello/index.mjs';

console.log(hello('Hugo', 33))


helloAsync('Fred', 66, (result) => {
    console.log(result);
})
