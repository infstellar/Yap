use pyo3::prelude::*;

use std::path::Path;
use std::env;
use std::f32;
use std::fs;
use std::sync::{Arc, Mutex, RwLock};
use std::ptr::null_mut;

pub mod capture;
pub mod dto;
pub mod common;
pub mod info;
pub mod pickupper;
pub mod inference;
// use capture;
use common::sleep;
use pickupper::pickup::{Pickupper, PickupCofig};

use rand::{self, Rng};

use winapi::um::winuser::{SetForegroundWindow, ShowWindow, SW_SHOW, SW_RESTORE};
use winapi::um::shellscalingapi::SetProcessDpiAwareness;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{CreateSolidBrush, RGB, SetTextColor};
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, ReleaseDC,
    SetLayeredWindowAttributes, UpdateWindow, WS_EX_LAYERED, WS_EX_TOPMOST,
    WS_POPUP, WM_PAINT, WM_CLOSE, WM_DESTROY, WM_ERASEBKGND, WM_NCHITTEST, WM_SIZE, WNDCLASSW,
    PAINTSTRUCT, RegisterClassW,  MSG, GetMessageW, TranslateMessage, DispatchMessageW, BeginPaint, 
    DT_SINGLELINE, DrawTextW, EndPaint, PostQuitMessage, HTCAPTION, InvalidateRect, 
    LWA_COLORKEY, DT_CALCRECT, FillRect, GetWindowLongW, GWL_EXSTYLE, WS_EX_TOOLWINDOW, SetWindowLongW, 
};
use winapi::shared::windef::{RECT, HWND, HBRUSH, POINT};
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};


use clap::{Arg, App};

use env_logger::{Env, Builder, Target};
use log::{info, warn};
use log4rs;

fn init() -> (){
    println!("Rust start: {}", env::var("RUST_PATH").unwrap_or(".".to_string()));
    
}

// 定义一个 Rust 结构体
#[pyclass]
struct PickupC {
    // pickupper: Pickupper,
    do_pk_signal: Arc<Mutex<bool>>,
    // _pickup_handle: std::thread::JoinHandle<()>
    // 其他成员变量...
}

// 实现 Pickup 的构造函数
#[pymethods]
impl PickupC {
    #[new]
    fn newf() -> Self {
        
        let version = common::get_version();
        // println!("rust: 01");
        // let matches = App::new("YAP - 原神自动拾取器")
        //     .author("Alex-Beng <pc98@qq.com>")
        //     .about("Genshin Impact Pickup Helper")
        //     .version(version.as_str())
        //     .arg(Arg::with_name("dump")
        //         .long("dump")
        //         .required(false)
        //         .takes_value(true)
        //         .help("输出模型预测结果、原始图像至指定的文件夹，debug专用"))
        //     .arg(Arg::with_name("dump_idx")
        //         .long("dump-idx")
        //         .short("i")
        //         .required(false)
        //         .takes_value(true)
        //         .default_value("0")
        //         .help("执行dump时，输出结果起始的index"))
        //     .arg(Arg::with_name("infer_gap")
        //         .long("infer-gap")
        //         .short("g")
        //         .required(false)
        //         .takes_value(true)
        //         .default_value("0")
        //         .help("一次检测推理拾取的间隔，单位ms"))
        //     .arg(Arg::with_name("channal")
        //         .long("channal")
        //         .short("c")
        //         .required(false)
        //         .takes_value(true)
        //         .default_value("gray")
        //         .help("灰度化时使用的通道，默认使用gray通道，另一个可选值为L*，L*通道可能无法工作"))
        //     .arg(Arg::with_name("pick_key")
        //         .long("pick-key")
        //         .required(false)
        //         .takes_value(true)
        //         .default_value("f")
        //         .help("拾取案件，正常人默认为F"))
        //     .arg(Arg::with_name("cosine-threshold")
        //         .long("cosine-threshold")
        //         .short("t")
        //         .required(false)
        //         .takes_value(true)
        //         .default_value("0.997")
        //         .help("拾取键轮廓匹配的余弦阈值，越大越严格，取值(0, 1)"))
        //     .arg(Arg::with_name("log")
        //         .long("log")
        //         .required(false)
        //         .takes_value(true)
        //         .default_value("warn")
        //         .help("日志等级，可选值为trace, debug, info, warn, error"))
        //     .arg(Arg::with_name("no_pickup")
        //         .long("no-pickup")
        //         .required(false)
        //         .takes_value(false)
        //         .help("不执行拾取，仅info拾取动作，debug专用"))
        //     .arg(Arg::with_name("hotkey")
        //         .required(false)
        //         .takes_value(false)
        //         .help("是否注册hotkey用于调整拾取时序，debug专用"))
        //     .arg(Arg::with_name("click_tp")
        //         .required(false)
        //         .takes_value(false)
        //         .help("是否自动点击传送"))
        //     .get_matches();
        // println!("rust: 01 1");
        //设置软件工作目录，避免下面未使用 PathBuf 的路径使用错误
        let mut current_exe_dir = std::path::PathBuf::from(env::var("RUST_PATH").unwrap());
        let _ = env::set_current_dir(&current_exe_dir);
        // println!("rust: 02");
        let dump: bool = false;
        let dump_path = "./dumps/";
        let cnt:u32 = "0".parse::<u32>().unwrap();
        let infer_gap: u32 = "0".parse::<u32>().unwrap();
        let mut channal = "gray";
        let no_pickup = false;
        let reg_hotkey = false;
        let click_tp = false;
        let pick_key = "f".parse::<char>().unwrap();
        let mut cosine_threshold: f32 = "0.997".parse::<f32>().unwrap();
        let mut single_mode = false;
        // println!("rust: 03");
        // 移动管理员权限检查、版本更新至logger初始化之后
        // if !common::is_admin() {
        //     common::error_and_quit_no_input("请以管理员身份运行该程序");
        // }
        // println!("rust: 04");
        // 设置 DPI 感知级别
        // 用于适配高DPI屏幕
        let dpi_awareness = unsafe { SetProcessDpiAwareness(2) };
        if dpi_awareness != 0 {
            warn!("SetProcessDpiAwareness failed，高DPI可能导致程序无法正常运行");
        }
        // println!("rust: 05");
        if let Some(v) = common::check_update() {
            warn!("检测到新版本，请手动更新：{}", v);
        }
        // println!("rust: 06");
        // 检查cosine threshold是否合法
        if cosine_threshold <= 0.0 || cosine_threshold >= 1.0 {
            warn!("cosine threshold 取值范围为(0, 1)，使用默认值0.997");
            cosine_threshold = 0.997;
        }
        
        // 尝试从可能存在的 config.json 读取可能存在的拾取参数
        let mut infer_gap_default = infer_gap;
        let mut f_internal_default = 50;
        let mut f_gap_default = 85;
        let mut scroll_gap_default = 70;
        let mut click_tp_default = click_tp;
        let mut pick_key_json = pick_key;
        let mut uid_mask_on = true;
        let mut press_y = true;
        let config_path = current_exe_dir.join("config.json").clone();
        if config_path.exists() {
            let config = fs::read_to_string(config_path).unwrap();
            let config: serde_json::Value = serde_json::from_str(&config).unwrap();
            // 拾取参数是可能存在，需要逐一检查
            if let Some(infer_gap) = config.get("infer_gap") {
                if infer_gap.is_u64() {
                    infer_gap_default = infer_gap.as_u64().unwrap() as u32;
                }
            }
            if let Some(f_internal) = config.get("f_internal") {
                if f_internal.is_u64() {
                    f_internal_default = f_internal.as_u64().unwrap() as u32;
                }
            }
            if let Some(f_gap) = config.get("f_gap") {
                if f_gap.is_u64() {
                    f_gap_default = f_gap.as_u64().unwrap() as u32;
                }
            }
            if let Some(scroll_gap) = config.get("scroll_gap") {
                if scroll_gap.is_u64() {
                    scroll_gap_default = scroll_gap.as_u64().unwrap() as u32;
                }
            }
            if let Some(click_tp) = config.get("click_tp") {
                if click_tp.is_boolean() {
                    click_tp_default = click_tp.as_bool().unwrap();
                }
            }
            if let Some(pick_key) = config.get("pick_key") {
                if pick_key.is_string() {
                    pick_key_json = pick_key.as_str().unwrap().parse::<char>().unwrap();
                }
            }
            // for cosine thre
            if let Some(cos_thre) = config.get("cos_thre") {
                if cos_thre.is_f64() {
                    cosine_threshold = cos_thre.as_f64().unwrap() as f32;
                }
            }
            // for single mode
            if let Some(sing_md) = config.get("single_mode") {
                if sing_md.is_boolean() {
                    single_mode = sing_md.as_bool().unwrap();
                }
            }
            // for uid mask
            if let Some(uid_mask) = config.get("uid_mask") {
                if uid_mask.is_boolean() {
                    uid_mask_on = uid_mask.as_bool().unwrap();
                }
            }
            // for press y
            if let Some(press_y_) = config.get("press_y") {
                if press_y_.is_boolean() {
                    press_y = press_y_.as_bool().unwrap();
                }
            }
        }
        // println!("rust: 07");
        // 检查dump_path是否存在，不存在则创建
        if dump && !Path::new(dump_path).exists() {
            fs::create_dir_all(dump_path).unwrap();
        }
        
        
        if channal != "L*" && channal != "gray" {
            // common::error_and_quit("channal参数必须为L*或gray");
            warn!("channal参数必须为L*或gray，使用默认值gray");
            channal = "gray";
        }

        let mut use_l = false;
        if channal == "L*" {
            use_l = true;
            info!("使用L*通道进行灰度化");
        }
        // println!("rust: 08");
        let do_pickup_signal = Arc::new(Mutex::new(!no_pickup));
        let infer_gap_signal = Arc::new(RwLock::new(infer_gap_default));
        let f_inter_signal = Arc::new(RwLock::new(f_internal_default));
        let f_gap_signal = Arc::new(RwLock::new(f_gap_default));
        let scroll_gap_signal = Arc::new(RwLock::new(scroll_gap_default));
        let click_tp_signal = Arc::new(Mutex::new(click_tp_default));
        let single_md_signal = Arc::new(Mutex::new(single_mode));

        // let mut is_cloud = false;
        loop{
            match capture::find_window_local() {
                Err(_) => {
                    warn!("未找到原神窗口");
                    common::sleep(5000);
                },
                Ok(h) => {
                    break
                },
            };
        }

        let hwnd = match capture::find_window_local() {
            Err(_) => {
                common::error_and_quit_no_input("未找到原神窗口，请确认原神已经开启");
            },
            Ok(h) => h,
        };

        
        common::sleep(1000);

        // get windows size
        let hwnd = common::WindowHandle::new(hwnd).unwrap();
        let rect = capture::get_client_rect(hwnd).unwrap();
        info!("left = {}, top = {}, width = {}, height = {}", rect.left, rect.top, rect.width, rect.height);
        
        capture::capture_absolute_image(hwnd, &rect).unwrap().save("test.png").unwrap();


        let info: info::PickupInfo;
        if rect.height * 16 == rect.width * 9 {
            info = info::PickupInfo::from_16_9(rect.width as u32, rect.height as u32, rect.left, rect.top);
        } 
        else if rect.height * 16 == rect.width * 10 {
            info = info::PickupInfo::from_16_10(rect.width as u32, rect.height as u32, rect.left, rect.top);
        } 
        else {
            common::error_and_quit_no_input("不支持的分辨率");
        }

        // 添加监听的hotkey
        let do_pickup_signal_clone: Arc<Mutex<bool>> = do_pickup_signal.clone();

        let f_inter_signal_clone = f_inter_signal.clone();
        let f_inter_signal_clone_d = f_inter_signal.clone();
        
        let f_gap_signal_clone = f_gap_signal.clone();
        let f_gap_signal_clone_d = f_gap_signal.clone();
        
        let infer_gap_signal_clone = infer_gap_signal.clone();
        let infer_gap_signal_clone_d = infer_gap_signal.clone();

        let scroll_gap_signal_clone = scroll_gap_signal.clone();
        let scroll_gap_signal_clone_d = scroll_gap_signal.clone();

        let click_tp_signal_clone = click_tp_signal.clone();

        let single_md_signal_clone = single_md_signal.clone();

        // 弃用
        // let info_for_artifacts = info.clone();

        let pk_config = PickupCofig {
            info,
            hwnd,
            bw_path: current_exe_dir.to_string_lossy().to_string().clone(),
            use_l,
            press_y,
            dump,
            dump_path: dump_path.to_string(),
            dump_cnt: cnt,
            pick_key: pick_key_json,
            cosin_thr: cosine_threshold,
            do_pickup: do_pickup_signal,
            infer_gap: infer_gap_signal,
            f_inter: f_inter_signal,
            f_gap: f_gap_signal,
            scroll_gap: scroll_gap_signal,
            click_tp: click_tp_signal,
            single_mode: single_md_signal,
        };
        // let uid_pos = pk_config.info.uid_pos.clone();
        // let uid_pos = RECT {
        //     left: uid_pos.left + pk_config.info.left,
        //     top: uid_pos.top + pk_config.info.top,
        //     right: uid_pos.right + pk_config.info.left,
        //     bottom: uid_pos.bottom + pk_config.info.top,
        // };
        // // println!("uid_pos: {}, {}, {}, {}", uid_pos.left, uid_pos.top, uid_pos.right, uid_pos.bottom);

        // 三个子线程+主线程pickupper
        // 由于主线程是while true，所以不需要join

        // 创建UID的遮罩窗口
        
        // let do_pk_signal = do_pickup_signal_clone.clone();
        // 监听快捷键
        // let pickupper = Pickupper::new(pk_config);
        // let _hotkey_handle = std::thread::spawn(move || {
        //     let mut hk = hotkey::Listener::new();
        //     // let do_pk_signal: Arc<Mutex<bool>> = do_pickup_signal_clone;
            
        //     let click_tp_signal_clone: Arc<Mutex<bool>> = click_tp_signal_clone;
        //     if reg_hotkey {

                
        //         let infer_gap_signal= infer_gap_signal_clone;
        //         let infer_gap_signal_d = infer_gap_signal_clone_d;
        //         // ALT + J 
        //         hk.register_hotkey(
        //             hotkey::modifiers::ALT,
        //             'J' as u32, 
        //             move || {
        //                 let mut signal = infer_gap_signal.write().unwrap();
        //                 warn!("ALT + J with {}", *signal);
        //                 // let mut signal = infer_gap_signal.write().unwrap();
        //                 *signal += 1;
        //                 warn!("ALT + J 增加 infer gap 至 {} ms", *signal);
                        
                        
        //             }
        //         ).unwrap();
        //         // ALT + K
        //         hk.register_hotkey(
        //             hotkey::modifiers::ALT,
        //             'K' as u32, 
        //             move || {
        //                 let mut signal = infer_gap_signal_d.write().unwrap();
        //                 warn!("ALT + K with {}", *signal);
        //                 if *signal == 0 {
        //                     return;
        //                 }
        //                 *signal -= 1;
        //                 warn!("ALT + K 减小 infer gap 至 {} ms", *signal);
                        
        //             }
        //         ).unwrap();

        //         let f_inter_signal = f_inter_signal_clone;
        //         let f_inter_signal_d = f_inter_signal_clone_d;
        //         // ALT + U
        //         hk.register_hotkey(
        //             hotkey::modifiers::ALT,
        //             'U' as u32, 
        //             move || {
        //                 let mut signal = f_inter_signal.write().unwrap();
        //                 warn!("ALT + U with {}", *signal);
        //                 *signal += 1;
        //                 warn!("ALT + U 增加 f_inter 至 {} ms", *signal);
                        
        //             }
        //         ).unwrap();
        //         // ALT + I
        //         hk.register_hotkey(
        //             hotkey::modifiers::ALT,
        //             'I' as u32, 
        //             move || {
        //                 let mut signal = f_inter_signal_d.write().unwrap();
        //                 warn!("ALT + I with {}", *signal);
        //                 if *signal == 0 {
        //                     return;
        //                 }
        //                 *signal -= 1;
        //                 warn!("ALT + I 减小 f_inter 至 {} ms", *signal);
                        
        //             }
        //         ).unwrap();

        //         let f_gap_signal = f_gap_signal_clone;
        //         let f_gap_signal_d = f_gap_signal_clone_d;
        //         // ALT + L
        //         hk.register_hotkey(
        //             hotkey::modifiers::ALT,
        //             'L' as u32, 
        //             move || {
        //                 let mut signal = f_gap_signal.write().unwrap();
        //                 warn!("ALT + L with {}", *signal);
        //                 *signal += 1;
        //                 warn!("ALT + L 增加 f_gap 至 {} ms", *signal);
                        
        //             }
        //         ).unwrap();
        //         // ALT + H
        //         hk.register_hotkey(
        //             hotkey::modifiers::ALT,
        //             'H' as u32, 
        //             move || {
        //                 let mut signal = f_gap_signal_d.write().unwrap();
        //                 warn!("ALT + H with {}", *signal);
        //                 if *signal == 0 {
        //                     return;
        //                 }
        //                 *signal -= 1;
        //                 warn!("ALT + H 减小 f_gap 至 {} ms", *signal);
                        
        //             }
        //         ).unwrap();

        //         let scroll_gap_signal = scroll_gap_signal_clone;
        //         let scroll_gap_signal_d = scroll_gap_signal_clone_d;
        //         // ALT + O
        //         hk.register_hotkey(
        //             hotkey::modifiers::ALT,
        //             'O' as u32, 
        //             move || {
        //                 let mut signal = scroll_gap_signal.write().unwrap();
        //                 warn!("ALT + O with {}", *signal);
        //                 *signal += 1;
        //                 warn!("ALT + O 增加 scroll_gap 至 {} ms", *signal);
                        
        //             }
        //         ).unwrap();
        //         // ALT + P
        //         hk.register_hotkey(
        //             hotkey::modifiers::ALT,
        //             'P' as u32, 
        //             move || {
        //                 let mut signal = scroll_gap_signal_d.write().unwrap();
        //                 warn!("ALT + P with {}", *signal);
        //                 if *signal == 0 {
        //                     return;
        //                 }
        //                 *signal -= 1;
        //                 warn!("ALT + P 减小 scroll_gap 至 {} ms", *signal);
                        
        //             }
        //         ).unwrap();
        //     }

            // 保留切换、强化、click tp
            // ALT + 0
            // hk.register_hotkey(
            //     hotkey::modifiers::ALT,
            //     '0' as u32, 
            //     move || {
            //         let mut signal = do_pk_signal.lock().unwrap();
            //         *signal = !*signal;
            //         warn!("ALT + 0 切换 pickup 模式为 {}", *signal);              
            //     }
            // ).unwrap();

            // ALT + 9
            // 切换click tp
        //     hk.register_hotkey(
        //         hotkey::modifiers::ALT,
        //         '9' as u32, 
        //         move || {
        //             let mut signal = click_tp_signal_clone.lock().unwrap();
        //             *signal = !*signal;
        //             warn!("ALT + 9 切换 click tp 模式为 {}", *signal);              
        //         }
        //     ).unwrap();

        //     // ALT + 8
        //     // 切换single mode
        //     hk.register_hotkey(
        //         hotkey::modifiers::ALT,
        //         '8' as u32, 
        //         move || {
        //             let mut signal = single_md_signal_clone.lock().unwrap();
        //             *signal = !*signal;
        //             warn!("ALT + Z 切换 single mode 模式为 {}", *signal);              
        //         }
        //     ).unwrap();

        //     hk.listen();
        // });
        // 监视原神进程是否结束
        // let _monitor_handel: std::thread::JoinHandle<_> = std::thread::spawn(move ||{
        //     loop {
        //         if is_cloud {
        //             let _hwnd = match capture::find_window_cloud() {
        //                 Ok(h) => h,
        //                 Err(_) => {
        //                     common::error_and_quit_no_input("原神进程已结束");
        //                 }
        //             };
        //         } else {
        //             let _hwnd = match capture::find_window_local() {
        //                 Ok(h) => h,
        //                 Err(_) => {
        //                     common::error_and_quit_no_input("原神进程已结束");
        //                 }
        //             };
        //         }
        //         sleep(5000);
        //     }
        // });
        let _pickup_handle = std::thread::spawn(move ||{
            // // println(12);
            Pickupper::new(pk_config).start();
        });
        PickupC {
            // pickupper: Pickupper::new(pk_config),
            do_pk_signal: do_pickup_signal_clone.clone(),
            // _pickup_handle: _pickup_handle
        }
    }

    // 添加一个 start 成员函数，调用 pickupper.use
    // fn runf(&mut self) -> std::thread::JoinHandle<()> {
        
    //     // _pickup_handle;
    //     return _pickup_handle;
        
    // }

    fn startf(&mut self) {
        let mut signal = self.do_pk_signal.lock().unwrap();
        if *signal == false{
            *signal = !*signal;
            warn!("ALT + 0 切换 pickup 模式为 {}", *signal);
        }
                
    }

    fn pausef(&mut self) {
        let mut signal = self.do_pk_signal.lock().unwrap();
        if *signal == true{
            *signal = !*signal;
            warn!("ALT + 0 切换 pickup 模式为 {}", *signal);
        }
    }
}

unsafe impl Send for PickupC {}
unsafe impl Send for PickupCofig {}

#[pymodule]
fn yap(m: &Bound<'_, PyModule>) -> PyResult<()> {
    init();
    // println!("rust: 0");
    log4rs::init_file(env::var("RUST_PATH").unwrap_or(".".to_string())+"\\log4rs.yaml", Default::default()).unwrap();
    // println!("rust: 1");
    m.add_class::<PickupC>()?;
    // println!("rust: 2");
    Ok(())
}

