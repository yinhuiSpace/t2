#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use battery::{Battery, Manager};
use serde::{Deserialize, Serialize};
use sysinfo::{Cpu, Disk, Disks, NetworkData, Networks, Process, System};

//系统----------------------------->
#[derive(Debug, Serialize, Deserialize)]
pub struct SysData {
    system_name: String,
    host_name: String,
    kernel_version: String,
    os_version: String,
}

impl SysData {
    pub fn new() -> Self {
        Self {
            system_name: System::name().unwrap_or(String::from("未知")),
            host_name: System::host_name().unwrap_or(String::from("未知")),
            kernel_version: System::kernel_version().unwrap_or(String::from("未知")),
            os_version: System::os_version().unwrap_or(String::from("未知")),
        }
    }
}

#[tauri::command]
fn sys_info() -> SysData {
    let mut sys = System::new_all();
    sys.refresh_all();
    SysData::new()
}

//进程----------------------------->
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessData {
    pid: String,
    name: String,
    mem: u64,
}

impl ProcessData {
    pub fn new(p: &Process) -> Self {
        Self {
            pid: p.pid().to_string(),
            name: String::from(p.name().to_str().unwrap()),
            mem: p.memory(),
        }
    }
}

#[tauri::command]
fn process_info() -> Vec<ProcessData> {
    let mut pds: Vec<ProcessData> = Vec::new();

    let mut sys = System::new_all();
    sys.refresh_all();

    let processes = sys.processes();

    for (pid, p) in processes {
        pds.push(ProcessData::new(p));
    }

    pds.sort_by(|a, b| a.mem.cmp(&b.mem));

    pds
}

//内存----------------------------->
#[derive(Serialize, Deserialize)]
pub struct MemData {
    total_memory: u64,
    total_swap: u64,
    used_memory: u64,
    used_swap: u64,
}

impl MemData {
    fn new(sys: &System) -> Self {
        Self {
            total_memory: sys.total_memory(),
            total_swap: sys.total_swap(),
            used_memory: sys.used_memory(),
            used_swap: sys.used_swap(),
        }
    }

    fn b_to_gb(b: u64) -> f64 {
        b as f64 / (1024. * 1024. * 1024.)
    }
}

#[tauri::command]
fn mem_info() -> MemData {
    let mut sys = System::new_all();
    sys.refresh_all();

    MemData::new(&sys)
}

//cpu----------------------------->
#[derive(Debug, Serialize, Deserialize)]
pub struct CPUData {
    chip_name: String,
    core_cnt: u32,
    core_list: Vec<CPUCoreData>,
    global_usage: f32,
}

impl CPUData {
    fn new(sys: &System, brand: String, cores: Vec<CPUCoreData>) -> Self {
        Self {
            chip_name: brand,
            core_cnt: sys.cpus().len() as u32,
            core_list: cores,
            global_usage: sys.global_cpu_usage(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CPUCoreData {
    id: u32,
    name: String,
    usage: f32,
    frequency: u64,
    brand: String,
}

impl CPUCoreData {
    pub fn new(id: u32, c: &Cpu) -> Self {
        Self {
            id: id,
            name: String::from(c.name()),
            usage: c.cpu_usage(),
            frequency: c.frequency(),
            brand: String::from(c.brand()),
        }
    }
}

#[tauri::command]
fn cpu_info() -> CPUData {
    let mut sys = System::new_all();
    let mut cores = Vec::new();
    sys.refresh_all();
    let mut cpus = sys.cpus();
    for (i, c) in cpus.iter().enumerate() {
        cores.push(CPUCoreData::new(i as u32, c));
    }
    CPUData::new(&sys, String::from(cpus[0].brand()), cores)
}

//电池----------------------------->
#[derive(Debug, Serialize, Deserialize)]
pub struct BatteryData {
    // 电池温度
    temperature: String,
    // 循环周期
    cycle_count: u32,
    // 充电状态
    state: i32,
    // 电量百分比
    percentage: f32,
    // // 还需多久充满
    // time_to_full: u32,
    // // 电池剩余使用时间
    // time_to_empty: u32,
    // 电池健康
    state_of_health: String,
}

impl BatteryData {
    pub fn new(battery: &Battery) -> Self {
        let t: String;
        if let Some(val) = battery.temperature() {
            t = format!("{:.2}℃", val.value - 273.15);
            println!("{}", val.value);
        } else {
            t = format!("未知");
        }
        Self {
            temperature: t,
            cycle_count: battery.cycle_count().unwrap_or(0),
            state: match battery.state() {
                battery::State::Full => 1,
                battery::State::Charging => 2,
                battery::State::Discharging => 3,
                battery::State::Empty => 0,
                _ => -1,
            },
            percentage: battery.state_of_charge().value * 100.,
            state_of_health: format!("{:.2}%", battery.state_of_health().value * 100.),
        }
    }
}

#[tauri::command]
fn battery_info() -> Vec<BatteryData> {
    let mut manager = Manager::new().unwrap();
    let batteries = manager.batteries().unwrap();
    let mut batteryDataList = Vec::new();
    for (_, b) in batteries.enumerate() {
        batteryDataList.push(BatteryData::new(&b.unwrap()));
    }
    batteryDataList
}

//网络----------------------------->
#[derive(Debug, Serialize, Deserialize)]
pub struct NetData {
    name: String,
    rec: u64,
    tran: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetInfo {
    ns: Vec<String>,
    ds: Vec<u64>,
    us: Vec<u64>,
}

impl NetData {
    pub fn new(name: &String, nd: &NetworkData) -> Self {
        Self {
            name: String::from(name),
            rec: nd.total_received(),
            tran: nd.total_transmitted(),
        }
    }
}

impl NetInfo {
    pub fn new(ns: Vec<String>, ds: Vec<u64>, us: Vec<u64>) -> Self {
        Self {
            ns: ns,
            ds: ds,
            us: us,
        }
    }
}

#[tauri::command]
fn net_info() -> NetInfo {
    let nds = net_data();
    let mut ns: Vec<String> = Vec::new();
    let mut ds: Vec<u64> = Vec::new();
    let mut us: Vec<u64> = Vec::new();
    for nd in nds {
        ns.push(nd.name);
        ds.push(nd.rec);
        us.push(nd.tran);
    }
    NetInfo::new(ns, ds, us)
}

#[tauri::command]
fn net_data() -> Vec<NetData> {
    let mut nets = Vec::new();
    let mut networks = Networks::new_with_refreshed_list();
    for (n, d) in &networks {
        nets.push(NetData::new(n, d));
    }
    nets
}

//硬盘----------------------------->
#[derive(Debug, Serialize, Deserialize)]
struct DiskData {
    name: String,
    fs: String,
    ty: String,
    is_rmv: bool,
    mnt: String,
    used: u64,
    total: u64,
}

impl DiskData {
    pub fn new(d: &Disk) -> Self {
        Self {
            name: String::from(d.name().to_str().unwrap_or("未知")),
            fs: String::from(d.file_system().to_str().unwrap_or("未知")),
            ty: match d.kind() {
                sysinfo::DiskKind::HDD => String::from("HDD"),
                sysinfo::DiskKind::SSD => String::from("SSD"),
                _ => String::from("未知"),
            },
            is_rmv: d.is_removable(),
            mnt: String::from(d.mount_point().to_str().unwrap_or("未知")),
            used: d.total_space() - d.available_space(),
            total: d.total_space(),
        }
    }
}

#[tauri::command]
fn disk_info() -> Vec<DiskData> {
    let mut ds = Vec::new();
    let disks = Disks::new_with_refreshed_list();
    for d in &disks {
        ds.push(DiskData::new(d));
    }
    ds
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            sys_info,
            cpu_info,
            battery_info,
            process_info,
            net_info,
            disk_info,
            mem_info,
            net_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
