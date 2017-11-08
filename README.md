# Rust and Design Pattern for Data Structure

- Sorter
- List

### Template Method Pattern

- Used when a system must implement an alogrithm
- with an invariant part, which always executes the same way
- but which depends on variant parts that are implemented differently depends on the circumstance
- The pattern consists of two parts:
  - template function: the function that always execute the same way (invariant), e.g sort
  - primitive function: the function is varied based on circumstance, e.g split & join

#### Templates for Comparision based Sorting Algorithms

- Merritt Sort Taxonomy  

- ```
        split
       /     \
      L1      L2
      |       |
    sort     sort
      |       |
     L1'      L2'
        \    /
         join
    ```
   ```

- ```python
   sort(a,lo,hi):
     if lo < hi: #at least two elements
       split(a,lo,mid,hi) #postcondition: low < mid <= high
       sort(a,lo,mid-1)
       sort(a,mid,hi)
       join(a,low,hi)
   ```

- run time analysis

   - T(n) refers to total time for a recursive function, e.g sort
   - O(n) refers to running time for a non-recursive function e.g split,join

## Decorator Pattern

- Decorator Design Pattern
- Parametric Decorator Pattern
- To Have ease of testing real performace vs theorical performance
  - ways to not modify the original code
  - consider the metrics, aka the standard measurement
  - consider the number of different operation and instruction in asmbly
  - cost for each operation may varies, idea of weighted sum
  - timer/clock is not useful becasue of time-sharing of today's OS design
- Assignment, Comparision, Probe(Ramdom Access Read)
- `sorter < Counter< data_type > >`
- ​

## Composite, State, Visitor Pattern

#### List

- Classic List
  - List & Node
  - single linked list
  - single linked list with tail pointer, or a dummy head
  - double linked list
  - double linked list with a dummy head
- Lists that adopted Composite,State,Vistor Design Pattern
  - ​