obj-m+=kbuf.o
 
all:
	make -C /lib/modules/$(shell uname -r)/build/ M=$(PWD) modules
	$(CC) kbuf-uclient.c -o uclient

clean:
	make -C /lib/modules/$(shell uname -r)/build/ M=$(PWD) clean
	rm uclient