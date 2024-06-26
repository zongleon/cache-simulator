#include "simulator.c"
#include <stdio.h>

int main() {

  sim_start(6, 11, 3);

  for (int i = 0; i < 10; i++) {
    if (i % 2 == 0) {
      for (int j = 0; j < 1000000; j++) {
        sim_access(32 * j);
      }
    } else {
      for (int j = 999999; j >= 0; j--) {
        sim_access(32 * j);
      }
    }
  }

  int misses = sim_finish();

  printf("Misses: %d\n", misses);
}
