# Steam Pack

이 저장소는 러스트(Rust) 학습을 위한 간단한 예제 모음입니다. 각 파일이 독립적인 `main()` 함수를 가지고 있어 특정 기능을 실습해 볼 수 있습니다.

## 디렉터리 구조

```
steam-pack/
├── Cargo.toml
├── Cargo.lock
└── src/
    ├── item1_closure.rs
    ├── item2_myerror.rs
    ├── item3_default.rs
    ├── item4_fatpoint.rs
    ├── item5_iterator.rs
    ├── item6_generic_vs_trait_object.rs
    ├── item7_partialord.rs
    └── item8_arc_mutex.rs
```

## 주요 내용

- **item1_closure.rs** – 제네릭 타입에 클로저를 적용하는 예제
- **item2_myerror.rs** – 사용자 정의 오류 타입과 `Result` 사용 예제
- **item3_default.rs** – `Default` 파생 구현을 활용하는 구조체 예제
- **item4_fatpoint.rs** – (현재 내용 없음)
- **item5_iterator.rs** – 반복자 메서드 체인을 이용한 예제
- **item6_generic_vs_trait_object.rs** – 제네릭과 트레이트 객체 비교 예제 (중복된 함수명이 있어 그대로는 컴파일되지 않을 수 있음)
- **item7_partialord.rs** – `PartialOrd`, `PartialEq` 사용 예제
- **item8_arc_mutex.rs** – 여러 스레드에서 `Arc`와 `Mutex`로 값 공유하기 예제

## 실행 방법

각 파일이 독립적인 바이너리이므로 Cargo 설정에는 별도의 `[[bin]]` 항목이 없습니다. 예제를 실행하려면 다음 중 한 가지 방법을 사용할 수 있습니다.

1. **`rustc` 사용**
   ```bash
   rustc src/item1_closure.rs && ./item1_closure
   ```
   단, 의존성이 있는 경우에는 `cargo`를 이용하여 빌드하는 편이 편리합니다.

2. **Cargo의 `[[bin]]` 설정 추가**
   `Cargo.toml`에 바이너리를 등록하면 `cargo run --bin <name>` 형식으로 실행할 수 있습니다. (예: `item1_closure` 등)

## 다음 단계

- 각 예제에 대한 설명과 실행 방법을 README에 추가하거나, `examples/` 디렉터리로 이동하여 Cargo에서 쉽게 실행할 수 있도록 개선해 보세요.
- 일부 파일은 완성되지 않았거나 컴파일 에러가 발생할 수 있으니 확인 후 수정이 필요합니다.
- 테스트 코드 작성 및 모듈화 방법을 검토해 보는 것도 좋습니다.

러스트 학습에 도움이 되기를 바랍니다.
