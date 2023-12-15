/// get_ip
/// 
/// Returns the IP Address of the current machine
/// Uses `ifconfig` to get the IP Address, so works only on Linux
/// 
/// # Returns
/// 
/// Returns the IP Address of the current machine as an `Option<String>`
/// 
/// # Example
/// 
/// ```
/// use ishan::get_ip;
/// 
/// let ip = get_ip().unwrap_or("NO IP".to_string());
/// 
/// println!("IP Address: {}", ip);
/// ```
/// 
/// # Panics
/// 
/// Panics if `ifconfig` is not installed on the system
pub fn get_ip()->Option<String>{
    ip_extractor::get_wlan(Some("wl")).first().unwrap().clone().inet
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ip() {
        assert!(get_ip().is_some())
    }
}
