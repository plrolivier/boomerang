#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/ioctl.h>


#define DEVICE_NAME "/dev/kbuf"
#define BUFFER_SIZE 1024
#define KBUF_IOCTL_PRINTK _IOC(_IOC_NONE, 'k', 1, 0)



int main() {
    int fd, ret;
    char read_buffer[BUFFER_SIZE];
    char write_buffer[] = "Hello, kernel module!";

    // Open the buffer module
    fd = open(DEVICE_NAME, O_RDWR);
    if (fd < 0) {
        perror("Failed to open the buffer module");
        return EXIT_FAILURE;
    }

    // Read from the buffer module
    ssize_t bytes_read = read(fd, read_buffer, sizeof(read_buffer));
    if (bytes_read < 0) {
        perror("Failed to read from the buffer module");
        close(fd);
        return EXIT_FAILURE;
    }
    printf("Read %d bytes from buffer module: %s\n", bytes_read, read_buffer);

    // Seek to the beginning of the buffer
    if (lseek(fd, 0, SEEK_SET) < 0) {
        perror("Failed to seek in the buffer module");
        close(fd);
        return EXIT_FAILURE;
    }
    printf("Reset file offset to the start\n");

    // Write to the buffer module
    ssize_t bytes_written = write(fd, write_buffer, sizeof(write_buffer) - 1);
    if (bytes_written < 0) {
        perror("Failed to write to the buffer module");
        close(fd);
        return EXIT_FAILURE;
    }
    printf("Wrote %d bytes to buffer module: %s\n", bytes_written, write_buffer);

    // Seek to the beginning of the buffer module
    if (lseek(fd, 0, SEEK_SET) < 0) {
        perror("Failed to seek in the buffer module");
        close(fd);
        return EXIT_FAILURE;
    }
    printf("Reset file offset to the start\n");

    // Read from the buffer module again
    bytes_read = read(fd, read_buffer, sizeof(read_buffer));
    if (bytes_read < 0) {
        perror("Failed to read from the buffer module");
        close(fd);
        return EXIT_FAILURE;
    }
    printf("Read from buffer module: %s\n", read_buffer);

    /* Print info the content of the buffer
    ret = ioctl(fd, KBUF_IOCTL_PRINTK, 0);
    if (ret < 0) {
        perror("Fail to issue KBUF_IOCTL_PRINTK command\n");
    }
    */

    // Close the buffer module
    if (close(fd) < 0) {
        perror("Failed to close the buffer module");
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
