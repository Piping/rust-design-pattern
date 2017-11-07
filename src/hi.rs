/** 
 * visibility control
   mod my_module {
      pub struct MergeSorter;
      struct MergeSort;
      impl Trait for MergeSort { ... }
      impl MergeSorter { 
         pub fn sort(&self){
            MergeSort.sort()
         }
      }
   } 
*/
struct MergeSorter;
struct QuickSorter;
struct SelectionSorter;
struct InsertionSorter;
trait Sorter<T: PartialOrd + Clone> {
    fn sort(&self, ar: &mut Vec<T>, low: usize, high: usize) {
        if high >= ar.len() {
            return;
        }
        if low < high {
            let mut mid = 0;
            //pre-condition 0<low<high<cap
            //post-condition low < mid <= high
            self.split(ar, low, &mut mid, high);
            self.sort(ar, low, mid - 1);
            self.sort(ar, mid, high);
            self.join(ar, low, mid, high);
        }
    }
    fn split(&self, ar: &mut Vec<T>, low: usize, mid: &mut usize, high: usize);
    fn join(&self, ar: &mut Vec<T>, low: usize, mid: usize, high: usize);
}

impl<T> Sorter<T> for MergeSorter
where
    T: PartialOrd + Clone,
{
    fn split(&self, ar: &mut Vec<T>, low: usize, mid: &mut usize, high: usize) {
        let _ = ar;
        *mid = (high - low + 1) / 2 + low;
    }
    fn join(&self, ar: &mut Vec<T>, low: usize, mid: usize, high: usize) {
        let (mut i, mut j) = (low, mid);
        let mut vec = Vec::new();
        // vec.resize(high-low+1,Default::default());
        ////? Optimization: how to create a vector<T> and size is len in rust
        ////? so we no longer to create many tempory vectors.
        for _ in low..high + 1 {
            if i >= mid {
                vec.push(ar[j].clone());
                j += 1;
            } else if j > high {
                vec.push(ar[i].clone());
                i += 1;
            } else {
                if ar[i] > ar[j] {
                    vec.push(ar[j].clone());
                    j += 1;
                } else {
                    vec.push(ar[i].clone());
                    i += 1;
                }
            }
        }
        for idx in low..high + 1 {
            ar[idx] = vec[idx - low].clone();
        }
    }
}
impl<T> Sorter<T> for QuickSorter
where
    T: PartialOrd + Clone,
{
    fn split(&self, ar: &mut Vec<T>, low: usize, mid: &mut usize, high: usize) {
      let key = ar[high].clone();
        *mid = low;
        for j in low..high + 1 {
            if ar[j] <= key {
                ar.swap(j, *mid);
                *mid += 1;
            }
        }
        *mid = if *mid > high { high } else { *mid };
    }
    fn join(&self, _ar: &mut Vec<T>, _low: usize, _mid: usize, _high: usize) {}
}
impl<T: PartialOrd + Clone> Sorter<T> for SelectionSorter {
    fn split(&self, ar: &mut Vec<T>, low: usize, mid: &mut usize, high: usize) {
        let mut index_of_max = low;
        for i in low + 1..high + 1 {
            if ar[i] > ar[index_of_max] {
                index_of_max = i;
            }
        }
        ar.swap(index_of_max, high);
        *mid = high;
    }
    fn join(&self, _ar: &mut Vec<T>, _low: usize, _mid: usize, _high: usize) {}
}
impl<T: PartialOrd + Clone> Sorter<T> for InsertionSorter {
    fn split(&self, _ar: &mut Vec<T>, _low: usize, mid: &mut usize, high: usize) {
        *mid = high;
    }
    fn join(&self, ar: &mut Vec<T>, _low: usize, mid: usize, _high: usize) {
        //precondition low < mid = high && ar[low..high] is sorted
        let key = ar[mid].clone();
        let mut j = mid - 1;
        while key < ar[j] && j > 0{
            ar[j+1] = ar[j].clone();
            j -= 1;            
        }
        // Version 2
        if j == 0 {
            ar[j+1] = ar[j].clone();
            j = j.wrapping_sub(1);
        } 
        ar[j.wrapping_add(1)] = key;
        // // Version 1
        // if j == 0 {
        //     ar[j+1] = ar[j].clone();
        //     ar[j] = key;
        // } else {
        //     ar[j+1] = key;
        // }
    }
}


fn main() {
    let i = 5;
    for n in 1..i {
        println!("{}", n);
    }
    let mut a = vec![12, 13, 14, 15, 4, 3, 2, 1, 4, 5, 6, 9, 8, 7];
    let mut b = a.clone();
    let mut c = a.clone();
    let mut d = a.clone();
    //let len  = a.len(); // optimize merge sort with size len vector
    let high = a.len() - 1;
    MergeSorter.sort(&mut a, 0, high);
    QuickSorter.sort(&mut b, 0, high);
    SelectionSorter.sort(&mut c, 0, high);
    InsertionSorter.sort(&mut d, 0, high);
    println!("MergeSorter:     {:?}", a);
    println!("QuickSorter:     {:?}", b);
    println!("SelectionSorter: {:?}", c);
    println!("InsertionSorter: {:?}", d);
    println!("{:05b}", 1.clone());
    for i in (1..10+1).rev(){
        println!("{:?}", i);
    }
    println!("{}", 4 / 3);
}
