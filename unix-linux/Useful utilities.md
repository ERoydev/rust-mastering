# 🧩 **hexdump — View Raw File Data**

**What it does:**  
Displays the raw contents of a file (or stream) in **hexadecimal and ASCII** form.

**When it’s useful:**
- When you need to **inspect binary files** (e.g., executables, images, compiled data).  
- To **check file headers or magic bytes** (identify unknown file types).  
- To **debug or reverse-engineer** binary formats or network data.  
- When analyzing **low-level data structures**, memory dumps, or disk images.  
- To verify **exact byte values** in files (useful for developers or system programmers).
- You can use it as a combination with `meld`, so when u have `hexdump` of two files you can compare them using meld

**Typical command:**  
```bash
hexdump -C file
```
or  
```bash
hd file
```

**Quick takeaway:**  
> Use `hexdump` when you want to *see exactly what bytes are inside a file* — especially if it’s not a normal text file.


# 🧩 **meld — Visual File and Folder Comparison Tool**

**What it does:**  
`meld` is a **graphical diff and merge tool** that helps you **visually compare** and **synchronize** files or directories.

**When it’s useful:**
- When you need to **compare two or more files** side-by-side to see differences.  
- To **merge changes** between versions of a file (great for coding or configuration).  
- When reviewing **Git or other version control diffs** visually.  
- To **compare folders** and quickly identify which files differ.  
- When you prefer a **GUI-based alternative** to command-line tools like `diff` or `git diff`.  

**Typical command:**  
```bash
meld file1 file2
```
or  
```bash
meld folder1 folder2
```

**Quick takeaway:**  
> Use `meld` when you want a **clear, visual way to spot and merge differences** between files or folders — perfect for developers and version control work.
