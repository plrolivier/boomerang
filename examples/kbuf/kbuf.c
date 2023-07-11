#include <linux/init.h>
#include <linux/module.h>
#include <linux/fs.h>
#include <linux/uaccess.h>
#include <linux/device.h>
#include <linux/kernel.h>
#include <linux/cdev.h>
#include <linux/mutex.h>


#define DEVICE_NAME "kbuf"
#define BUFFER_SIZE 1024


MODULE_LICENSE("GPL");
MODULE_AUTHOR("Paul OLIVIER");
MODULE_DESCRIPTION("A simple buffer in kernel memory");
MODULE_VERSION("1.0");

static int major_nb;
static struct class *kbuf_class = NULL;
static struct device *kbuf_dev = NULL;

static char kb_buffer[BUFFER_SIZE] = {0};
static int kb_offset = 0;
static DEFINE_MUTEX(kb_mutex);


static int kbuf_open(struct inode *inode, struct file *file);
static int kbuf_release(struct inode *inode, struct file *file);
static ssize_t kbuf_read(struct file *file, char __user *ubuf, size_t count, loff_t *f_pos);
static ssize_t kbuf_write(struct file *file, const char __user *ubuf, size_t count, loff_t *f_pos);
static loff_t kbuf_llseek(struct file *file, loff_t offset, int whence);
static long kbuf_ioctl(struct file *file, unsigned int cmd, unsigned long arg);
//static void print_kb_buffer(void);

static struct file_operations kbuf_fops = {
    .owner = THIS_MODULE,
    .open = kbuf_open,
    .release = kbuf_release,
    .read = kbuf_read,
    .write = kbuf_write,
    .llseek = kbuf_llseek,
    .unlocked_ioctl = kbuf_ioctl,
};



static int kbuf_open(struct inode *inode, struct file *file)
{
    if (!mutex_trylock(&kb_mutex)) {
        pr_err("[KBUF] Device already open\n");
        return -EBUSY;
    }

    pr_info("[KBUF] Open kernel buffer\n");
    return 0;
}

static int kbuf_release(struct inode *inode, struct file *file)
{
    mutex_unlock(&kb_mutex);
    pr_info("[KBUF] Close kernel buffer\n");
    return 0;
}

static ssize_t kbuf_read(struct file *file, char __user *ubuf, size_t count, loff_t *f_pos)
{
    ssize_t bytes_read = 0;

    if (kb_offset >= BUFFER_SIZE)
        return 0;

    bytes_read = min(count, (size_t)(BUFFER_SIZE - kb_offset));
    if (copy_to_user(ubuf, kb_buffer + kb_offset, bytes_read))
        return -EFAULT;

    kb_offset += bytes_read;
    *f_pos += bytes_read;

    pr_info("[KBUF] Read %zd bytes from buffer\n", bytes_read);
    return bytes_read;
}

static ssize_t kbuf_write(struct file *file, const char __user *ubuf, size_t count, loff_t *f_pos)
{
    ssize_t bytes_written = 0;

    if (kb_offset >= BUFFER_SIZE)
        return -ENOSPC;

    bytes_written = min(count, (size_t)(BUFFER_SIZE - kb_offset));
    if (copy_from_user(kb_buffer + kb_offset, ubuf, bytes_written))
        return -EFAULT;

    kb_offset += bytes_written;
    *f_pos += bytes_written;

    pr_info("[KBUF] Wrote %zd bytes to buffer\n", bytes_written);
    return bytes_written;
}

static loff_t kbuf_llseek(struct file *file, loff_t offset, int whence)
{
    loff_t new_pos = 0;

    switch (whence) {
        case 0: /* SEEK_SET */
            new_pos = offset;
            break;
        case 1: /* SEEK_CUR */
            new_pos = kb_offset + offset;
            break;
        case 2: /* SEEK_END */
            new_pos = BUFFER_SIZE;
            break;
        default:
            return -EINVAL;
    }

    if (new_pos < 0)
        return -EINVAL;

    kb_offset = new_pos;
    pr_info("[KBUF] New buffer offset is %d\n", kb_offset);
    return new_pos;
}

#define KBUF_IOCTL_PRINTK _IOC(_IOC_NONE, 'k', 1, 0)
#define LINE_SIZE 16
#define ROW_NB BUFFER_SIZE / LINE_SIZE

static void printk_kb_buffer(void)
{
    char line[LINE_SIZE * 3 + 1];
    size_t line_pos = 0;

    pr_info("[KBUF] kernel buffer offset is %d", kb_offset);
    for (size_t i=0; i<BUFFER_SIZE; i++) {
        line_pos += sprintf(line + line_pos, "%02X ", kb_buffer[i]);
        if ((i+1) % LINE_SIZE == 0) {
            pr_info("[KBUF] %s\n", line);
            line_pos = 0;
        }
    }
}

static long kbuf_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
{

    switch (cmd) {
        case KBUF_IOCTL_PRINTK:
            printk_kb_buffer();
            break;

        default:
            return -ENOTTY;
    }

    return 0;
}


static int __init kbuf_init(void)
{
    int ret = 0;
    pr_debug("[KBUF] Initializing kernel buffer module");

    major_nb = register_chrdev(0, DEVICE_NAME, &kbuf_fops);
    if (major_nb < 0) {
        pr_err("[KBUF] Failed to register major number\n");
        return ret;
    }
    pr_debug("[KBUF] Register with major number: %d\n", ret);

    kbuf_class = class_create(THIS_MODULE, "kbufclass");
    if (IS_ERR(kbuf_class)) {
        unregister_chrdev(major_nb, DEVICE_NAME);
        pr_err("[KBUF] Failed to register device class\n");
        return PTR_ERR(kbuf_class);
    }
    pr_debug("[KBUF] Register device class\n");

    kbuf_dev = device_create(kbuf_class, NULL, MKDEV(major_nb, 0), NULL, DEVICE_NAME);
    if (IS_ERR(kbuf_dev)) {
        class_destroy(kbuf_class);
        unregister_chrdev(major_nb, DEVICE_NAME);
        pr_err("[KBUF] Failed to create the device\n");
        return PTR_ERR(kbuf_dev);

    }
    pr_debug("[KBUF] Create device class\n");

    mutex_init(&kb_mutex);

    pr_info("[KBUF] Load kernel buffer module\n");
    return 0;
}

static void __exit kbuf_exit(void)
{
    mutex_destroy(&kb_mutex);
    device_destroy(kbuf_class, MKDEV(major_nb, 0));
    class_unregister(kbuf_class);
    class_destroy(kbuf_class);
    unregister_chrdev(major_nb, DEVICE_NAME);
    pr_info("[KBUF] Remove kernel buffer module\n");
}

module_init(kbuf_init);
module_exit(kbuf_exit);
