# Weak Pointers

**Weak<T> remembers where T was, but does not keep T alive.**

For example imagine, that every node in a tree structure contain an Arc to each of its child nodes.
That way when a node is dropped, its child nodes that are no longer in use are all (recursively) dropped as well.

### Problem

The problem is if a child node contains an Arc to its parent node, neither will be dropped
since there's always at least one Arc that still refers to it.

### Solution

Solution for that problem provided from std library's Arc: `Weak<T>`.
A `Weak<T>` is called a weak pointer, behaves like `Arc<T>`, but does not prevent an object from getting dropped.
So `T` can be shared between several `Arc<T>` and `Weak<T>`, but when all `Arc<T>` objects are gone,
the `T` is dropped, regardless of whether there are any `Weak<T>` objects left.

### How Weak is used

So `Weak<T>` can exist without a `T`, and thus cannot provide a &T unconditionally, like `Arc<T>` can.
To access `T` given a `Weak<T>`, it can be upgraded to an `Arc<T>` through its `upgrade()` method.
This method returns an `Option<Arc<T>>`, returning `None` if `T` has already been dropped.

### Usage

In Arc-based structure, Weak can be used to break cycles. Child nodes in a tree structure could use `Weak`
rather than `Arc` for their parent node. That way dropping parent node is not prevented through the existence of its
child nodes.