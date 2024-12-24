use fastcgi::Request;
use std::net::TcpListener;
use std::io::Write;

fn main() {
    // 9000番ポートで待ち受け
    let listener = TcpListener::bind("0.0.0.0:9000").expect("9000番ポートのバインドに失敗");
    println!("FastCGI server listening on port 9000...");

    fastcgi::run_tcp(
        
        |mut req: Request| {
            // リクエストごとに呼ばれる処理
            // 出力ストリームへ書き込み
            write!(
                &mut req.stdout(),
                "Content-Type: text/plain\r\n\r\nHello, FastCGI with Rust!"
            )
            // エラー時の処理
            .unwrap_or_else(|err| eprintln!("Error writing response: {}", err));
        },
        &listener,
    );
}