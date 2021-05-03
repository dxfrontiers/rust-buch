#include <stdio.h>
#include <stdlib.h>

int add(size_t a, size_t b){
  return a + b;
}

void increment(size_t *target){
  *target+=1;
}

/*
Creates a dangling pointer to the local variable `i` and returns it
This might lead to all sorts of interesting but unintended behaviour

We use this sample to demonstrate the lack of control rust has over external function calls
*/
size_t* dangling(){
  size_t i = 7;
  size_t* ptr = &i;
  return ptr;
}

/*
Creates a null pointer and returns it
This might lead to all sorts of interesting but unintended behaviour

We use this sample to demonstrate the lack of control rust has over external function calls
*/
size_t* zero(){
  size_t* ptr = NULL;
  return ptr;
}