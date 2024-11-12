use std::fs;
use std::path::Path;
use std::process::Command;

#[tokio::main]
async fn main() {
    // 변환할 파일들이 있는 폴더 경로
    let input_folder_path = "D:\\일반폴더\\BackUp_IPhone\\1_heic";
    let output_folder_path = "D:\\일반폴더\\BackUp_IPhone\\converted_images";

    // 출력 폴더가 없다면 생성
    fs::create_dir_all(output_folder_path).expect("Failed to create output folder");

    // 폴더 내 모든 파일을 탐색
    for entry in fs::read_dir(input_folder_path).expect("Failed to read input folder") {
        if let Ok(entry) = entry {
            let path = entry.path();

            // .heic 파일만 처리
            if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("heic")) {
                let input_file_path = path.to_str().unwrap();
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let output_file_path = format!("{}\\{}.jpg", output_folder_path, file_stem);

                // FFmpeg 명령어 실행
                let output = Command::new("ffmpeg")
                    .arg("-i")
                    .arg(input_file_path)
                    .arg(&output_file_path)
                    .output()
                    .expect("Failed to execute FFmpeg command");

                if output.status.success() {
                    println!("{} 변환 성공!", input_file_path);
                } else {
                    eprintln!("{} 변환 실패: {:?}", input_file_path, output);
                }
            }
        }
    }
    println!("모든 변환 완료!");
}




// 5. EXE 파일로 빌드
// 러스트는 기본적으로 Windows EXE 파일을 생성할 수 있습니다. 
// 프로젝트를 빌드하여 EXE 파일을 생성합니다:

// 빌드 명령어:

// bash
// Copy code
// cargo build --release
// EXE 파일 위치: 빌드된 EXE 파일은 target/release/ 디렉터리에 생성됩니다.

// 6. 실행
// 빌드가 완료되면 target/release/heic_converter.exe 파일이 생성됩니다. 
// 이 파일을 실행하여 HEIC 파일을 변환할 수 있습니다.

// bash
// Copy code
// target/release/heic_converter.exe
// 7. EXE 파일을 배포
// EXE 파일을 다른 사용자와 공유하려면, FFmpeg 라이브러리도 함께 배포해야 합니다. 
// FFmpeg는 외부 라이브러리로, 시스템에 설치되어 있어야만 변환이 제대로 작동합니다.