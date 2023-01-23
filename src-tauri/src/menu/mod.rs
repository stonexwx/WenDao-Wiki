/*!
    按钮操作的一些方法
 */
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use serde_json::map;

mod file;
/**打开操作
文件打开操作分成了两部分，由于tauri的事件传输速度比vue渲染速度快，导致新窗口很有可能接收不到事件信息
故打开文件分为：1. 创建窗口打开文件。2.在原窗口打开文件
1的情况需要先选文件，然后由前端将目标文件路径返回给Rust读取文件内容
2直接打开文件获取内容即可
**/
pub fn open_menu() -> Result<HashMap<String,String>,Box<dyn Error>>{
    file::open_file()
}

pub fn choose_file() -> PathBuf{
    file::get_open_file_path()
}

pub fn get_file_content(path: &str) -> Result<HashMap<String,String>,Box<dyn Error>>{
    file::open_file_for_path(path)
}
//
// pub fn save_menu(text: String) -> Result<String,Box<dyn Error>>{
//
// }

