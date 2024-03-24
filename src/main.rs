mod splay;

type Addr = std::ffi::c_longlong;
type Int = std::ffi::c_int;
struct Cache {
    block_bits: u8,
    set_bits: u8,
    sets: Vec<splay::LRUSplay>,
    miss: i32,
}

impl Cache {
    fn access(&mut self, addr: Addr) {
        let set = (addr >> self.block_bits) & ((1 << self.set_bits) - 1);
        let tag = addr >> (self.block_bits + self.set_bits);
        if !self.sets[set as usize].access(tag) {
            self.miss += 1;
        }
    }
}

extern "C" {
    // void sim_start(int B, int S, int W) {}
    fn sim_start(B: Int, S: Int, W: Int);

    // void sim_access(long long acc) {}
    fn sim_access(acc: Addr);

    // int sim_finish(void) { return 0; }
    fn sim_finish() -> Int;
}
#[allow(non_snake_case)]
unsafe fn matrix_multiply(a: Addr, b: Addr, c: Addr, n: i64, B: Int, S: Int, W: Int) -> (i32, i32) {
    #[cfg(not(miri))]
    sim_start(B, S, W);
    let mut cache = Cache {
        block_bits: B as u8,
        set_bits: S as u8,
        sets: vec![],
        miss: 0,
    };
    for _ in 0..(1 << 3) {
        cache.sets.push(splay::LRUSplay::new(1 << W));
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                // C[i * n + j] += A[i * n + k] * B[k * n + j];
                // access A[i * n + k]
                cache.access(a + (i * n + k) * 8);
                #[cfg(not(miri))]
                sim_access(a + (i * n + k) * 8);
                // access B[k * n + j]
                cache.access(b + (k * n + j) * 8);
                #[cfg(not(miri))]
                sim_access(b + (k * n + j) * 8);
                // access C[i * n + j]
                cache.access(c + (i * n + j) * 8);
                #[cfg(not(miri))]
                sim_access(c + (i * n + j) * 8);
                // access C[i * n + j]
                cache.access(c + (i * n + j) * 8);
                #[cfg(not(miri))]
                sim_access(c + (i * n + j) * 8);
            }
        }
    }
    #[cfg(miri)]
    return (0, cache.miss);
    #[cfg(not(miri))]
    return (sim_finish(), cache.miss);
}

#[allow(non_snake_case)]
unsafe fn matrix_multiply2(
    a: Addr,
    b: Addr,
    c: Addr,
    n: i64,
    B: Int,
    S: Int,
    W: Int,
) -> (i32, i32) {
    #[cfg(not(miri))]
    sim_start(B, S, W);
    let mut cache = Cache {
        block_bits: B as u8,
        set_bits: S as u8,
        sets: vec![],
        miss: 0,
    };
    for _ in 0..(1 << 3) {
        cache.sets.push(splay::LRUSplay::new(1 << W));
    }

    for i in 0..n {
        for k in 0..n {
            for j in 0..n {
                // C[i * n + j] += A[i * n + k] * B[k * n + j];
                // access A[i * n + k]
                cache.access(a + (i * n + k) * 8);
                #[cfg(not(miri))]
                sim_access(a + (i * n + k) * 8);
                // access B[k * n + j]
                cache.access(b + (k * n + j) * 8);
                #[cfg(not(miri))]
                sim_access(b + (k * n + j) * 8);
                // access C[i * n + j]
                cache.access(c + (i * n + j) * 8);
                #[cfg(not(miri))]
                sim_access(c + (i * n + j) * 8);
                // access C[i * n + j]
                cache.access(c + (i * n + j) * 8);
                #[cfg(not(miri))]
                sim_access(c + (i * n + j) * 8);
            }
        }
    }
    #[cfg(miri)]
    return (0, cache.miss);
    #[cfg(not(miri))]
    return (sim_finish(), cache.miss);
}

#[allow(non_snake_case)]
unsafe fn matrix_vec_multiply(
    a: Addr,
    b: Addr,
    c: Addr,
    n: i64,
    B: Int,
    S: Int,
    W: Int,
) -> (i32, i32) {
    #[cfg(not(miri))]
    sim_start(B, S, W);
    let mut cache = Cache {
        block_bits: B as u8,
        set_bits: S as u8,
        sets: vec![],
        miss: 0,
    };
    for _ in 0..(1 << 3) {
        cache.sets.push(splay::LRUSplay::new(1 << W));
    }

    for i in 0..n {
        for k in 0..n {
            // C[i] += A[i * n + k] * B[i];
            // access A[i * n + k]
            cache.access(a + (i * n + k) * 8);
            #[cfg(not(miri))]
            sim_access(a + (i * n + k) * 8);
            // access B[i]
            cache.access(b + i * 8);
            #[cfg(not(miri))]
            sim_access(b + i * 8);
            // access C[i]
            cache.access(c + i * 8);
            #[cfg(not(miri))]
            sim_access(c + i * 8);
            // access C[i]
            cache.access(c + i * 8);
            #[cfg(not(miri))]
            sim_access(c + i * 8);
        }
    }
    #[cfg(miri)]
    return (0, cache.miss);
    #[cfg(not(miri))]
    return (sim_finish(), cache.miss);
}

fn main() {
    unsafe {
        let n = 100;
        let (sim_miss, splay_miss) = matrix_multiply(0, 8 * n * n, 16 * n * n, n, 6, 3, 4);
        println!("=== Matrix Multiply (IJK, n = {n}) ===");
        println!("Simulation: {}, Expected: {}", sim_miss, splay_miss);
    }
    unsafe {
        let n = 321;
        let (sim_miss, splay_miss) = matrix_multiply(0, 8 * n * n, 16 * n * n, n, 6, 3, 4);
        println!("=== Matrix Multiply (IJK, n = {n}) ===");
        println!("Simulation: {}, Expected: {}", sim_miss, splay_miss);
    }
    unsafe {
        let n = 100;
        let (sim_miss, splay_miss) = matrix_multiply2(0, 8 * n * n, 16 * n * n, n, 6, 3, 4);
        println!("=== Matrix Multiply (IKJ, n = {n}) ===");
        println!("Simulation: {}, Expected: {}", sim_miss, splay_miss);
    }
    unsafe {
        let n = 321;
        let (sim_miss, splay_miss) = matrix_multiply2(0, 8 * n * n, 16 * n * n, n, 6, 3, 4);
        println!("=== Matrix Multiply (IKJ, n = {n}) ===");
        println!("Simulation: {}, Expected: {}", sim_miss, splay_miss);
    }
    unsafe {
        let n = 100;
        let (sim_miss, splay_miss) =
            matrix_vec_multiply(0, 8 * n * n, 8 * n * n + 8 * n, n, 6, 3, 4);
        println!("=== Matrix Vector Multiply (n = {n}) ===");
        println!("Simulation: {}, Expected: {}", sim_miss, splay_miss);
    }
    unsafe {
        let n = 321;
        let (sim_miss, splay_miss) =
            matrix_vec_multiply(0, 8 * n * n, 8 * n * n + 8 * n, n, 6, 3, 4);
        println!("=== Matrix Vector Multiply (n = {n}) ===");
        println!("Simulation: {}, Expected: {}", sim_miss, splay_miss);
    }
}
