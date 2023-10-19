use regex::Regex;
use revm::OpCode;
use revm_utils::time::convert_ns_to_secs;
use revm_utils::types::*;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
struct TotalRecord {
    block_numbers: Vec<u64>,
    total_opcode_exec_ret: BTreeMap<u8, (u64, u128)>,
    all_host_time: Vec<HostTime>,
    all_cache_hits: Vec<(u64, u64, u64, u64)>,
    all_cache_misses: Vec<(u64, u64, u64, u64)>,
    all_cache_misses_penalty: Vec<(u128, u128, u128, u128)>,
    all_cachedb_size: Vec<usize>,
}

impl TotalRecord {
    fn update_block_number(&mut self, block_number: u64) {
        self.block_numbers.push(block_number);
    }

    fn update_revm_record(&mut self, single_record: &RevmMetricRecord) {
        if let Some(opcode_time) = single_record.opcode_time.as_ref() {
            for (key, value) in opcode_time {
                {
                    match self.total_opcode_exec_ret.entry(*key) {
                        Entry::Occupied(mut entry) => {
                            let mut entry = entry.get_mut();
                            entry.0 += value.0;
                            entry.1 += value.1 as u128;
                        }
                        Entry::Vacant(entry) => {
                            entry.insert((value.0, value.1 as u128));
                        }
                    };
                }
            }
        }

        self.all_host_time.push(single_record.host_time);
        self.all_cache_hits.push(single_record.cache_hits);
        self.all_cache_misses.push(single_record.cache_misses);
        self.all_cache_misses_penalty
            .push(single_record.cache_misses_penalty);
        // =================== debug ===================
        // let mut total_host_fn_time: u128 = 0;
        // total_host_fn_time += single_record.host_time.step;
        // total_host_fn_time += single_record.host_time.step_end;
        // total_host_fn_time += single_record.host_time.env;
        // total_host_fn_time += single_record.host_time.load_account;
        // total_host_fn_time += single_record.host_time.block_hash;
        // total_host_fn_time += single_record.host_time.balance;
        // total_host_fn_time += single_record.host_time.code;
        // total_host_fn_time += single_record.host_time.code_hash;
        // total_host_fn_time += single_record.host_time.sload;
        // total_host_fn_time += single_record.host_time.sstore;
        // total_host_fn_time += single_record.host_time.log;
        // total_host_fn_time += single_record.host_time.selfdestruct;
        // total_host_fn_time += single_record.host_time.create;
        // total_host_fn_time += single_record.host_time.call;
        // println!(
        //     "total host function time(secs) ==> {:?}",
        //     convert_ns_to_secs(total_host_fn_time)
        // );
    }

    fn update_cachedb_size(&mut self, cachedb_size: usize) {
        self.all_cachedb_size.push(cachedb_size);
    }

    fn print(&self) {
        println!("");
        println!("===============================Metric of instruction=====================");
        println!("Opcode                            total_times        spend_time(seconds)");
        let mut total_opcode_time: u128 = 0;
        for (op, metric) in &self.total_opcode_exec_ret {
            if let Some(op) = OpCode::try_from_u8(*op) {
                println!(
                    "{:20}{:20}{:30}",
                    op.as_str(),
                    metric.0,
                    convert_ns_to_secs(metric.1)
                );
                total_opcode_time += metric.1;
            }
        }
        println!(
            "total_opcode_time(secs) ===> {:?}",
            convert_ns_to_secs(total_opcode_time)
        );
        println!("");

        println!("");
        let mut total_host_time: HostTime = HostTime::default();
        let mut total_host_fn_time: u128 = 0;
        let _ = self
            .all_host_time
            .iter()
            .map(|v| {
                total_host_time.step += v.step;
                total_host_fn_time += v.step;

                total_host_time.step_end += v.step_end;
                total_host_fn_time += v.step_end;

                total_host_time.env += v.env;
                total_host_fn_time += v.env;

                total_host_time.load_account += v.load_account;
                total_host_fn_time += v.load_account;

                total_host_time.block_hash += v.block_hash;
                total_host_fn_time += v.block_hash;

                total_host_time.balance += v.balance;
                total_host_fn_time += v.balance;

                total_host_time.code += v.code;
                total_host_fn_time += v.code;

                total_host_time.code_hash += v.code_hash;
                total_host_fn_time += v.code_hash;

                total_host_time.sload += v.sload;
                total_host_fn_time += v.sload;

                total_host_time.sstore += v.sstore;
                total_host_fn_time += v.sstore;

                total_host_time.log += v.log;
                total_host_fn_time += v.log;

                total_host_time.selfdestruct += v.selfdestruct;
                total_host_fn_time += v.selfdestruct;

                total_host_time.create += v.create;
                total_host_fn_time += v.create;

                total_host_time.call += v.call;
                total_host_fn_time += v.call;
            })
            .collect::<Vec<_>>();

        println!("===============================Metric of HostTime=====================");
        println!("Host function                             total_spend_time(seconds)");
        println!(
            "step                                      {:?}",
            convert_ns_to_secs(total_host_time.step)
        );
        println!(
            "step_end                                  {:?}",
            convert_ns_to_secs(total_host_time.step_end)
        );
        // println!("env                                        {:?}", convert_ns_to_secs(total_host_time.env));
        println!(
            "load_account                              {:?}",
            convert_ns_to_secs(total_host_time.load_account)
        );
        println!(
            "block_hash                                {:?}",
            convert_ns_to_secs(total_host_time.block_hash)
        );
        println!(
            "balance                                   {:?}",
            convert_ns_to_secs(total_host_time.balance)
        );
        println!(
            "code                                      {:?}",
            convert_ns_to_secs(total_host_time.code)
        );
        println!(
            "code_hash                                 {:?}",
            convert_ns_to_secs(total_host_time.code_hash)
        );
        println!(
            "sload                                     {:?}",
            convert_ns_to_secs(total_host_time.sload)
        );
        println!(
            "sstore                                    {:?}",
            convert_ns_to_secs(total_host_time.sstore)
        );
        println!(
            "log                                       {:?}",
            convert_ns_to_secs(total_host_time.log)
        );
        println!(
            "selfdestruct                              {:?}",
            convert_ns_to_secs(total_host_time.selfdestruct)
        );
        println!(
            "create                                    {:?}",
            convert_ns_to_secs(total_host_time.create)
        );
        println!(
            "call                                      {:?}",
            convert_ns_to_secs(total_host_time.call)
        );
        println!(
            "total host function time(secs) ==> {:?}",
            convert_ns_to_secs(total_host_fn_time)
        );
        println!("");

        // println!("The trend of time spent on the host function");
        // println!("step function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v.step);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("step_end function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v.step_end);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("env function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v.env);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("load_account function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.load_account));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("block_hash function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.block_hash));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("balance function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.balance));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("code function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.code));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("code_hash function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.code_hash));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("sload function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.sload));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("sstore function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.sstore));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("log function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.log));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("selfdestruct function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.selfdestruct));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("create function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.create));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("call function: ");
        // let _ = self
        //     .all_host_time
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.call));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        println!("");
        println!("===============================Metric of CacheDb=====================");
        println!("===============================Hit in CacheDb=====================");
        let mut total_cache_hits: (u64, u64, u64, u64) = (0, 0, 0, 0);
        let _ = self
            .all_cache_hits
            .iter()
            .map(|v| {
                total_cache_hits.0 += v.0;
                total_cache_hits.1 += v.1;
                total_cache_hits.2 += v.2;
                total_cache_hits.3 += v.3;
            })
            .collect::<Vec<_>>();

        println!("CacheDb functions                         Hit times");
        println!(
            "hit_in_basic                              {:?}",
            total_cache_hits.1
        );
        println!(
            "hit_in_code_by_hash                       {:?}",
            total_cache_hits.3
        );
        println!(
            "hit_in_storage                            {:?}",
            total_cache_hits.2
        );
        println!(
            "hit_in_block_hash                         {:?}",
            total_cache_hits.0
        );
        println!(
            "total_hit_times  ===>  {:?}",
            total_cache_hits.0 + total_cache_hits.1 + total_cache_hits.2 + total_cache_hits.3
        );

        // println!("The trend of hit times");
        // println!("hit in basic function: ");
        // let _ = self
        //     .all_cache_hits
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v.1);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("hit in code_by_hash function: ");
        // let _ = self
        //     .all_cache_hits
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v.3);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("hit in storage function: ");
        // let _ = self
        //     .all_cache_hits
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v.2);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("hit in block_hash function: ");
        // let _ = self
        //     .all_cache_hits
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v.0);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("total hit: ");
        // let _ = self
        //     .all_cache_hits
        //     .iter()
        //     .map(|v| {
        //         let t = v.0 + v.1 + v.2 + v.3;
        //         println!("{:?}", t);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        println!("");
        println!("===============================Miss in CacheDb=====================");
        let mut total_cache_misses: (u64, u64, u64, u64) = (0, 0, 0, 0);
        let _ = self
            .all_cache_misses
            .iter()
            .map(|v| {
                total_cache_misses.0 += v.0;
                total_cache_misses.1 += v.1;
                total_cache_misses.2 += v.2;
                total_cache_misses.3 += v.3;
            })
            .collect::<Vec<_>>();

        println!("CacheDb functions                          Miss times");
        println!(
            "miss_in_basic                              {:?}",
            total_cache_misses.1
        );
        println!(
            "miss_in_code_by_hash                       {:?}",
            total_cache_misses.3
        );
        println!(
            "miss_in_storage                            {:?}",
            total_cache_misses.2
        );
        println!(
            "miss_in_block_hash                         {:?}",
            total_cache_misses.0
        );
        println!(
            "total_miss_times  ===>  {:?}",
            total_cache_misses.0
                + total_cache_misses.1
                + total_cache_misses.2
                + total_cache_misses.3
        );

        // println!("The trend of miss times");
        // println!("miss in basic function: ");
        // let _ = self
        //     .all_cache_misses
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v.1);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("miss in code_by_hash function: ");
        // let _ = self
        //     .all_cache_misses
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v.3);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("miss in storage function: ");
        // let _ = self
        //     .all_cache_misses
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v.2);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("miss in block_hash function: ");
        // let _ = self
        //     .all_cache_misses
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v.0);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("total miss: ");
        // let _ = self
        //     .all_cache_misses
        //     .iter()
        //     .map(|v| {
        //         let t = v.0 + v.1 + v.2 + v.3;
        //         println!("{:?}", t);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        println!("");
        println!("===============================Misses penalty in CacheDb=====================");
        let mut total_cache_misses_penalty: (u128, u128, u128, u128) = (0, 0, 0, 0);
        let _ = self
            .all_cache_misses_penalty
            .iter()
            .map(|v| {
                total_cache_misses_penalty.0 += v.0;
                total_cache_misses_penalty.1 += v.1;
                total_cache_misses_penalty.2 += v.2;
                total_cache_misses_penalty.3 += v.3;
            })
            .collect::<Vec<_>>();

        println!("CacheDb functions                          Miss penalty_time");
        println!(
            "miss_penalty_in_basic(secs)                              {:?}",
            convert_ns_to_secs(total_cache_misses_penalty.1)
        );
        println!(
            "miss_penalty_in_code_by_hash(secs)                       {:?}",
            convert_ns_to_secs(total_cache_misses_penalty.3)
        );
        println!(
            "miss_penalty_in_storage(secs)                            {:?}",
            convert_ns_to_secs(total_cache_misses_penalty.2)
        );
        println!(
            "miss_penalty_in_block_hash(secs)                         {:?}",
            convert_ns_to_secs(total_cache_misses_penalty.0)
        );
        println!(
            "total_misses_penalty_times(secs)  ===>  {:?}",
            convert_ns_to_secs(
                total_cache_misses_penalty.0
                    + total_cache_misses_penalty.1
                    + total_cache_misses_penalty.2
                    + total_cache_misses_penalty.3
            )
        );

        // println!("The trend of misses penalty time");
        // println!("misses penalty time in basic function: ");
        // let _ = self
        //     .all_cache_misses_penalty
        //     .iter()
        //     .map(|v| println!("{:?}", convert_ns_to_secs(v.1)))
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("misses penalty time in code_by_hash function: ");
        // let _ = self
        //     .all_cache_misses_penalty
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.3));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("misses penalty time in storage function: ");
        // let _ = self
        //     .all_cache_misses_penalty
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.2));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("misses penalty time in block_hash function: ");
        // let _ = self
        //     .all_cache_misses_penalty
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", convert_ns_to_secs(v.0));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("total misses penalty:  ");
        // let _ = self
        //     .all_cache_misses_penalty
        //     .iter()
        //     .map(|v| {
        //         let t = v.0 + v.1 + v.2 + v.3;
        //         println!("{:?}", convert_ns_to_secs(t));
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("");
        // println!("===============================CacheDb size=====================");
        // println!("CacheDb size in all times ");
        // let _ = self
        //     .all_cachedb_size
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");

        // println!("");
        // println!("===============================All BlockNumbers=====================");
        // println!("BlockNumber in all times ");
        // let _ = self
        //     .block_numbers
        //     .iter()
        //     .map(|v| {
        //         println!("{:?}", v);
        //     })
        //     .collect::<Vec<_>>();
        // println!("");
    }
}

fn main() {
    let mut total_record = TotalRecord::default();

    // let file_path = "/home/andy/Source/work/experiment/20231004/data/revm_log_with_max_th1-1";
    // let file_path = "/home/andy/Source/work/experiment/20231004/data/revm_log_with_th1-1";
    let file_path = "/home/andy/Source/work/experiment/20231007/revm_log_with_th2-1";
    // let file_path = "./tt1";
    // let file_path = "./tt3";
    // let file_path = "/home/andy/Source/work/reth/log1";
    // let file_path = "/home/andy/Source/work/reth/min_log";
    // let file_path = "./5m_log";
    // let file_path = "./33log";

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let block_number_regex = Regex::new(r"break =+,\s*block_number:\s*(\d+)").unwrap();
    let revm_record_regex = Regex::new(r"revm_record = RevmMetricRecord \{(.*)\}").unwrap();
    let cachedb_size_regex = Regex::new(r"cachedb_size = (\d+)").unwrap();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(captures) = block_number_regex.captures(&line) {
                let block_number = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                total_record.update_block_number(block_number);
            } else if let Some(captures) = revm_record_regex.captures(&line) {
                let record_str = captures.get(1).unwrap().as_str();
                let record = parse_revm_metric_record(record_str);
                total_record.update_revm_record(&record);
            } else if let Some(captures) = cachedb_size_regex.captures(&line) {
                let cachedb_size = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                total_record.update_cachedb_size(cachedb_size);
            }
        }
    }

    total_record.update_block_number(10000000);

    total_record.print();
}

fn parse_revm_metric_record(record_str: &str) -> RevmMetricRecord {
    let mut opcode_time: Option<HashMap<u8, (u64, u64)>> = None;
    let mut host_time = HostTime {
        step: 0,
        step_end: 0,
        env: 0,
        load_account: 0,
        block_hash: 0,
        balance: 0,
        code: 0,
        code_hash: 0,
        sload: 0,
        sstore: 0,
        log: 0,
        selfdestruct: 0,
        create: 0,
        call: 0,
    };
    let mut cache_hits = (0, 0, 0, 0);
    let mut cache_misses = (0, 0, 0, 0);
    let mut cache_misses_penalty = (0, 0, 0, 0);

    let field_regex =
        Regex::new(r"(\w+):\s*((?:\{(?:[^{}]|(?R))*\}|(?:\([^()]*\))|[^,])+)(?:,|$)").unwrap();

    for captures in field_regex.captures_iter(record_str) {
        let field_name = captures.get(1).unwrap().as_str();
        let field_value = captures.get(2).unwrap().as_str();
        // println!(
        //     "field_name: {:?}, field_value: {:?}",
        //     field_name, field_value
        // );

        match field_name {
            "opcode_time" => {
                opcode_time = parse_opcode_time(field_value);
            }
            "host_time" => {
                host_time = parse_host_time(field_value).unwrap_or_default();
            }
            "cache_hits" => {
                cache_hits = parse_cache(field_value).unwrap_or_default();
            }
            "cache_misses" => {
                cache_misses = parse_cache(field_value).unwrap_or_default();
            }
            "cache_misses_penalty" => {
                cache_misses_penalty = parse_cache_misses_penalty(field_value).unwrap_or_default();
            }
            _ => {
                // opcode_time = parse_opcode_time(field_value);
            }
        }
    }

    RevmMetricRecord {
        opcode_time,
        host_time,
        cache_hits,
        cache_misses,
        cache_misses_penalty,
    }
}

fn parse_opcode_time(opcode_time_str: &str) -> Option<HashMap<u8, (u64, u64)>> {
    // println!("opcode_time_str: {:?}", opcode_time_str);
    if opcode_time_str == "None" {
        // println!("None");
        return None;
    }

    let mut opcode_time = HashMap::new();
    let entry_regex = Regex::new(r"(\d+): \((\d+), (\d+)\)").unwrap();
    for captures in entry_regex.captures_iter(opcode_time_str) {
        let opcode: u8 = captures.get(1).unwrap().as_str().parse().unwrap();
        let counter: u64 = captures.get(2).unwrap().as_str().parse().unwrap();
        let execution_time: u64 = captures.get(3).unwrap().as_str().parse().unwrap();

        opcode_time.insert(opcode, (counter, execution_time));
    }

    Some(opcode_time)
}

fn parse_host_time(host_time_str: &str) -> Option<HostTime> {
    if host_time_str == "None" {
        return None;
    }

    let mut host_time = HostTime {
        step: 0,
        step_end: 0,
        env: 0,
        load_account: 0,
        block_hash: 0,
        balance: 0,
        code: 0,
        code_hash: 0,
        sload: 0,
        sstore: 0,
        log: 0,
        selfdestruct: 0,
        create: 0,
        call: 0,
    };

    let entry_regex = Regex::new(r"step: (\d+), step_end: (\d+), env: (\d+), load_account: (\d+), block_hash: (\d+), balance: (\d+), code: (\d+), code_hash: (\d+), sload: (\d+), sstore: (\d+), log: (\d+), selfdestruct: (\d+), create: (\d+), call: (\d+)").unwrap();

    if let Some(captures) = entry_regex.captures(host_time_str) {
        host_time.step = captures.get(1).unwrap().as_str().parse().unwrap();
        host_time.step_end = captures.get(2).unwrap().as_str().parse().unwrap();
        host_time.env = captures.get(3).unwrap().as_str().parse().unwrap();
        host_time.load_account = captures.get(4).unwrap().as_str().parse().unwrap();
        host_time.block_hash = captures.get(5).unwrap().as_str().parse().unwrap();
        host_time.balance = captures.get(6).unwrap().as_str().parse().unwrap();
        host_time.code = captures.get(7).unwrap().as_str().parse().unwrap();
        host_time.code_hash = captures.get(8).unwrap().as_str().parse().unwrap();
        host_time.sload = captures.get(9).unwrap().as_str().parse().unwrap();
        host_time.sstore = captures.get(10).unwrap().as_str().parse().unwrap();
        host_time.log = captures.get(11).unwrap().as_str().parse().unwrap();
        host_time.selfdestruct = captures.get(12).unwrap().as_str().parse().unwrap();
        host_time.create = captures.get(13).unwrap().as_str().parse().unwrap();
        host_time.call = captures.get(14).unwrap().as_str().parse().unwrap();
    }

    Some(host_time)
}

fn parse_cache(cache_str: &str) -> Option<(u64, u64, u64, u64)> {
    if cache_str == "None" {
        return None;
    }

    let entry_regex = Regex::new(r"(\d+), (\d+), (\d+), (\d+)").unwrap();

    if let Some(captures) = entry_regex.captures(cache_str) {
        let host: u64 = captures.get(1).unwrap().as_str().parse().unwrap();
        let storage: u64 = captures.get(2).unwrap().as_str().parse().unwrap();
        let account: u64 = captures.get(3).unwrap().as_str().parse().unwrap();
        let contract_code: u64 = captures.get(4).unwrap().as_str().parse().unwrap();

        return Some((host, storage, account, contract_code));
    }

    None
}

fn parse_cache_misses_penalty(cache_misses_penalty_str: &str) -> Option<(u128, u128, u128, u128)> {
    if cache_misses_penalty_str == "None" {
        return None;
    }

    let entry_regex = Regex::new(r"(\d+), (\d+), (\d+), (\d+)").unwrap();

    if let Some(captures) = entry_regex.captures(cache_misses_penalty_str) {
        let penalty1: u128 = captures.get(1).unwrap().as_str().parse().unwrap();
        let penalty2: u128 = captures.get(2).unwrap().as_str().parse().unwrap();
        let penalty3: u128 = captures.get(3).unwrap().as_str().parse().unwrap();
        let penalty4: u128 = captures.get(4).unwrap().as_str().parse().unwrap();

        return Some((penalty1, penalty2, penalty3, penalty4));
    }

    None
}
