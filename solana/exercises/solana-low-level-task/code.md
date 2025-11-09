

Notes from explanations i remember, revisit and refill the gaps:

1. When CPU registers are handling instruction, for example in SVM i have 1-5 are general purpose registers for the input/fn arguments or some computations, so since i can use only 5 of them if more than 5 arguments are passed which rust allows, that makes the SVM VM to start to use the stack to store them.


2. Memory fragmentation in Linux, Buddies..
- So when i have a big buffer [] with some data stored in it that makes the system to allocate and reallocate which can lead to memory fragmentation overtime because i build a big buffer that are like two big buffers that are encapsulated together to form a Big buddie. And over time the system memory, when the machine works fo long time can fragment the memory because of that.


3. Cache-line i can have a problem with multithread in CPU for example:
- I have buffer(imagine it like an object) with two fields in it 
```
{
    &[u64],
    &[u64]
}
```

And when one of the threads is using the first one and the second thread is using the second one there is a problem:
1. For some reason even if they both touch different fields for some reason that apply some change and the second can occur a problem related to multithreading.
2. So overall the system will behave better if these two are divided in different buffers to be used in the cache-line.
