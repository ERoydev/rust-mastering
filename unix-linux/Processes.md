## Inspecting Processes and Ports on macOS/Linux

### 1. Checking Running Processes with `ps`

The `ps` command is used to display information about currently running processes.

For example, if you want to inspect processes related to **Anchor** (the Solana development framework), you can filter by keyword:

```bash
ps -xo pid,ppid,stat,command | grep anchor
```

`x` -> Include all processes, even those without a terminal (Shows background/daemon processes)

This command displays any running processes that include “anchor” in their command line, showing:

- **PID** — Process ID  
- **PPID** — Parent Process ID (which process started it)  
- **STAT** — Current process status (running, sleeping, etc.)  
- **COMMAND** — The full command that launched the process  

---

### 2. Checking Which Process Is Using a Port with `lsof`

To find out which process is using a specific port (for example, port **6612**), use the `lsof` command:

```bash
lsof -n -i :6612
```

Example output:

```
node     90824 emilemilovroydev   16u  IPv4 0x91c8cac215059cef      0t0  TCP 127.0.0.1:6612->127.0.0.1:58023 (ESTABLISHED)
codelldb 90850 emilemilovroydev   17u  IPv4 0x6e213d667b3db52d      0t0  TCP 127.0.0.1:58023->127.0.0.1:6612 (ESTABLISHED)
```

From this output, we can see that a **Node.js** process has established a TCP connection with **CodeLLDB**, along with their corresponding PIDs.

---
