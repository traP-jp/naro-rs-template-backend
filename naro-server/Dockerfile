# Rust のベースイメージを指定
FROM rust:latest

# コマンドを実行するコンテナ内のディレクトリをworkに指定
WORKDIR /work

# ローカルのカレントディレクトリをコンテナのカレントディレクトリ(work)にコピー
COPY . .

# Rust のプログラムをビルド
RUN cargo build --release

# ビルドしたものを実行
ENTRYPOINT ["./target/release/naro-server"]