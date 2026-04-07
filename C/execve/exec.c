#include <stdio.h>
#include <unistd.h>

/*
int execve(const char *filename, char *const argv[],
           char *const envp[]);
*/

int main(void)
{
    // argv is an array of agument strings passed to the new program.
    // The first of these strings should containt the filename associated with the file being executed.
    char *args[] = {"/bin/ls", "-l", NULL};
    // both argv and envp arrays must be terminated by a NULL pointer.

    char *envp[] = {NULL};

    int val = execve(args[0], args, envp);

    if (val == -1)
    {
        perror("Error executing execve");
    }

    printf("Done with execve\n");
    return 0;
}
/*
execve() on success does not return anything, and the text, data, bss, and stack of the calling process
are replaced by those of the new program loaded.

Kogato execute-na programata `exec`, koqto suzdava tozi proccess `execve()` system calla e uspeshen
memory-to na initial program-ata se replace-va s tova na novata programa (v nashiq sluchai `ls`).
Taka `printf` nikoga ne se execute-va, zashtoto biva override-nato


On failure, -1 is returned, and errno is set to indicate the error.
*/