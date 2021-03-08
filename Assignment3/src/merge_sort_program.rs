
pub fn sort(arr:&mut [usize],first:usize,last:usize){
    if first < last {
        let mid = first + (last-first)/2;
        sort(arr, first, mid);
        sort(arr, mid + 1, last);
        merge(arr,first,mid,last);
    }
}

fn merge(arr:&mut [usize],first:usize,mid:usize,last:usize) {
  let left_num = mid - first + 1;
  let right_num = last - mid;
  
  let mut left:[usize;6]=[0;6];
  let mut right:[usize;6]=[0;6];
  let mut index =0;
  let mut ind=0;
  let mut i = 0;
  let mut j = 0;

    while index < left_num {
      left[index] = arr[first + index];
      index += 1;
  }
  
  while ind < right_num {
    right[ind] = arr[mid +1 + ind];
    ind += 1;  
  }
  
  let mut k = first;
  while i < left_num && j < right_num {
      if left[i] <= right[j] {
                arr[k] = left[i];
                i+=1;
            }
      else {
                arr[k] = right[j];
                j+=1;
            }
      k+=1;
  }
  
  while i < left_num {
            arr[k] = left[i];
            i+=1;
            k+=1;
         }
   
   while j < right_num {
            arr[k] = right[j];
            j+=1;
            k+=1;
        }
  
 }
