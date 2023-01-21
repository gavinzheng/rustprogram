// `testasync()`返回一个`Future<Output = u8>`,
// 当调用`testasync().await`时，该`Future`将被运行，调用结束后我们将获取到一个`u8`值
async fn testasync() -> u8 { 100 }  // 异步声明函数

fn bar() -> impl Future<Output = u8> {
    // 下面的`async`语句块返回`Future<Output = u8>`
    async {					// 异步代码块
        let x: u8 = testasync().await;
        x + 5
    }
}

fn main() {
  let x: u8 = bar().await;
}