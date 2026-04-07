#include <stdio.h>
#include <sys/types.h>
#include <stdlib.h>
#include <unistd.h>

// resource: https://medium.com/@The_Anshuman/lets-understand-fork-in-linux-cb59e56ceb41

int main()
{
    // make two process which run same
    // program after this instruction
    pid_t p = fork();

    if (p == 0)
    {
        // if p is 0, then we are in the child process (0 is a way to identify the child process)
        printf("We are in the child process, process_id(pid) = %d \n", p);
    }
    else if (p > 0)
    {
        // if p is greater than 0, then we are in the parent process and p is the process id(real process ID not 0) of the child process
        printf("We are in the parent process, process_id(pid) = %d \n", p);
    }
    if (p < 0)
    {
        // fork syscall failed, maybe no space for new process, or too many processes
        perror("fork fail");
        exit(1);
    }

    return 0;
}