#include "simulator.c"
#include <stdio.h>

int main() {

  sim_start(6, 11, 3);

  for (int i = 0; i < 10; i++) {
    for (int j = 0; j < 1000000; j++) {
      sim_access(4 * j);
    }
  }

  int misses = sim_finish();

  printf("Misses: %d\n", misses);
}
