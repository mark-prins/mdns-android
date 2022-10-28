use std::{thread, time::Duration};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    println!("Hello World");

    let responder = libmdns::Responder::new().unwrap();
    let _svc = responder.register(
        "_http._tcp".to_owned(),
        "libmdns Web Server".to_owned(),
        80,
        &["path=/"],
    );

    loop {
        thread::sleep(Duration::from_secs(100));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
