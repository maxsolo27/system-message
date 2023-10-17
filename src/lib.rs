use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
};
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

pub enum MessageType{
    Warn,
    Info, 
    Error,
    Ok,
    Help, 
}

pub struct MessageCall{
    title: &'static str,
    description: &'static str,
    type_msg: MessageType
}

impl MessageCall{
    pub fn create(&self){
        let title: Vec<u16> = OsStr::new(&self.title).encode_wide().chain(once(0)).collect();
        let description: Vec<u16> = OsStr::new(&self.description).encode_wide().chain(once(0)).collect();
        
        let message: MESSAGEBOX_STYLE = match &self.type_msg{
            MessageType::Error => MB_ICONERROR,
            MessageType::Warn => MB_ICONWARNING,
            MessageType::Info => MB_ICONINFORMATION,
            MessageType::Ok => MB_OK,
            MessageType::Help => MB_ICONQUESTION,
        };
    
        unsafe {
            MessageBoxW(None, PCWSTR::from_raw(description.as_ptr() as *const u16), PCWSTR::from_raw(title.as_ptr() as *const u16), message);
        }
    }
}

pub fn message_call(title: &str, description: &str, type_message: MessageType){
    let title: Vec<u16> = OsStr::new(title).encode_wide().chain(once(0)).collect();
    let description: Vec<u16> = OsStr::new(description).encode_wide().chain(once(0)).collect();
    
    let message: MESSAGEBOX_STYLE = match type_message{
        MessageType::Error => MB_ICONERROR,
        MessageType::Warn => MB_ICONWARNING,
        MessageType::Info => MB_ICONINFORMATION,
        MessageType::Ok => MB_OK,
        MessageType::Help => MB_ICONQUESTION,
    };

    unsafe {
        MessageBoxW(None, PCWSTR::from_raw(description.as_ptr() as *const u16), PCWSTR::from_raw(title.as_ptr() as *const u16), message);
    }
}
