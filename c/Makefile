##################################

CC      := gcc
CFLAGS  := -I.

LIB_C := minvideo.c
LIB_O := minvideo.o
SO    := minvideo.so

ALL_C := main.c $(LIB_C)
ALL_O := main.o $(LIB_O)
EXE   := minvideo_c

##################################

.PHONY: lib
lib: $(LIB_O) # TODO: compile .so

.PHONY: all
all: $(ALL_O)
	$(CC) -o $(EXE) $(ALL_O) $(CFLAGS)

.PHONY: clean
clean:
	rm -f *.o
	rm -f *.out
	rm -f *.d
	rm -f $(SO)
	rm -f $(EXE)

$(LIB_O): $(LIB_C)
	$(CC) -c -o $(LIB_C) $(CFLAGS)

$(ALL_O): $(ALL_C)
	$(CC) -c $(ALL_C) $(CFLAGS)