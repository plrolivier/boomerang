#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>


char *read_filename = "random.data";
char *write_filename = "write_random.data";


int main() {
    int fd;
    char buffer[4096];
    ssize_t res;

    // 1. Open the file in read-only mode
    fd = open(read_filename, O_RDONLY);
    if (fd == -1) {
        perror("Failed to open the read file");
        return 1;
    }

    // 2. Read and display the content of the file
    printf("Content of the file:\n");
    res = read(fd, buffer, sizeof(buffer));
    if (res < 0) {
        perror("Failed to read the file");
        close(fd);
        return 1;
    }
    // Print it to stdout
    write(1, buffer, res);

    // 3. Close the file
    if (close(fd) == -1) {
        perror("Error closing the file");
        return 1;
    }

    // 4. Open a new file
    fd = open(write_filename, O_WRONLY | O_CREAT | O_TRUNC, S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH);
    if (fd < 0) {
        perror("Failed to open the write file");
        return 1;
    }

    // 5. Write the read data to a new file
    res = write(fd, buffer, res);
    if (res < 0) {
        perror("Failed to write the buffer");
        close(fd);
        return 1;
    }

    // 6. Close the new file
    if (close(fd) == -1) {
        perror("Error closing the file");
        return 1;
    }

    return 0;
}
