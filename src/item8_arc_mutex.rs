use std::sync::{Arc, Mutex};
use std::thread;

// 카운터를 나타내는 구조체 정의
struct Counter {
    value: Mutex<i32>,
}

fn main() {
    // Arc를 사용하여 여러 스레드에서 구조체를 공유할 수 있도록 함
    let counter = Arc::new(Counter {
        value: Mutex::new(0),
    });

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Mutex를 잠그고 카운터 값을 증가시킴
            let mut num = counter.value.lock().unwrap();
            *num += 1;
            // Mutex는 스코프를 벗어나면 자동으로 해제됨
        });
        handles.push(handle);
    }

    // 모든 스레드가 종료될 때까지 대기
    for handle in handles {
        handle.join().unwrap();
    }

    // 최종 카운터 값 출력
    println!("Result: {}", *counter.value.lock().unwrap());
}
