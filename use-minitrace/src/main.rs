#![allow(clippy::new_without_default)]

use std::time::Duration;

use minitrace::prelude::*;
use tracing::info;
use tracing_subscriber;

use minitrace::collector::Config;
use minitrace::collector::Reporter;

fn func1(i: u64) {
    let _guard = LocalSpan::enter_with_local_parent("func1");
    std::thread::sleep(Duration::from_millis(i));
    // func2(i);
}

// #[trace]
fn func2(i: u64) {
    let _guard = LocalSpan::enter_with_local_parent("func2");
    std::thread::sleep(Duration::from_millis(i));

    // minitrace::set_reporter(MyReporter2, Config::default());

    // let root = Span::root("root1", SpanContext::random());
    // {
    //     let _guard = root.set_local_parent();

    //     let _guard = LocalSpan::enter_with_local_parent("print1");

    //     for _ in 0..5 {
    //         println!("++++++");
    //     }
    // }
    // minitrace::flush();
}

#[trace]
fn func3(i: u64) {
    std::thread::sleep(Duration::from_millis(i));
}

fn test(cnt: u32) {
    minitrace::set_reporter(MyReporter2, Config::default());

    // let root = Span::root("root", SpanContext::random());
    let root = Span::root("root", SpanContext::random());
    {
        // // let _guard = root.set_local_parent();

        // // let _guard = LocalSpan::enter_with_local_parent("func1-0");
        // let _guard = Span::enter_with_parent("func1-0", &root);
        // func1(30);

        if cnt == 1 || cnt == 4 {
            let _guard = Span::enter_with_parent("func2-0", &root);
        }
        // // let _guard = LocalSpan::enter_with_local_parent("func2-0");
        func2(3000);

        // if cnt == 3 {
        //     let _guard = Span::enter_with_parent("func3-0", &root);
        // }
    }
    minitrace::flush();
}

fn tt() {
    use minitrace::collector::Config;
    use minitrace::collector::ConsoleReporter;
    use minitrace::prelude::*;

    // minitrace::set_reporter(ConsoleReporter, Config::default());
    minitrace::set_reporter(MyReporter, Config::default());

    {
        let root_span = Span::root("root", SpanContext::random());
        {
            let child_span = Span::enter_with_parent("print1", &root_span);
            func1(30);
        }
    }

    minitrace::flush();
}

fn main() {
    tracing_subscriber::fmt::init();
    for i in 0..8 {
        test(i);
        // tt();
    }
}

struct MyReporter;

impl Reporter for MyReporter {
    fn report(&mut self, spans: &[SpanRecord]) {
        for v in spans {
            if v.name == "func3" {
                info!(
                    target : "revm-test-sload",
                    "v.name = {:?}, v.duration_ns = {:?}, v.begin_time = {:?}",
                    v.name,
                    v.duration_ns,
                    v.begin_time_unix_ns,
                );
            }
        }
    }
}

struct MyReporter2;

impl Reporter for MyReporter2 {
    fn report(&mut self, spans: &[SpanRecord]) {
        for v in spans {
            info!(
                target : "revm-test-sload",
                "v.name = {:?}, v.duration_ns = {:?}, v.begin_time = {:?}",
                v.name,
                v.duration_ns,
                v.begin_time_unix_ns,
            );
        }
    }
}
// fn print_time(
//     writer: &mut BufWriter<File>,
//     span_records: &[SpanRecord],
//     event: &str,
// ) -> Result<()> {
//     for v in span_records {
//         if v.name == event {
//             info!(
//                 target : "revm-test-sload",
//                 event,
//                 v.duration_ns,
//             );
//             writeln!(writer, "{}", v.duration_ns)?;
//         }
//     }
//     writer.flush()?;

//     Ok(())
// }
