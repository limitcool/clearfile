use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
fn main() {
        // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry()
    .with(fmt::layer())
    .init();
    let files = get_files("./".to_string());
    for file in files {
        let ok = fs::metadata(file.clone()).unwrap().modified().unwrap();
        let since_the_epoch = ok.duration_since(UNIX_EPOCH).unwrap();
        let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
        if  timestamp() -ms > 86400_000{
            // println!("移除文件:{}", file);
            tracing::info!("移除文件:{}", file);
            fs::remove_file(file).unwrap();
        }
        
    }
    tracing::info!("移除任务完成");
}

fn get_files(path:String)-> Vec<String>{
        // vec.drain_filter drain的意思是 排出 的意思，所以这个函数就是排出过滤器，接收一个回调函数，然后把回调函数里面返回true的元素就会排出，自然也就从原本的vec里面删除掉了。然后有需要的话还可以搜集排出的元素。
        // 读取当前目录下的所有文件
        let paths = fs::read_dir(path).unwrap();
        let mut files: Vec<String> = Vec::new();
        for path in paths {
            files.push(path.unwrap().path().display().to_string());
        }
        // 去除数组中的元素,只保留.mp4结尾的文件
        files.retain(|x| x.ends_with(".mp4") || x.ends_with(".MP4") );
        return files

}

fn timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}
