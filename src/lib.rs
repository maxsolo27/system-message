mod windows;

/// Create message.
///
/// # Examples
///
/// ```
/// message_call("title", "description", MessageType::Ok);
/// ```

enum MessageType{
    Warn,
    Info, 
    Error,
    Ok,
    Help, 
}


