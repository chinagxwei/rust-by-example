struct A;

struct S(A);

struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn example_14_1_gen_fn() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    generic::<char>(SGen('a'));

    generic(SGen('b'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_14_1_gen_fn() {
        example_14_1_gen_fn();
    }
}