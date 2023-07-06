obj-m+=kbuf.o
 
all:
	make -C /lib/modules/$(shell uname -r)/build/ M=$(PWD) modules
	$(CC) test-kbuf.c -o test-kbuf

clean:
	make -C /lib/modules/$(shell uname -r)/build/ M=$(PWD) clean
	rm test-kbuf
