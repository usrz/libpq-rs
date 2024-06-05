const libpq = require("./libpq_rs.node")
console.log("LIBPQ", libpq)

console.log("CALLING");
console.log("CALLED", libpq.foo("hello", 1, (...args) => {
  console.log('IN JS CALLBACK', args)
  libpq.bar();
  // throw new Error("Hello, world!")
}));
// console.log("AGAIN", libpq.foo("hello", 1, true));
