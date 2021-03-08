/// This function Linear search element from the array of numbers.
///
/// #Arguments
///
/// Array of numbers, number that need to be searched in array , index as a loop variable
///
/// #Return
///
/// prints the index at which number is found
pub fn linear_search(arr:&[usize],num:usize,index:usize)
{
    let len = arr.len();
    if index < len
    {
        if arr[index] == num
        {
            println!("Linear Search Result");
            println!("Number {} is found at {}",num,index);
        }
        let index=index+1;
        linear_search(arr,num,index);
    }
}
/// This function Binary search element from the array of numbers.
///
/// #Arguments
///
/// Sorted array of numbers, number that need to be searched in array , first element of array , last elements of array
///
/// #Return
///
/// prints the index at which number is found
pub fn binary_search(arr:&[usize],first:usize,last:usize,num:usize)
{
    if last>=first
    {
        let mid = first + (last-first)/2;
        if arr[mid] == num
        {
            println!("Binary Search Result");
            println!("Number {} is found at {}",num,mid);
        }
        if arr[mid] > num{
            let mid = mid-1;
            binary_search(arr, first, mid, num);
        }
        else
        {
            let mid = mid+1;
            binary_search(arr, mid, last, num);
        }

    }
}