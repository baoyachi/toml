#![allow(elided_lifetimes_in_paths)]

const NUM_ENTRIES: &[usize] = &[10, 100];

mod map {
    use super::NUM_ENTRIES;

    #[divan::bench(args = NUM_ENTRIES)]
    fn toml_edit(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml_edit::DocumentMut>().unwrap());
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn toml(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml::Table>().unwrap());
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn toml_v05(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml_old::Value>().unwrap());
    }

    fn gen(num_entries: usize) -> String {
        let mut s = String::new();
        for i in 0..num_entries {
            s += &format!("[header_no_{i}]\n");
            s += "entry = 42\n";
        }
        s
    }
}

mod array {
    use super::NUM_ENTRIES;

    #[divan::bench(args = NUM_ENTRIES)]
    fn toml_edit(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml_edit::DocumentMut>().unwrap());
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn toml(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml::Table>().unwrap());
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn toml_v05(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml_old::Value>().unwrap());
    }

    fn gen(num_entries: usize) -> String {
        let mut s = String::new();
        for _ in 0..num_entries {
            s += "[[header]]\n";
            s += "entry = 42\n";
        }
        s
    }
}

fn main() {
    divan::main();
}
