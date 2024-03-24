#include <stdlib.h>

// the cache line
struct CacheLine {
  long long addr;
  int age;
};

// the cache set
struct CacheSet {
  int current_age;
  struct CacheLine *set;
};

// the cache
struct SACache {
  int blocks;
  int sets;
  long long set_mask;

  long long tag_mask;

  int lines_per_set;

  int misses;
  struct CacheSet *cache;
};

struct SACache *cache;

// initialize simulation
void sim_start(int B, int S, int W) {
  cache = malloc(sizeof(struct SACache));
  long long block_mask = (1 << (B)) - 1;
  cache->blocks = B;
  cache->sets = (1 << S);
  cache->set_mask = ((1 << (S + B)) - 1) & ~block_mask;
  cache->tag_mask = ~((1 << (S + B)) - 1);
  cache->lines_per_set = (1 << W);
  cache->misses = 0;
  cache->cache = malloc(cache->sets * sizeof(struct CacheSet));

  for (int i = 0; i < cache->sets; ++i) {
    cache->cache[i].set =
        malloc(cache->lines_per_set * sizeof(struct CacheLine));
    for (int j = 0; j < cache->lines_per_set; ++j) {
      struct CacheLine *line = &cache->cache[i].set[j];
      line->addr = 0;
      line->age = -1;
    }
    cache->cache[i].current_age = -1;
  }
}

// access one memory block
void sim_access(long long acc) {
  long long block_addr = acc & cache->tag_mask;
  int set_nr = (acc & cache->set_mask) >> cache->blocks;
  struct CacheSet *set = &cache->cache[set_nr];

  struct CacheLine *line = &set->set[0];
  for (int i = 0; i < cache->lines_per_set; ++i) {
    line = &set->set[i];
    // cache hit
    if (block_addr == line->addr && line->age != -1) {
      set->current_age += 1;
      line->age = set->current_age;
      return;
    }
  }

  // cache miss
  cache->misses += 1;

  line = &set->set[0];
  // find least recently used (smallest age)
  int replace_idx = 0;
  int smallest_age = 2147483647;
  for (int i = 0; i < cache->lines_per_set; ++i) {
    line = &set->set[i];
    if (line->age < smallest_age) {
      replace_idx = i;
      smallest_age = line->age;
    }
  }

  // replace lru with new addr
  line = &set->set[replace_idx];
  set->current_age += 1;
  line->age = set->current_age;
  line->addr = block_addr;
}

// return the number of misses
int sim_finish(void) {
  int misses = cache->misses;
  // Free cache lines
  for (int i = 0; i < cache->sets; i++) {
    free(cache->cache[i].set);
  }

  // Free cache sets
  free(cache->cache);

  // Free cache structure
  free(cache);
  return misses;
}
