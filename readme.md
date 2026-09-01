<div align="center">

# LUCKY BALL

### Rust · Bevy · Rapier3D · WebAssembly · 1인 프로젝트

**glTF 추첨 장치를 3D 물리로 구동하는 크로스플랫폼 번호 추첨 시뮬레이터**

[웹 데모](https://limits1214.github.io/lucky_ball/)

</div>

## 프로젝트 정보

| 항목 | 내용 |
| --- | --- |
| 장르 | 3D 물리 시뮬레이션 |
| 개발 기간 | 2024.10.01 ~ 2025.08.20 |
| 개발 인원 | 1명 |
| 개발자 | [limits1214](https://github.com/limits1214) · 임성윤 |
| 플랫폼 구성 | Desktop · Web · Android · iOS |
| 공개 배포 | WebGL2 · GitHub Pages |
| 기술 스택 | • Rust 2021<br>• Bevy 0.15.1 · Rapier3D 0.28<br>• Bevy Tweening · Asset Loader · Bevy UI · glTF<br>• WebAssembly · wasm-bindgen · JNI · Swift/C FFI<br>• GitHub Actions · GitHub Pages |

## 시연 영상

https://github.com/user-attachments/assets/0bb79a4e-2427-4ce8-94ce-7e7ff850e20e

## 주요 콘텐츠

- 6/45, 5/69, 1/26으로 구성된 독립 번호 추첨 프리셋
- 1–70번 공의 포함 여부와 1–20개 추첨 수를 설정하는 사용자 지정 규칙
- 공 투입 → 혼합 → 포착 → 배출로 이어지는 물리 기반 추첨 과정
- 추첨 결과의 번호·규칙·시각 저장과 목록 조회·페이지 이동·삭제
- 사용자 지정 규칙과 추첨 결과를 유지하는 플랫폼별 영구 저장

## 구현 내용

### 물리 기반 추첨 시스템

- glTF 모델의 공·믹서·추첨 스틱·출구 덮개를 Rapier3D 강체와 콜라이더로 구성
- 센서 충돌과 상태 이벤트로 공의 포착·선택·배출 및 추첨 종료 조건 판정
- Tweening과 GameStep 상태를 결합해 장치 동작과 추첨 단계를 순차 제어

### UI 및 데이터

- Bevy ECS의 Component·Resource·Event 구조로 게임 로직과 UI 흐름 분리
- 프리셋 선택·사용자 규칙 편집·추첨 결과·저장 번호 목록 UI 구현
- UUID와 추첨 시각을 포함한 결과 모델 및 페이지 단위 조회·삭제 처리

### 플랫폼 구조

- 공통 Rust 코어와 플랫폼별 FFI 계층을 분리해 동일한 게임 로직 재사용
- Web localStorage, Desktop 파일, Android SharedPreferences, iOS UserDefaults 저장소 연결
- WebAssembly 릴리스 빌드와 GitHub Pages 자동 배포 워크플로 구성

Android와 iOS 디렉터리는 플랫폼별 빌드·저장소 연동 구성을 포함하며, 공개 웹 데모는 WebGL2로 실행됩니다.
