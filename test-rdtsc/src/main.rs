use revm_utils::time::*;
use std::thread;
use std::time::Duration;
fn main() {
    //  let duration = Duration::from_secs(60);
    // let cpu_frequency = get_cpu_frequency().expect("Get cpu frequency error!");
    // let mut time_record = TimeRecorder::now();
    // thread::sleep(duration);
    // let time = time_record.elapsed().to_nanoseconds(cpu_frequency);

    // println!("write_to_db ============ {:?} s, cpu_frequency: {:?}", convert_ns_to_secs(time.into()), cpu_frequency);
    println!("second ============ {:?} second ", convert_ns_to_secs(175054257));

    println!("Hello, world!");
}
