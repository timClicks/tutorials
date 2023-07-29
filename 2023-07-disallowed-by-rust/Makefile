CC=gcc
CFLAGS=-Wall -Wextra -Werror

CXX=g++
CXXFLAGS= -std=c++14 -Wall -Wextra

.PHONY: all
all: shared-owner use-after-free equality nullptr invalid-iterator

shared-owner: go-shared-owner/main.go
	go build -o shared-owner go-shared-owner/main.go

nullptr: java-null-ptr/Main.class java-null-ptr/nullptr.sh
	cp java-null-ptr/nullptr.sh nullptr
	chmod +x nullptr

invalid-iterator: cpp-iterator/main.cpp
	$(CXX) $(CXXFLAGS) -o invalid-iterator cpp-iterator/main.cpp

use-after-free: c-use-after-free/main.o
	$(CC) $(CFLAGS) -o use-after-free c-use-after-free/main.o
	chmod +x use-after-free

equality: c-equality/main.o
	$(CC) $(CFLAGS) -o equality c-equality/main.o
	chmod +x equality

c-equality/main.o: c-equality/main.c
	$(CC) $(CFLAGS) -c c-equality/main.c -o c-equality/main.o

c-use-after-free/main.o: c-use-after-free/main.c
	$(CC) $(CFLAGS) -c c-use-after-free/main.c -o c-use-after-free/main.o

java-null-ptr/Main.class: java-null-ptr/Main.java
	cd java-null-ptr && javac Main.java




clean:
	rm -f invalid-iterator
	rm -f shared-owner
	rm -f use-after-free
	rm -f nullptr
	rm -f java-null-ptr/*.class
	rm -f equality
	rm -f c-use-after-free/main.o
	rm -f c-equality/main.o

