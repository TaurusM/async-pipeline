#[cfg(test)]
mod tests {
    use crate::begin;
    use crate::link::{Linkable, Pipeline};
    use serde::{Deserialize, Serialize};
    use std::fmt::Display;
    use std::fs;
    use std::time::Duration;
    use tokio::time::sleep;

    #[derive(Debug, Clone)]
    pub struct TestError {
        pub code: String,
        pub message: Option<String>,
    }

    impl Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "response error in app",)
        }
    }

    impl std::error::Error for TestError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(self)
        }

        fn description(&self) -> &str {
            "description() is deprecated; use Display"
        }

        fn cause(&self) -> Option<&dyn std::error::Error> {
            self.source()
        }
    }

    #[tokio::test]
    async fn test_pipeline() {
        let p = begin::<i32>()
            .then_result(|x| match x % 2 {
                0 => Ok(x),
                1 => Err(std::io::Error::from_raw_os_error(x).into()),
                _ => {
                    panic!("no way")
                }
            })
            .then(|x| {
                log::info!("{} pass by", x);
                x
            })
            .then_async_result(|x| async move {
                match x % 3 {
                    0 => Ok(x),
                    1 => Err("something wrong".into()),
                    _ => {
                        panic!("no way")
                    }
                }
            })
            .then_async(|x| async move {
                log::info!("{} pass by async", x);
                sleep(Duration::from_secs(1)).await;
                x
            });
        let v = p.process(12).await.unwrap();
        assert_eq!(v, 12);
        let v = p.process(11).await.err().unwrap();
        log::info!("err: {}", v);
        let v = p.process(10).await.err().unwrap();
        log::info!("err: {}", v);
    }

    #[tokio::test]
    async fn handle_differen_err() {
        #[derive(Serialize, Deserialize)]
        struct Fire {
            a: i32,
            b: String,
        }
        let p = begin()
            .then_async_result(|x: i32| async move {
                let _: Fire = serde_json::from_str("adb")?;
                let _ = fs::read("nonono")?;
                Ok(x)
            })
            .then_result(|x| {
                let _: Fire = serde_json::from_str("adb")?;
                let _ = fs::read("nonono")?;
                Ok(x)
            });
        let r = p.process(10).await;
        println!("{}", r.err().unwrap())
    }
}
