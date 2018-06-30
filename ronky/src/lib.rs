//#![deny(missing_docs,
//        missing_debug_implementations, missing_copy_implementations,
//        trivial_casts, trivial_numeric_casts,
//        unsafe_code,
//        unstable_features,
//        unused_import_braces, unused_qualifications)]
//
use std::path::PathBuf;
use std::path::Path;


#[derive(Debug, PartialEq)]

/// The struct that contains which items are to be displayed
/// - true for display of the given item
/// - false if the given is not to be displayed
/// - The default path for the configuration file is `/etc/ronky/ronky.conf`
//pub struct Config {
//  pub battery: bool,
//  pub CPU: bool, 
//  pub CPU_temperature: bool,
//  pub GPU: bool,
//  pub GPU_temperature: bool,
//  pub ram: bool,
//  pub ram_temperature: bool,
//  pub date: bool,
//  pub time: bool,
//  pub volume: bool,
//  pub hard_drive_usage: bool,
//  pub config_file: bool,
//  pub config_file_path: Option<PathBuf>,
//}

/// Creates a new Config
/// Takes a vector of &String - the arguments passed to ronky
/// 
/// Outputs a Result - Ok(Config) or Err 
///
/// The output Config has all values set to false by default(except for config_file_path, 
/// set to /etc/ronky/ronky.conf)
/// 
/// # Examples
///
/// ```
/// 
/// use std::path::PathBuf;
/// let test_output = ronky::Config { battery: true, CPU: false, CPU_temperature: false, GPU: false, 
///     GPU_temperature: true, ram: false, 
///     ram_temperature: false, date: true, time: false, volume: false, hard_drive_usage: false, 
///     config_file: false, config_file_path: Some(PathBuf::from("/etc/ronky/ronky.conf")) };
///
/// let test_config: &Vec<String> = &vec!["some_stuff_whatever".to_string(), 
/// "--battery".to_string(), "--GPU_temperature".to_string(), "--date".to_string() ];
///
/// assert_eq!( ronky::Config::new(&test_config), Ok(test_output) ) ;
///
/// ```
///    
/// # Errors
///
/// new() will return an Err if: 
///  - The `--config` flag is given, but no file is passed as an argument
///  - The config file doesn't exist or permissions render it inaccessible
///  - An invalid flag is passed


//impl Config {
//         
//  pub fn new(args: &[String]) -> Result<Config, &'static str> {
//    let mut battery = false;
//    let mut CPU = false;
//    let mut CPU_temperature = false;
//    let mut GPU = false;
//    let mut GPU_temperature = false;
//    let mut ram = false;
//    let mut ram_temperature = false;
//    let mut date = false;
//    let mut time = false;
//    let mut volume = false;
//    let mut hard_drive_usage = false; 
//    let mut config_file = false;
//    let mut config_file_path: Option<PathBuf> = Some(PathBuf::from("/etc/ronky/ronky.conf"));
//    
//    // this is a counting bit to account for invalid flags
//    // if a value is not a valid flag and next_is_config is
//    // set to false, then new() returns an Err
//    let mut next_is_config: bool = false;
//  
//    for x in 1..args.len() {
//        println!("arg is: {}", args[x]);
//      match args[x].as_str() {
//        "--battery" => battery = true,
//        "--CPU" => CPU = true,
//        "--CPU_temperature" => CPU_temperature = true,
//        "--GPU" => GPU = true,
//        "--GPU_temperature" => GPU_temperature = true,
//        "--ram" => ram = true,
//        "--ram_temperature" => ram_temperature = true,
//        "--date" => date = true,
//        "--time" => time = true,
//        "--volume" => volume = true,
//        "--hard_drive_usage" => hard_drive_usage = true,
//        "--config" => {
//              if x == ( args.len() - 1 ) {
//                  return Err("Must specify config file path if using --config flag")
//              } else if Path::new(&args[x + 1]).is_file() {
//                config_file_path = Some(PathBuf::from(&args[x + 1].clone()));
//                config_file = true;
//                next_is_config = true;
//              } else {
//                  return Err("Invalid config path, check validity and/or permissions")
//              }
//          },
//        _ => {
//          if next_is_config { 
//            next_is_config = false;
//          } else {
//            return Err("Invalid flag")
//          }
//        },
//      }
//    }
//    Ok (Config { battery, CPU, CPU_temperature, GPU, GPU_temperature, ram, ram_temperature, date, time, volume, hard_drive_usage, config_file, config_file_path } )
//  }
//}
//

/// Determines which value is to be displayed

enum DisplayValue {

/// The current percentage of battery remaining
/// In percentage by default
  Battery,
  CPU,
  CPUTemperature,
  GPU,
  GPUTemperature,
  Ram,
  RamTemperature,
  Date,
  Time,
  AudioVolume,
  HardDriveUsage,
}
