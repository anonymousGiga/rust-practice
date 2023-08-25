//! This example is part unit test and part demonstration.
//!
//! We show all of the registration macros, as well as all of the "emission" macros, the ones you
//! would actually call to update a metric.
//!
//! We demonstrate the various permutations of values that can be passed in the macro calls, all of
//! which are documented in detail for the respective macro.
use std::sync::Arc;

use metrics::{
    absolute_counter, counter, decrement_gauge, describe_counter, describe_gauge,
    describe_histogram, gauge, histogram, increment_counter, increment_gauge, register_counter,
    register_gauge, register_histogram, KeyName, SharedString,
};
use metrics::{Counter, CounterFn, Gauge, GaugeFn, Histogram, HistogramFn, Key, Recorder, Unit};

use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Mul,
};

#[inline(always)]
fn rdtsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}

fn get_cpu_frequency() -> Option<f64> {
    let file = File::open("/proc/cpuinfo").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(freq_str) = line.strip_prefix("cpu MHz\t\t: ") {
            let frequency: f64 = freq_str.parse().unwrap();
            return Some(frequency * 1e6);
        }
    }

    None
}

fn cycles_to_nanoseconds(cycles: u64, frequency: f64) -> u64 {
    let ns_per_cycle = 1_000_000_000 as f64 / frequency;
    (cycles as f64 * ns_per_cycle) as u64
}

struct PrintHandle(Key);

impl CounterFn for PrintHandle {
    fn increment(&self, value: u64) {
        println!("counter increment for '{}': {}", self.0, value);
    }

    fn absolute(&self, value: u64) {
        println!("counter absolute for '{}': {}", self.0, value);
    }
}

impl GaugeFn for PrintHandle {
    fn increment(&self, value: f64) {
        println!("gauge increment for '{}': {}", self.0, value);
    }

    fn decrement(&self, value: f64) {
        println!("gauge decrement for '{}': {}", self.0, value);
    }

    fn set(&self, value: f64) {
        println!("gauge set for '{}': {}", self.0, value);
    }
}

impl HistogramFn for PrintHandle {
    fn record(&self, value: f64) {
        println!("histogram record for '{}': {}", self.0, value);
    }
}

#[derive(Default)]
struct PrintRecorder;

impl Recorder for PrintRecorder {
    fn describe_counter(&self, key_name: KeyName, unit: Option<Unit>, description: SharedString) {
        println!(
            "(counter) registered key {} with unit {:?} and description {:?}",
            key_name.as_str(),
            unit,
            description
        );
    }

    fn describe_gauge(&self, key_name: KeyName, unit: Option<Unit>, description: SharedString) {
        println!(
            "(gauge) registered key {} with unit {:?} and description {:?}",
            key_name.as_str(),
            unit,
            description
        );
    }

    fn describe_histogram(&self, key_name: KeyName, unit: Option<Unit>, description: SharedString) {
        println!(
            "(histogram) registered key {} with unit {:?} and description {:?}",
            key_name.as_str(),
            unit,
            description
        );
    }

    fn register_counter(&self, key: &Key) -> Counter {
        Counter::from_arc(Arc::new(PrintHandle(key.clone())))
    }

    fn register_gauge(&self, key: &Key) -> Gauge {
        Gauge::from_arc(Arc::new(PrintHandle(key.clone())))
    }

    fn register_histogram(&self, key: &Key) -> Histogram {
        Histogram::from_arc(Arc::new(PrintHandle(key.clone())))
    }
}

fn init_print_logger() {
    let recorder = PrintRecorder::default();
    metrics::set_boxed_recorder(Box::new(recorder)).unwrap()
}

fn main() {
    let fre = get_cpu_frequency().unwrap();

    let server_name = "web03".to_string();

    init_print_logger();

    let common_labels = &[("listener", "frontend")];

    // Go through describing the metrics:
    describe_counter!("requests_processed", "number of requests processed");
    describe_counter!("bytes_sent", Unit::Bytes, "total number of bytes sent");
    describe_gauge!("connection_count", "current number of client connections");
    describe_histogram!(
        "svc.execution_time",
        Unit::Milliseconds,
        "execution time of request handler"
    );
    describe_gauge!("unused_gauge", "some gauge we'll never use in this program");
    describe_histogram!(
        "unused_histogram",
        Unit::Seconds,
        "some histogram we'll also never use in this program"
    );

    // // And registering them:
    // let counter1 = register_counter!("test_counter");
    // counter1.increment(1);
    mylib::add(2, 2);
    let start_time = rdtsc();
    let counter2 = register_counter!("test_counter", "type" => "absolute");
    let end_time = rdtsc();
    counter2.absolute(42);
    let end_time2 = rdtsc();
    let cycles = end_time - start_time;
    let elapsed_ns = cycles_to_nanoseconds(cycles, fre);
    println!("elapsed_ns ===================== {:?}", elapsed_ns);
    let cycles = end_time2 - end_time;
    let elapsed_ns = cycles_to_nanoseconds(cycles, fre);
    println!("elapsed_ns ===================== {:?}", elapsed_ns);

    let gauge1 = register_gauge!("test_gauge");
    gauge1.increment(1.0);
    let gauge2 = register_gauge!("test_gauge", "type" => "decrement");
    gauge2.decrement(1.0);
    let start_time = rdtsc();
    let gauge3 = register_gauge!("test_gauge", "type" => "set");
    let end_time = rdtsc();
    gauge3.set(3.1459);
    let end_time2 = rdtsc();
    let cycles = end_time - start_time;
    let elapsed_ns = cycles_to_nanoseconds(cycles, fre);
    println!("elapsed_ns ===================== {:?}", elapsed_ns);
    let cycles = end_time2 - end_time;
    let elapsed_ns = cycles_to_nanoseconds(cycles, fre);
    println!("elapsed_ns ===================== {:?}", elapsed_ns);

    let histogram1 = register_histogram!("test_histogram");
    histogram1.record(0.57721);

    // All the supported permutations of `counter!` and its increment/absolute versions:
    counter!("bytes_sent", 64);
    counter!("bytes_sent", 64, "listener" => "frontend");
    counter!("bytes_sent", 64, "listener" => "frontend", "server" => server_name.clone());
    counter!("bytes_sent", 64, common_labels);

    increment_counter!("requests_processed");
    increment_counter!("requests_processed", "request_type" => "admin");
    increment_counter!("requests_processed", "request_type" => "admin", "server" => server_name.clone());
    increment_counter!("requests_processed", common_labels);

    absolute_counter!("bytes_sent", 64);
    absolute_counter!("bytes_sent", 64, "listener" => "frontend");
    absolute_counter!("bytes_sent", 64, "listener" => "frontend", "server" => server_name.clone());
    absolute_counter!("bytes_sent", 64, common_labels);
    println!("++++++++++++++++++++++++++++++++++ 1");

    // All the supported permutations of `gauge!` and its increment/decrement versions:
    gauge!("connection_count", 300.0);
    gauge!("connection_count", 300.0, "listener" => "frontend");
    gauge!("connection_count", 300.0, "listener" => "frontend", "server" => server_name.clone());
    gauge!("connection_count", 300.0, common_labels);
    increment_gauge!("connection_count", 300.0);
    increment_gauge!("connection_count", 300.0, "listener" => "frontend");
    increment_gauge!("connection_count", 300.0, "listener" => "frontend", "server" => server_name.clone());
    increment_gauge!("connection_count", 300.0, common_labels);
    decrement_gauge!("connection_count", 300.0);
    decrement_gauge!("connection_count", 300.0, "listener" => "frontend");
    decrement_gauge!("connection_count", 300.0, "listener" => "frontend", "server" => server_name.clone());
    decrement_gauge!("connection_count", 300.0, common_labels);
    println!("++++++++++++++++++++++++++++++++++ 2");

    // All the supported permutations of `histogram!`:
    histogram!("svc.execution_time", 70.0);
    histogram!("svc.execution_time", 70.0, "type" => "users");
    histogram!("svc.execution_time", 70.0, "type" => "users", "server" => server_name.clone());
    histogram!("svc.execution_time", 70.0, common_labels);
    println!("++++++++++++++++++++++++++++++++++ 3");
}
