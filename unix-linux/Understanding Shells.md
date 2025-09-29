# Understanding Shells and `zsh -il`

## 1. What is a Shell?

A **shell** is a command-line interface that allows users to interact with the operating system. It interprets commands, runs programs, and manages processes. Common shells include:

- **Bash** (`bash`)
- **Zsh** (`zsh`)
- **Fish** (`fish`)

Shells can be categorized as:

- **Interactive Shells**: Accept user input from a terminal. Examples: running `bash` or `zsh` directly.  
- **Non-Interactive Shells**: Run scripts without user interaction. Example: `bash script.sh`  
- **Login Shells**: Read configuration files meant for login sessions (e.g., `.zprofile`, `.zlogin`).  
- **Non-Login Shells**: Do not read login configuration files (usually for new terminals opened from an already logged-in session).

---

## 2. `zsh -il`

The command:

```bash
zsh -il
```

- `-i` → starts an **interactive shell**.  
- `-l` → starts a **login shell** (reads login configuration files).  

### Behavior

- Creates a **new child shell**.
- The **parent shell is blocked** until the child shell exits.
- Commands executed inside the child shell do **not affect the parent shell** directly.
- Output from the child shell is visible inside the new shell session, not in the original shell.

### Example

```bash
$ echo "Parent shell"
Parent shell
$ zsh -il
# New shell starts, parent shell is blocked
$ echo "Inside child shell"
Inside child shell
$ exit
# Back to parent shell
```

---

## 3. Running Commands Without Blocking

To run a command in a login shell **without keeping the shell open**, use:

```bash
zsh -il -c "your_command_here"
```

Example:

```bash
zsh -il -c "echo Hello from login shell"
```

---

## 4. Summary Table

| Option | Meaning | Effect |
|--------|---------|--------|
| `-i`   | Interactive | Accepts user input |
| `-l`   | Login       | Reads login configuration files |
| `-il`  | Interactive Login | Spawns a new shell; parent blocked until exit |

---

## References

- [Zsh Manual](http://zsh.sourceforge.net/Doc/Release/)
- [Unix Shell Types](https://www.gnu.org/software/bash/manual/bash.html)
