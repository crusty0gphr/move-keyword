// representation how move keyword works in rust
// --

// here's an example of a function that excepts
// a closure as an argument
//
// we require a closure to implement a FnOnce tarin
// so the f can be called only once
//
// also we define a lifetime for our function with a 'static lifetime modifier
// this basically means now that we want this closure to live
// until main application was terminated
fn closure<F: FnOnce() + 'static>(f: F) {
    f();
}


fn main() {
    let mut v = vec![0,1,2];
    v.push(4);


    // the move keyword here simply takes ownership
    // of all the variabl used inside of the closure
    // or simply takes ownership of all the variables used inside of a closure
    closure(move ||{
        v.push(5);
    });
    // without the move keyword this closure
    // would have lived just until here and wont be able to
    // use the vectore defined inside the main
}
