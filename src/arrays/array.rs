//What is an array ? 
//An array is a contiguous combination of memory cells with fixed size
//an example of an array is => [0000][0008][0016][0024][0032]....[0080]
//This is an example of an array of 10 elements, each in front in the other, 
//when wee have an array in a variable, what wee have is the reference to the first element,
//and with knowing the size of the first cell and simple mathematics, we can 
//access all the elements of the array 
//for accessing the 5 index for example it would be : 
//5 * sizeof(cell) + initial location = 5 * 8 + 0 = 0040,
//and with that wee can access the 5 location in the array in constant time, only with one 
//sum operation 
use std::mem;

fn main() {
    let mut arr: Vec<i32> = vec![1,2,3,4,5,6,7];
    println!("This is an array of i32, {:?}, each of size: {}", arr, mem::size_of::<i32>());
    for i in 0..arr.len() {
        println!("The {} is the memory address: {:p}", i, &arr[i]);
    }

    println!("As you can see each element is located in front in the other with {} difference", mem::size_of::<i32>());

    //complexity for common operations 
    //          Add | Remove | Find
    //-----------------------------
    //Beginning|O(n)|  O(n)  |  O(n)
    //End      |O(1)|  O(1)  |  O(n)
    //Middle   |O(n)|  O(n)  |  O(n)
    

    //Add 
    //At the end 
    arr.push(7);
    //At the middle 
    arr.insert(arr.len() / 2, 6);
    //At the beginning
    arr.insert(0, 0);

    //Remove 
    //At the end
    arr.pop();
    //At the middle 
    arr.remove(arr.len() / 2);
    //At the begining 
    arr.remove(0);
    
    //For finding is very trivial, simply run a for loop and search for the desired value,
    //if the array is sorted we can use more advanced techniques such as binary search 


}
