#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>
#include <unistd.h>
#include <stdlib.h>

// The idea of this exercise is to do what `execve` cannot by itself. So on in my case to print "Done" after execve succeeds.

int main(void)
{
    char *args[] = {"/bin/ls", "-l", NULL};
    char *envp[] = {NULL};

    pid_t p = fork();
    int stat; // for waitpid, the `stat` is (status) a pointer to an int where the exit status of child process will be stored

    if (p == 0) // child
    {
        execve(args[0], args, envp);
    }
    else if (p > 0) // parent
    {
        // Makes the parent process sleep until the child process finishes execution.
        waitpid(p, &stat, 0); // Technically if i dont care about the exist status i can just pass `NULL`

        if (stat == 0)
        {
            printf("Done with execve\n");
        }
        else
        {
            printf("Error executing execve\n");
        }
    }
    else
    {
        // error during fork
        perror("fork fail;");
        exit(1);
    }
}
