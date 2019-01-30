CC=clang
CXX=clang++
RM=rm -f
CPPFLAGS=-g -H
LDFLAGS=-g
LDLIBS=-lX11

SRCS=example.cpp
OBJS=$(subst .cpp,.o,$(SRCS))

all: example

example: $(OBJS)
	$(CXX) $(LDFLAGS) -o example $(OBJS) $(LDLIBS) 

depend: .depend

.depend: $(SRCS)
	$(RM) ./.depend
	$(CXX) $(CPPFLAGS) -MM $^>>./.depend;

clean:
	$(RM) $(OBJS)

distclean: clean
	$(RM) *~ .depend

include .depend
