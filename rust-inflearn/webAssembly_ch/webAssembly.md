# WEBASSEMBLY
> 웹어셈블리는 스택기반 가상기계를 위한 바이너리 명령어 포맷으로 클라이언트와 서버 응용프로그램을 웹에 배포할 수 있다.  
> 자바스크립트는 text 소스 파일로 바로 해석하면서 실행하지만 
> WEBASSEMBLY는 마치 로컬에 있는 바이너리 실행파일처럼 컴팩트한 바이너리 포맷을 바로 실행할 수 있다.
> 
## WEBASSEMBLY WASM 
* 대부분의 웹 브라우저에서 지원
* 자바스크립트가 실행되듯, WASM 바이너리 실행 가능
* 네이티브 수준의 빠른 성능
* 다양한 언어로 작성한 코드를 컴파일해서 WASM 바이너리 생성
* Rust 코드로도 WASM 바이너리 훌륭히 생성 가능

## Rust의 WEBASSEMBLY
> Garbage Collection이 없기에 성능이 예측 가능하다
> 코드가 매우 작다
> 문서와 여러 도구가 현대적이라 개발이 편하다