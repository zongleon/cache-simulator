#include "simulator.c"
#include <stdio.h>
#include <time.h>

int main() {

  printf("Cache: 2^6 bytes per cacheline\n");
  printf("Cache: 2^11 sets\n\n");
  for (int lines_per_cache = 1; lines_per_cache <= 8; lines_per_cache++) {

    int accesses = 0;
    clock_t t = clock();
    sim_start(6, 11, lines_per_cache);

    for (int i = 0; i < 10; i++) {
      if (i % 2 == 0) {
        for (int j = 0; j < 1000000; j++) {
          sim_access(32 * j);
          accesses += 1;
        }
      } else {
        for (int j = 999999; j >= 0; j--) {
          sim_access(32 * j);
          accesses += 1;
        }
      }
    }

    int misses = sim_finish();

    printf("2^%d cachelines per set:\n", lines_per_cache);
    printf("%8d misses | %10d accesses | %4f miss ratio\n", misses, accesses,
           (float)misses / accesses);
    printf("Time elapsed: %lu ms\n\n", (clock() - t) * 1000 / CLOCKS_PER_SEC);
  }
}
