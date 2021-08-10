fn main() {
    // This is an example of line comment

    // println!("Hello world on comments")

    /*
     * This is a block comment type
     * it's very similar the ones we use in php
     */

    /*
        We can create a block of comments without the column of *,
        we put it there just for style, it looks very nice
    */

    let x = 5 + /* 10 + */  5;
    println!("Is `x` 10 or 20? x = {}", x);

}