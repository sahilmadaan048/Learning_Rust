/*
Stack Memory:
Stack memory is a contiguous block of memory allocated to the program automatically by the operating system when a function is called. This memory is used to store local variables and function arguments and is automatically managed by the program's runtime system. The stack memory is fast to access but has a limited size, and the size of memory is fixed at compile time.

Heap Memory:
Heap memory is a memory area that is dynamically allocated during runtime and is used to store data structures whose size cannot be determined at compile-time or need to outlive the function's scope. The heap memory is slower to access than stack memory but has a larger size, and the size can be changed dynamically.
*/
fn main() {
    println!("stack and heap practice in rust !");
    stack_mem_ex();
    heap_mem_ex();
}

fn stack_mem_ex() {
    let x=6;
    let y=6;

    let sum = add(x,y);
    println!("sum is {sum}");

    fn add(a:i32, b:i32) ->i32{
        a+b
    }
}

fn heap_mem_ex() {
    let mut v = Vec::new();  //dynamically allocated memory for vecror v

    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}",v);

    v.push(4);
    v.push(5);
    v.push(6);

    println!("{:?}",v);
}
