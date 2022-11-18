use std::process::Command;
use async_std::fs;

#[async_std::main]
async fn main() {
    let arge: Vec<String> = std::env::args().collect();
    // println!("{:?}", &arge[..]);
    // 克隆路径，防止进入线程不能使用路径参数
    let srcs = arge.clone();

    let results = async_std::task::spawn(async move{
        // std::thread::sleep(std::time::Duration::from_secs(5));
        Command::new("cmd").arg("/C")
            .arg("ffmpeg")
            .arg("-i")
            .arg(format!("{} {}", srcs[1], srcs[2])) //视频路径
            .arg("-i")
            .arg(format!("{}", srcs[3])) //音频路径
            .args(["-vcodec", "copy", "-acodec", "copy"])
            .arg(format!("./{}.mp4", srcs[4])) //输出路径
            .status()
            .expect("failed to execute process");
        "Ok"
    });

    // 等待线程
    let i = results.await;
    // 不等待线程
    // let i = results;

    if i == "Ok"{
        let e1 = fs::remove_file(std::path::PathBuf::from(format!("{} {}",arge[1], arge[2]))).await;
        let e2 = fs::remove_file(std::path::PathBuf::from(format!("{}",arge[3]))).await;

        println!("File remove success: video: {:?} , audio: {:?}", e1, e2);
    };

}
