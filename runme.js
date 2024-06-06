const libpq = require("./libpq_rs.node")
console.log("LIBPQ", libpq)

console.log("CALLING");
console.log("CALLED", libpq.foo(Symbol(), Symbol("foobar"), (...args) => {
  console.log('IN JS CALLBACK', args)
  libpq.bar(libpq.external);
  // throw new Error("Hello, world!")
}));



// setTimeout(() => console.log("done"), 5000)
// console.log("AGAIN", libpq.foo("hello", 1, true));
