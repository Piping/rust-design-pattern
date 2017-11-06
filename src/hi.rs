trait Sorter<T:PartialOrd+Clone> {
   fn sort(&self, ar: &mut Vec<T>, low: usize, high: usize) {
      if high < ar.len() && low < high {
         let mut mid = 0;
         self.split(ar, low, &mut mid, high);
         self.sort(ar, low, mid-1);
         self.sort(ar, mid, high);
         self.join(ar, low, mid, high);
      }
   }
   fn split(&self, ar: &mut Vec<T>, low: usize, mid: &mut usize, high: usize);
   fn join(&self, ar: &mut Vec<T>, low: usize, mid: usize, high: usize);
}
struct MergeSorter;

impl<T: PartialOrd+Clone> Sorter<T> for MergeSorter {
   fn split(&self, ar: &mut Vec<T>, low: usize, mid: &mut usize, high: usize) {
      let _ = ar;
      *mid = (high - low + 1) / 2 + low;
   }
   fn join(&self, ar: &mut Vec<T>, low: usize, mid: usize, high: usize) {
      
      let (mut i, mut j) = (low, mid);
      let mut vec = Vec::new();
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
         ar[idx] = vec[idx-low].clone();
      }
   }
}
fn main() {
   let i = 5;
   for n in 1..i {
      println!("{}", n);
   }

   let mut a = vec!(12,13,14,15,4,3,2,1,4,5,6,9,8,7);

   let len = a.len();
   MergeSorter.sort(&mut a,0,len-1);

   println!("Sorted:{:?}", a);
   println!("{:05b}", 1);
   println!("{}", 4 / 3);
}
