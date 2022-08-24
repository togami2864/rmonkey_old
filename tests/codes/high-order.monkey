let twice = fn(f, x) { return f(f(x));};
let addTwo = fn(x) { return x + 2;};
twice(addTwo, 2);