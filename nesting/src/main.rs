use revm_utils::time::*;

#[derive(Debug)]
struct Interperter {
    #[cfg(feature = "enable_get_time")]
    call_time: u64,
}

impl Interperter {
    fn step(&mut self, cpu_frequency: f64, cnt: &mut u64, total_counter: &mut u64) {
        #[cfg(feature = "enable_get_time")]
        let mut time_record = TimeRecorder::now();

        interpreter_call_inner(cpu_frequency, cnt, total_counter);

        #[cfg(feature = "enable_get_time")]
        {
            let time = time_record.elapsed().to_cycles();
            self.call_time += time;
        }
    }

    #[cfg(feature = "enable_get_time")]
    fn get_total_time(&self) -> u64 {
        self.call_time
    }
}

fn main() {
    let cpu_frequency = get_cpu_frequency().expect("Get cpu frequency error!");
    let mut time_record = TimeRecorder::now();
    let mut cnt = 0;
    let mut total_counter = 0;
    let total_time = revm_call_inner(cpu_frequency, &mut cnt, &mut total_counter);
    let time = time_record.elapsed().to_nanoseconds(cpu_frequency);
    println!("total_time = {:?} cycles", total_time);
    println!(
        "total_time = {:?} ns",
        convert_to_nanoseconds(total_time, cpu_frequency)
    );
    println!("total counter = {:?}", total_counter);
    println!("main time = {:?} ns ", time);

    extra(cpu_frequency);
}

fn revm_call_inner(cpu_frequency: f64, cnt: &mut u64, total_counter: &mut u64) -> u64 {
    *total_counter += 1;
    // println!("cnt = {:?}", cnt);
    #[cfg(not(feature = "enable_get_time"))]
    let mut interpreter = Interperter {};

    #[cfg(feature = "enable_get_time")]
    let mut interpreter = Interperter { call_time: 0u64 };

    interpreter.step(cpu_frequency, cnt, total_counter);

    #[cfg(not(feature = "enable_get_time"))]
    return 0;

    #[cfg(feature = "enable_get_time")]
    interpreter.get_total_time()
}

fn interpreter_call_inner(cpu_frequency: f64, cnt: &mut u64, total_counter: &mut u64) {
    *cnt += 1;
    if *cnt > 10 {
        return;
    }

    host_call(cpu_frequency, cnt, total_counter);
}

fn host_call(cpu_frequency: f64, cnt: &mut u64, total_counter: &mut u64) {
    revm_call_inner(cpu_frequency, cnt, total_counter);
}

// // #[inline(always)]
// fn rdtsc() -> u64 {
//     unsafe { core::arch::x86_64::_rdtsc() }
// }

fn extra(cpu_frequency: f64) {
    let mut time_record1 = TimeRecorder::now();
    // let s = rdtsc();
    let mut time_record = TimeRecorder::now();
    // let _time1 = time_record.elapsed().to_nanoseconds(cpu_frequency);
    // let _time1 = time_record.elapsed().to_cycles();
    let _time1 = time_record.elapsed();
    // let time = time_record1.elapsed().to_nanoseconds(cpu_frequency);
    // let e = rdtsc() - s;
    let time = time_record1.elapsed().to_cycles();
    // println!("cycles ========================== {:?}", e);
    println!("cycles ========================== {:?}", time);
    println!(
        "time  ========================== {:?}",
        convert_to_nanoseconds(time, cpu_frequency)
    );
}
