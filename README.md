# Rust and Design Pattern for Data Structure 
- Currently the abstract sorting function is implemented.

### Template Method Pattern
- Used when a system must implement an alogrithm
- with an invariant part, which always executes the same way
- but which depends on variant parts that are implemented differently depends on the circumstance

#### Templates for Comparision based Sorting Algorithm
- Merritt Sort Taxonomy  

- ```
        split
        /   \
   sort L1, sort L2
        |    |
   sorted L1', sorted L2'
        \    /
         join
   ```

