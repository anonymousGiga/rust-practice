.PHONY: clean
libpass.a:libpass.o
	ar rcs $@ $^
libpass.o:pass.c
	gcc -o libpass.o -c pass.c
clean:
	rm libpass.a libpass.o