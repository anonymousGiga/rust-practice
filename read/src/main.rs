use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // pure_opcode(5.61)?;
    // print()?;
    pure_cat(5.61)?;

    Ok(())
}

fn pure_opcode(rdt: f64) -> io::Result<()> {

    // 打开文件
    // let file = File::open("/home/andy/Source/work/experiment/20231218/opcode1")?;
    let file = File::open("/home/andy/Source/work/experiment/20240115/opcode1.txt")?;
    let reader = io::BufReader::new(file);

    let mut total = 0.0;
    let mut start = true;
    // 循环读取每一行数据
    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split_whitespace().collect();

        // 检查行中是否有足够的字段
        if fields.len() >= 2 {
            // 将每个值读取到变量中并打印出来
            let value1: &str = fields[0];
            let value2: i64 = fields[1].parse().unwrap();
            let value3: f64 = fields[2].parse().unwrap();
            let value4: f64 = fields[3].parse().unwrap();
            let _value5: f64 = fields[4].parse().unwrap();
            let value6: f64 = fields[5].parse().unwrap();
            let value7: f64 = fields[6].parse().unwrap();
            let value8: f64 = fields[7].parse().unwrap();
            let value9: f64 = fields[8].parse().unwrap();
            let value10: f64 = fields[9].parse().unwrap();
            let value11: &str = fields[10];


            let rdtsc = rdt * value2 as f64 / 1_000_000_000.0;
            let after_v4 = if value4 > rdtsc {
                value4 - rdtsc 
            } else {
                0.0
            };
            if start {
                total = after_v4;
                start = false;
            } 

            let after_v5 = after_v4 * 100.0 / total;
            let after_v6 = if value6 > rdt{
                value6 - rdt
            } else {
                0.0
            };
            let middle = 15;
            // 打印读取的值
            println!(
                "{:<15}  {:>15}  {:>middle$.3}  {:>middle$.2}  {:>middle$.3}  {:>middle$.2}  {:>middle$.3}  {:>middle$.2}  {:>middle$} {:>middle$} {:>middle$}",
                value1, value2, value3, after_v4, after_v5, after_v6, value7, value8, value9, value10, value11
            );
        }
    }
    Ok(())
}

fn print() -> io::Result<()> {
    // 打开文件
    // let file = File::open("/home/andy/Source/work/experiment/20231218/opcode_pure1")?;
    let file = File::open("/home/andy/Source/work/experiment/20240115/order_opcode.txt")?;
    let reader = io::BufReader::new(file);

    // 循环读取每一行数据
    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split_whitespace().collect();

        // 检查行中是否有足够的字段
        if fields.len() >= 2 {
            // 将每个值读取到变量中并打印出来
            let value1: &str = fields[0];
            let value2: i64 = fields[1].parse().unwrap();
            let value3: f64 = fields[2].parse().unwrap();
            let value4: f64 = fields[3].parse().unwrap();
            let value5: f64 = fields[4].parse().unwrap();
            let value6: f64 = fields[5].parse().unwrap();
            let value7: f64 = fields[6].parse().unwrap();
            let value8: f64 = fields[7].parse().unwrap();
            let value9: f64 = fields[8].parse().unwrap();
            let value10: f64 = fields[9].parse().unwrap();
            let value11: &str = fields[10];
            let value12: f64 = fields[11].parse().unwrap();


            let middle = 15;
            // 打印读取的值
            println!(
                "{:<15}  {:>15}  {:>middle$.3}  {:>middle$.2}  {:>middle$.3} {:>middle$.3} {:>middle$.2}  {:>middle$.3}  {:>middle$.2}  {:>middle$.2} {:>middle$.2} {:>middle$}",
                value1, value2, value3, value4, value5, value12, value6, value7, value8, value9, value10, value11
            );
        }
    }

    Ok(())
}

fn pure_cat(rdt: f64) -> io::Result<()> {
    // 打开文件
    // let file = File::open("/home/andy/Source/work/experiment/20231218/cat")?;
    let file = File::open("/home/andy/Source/work/experiment/20240115/cc")?;
    let reader = io::BufReader::new(file);

    let total = 52346.45;
    // 循环读取每一行数据
    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split_whitespace().collect();

        // 检查行中是否有足够的字段
        if fields.len() >= 2 {
            // 将每个值读取到变量中并打印出来
            let value1: &str = fields[0];
            let value2: i64 = fields[1].parse().unwrap();
            let value3: f64 = fields[2].parse().unwrap();
            let value4: f64 = fields[3].parse().unwrap();
            let _value5: f64 = fields[4].parse().unwrap();
            let value6: f64 = fields[5].parse().unwrap();

            let rdtsc = rdt * value2 as f64 / 1_000_000_000.0;
            let after_v4 = if value4 > rdtsc {
                value4 - rdtsc 
            } else {
                0.0
            };
            let after_v5 = after_v4 * 100.0 / total;

            let after_v6 = if value6 > rdt {
                value6 - rdt
            } else {
                0.0
            };


            let middle = 15;
            // 打印读取的值
            println!(
                "{:<15}  {:>15}  {:>middle$.3}  {:>middle$.2}  {:>middle$.3} {:>middle$.3} ",
                value1, value2, value3, after_v4, after_v5, after_v6 
            );
        }
    }

    Ok(())
}
