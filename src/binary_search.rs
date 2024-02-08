
fn binary_search<T:PartialEq>(arr:T,target:T)->Pa{
    
    let mut i =0;
    let mut e=arr.len()-1;

    while i<=e{
        let mut mid=(i+(e-i))/2;
        if arr[mid]==target{
            return mid;
        }
        else if mid > target{
            low=mid+1;
        }
        else if mid<target{
            high=mid-1;
        }
    

}
