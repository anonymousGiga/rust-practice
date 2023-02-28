LDFLAGS := -Wl,--gc-sections -lpthread -ldl

exe: main.o err-delt/target/debug/liberr_delt.a
	$(CC) -o $@ $^ $(LDFLAGS)


main.o: main.c | exe 
	$(CC) -o $@ -c $<

clean:
	rm -rf exe *.o