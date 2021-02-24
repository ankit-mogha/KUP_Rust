
pub fn sort(arr:&mut [usize],first:usize,last:usize){
    if first < last {
        let mid = first + (last-first)/2;
        sort(arr, first, mid);
        sort(arr, mid + 1, last);
        merge(arr,first,mid,last);
    }
}

fn merge(arr:&mut [usize],first:usize,mid:usize,last:usize) {
  let n1 = mid - first + 1;
  let n2 = last - mid;
  
  let mut l:[usize;6]=[0;6];
  let mut r:[usize;6]=[0;6];
  let mut index =0;
  let mut ind=0;
  let mut i = 0;
  let mut j = 0;

    while index < n1 {
      l[index] = arr[first + index];
      index += 1;
  }
  
  while ind < n2 {
    r[ind] = arr[mid +1 + ind];
    ind += 1;  
  }
  
  let mut k = first;
  while i < n1 && j < n2 {
      if l[i] <= r[j] {
                arr[k] = l[i];
                i+=1;
            }
      else {
                arr[k] = r[j];
                j+=1;
            }
      k+=1;
  }
  
  while i < n1 {
            arr[k] = l[i];
            i+=1;
            k+=1;
         }
   
   while j < n2 {
            arr[k] = r[j];
            j+=1;
            k+=1;
        }
  
 }
