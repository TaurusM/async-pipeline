#[cfg(test)]
mod tests {
    use crate::begin;
    use crate::link::{Linkable, Pipeline};
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::time::Duration;
    use tokio::time::sleep;

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
                    1 => Err(anyhow::anyhow!("something wrong")),
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
