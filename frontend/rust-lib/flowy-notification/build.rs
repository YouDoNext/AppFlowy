fn main() {
  #[cfg(feature = "dart")]
  flowy_codegen::protobuf_file::dart_gen(env!("CARGO_PKG_NAME"));

  #[cfg(feature = "tauri_ts")]
  flowy_codegen::protobuf_file::ts_gen(env!("CARGO_PKG_NAME"), flowy_codegen::Project::Tauri);

  #[cfg(feature = "web_ts")]
  flowy_codegen::protobuf_file::ts_gen(
    env!("CARGO_PKG_NAME"),
    flowy_codegen::Project::Web {
      relative_path: "../../".to_string(),
    },
  );
}
