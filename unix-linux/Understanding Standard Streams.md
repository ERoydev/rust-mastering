# Understanding Standard Streams in Unix/Linux

In Unix/Linux, **standard streams** are the default input/output channels that every process can use. These are represented as special files in the `/dev` directory:

- `/dev/stdin` → Standard input (keyboard or piped input)  
- `/dev/stdout` → Standard output (terminal or redirected output)  
- `/dev/stderr` → Standard error (terminal, used for error messages)  

These streams are **used by applications** to communicate with the outside world in a standardized way. Applications don’t need to know whether the input comes from a keyboard, a file, or a pipe — they just use these standard channels and the OS handles the rest.

---

## 1. Standard Input (`/dev/stdin`)

- Represents the **input stream** of a process.  
- By default, connected to the **keyboard**.  
- Can also be redirected from a file or another program.  

### Example:

```bash
# Reading from a file as input
cat /dev/stdin < input.txt
```

Or piping input from another command:

```bash
echo "Hello World" | cat /dev/stdin
```

---

## 2. Standard Output (`/dev/stdout`)

- Represents the **output stream** of a process.  
- By default, goes to the **terminal**.  
- Can be redirected to a file or piped to another command.  

### Example:

```bash
# Writing directly to standard output
echo "Hello World" > /dev/stdout
```

Redirecting output to a file:

```bash
echo "Hello World" > /dev/stdout > output.txt
```

---

## 3. Standard Error (`/dev/stderr`)

- Represents the **error stream** of a process.  
- By default, goes to the terminal (separate from standard output).  
- Useful for logging errors separately from normal output.  

### Example:

```bash
# Writing an error message
echo "Something went wrong!" > /dev/stderr
```

Redirecting standard error to a file:

```bash
ls non_existing_file 2> error.log
```

---

## 4. How Applications Use These Streams

Applications interact with these streams **without caring about the underlying device**. They simply:

- Read input from **stdin**  
- Write normal output to **stdout**  
- Write errors to **stderr**  

The operating system handles the redirection, piping, or file I/O transparently.  

For example, in Solana development (see reference to **Gimlet with `solana-lldb`**), you can inspect how a program reads and writes through these streams while debugging.

### Diagram (Conceptual)

```
   +-------------------+
   |    Application    |
   |-------------------|
   | stdin   stdout stderr |
   +----+------+--------+
        |      |       
        |      |       
     Keyboard Terminal / Files / Pipes
```

---

## 5. Practical Tips

- **Piping between programs:**  

```bash
cat file.txt | grep "pattern" > /dev/stdout
```

- **Redirecting errors while keeping normal output:**  

```bash
command > /dev/stdout 2> /dev/stderr
```

- **Reading input from a script or command:**  

```bash
read line < /dev/stdin
echo "You typed: $line"
```

### Inspect File Descriptors with lsof

You can use `lsof` command to inspect file descriptors for a running process, specifically focusing on checking where process's standart error (stderr) is directed.

```
lsof -p <PID> -a -d <FD>
```

- `-p <PID>`: Filter by process ID.
- `-a`: Combine conditions (logical AND).
- `-d <FD>`: Filter by file descriptor number (e.g., 2 for stderr).

#### Example
To check where stderr (file descriptor 2) of process 3206 is pointing:
```
lsof -p 3206 -a -d 2
```
This is useful for debugging, logging, or verifying where a process writes its error output (e.g., a file, terminal, or pipe).

using `libc::dup2` on unsafe rust you can change that to point to somewhere else.


---

### Summary

| Stream      | File              | Default Location      |
|------------|-----------------|--------------------|
| Standard Input  | `/dev/stdin`   | Keyboard / pipe     |
| Standard Output | `/dev/stdout`  | Terminal / file     |
| Standard Error  | `/dev/stderr`  | Terminal / file     |

These special files make it easy to treat input/output like regular files while keeping the flexibility of piping and redirection. Applications, including those in blockchain debugging (Gimlet / Solana-lldb), rely on these streams extensively for interaction and logging.
