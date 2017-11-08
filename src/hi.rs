struct MergeSorter<T> {
    vec: std::cell::RefCell<Vec<T>>,
}
impl<T: PartialOrd + Clone + Default> MergeSorter<T> {
    pub fn new(len: usize) -> Self {
        let mut temp: Vec<T> = Vec::new();
        temp.resize(len, Default::default());
        MergeSorter {
            vec: std::cell::RefCell::new(temp),
        }
    }
    /**  visibility control */
    pub fn sort(&self, ar: &mut Vec<T>, low: usize, high: usize) {
        Sorter::sort(self, ar, low, high);
    }
}
struct QuickSorter;
struct SelectionSorter;
struct InsertionSorter;
struct HeapSorter<T> {
    marker: std::marker::PhantomData<T>,
}

impl<T: PartialOrd + Clone> HeapSorter<T> {
    pub fn new() -> Self {
        HeapSorter {
            marker: std::marker::PhantomData,
        }
    }
    pub fn sort(&self, ar: &mut Vec<T>, low: usize, high: usize) {
        self.heapfy(ar, low, high);
        Sorter::sort(self, ar, low, high);
    }
    fn heapfy(&self, ar: &mut Vec<T>, low: usize, high: usize) {
        // largest index of a node with at least one child
        let mut idx = (high - 1 - low) / 2 + low;
        while idx >= low && idx > 0 {
            self.shift_down(ar, low, idx, high);
            idx -= 1;
        }
        if idx == 0 {
            self.shift_down(ar, low, idx, high);
        }
    }
    fn _shift_down(&self, ar: &mut Vec<T>, low: usize, i: usize, high: usize) {
        let mut child = i - low + i + 1; // index of left child
        if child <= high {
            if child < high && ar[child] < ar[child + 1] {
                child += 1;
            }
            if ar[i] < ar[child] {
                ar.swap(i, child);
                self._shift_down(ar, low, child, high);
            }
        }
    }
    fn shift_down(&self, ar: &mut Vec<T>, low: usize, i: usize, high: usize) {
        let mut j = i;
        loop {
            let mut child = j - low + j + 1; // index of left child
            if child > high {
                return;
            };
            if child < high && ar[child] < ar[child + 1] {
                child += 1;
            }
            if ar[j] >= ar[child] {
                return;
            }
            ar.swap(j, child);
            j = child;
        }
    }
}

impl<T> Sorter<T> for HeapSorter<T>
where
    T: PartialOrd + Clone,
{
    fn split(&self, ar: &mut Vec<T>, low: usize, mid: &mut usize, high: usize) {
        //pre-condition: maxheap(a[lo..hi])
        //post-condition: maxheap(a[lo..hi-1])
        //post-condition: a[hi] == old a[low], max && mid == hi
        ar.swap(low, high);
        self.shift_down(ar, low, low, high - 1);
        *mid = high;
    }
    fn join(&self, _ar: &mut Vec<T>, _low: usize, _mid: usize, _high: usize) {}
}


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
impl<T> Sorter<T> for MergeSorter<T>
where
    T: PartialOrd + Clone + Default,
{
    fn split(&self, ar: &mut Vec<T>, low: usize, mid: &mut usize, high: usize) {
        let _ = ar;
        *mid = (high - low + 1) / 2 + low;
    }
    fn join(&self, ar: &mut Vec<T>, low: usize, mid: usize, high: usize) {
        let (mut i, mut j) = (low, mid);
        let mut vec = self.vec.borrow_mut();
        for k in low..high + 1 {
            if i >= mid {
                vec[k] = ar[j].clone();
                j += 1;
            } else if j > high {
                vec[k] = ar[i].clone();
                i += 1;
            } else {
                if ar[i] > ar[j] {
                    vec[k] = ar[j].clone();
                    j += 1;
                } else {
                    vec[k] = ar[i].clone();
                    i += 1;
                }
            }
        }
        for idx in low..high + 1 {
            ar[idx] = vec[idx].clone();
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
        while key < ar[j] && j > 0 {
            ar[j + 1] = ar[j].clone();
            j -= 1;
        }
        // Version 2
        if j == 0 {
            ar[j + 1] = ar[j].clone();
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
    let mut e = a.clone();
    let len = a.len(); // optimize merge sort with size len vector
    let high = a.len() - 1;
    MergeSorter::new(len).sort(&mut a, 0, high);
    QuickSorter.sort(&mut b, 0, high);
    SelectionSorter.sort(&mut c, 0, high);
    InsertionSorter.sort(&mut d, 0, high);
    HeapSorter::new().sort(&mut e, 0, high);
    println!("MergeSorter:     {:?}", a);
    println!("QuickSorter:     {:?}", b);
    println!("SelectionSorter: {:?}", c);
    println!("InsertionSorter: {:?}", d);
    println!("HeapSorter:      {:?}", e);
    println!("{:05b}", 1.clone());
    for i in (1..10 + 1).rev() {
        println!("{:?}", i);
    }
    println!("{}", 4 / 3);
}
